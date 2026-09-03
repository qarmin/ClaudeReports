// Standalone reproduction for bug-hunt candidate 011 (vello issue #1063):
// `Cap::Butt` + `Join::Miter` renders a hole at a sharp stroke corner.
//
// This renders the *exact* scene from the issue through vello's real public API
// (`Scene::stroke` + `Renderer::render_to_texture`, driven by a real wgpu device), reads the
// resulting texture back to the CPU, and checks whether the analytically-guaranteed-filled
// region of the stroked path is actually fully covered.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (a black/background pixel
// was found strictly inside the region that must be filled regardless of join geometry).
//
// This environment has no real GPU; set `WGPU_BACKEND=gl` (and, if needed,
// `LIBGL_ALWAYS_SOFTWARE=1`) to use Mesa's llvmpipe software rasterizer over the GL backend - see
// this candidate's `justfile`/README for the exact invocation. Any working wgpu backend
// (Vulkan/Metal/DX12/GL) exercises the same real GPU compute-shader stroke-expansion code path.

use std::num::NonZeroUsize;

use anyhow::{Result, anyhow, bail};
use vello::kurbo::{Affine, BezPath, Cap, Join, Point, Stroke};
use vello::peniko::Color;
use vello::peniko::color::palette;
use vello::util::{RenderContext, block_on_wgpu};
use vello::wgpu::{
    self, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, TexelCopyBufferInfo,
    TextureDescriptor, TextureFormat, TextureUsages,
};
use vello::{RendererOptions, Scene};

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

    // The exact repro from https://github.com/linebender/vello/issues/1063, built through
    // vello's real public `Scene` API.
    let mut scene = Scene::new();
    let stroke = Stroke {
        width: 50.0,
        join: Join::Miter,
        miter_limit: 10.0,
        start_cap: Cap::Butt,
        end_cap: Cap::Butt,
        ..Default::default()
    };
    let mut path = BezPath::new();
    path.move_to(Point::new(110., 50.));
    path.line_to(Point::new(110., 60.));
    path.line_to(Point::new(100., 60.));
    scene.stroke(&stroke, Affine::IDENTITY, Color::WHITE, None, &path);

    let width = 200_u32;
    let height = 200_u32;
    let render_params = vello::RenderParams {
        base_color: palette::css::BLACK,
        width,
        height,
        antialiasing_method: vello::AaConfig::Area,
    };

    let size = Extent3d {
        width,
        height,
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
        .render_to_texture(device, queue, &scene, &view, &render_params)
        .or_else(|_| bail!("Got non-Send/Sync error from rendering"))?;

    let padded_byte_width = (width * 4).next_multiple_of(256);
    let buffer_size = padded_byte_width as u64 * height as u64;
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
    let mut result_unpadded = Vec::<u8>::with_capacity((width * height * 4) as usize);
    for row in 0..height {
        let start = (row * padded_byte_width) as usize;
        result_unpadded.extend(&data[start..start + (width * 4) as usize]);
    }
    drop(data);
    buffer.unmap();

    let out_path = "/tmp/bughunt_1063_repro.png";
    let mut file = std::fs::File::create(out_path)?;
    let mut png_encoder = png::Encoder::new(&mut file, width, height);
    png_encoder.set_color(png::ColorType::Rgba);
    png_encoder.set_depth(png::BitDepth::Eight);
    let mut writer = png_encoder.write_header()?;
    writer.write_image_data(&result_unpadded)?;
    writer.finish()?;
    println!("Wrote result ({width}x{height}) to {out_path}");

    let get_px = |x: u32, y: u32| -> u8 {
        let idx = ((y * width + x) * 4) as usize;
        result_unpadded[idx]
    };

    println!("ASCII dump, rows y=30..90, cols x=75..145 ('#'=bright, '.'=dark):");
    for y in 30..90u32 {
        let mut row = String::new();
        for x in 75..145u32 {
            row.push(if get_px(x, y) > 128 { '#' } else { '.' });
        }
        println!("{row}");
    }

    // Analytical ground truth: the two stroked segments are axis-aligned rectangles
    // (Cap::Butt means no extension past the segment endpoints). Their union is the
    // unambiguous correct fill for this input regardless of which join algorithm is used,
    // since a 90-degree corner's miter point coincides exactly with the rectangle corner.
    // Segment A (vertical, (110,50)-(110,60), half-width 25): x in [85,135], y in [50,60].
    // Segment B (horizontal, (110,60)-(100,60), half-width 25): x in [100,110], y in [35,85].
    // Sampled at pixel centers against inclusive float bounds so a pixel whose center sits
    // exactly on an analytic edge (legitimately partially covered by antialiasing there) isn't
    // miscounted as a "hole".
    let in_rect_a = |x: f64, y: f64| (85.0..=135.0).contains(&x) && (50.0..=60.0).contains(&y);
    let in_rect_b = |x: f64, y: f64| (100.0..=110.0).contains(&x) && (35.0..=85.0).contains(&y);

    let mut holes = Vec::new();
    for y in 35..85u32 {
        for x in 85..135u32 {
            let (xf, yf) = (x as f64 + 0.5, y as f64 + 0.5);
            if !(in_rect_a(xf, yf) || in_rect_b(xf, yf)) {
                continue;
            }
            let r = get_px(x, y);
            if r < 128 {
                holes.push((x, y, r));
            }
        }
    }

    println!("holes found inside guaranteed-fill union: {}", holes.len());
    for h in holes.iter().take(20) {
        println!("  {h:?}");
    }
    if !holes.is_empty() {
        eprintln!(
            "REPRODUCED: {} pixel(s) inside the guaranteed-fill region are unfilled",
            holes.len()
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - guaranteed-fill region fully covered");
    }
    Ok(())
}
