// Standalone reproduction for bug-hunt candidate 014 (vello issue #1198):
// a clip-only layer (`Scene::push_clip_layer`, the current replacement for the removed
// `Mix::Clip`) is documented/expected to behave like `Scene::push_layer(.., Mix::Normal, ..)`
// while allowing the renderer to skip creating an isolated blend group - but the renderer never
// actually implements that skip, so a real blend mode (e.g. `Mix::Multiply`) nested directly
// inside a `push_clip_layer` sees a transparent isolated backdrop instead of the genuine one,
// producing wrong (unblended) output across most of the clipped shape.
//
// This renders two scenes through vello's real public API (`Scene::push_layer` /
// `Scene::push_clip_layer` + `Renderer::render_to_texture`, driven by a real wgpu device):
//
//   - "no_clip": the exact same content, but using only a single, direct blend layer
//     (`push_layer(.., Mix::Multiply, .., &triangle)`) with no enclosing clip-only layer at all.
//     This is an unambiguous ground truth: with only one layer in play there is no isolation
//     question to get wrong, so the Multiply blend can only ever see the real backdrop.
//   - "clip": `push_clip_layer(.., &triangle)` wrapping an inner `push_layer(.., Mix::Multiply,
//     .., &triangle)`, i.e. exactly the scene from
//     https://github.com/linebender/vello/issues/1198 and from this project's own
//     `vello_tests/tests/known_issues.rs::clip_blends` regression test (which currently passes
//     only because its checked-in reference snapshot was captured with this bug already present).
//
// Per the documented "Clip behaves like Normal" contract, and per the "make Clip act like no
// clip at all" resolution direction the maintainers settled on for issue #1198 (see the issue
// body's own discussion), these two scenes should render identically. They currently do not.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (the two scenes differ by
// more than a small antialiasing-edge tolerance).
//
// This environment has no real GPU; set `WGPU_BACKEND=gl` (and, if needed,
// `LIBGL_ALWAYS_SOFTWARE=1`) to use Mesa's llvmpipe software rasterizer over the GL backend - see
// this candidate's `justfile` for the exact invocation. Any working wgpu backend
// (Vulkan/Metal/DX12/GL) exercises the same real GPU coarse/fine rasterization shaders.

use std::num::NonZeroUsize;

use anyhow::{Result, anyhow, bail};
use vello::kurbo::{Affine, Rect, Triangle};
use vello::peniko::color::palette;
use vello::peniko::{Fill, Mix};
use vello::util::{RenderContext, block_on_wgpu};
use vello::wgpu::{
    self, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, TexelCopyBufferInfo,
    TextureDescriptor, TextureFormat, TextureUsages,
};
use vello::{RendererOptions, Scene};

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

fn build_scene(use_clip_layer: bool) -> Scene {
    let rect = Rect::from_origin_size((0., 0.), (100., 100.));
    let triangle = Triangle::from_coords((50., 0.), (0., 100.), (100., 100.));

    let mut scene = Scene::new();
    scene.fill(
        Fill::EvenOdd,
        Affine::IDENTITY,
        palette::css::BLUE,
        None,
        &rect,
    );
    if use_clip_layer {
        // The current public replacement for the old `Mix::Clip`: a clip-only layer that is
        // documented to behave like `Mix::Normal` while allowing isolation to be skipped.
        scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &triangle);
    }
    scene.push_layer(
        Fill::NonZero,
        Mix::Multiply,
        1.0,
        Affine::IDENTITY,
        &triangle,
    );
    scene.fill(
        Fill::EvenOdd,
        Affine::IDENTITY,
        palette::css::AQUAMARINE,
        None,
        &rect,
    );
    scene.pop_layer();
    if use_clip_layer {
        scene.pop_layer();
    }
    scene
}

async fn render(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    renderer: &mut vello::Renderer,
    scene: &Scene,
) -> Result<Vec<u8>> {
    let render_params = vello::RenderParams {
        base_color: palette::css::BLACK,
        width: WIDTH,
        height: HEIGHT,
        antialiasing_method: vello::AaConfig::Area,
    };
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
        .render_to_texture(device, queue, scene, &view, &render_params)
        .or_else(|_| bail!("Got non-Send/Sync error from rendering"))?;

    let padded_byte_width = (WIDTH * 4).next_multiple_of(256);
    let buffer_size = padded_byte_width as u64 * HEIGHT as u64;
    let buffer = device.create_buffer(&BufferDescriptor {
        label: Some("val"),
        size: buffer_size,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Copy out buffer"),
    });
    encoder.copy_texture_to_buffer(
        target.as_image_copy(),
        TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_byte_width),
                rows_per_image: None,
            },
        },
        size,
    );
    queue.submit([encoder.finish()]);
    let buf_slice = buffer.slice(..);

    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buf_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    if let Some(recv_result) = block_on_wgpu(device, receiver.receive()) {
        recv_result?;
    } else {
        bail!("channel was closed");
    }

    let data = buf_slice.get_mapped_range();
    let mut result_unpadded = Vec::<u8>::with_capacity((WIDTH * HEIGHT * 4) as usize);
    for row in 0..HEIGHT {
        let start = (row * padded_byte_width) as usize;
        result_unpadded.extend(&data[start..start + (WIDTH * 4) as usize]);
    }
    drop(data);
    buffer.unmap();
    Ok(result_unpadded)
}

fn write_png(path: &str, data: &[u8]) -> Result<()> {
    let mut file = std::fs::File::create(path)?;
    let mut png_encoder = png::Encoder::new(&mut file, WIDTH, HEIGHT);
    png_encoder.set_color(png::ColorType::Rgba);
    png_encoder.set_depth(png::BitDepth::Eight);
    let mut writer = png_encoder.write_header()?;
    writer.write_image_data(data)?;
    writer.finish()?;
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();
    pollster::block_on(run())
}

async fn run() -> Result<()> {
    let mut context = RenderContext::new();
    let device_id = context
        .device(None)
        .await
        .ok_or_else(|| anyhow!("No compatible device found"))?;
    let device_handle = &mut context.devices[device_id];
    let device = &device_handle.device;
    let queue = &device_handle.queue;
    println!("Using adapter: {:?}", device_handle.adapter().get_info());

    let mut renderer = vello::Renderer::new(
        device,
        RendererOptions {
            use_cpu: false,
            num_init_threads: NonZeroUsize::new(1),
            antialiasing_support: vello::AaSupport::area_only(),
            ..Default::default()
        },
    )
    .or_else(|_| bail!("Got non-Send/Sync error from creating renderer"))?;

    let no_clip_scene = build_scene(false);
    let clip_scene = build_scene(true);

    let no_clip_pixels = render(device, queue, &mut renderer, &no_clip_scene).await?;
    let clip_pixels = render(device, queue, &mut renderer, &clip_scene).await?;

    write_png("/tmp/bughunt_1198_no_clip.png", &no_clip_pixels)?;
    write_png("/tmp/bughunt_1198_clip.png", &clip_pixels)?;
    println!("Wrote results to /tmp/bughunt_1198_no_clip.png and /tmp/bughunt_1198_clip.png");

    // Per-pixel comparison. A small tolerance (8/255) absorbs ordinary antialiasing-edge
    // rounding differences between the two rasterization paths; the actual bug produces a
    // dramatic difference (unblended source color instead of a Multiply blend) across most of
    // the interior of the clipped triangle, far past this tolerance.
    const TOLERANCE: i32 = 16;
    let mut bad_pixels = 0usize;
    let mut max_diff = 0i32;
    for (a, b) in no_clip_pixels.iter().zip(clip_pixels.iter()) {
        let d = (*a as i32 - *b as i32).abs();
        max_diff = max_diff.max(d);
        if d > TOLERANCE {
            bad_pixels += 1;
        }
    }
    let total_channels = no_clip_pixels.len();
    println!(
        "bad_pixels(channels)={bad_pixels} / {total_channels}, max_channel_diff={max_diff}"
    );

    if bad_pixels > 50 {
        eprintln!(
            "REPRODUCED: push_clip_layer + nested Mix::Multiply differs from the no-clip \
             ground truth in {bad_pixels} channel samples (max diff {max_diff}/255) - the \
             clip-only layer is still creating a real isolated blend group instead of letting \
             the nested Multiply blend see the genuine backdrop."
        );
        std::process::exit(1);
    }
    println!("did not reproduce - clip and no-clip renders match within tolerance");
    Ok(())
}
