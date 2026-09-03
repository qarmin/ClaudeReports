// Standalone reproduction for bug-hunt candidate 020 (vello issue #417's underlying mechanism):
// fine.wgsl's `fill_path` analytic-area formula subtracts a fixed `1.0e-6` fudge from `xmin` to
// avoid a 0/0 division for an exactly-vertical edge. For an edge that is *almost* but not exactly
// vertical (a per-scanline horizontal span on the order of 1e-9..1e-6 device pixels - entirely
// plausible for any nearly-vertical edge, e.g. a very slightly non-axis-aligned rectangle side),
// the fudge combines with f32 catastrophic cancellation in `d*d - c*c` to produce a coverage value
// that is off by several percent, and - because the amount of f32 rounding differs row-to-row even
// though the true edge position barely moves - this shows up as a persistent alternating banding
// pattern along what should be a perfectly smooth, constant-coverage edge.
//
// This renders a real filled path (a tall rectangle whose left edge is sheared by a few
// millionths of a device pixel over its height - i.e. an edge that is visually indistinguishable
// from perfectly vertical) through vello's real public API (`Scene::fill` +
// `Renderer::render_to_texture`, driven by a real wgpu device), reads the resulting texture back
// to the CPU, and checks whether the antialiased coverage along that edge is the smooth constant
// it mathematically must be, or shows the banding artifact.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (persistent banding found).
//
// This environment has no real GPU; set `WGPU_BACKEND=gl` (and, if needed,
// `LIBGL_ALWAYS_SOFTWARE=1`) to use Mesa's llvmpipe software rasterizer over the GL backend - see
// this candidate's `justfile`/README for the exact invocation. Any working wgpu backend
// (Vulkan/Metal/DX12/GL) exercises the exact same real WGSL `fill_path` shader code.

use std::num::NonZeroUsize;

use anyhow::{Result, anyhow, bail};
use vello::kurbo::{Affine, BezPath, Point};
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

    // A tall rectangle whose left edge is sheared by DX_TOTAL over HEIGHT device pixels - a slope
    // of DX_TOTAL/HEIGHT ~= 2e-8, i.e. an edge that no user could visually distinguish from
    // perfectly vertical (over 200 device pixels of height it drifts by 4 millionths of a pixel).
    // x=0.3 puts the edge at a fractional pixel offset inside device-space column 0, so the left
    // 0..1 pixel column has a well-defined, and here provably near-constant, correct coverage.
    const HEIGHT: f64 = 200.0;
    const DX_TOTAL: f64 = 4e-6;
    const X0: f64 = 0.3;

    let mut path = BezPath::new();
    path.move_to(Point::new(X0, 0.0));
    path.line_to(Point::new(X0 + 50.0, 0.0));
    path.line_to(Point::new(X0 + 50.0 + DX_TOTAL, HEIGHT));
    path.line_to(Point::new(X0 + DX_TOTAL, HEIGHT));
    path.close_path();

    let mut scene = Scene::new();
    scene.fill(
        vello::peniko::Fill::NonZero,
        Affine::IDENTITY,
        Color::WHITE,
        None,
        &path,
    );

    let width = 8_u32;
    let height = HEIGHT as u32 + 10;
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

    let out_path = "/tmp/bughunt_417_repro.png";
    let mut file = std::fs::File::create(out_path)?;
    let mut png_encoder = png::Encoder::new(&mut file, width, height);
    png_encoder.set_color(png::ColorType::Rgba);
    png_encoder.set_depth(png::BitDepth::Eight);
    let mut writer = png_encoder.write_header()?;
    writer.write_image_data(&result_unpadded)?;
    writer.finish()?;
    println!("Wrote result ({width}x{height}) to {out_path}");

    let get_r = |x: u32, y: u32| -> u8 {
        let idx = ((y * width + x) * 4) as usize;
        result_unpadded[idx]
    };

    // Column x=0 sits entirely inside the left edge's own [0,1) pixel span (the shape's right
    // edge is at x ~= 50.3, far from this column). The true, mathematically-correct coverage
    // there is the constant `1.0 - X0` (~0.7, i.e. ~178/255) for every row: the edge's real slope
    // (DX_TOTAL/HEIGHT ~= 2e-8) moves it by less than 1e-5 pixels total across the whole height,
    // nowhere near enough to change the rounded 8-bit pixel value even once.
    let expected_frac = 1.0 - X0.fract();
    let expected_u8 = (expected_frac * 255.0).round() as i32;
    println!("expected (true, constant) column-0 pixel value: {expected_u8}");

    // Skip the first/last few rows (segment endpoint / tile-row clamping edge effects), sample a
    // long inner run of rows.
    let sample_rows: Vec<u32> = (5..(HEIGHT as u32 - 5)).collect();
    let values: Vec<i32> = sample_rows.iter().map(|&y| get_r(0, y) as i32).collect();

    println!("row  pixel  diff-from-expected");
    for (y, v) in sample_rows.iter().zip(values.iter()) {
        println!("{y:4} {v:5} {:+4}", v - expected_u8);
    }

    // Detect persistent banding: count how many rows deviate from the expected constant by more
    // than a small tolerance (2, i.e. within ordinary 8-bit rounding noise), and how many
    // consecutive-row jumps of at least 2 levels occur - a smooth, correctly-antialiased edge with
    // a real slope this tiny cannot legitimately produce either.
    let tol = 2;
    let off_by_more_than_tol = values.iter().filter(|&&v| (v - expected_u8).abs() > tol).count();
    let mut jump_count = 0;
    for w in values.windows(2) {
        if (w[1] - w[0]).abs() >= 2 {
            jump_count += 1;
        }
    }
    let max_dev = values.iter().map(|&v| (v - expected_u8).abs()).max().unwrap_or(0);

    println!();
    println!(
        "rows sampled: {}, rows off by >{tol} from expected constant: {off_by_more_than_tol}, \
         row-to-row jumps >=2 levels: {jump_count}, max deviation from expected: {max_dev}",
        values.len()
    );

    if off_by_more_than_tol >= 10 && jump_count >= 10 {
        eprintln!(
            "REPRODUCED: persistent banding along a nominally-vertical edge - {off_by_more_than_tol} \
             rows deviate from the true constant coverage by more than {tol} levels (max {max_dev}), \
             with {jump_count} row-to-row jumps of >=2 levels, despite the edge's real slope being \
             far too small to legitimately change the rounded pixel value even once."
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - column stayed within tolerance of the true constant value");
    }
    Ok(())
}
