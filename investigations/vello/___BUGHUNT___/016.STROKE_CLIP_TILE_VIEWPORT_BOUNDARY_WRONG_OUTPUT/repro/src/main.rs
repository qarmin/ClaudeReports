// Reproduction for https://github.com/linebender/vello/issues/1483
//
// "Stroking a rect on window boundary": the reporter's exact replacement for
// `add_shapes_to_scene` in `examples/simple`, run through vello's real public rendering API
// (`Renderer::render_to_texture`, the same entry point `examples/simple` itself calls), at the
// same window size (1044x800) and antialiasing mode (Msaa16) the `simple` example actually uses.
//
// The reported bug is a single-artifact input: a malformed *rendered result*, not a second
// participant's behavior - so a single-process headless render through the real public API is a
// complete reproduction path (see CONFIRMATION.md section 11's "single external artifact, no
// second participant" carve-out).
//
// Oracle: the rect `Rect::from_origin_size((113.5, 0.5), (103.0, 61.0))` stroked with width 1.0
// produces a 1px-wide outline whose stroked extent is exactly pixel-aligned (half the stroke
// width lands exactly on integer pixel boundaries), so every lit (non-background) pixel must lie
// within the outline's bounding box: x in [113, 217), y in [0, 62). Any non-background pixel
// outside that box is exactly the kind of stray-artifact defect the issue describes.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

use vello::kurbo::{Affine, Rect, Stroke};
use vello::peniko::Color;
use vello::wgpu::{
    self, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, TexelCopyBufferInfo,
    TextureDescriptor, TextureFormat, TextureUsages,
};
use vello::util::block_on_wgpu;
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene};

const WIDTH: u32 = 1044;
const HEIGHT: u32 = 800;

// The outline's stroked extent, in pixels: x in [OUT_X0, OUT_X1), y in [OUT_Y0, OUT_Y1).
const OUT_X0: u32 = 113;
const OUT_X1: u32 = 217;
const OUT_Y0: u32 = 0;
const OUT_Y1: u32 = 62;

fn build_scene() -> Scene {
    // Verbatim reporter repro (the exact replacement given for `add_shapes_to_scene` in
    // `examples/simple/src/main.rs`).
    let mut scene = Scene::new();
    let rect = Rect::from_origin_size((113.5, 0.5), (103.0, 61.0));
    scene.stroke(
        &Stroke::new(1.0),
        Affine::IDENTITY,
        Color::from_rgb8(255, 0, 0),
        None,
        &rect,
    );
    scene
}

async fn render(scene: &Scene) -> Vec<u8> {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("no compatible wgpu adapter found");
    println!("Using adapter: {:?}", adapter.get_info());
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("failed to create device");

    let mut renderer = Renderer::new(
        &device,
        RendererOptions {
            use_cpu: false,
            num_init_threads: std::num::NonZeroUsize::new(1),
            antialiasing_support: std::iter::once(AaConfig::Msaa16).collect(),
            pipeline_cache: None,
        },
    )
    .expect("failed to create renderer");

    let size = Extent3d {
        width: WIDTH,
        height: HEIGHT,
        depth_or_array_layers: 1,
    };
    let target = device.create_texture(&TextureDescriptor {
        label: Some("Target texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: TextureFormat::Rgba8Unorm,
        usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = target.create_view(&wgpu::TextureViewDescriptor::default());

    renderer
        .render_to_texture(
            &device,
            &queue,
            scene,
            &view,
            &RenderParams {
                base_color: vello::peniko::color::palette::css::BLACK,
                width: WIDTH,
                height: HEIGHT,
                antialiasing_method: AaConfig::Msaa16,
            },
        )
        .expect("render_to_texture failed");

    // Read the rendered texture back to a CPU buffer.
    let bytes_per_pixel = 4;
    let unpadded_bytes_per_row = WIDTH * bytes_per_pixel;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;

    let buffer = device.create_buffer(&BufferDescriptor {
        label: Some("Readback buffer"),
        size: (padded_bytes_per_row * HEIGHT) as u64,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Copy out"),
    });
    encoder.copy_texture_to_buffer(
        target.as_image_copy(),
        TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: None,
            },
        },
        size,
    );
    queue.submit([encoder.finish()]);

    let slice = buffer.slice(..);
    let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
    slice.map_async(wgpu::MapMode::Read, move |res| {
        tx.send(res).ok();
    });
    block_on_wgpu(&device, rx.receive())
        .expect("channel closed")
        .expect("failed to map buffer");

    let data = slice.get_mapped_range();
    let mut out = vec![0_u8; (unpadded_bytes_per_row * HEIGHT) as usize];
    for row in 0..HEIGHT {
        let src_start = (row * padded_bytes_per_row) as usize;
        let dst_start = (row * unpadded_bytes_per_row) as usize;
        out[dst_start..dst_start + unpadded_bytes_per_row as usize]
            .copy_from_slice(&data[src_start..src_start + unpadded_bytes_per_row as usize]);
    }
    out
}

fn main() {
    let scene = build_scene();
    let pixels = pollster::block_on(render(&scene));

    let mut stray = Vec::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = ((y * WIDTH + x) * 4) as usize;
            let (r, g, b) = (pixels[idx], pixels[idx + 1], pixels[idx + 2]);
            if r == 0 && g == 0 && b == 0 {
                continue;
            }
            let in_bounds = (OUT_X0..OUT_X1).contains(&x) && (OUT_Y0..OUT_Y1).contains(&y);
            if !in_bounds {
                stray.push((x, y, r, g, b));
            }
        }
    }

    if stray.is_empty() {
        println!(
            "did not reproduce - all non-background pixels lie within the expected outline bounding box x=[{OUT_X0},{OUT_X1}) y=[{OUT_Y0},{OUT_Y1})"
        );
    } else {
        eprintln!(
            "REPRODUCED - {} stray non-background pixel(s) outside the expected outline bounding box:",
            stray.len()
        );
        for (x, y, r, g, b) in stray.iter().take(20) {
            eprintln!("  ({x}, {y}) = rgb({r}, {g}, {b})");
        }
        std::process::exit(1);
    }
}
