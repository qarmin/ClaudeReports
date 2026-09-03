// Standalone reproduction for bug-hunt candidate 087
// (CPU_SHADER_LINES_BLEND_SPILL_BUMP_ALLOC_MISSING_FAILED_CHECK).
//
// Drives vello's real, public entry point (`vello::Renderer::render_to_texture` with
// `RendererOptions::use_cpu: true`), fed with real `Scene`/`BezPath` geometry through a real
// (software) `wgpu` device, exactly the same way a real application using `use_cpu: true` would.
//
// This demonstrates `vello_shaders/src/cpu/flatten.rs`'s `write_line` panicking with an
// index-out-of-bounds when a real scene's flattened-line demand exceeds the `lines` buffer's
// fixed hand-picked capacity (`vello_encoding::config::BufferSizes::new`, 1<<21 = 2,097,152) - a
// condition the real GPU path (`flatten.wgsl`'s own `write_line`) handles safely via an explicit
// `if line_ix < config.lines_size` guard before ever indexing into the buffer.

use std::num::NonZeroUsize;
use std::panic::AssertUnwindSafe;

use anyhow::{Result, anyhow};
use vello::kurbo::{Affine, BezPath};
use vello::peniko::Color;
use vello::util::RenderContext;
use vello::wgpu::{self, Extent3d, TextureDescriptor, TextureFormat, TextureUsages};
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene};

async fn try_render_path(
    context: &mut RenderContext,
    device_id: usize,
    renderer: &mut Renderer,
    path: &BezPath,
    width: u32,
    height: u32,
    label: &str,
) {
    let mut scene = Scene::new();
    scene.fill(
        vello::peniko::Fill::NonZero,
        Affine::IDENTITY,
        Color::from_rgba8(255, 255, 0, 255),
        None,
        path,
    );

    let device_handle = &mut context.devices[device_id];
    let device = &device_handle.device;
    let queue = &device_handle.queue;

    let params = RenderParams {
        base_color: Color::from_rgba8(0, 0, 0, 255),
        width,
        height,
        antialiasing_method: AaConfig::Area,
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

    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        renderer.render_to_texture(device, queue, &scene, &view, &params)
    }));
    match result {
        Ok(Ok(())) => println!("{label}: rendered OK, no panic"),
        Ok(Err(e)) => println!("{label}: rendered with Err({e:?}), no panic"),
        Err(_) => println!("{label}: PANICKED (see stderr above for the real backtrace)"),
    }
}

async fn make_renderer(context: &mut RenderContext) -> Result<(usize, Renderer)> {
    let device_id = context
        .device(None)
        .await
        .ok_or_else(|| anyhow!("No compatible device found"))?;
    let renderer = Renderer::new(
        &context.devices[device_id].device,
        RendererOptions {
            use_cpu: true,
            num_init_threads: NonZeroUsize::new(1),
            antialiasing_support: std::iter::once(AaConfig::Area).collect(),
            pipeline_cache: None,
        },
    )
    .map_err(|_| anyhow!("Got non-Send/Sync error from creating renderer"))?;
    Ok((device_id, renderer))
}

fn main() -> Result<()> {
    pollster::block_on(async {
        println!(
            "=== flatten.rs `lines` buffer overflow (one path, > 2,097,152 line_to segments) ==="
        );
        const CANVAS: u32 = 512;
        // config.lines_size == 1 << 21 == 2_097_152. Use comfortably more than that so the
        // overflow is hit regardless of any small fixed per-path overhead.
        const N_LINES: u32 = 2_200_000;

        let mut context = RenderContext::new();
        let (device_id, mut renderer) = make_renderer(&mut context).await?;

        let mut path = BezPath::new();
        path.move_to((0.0, 0.0));
        // Zig-zag across a small canvas: each segment is a plain straight line, so `flatten`
        // emits exactly one `LineSoup` per `line_to` (no curve subdivision, no join/cap
        // multiplication from stroking).
        for i in 0..N_LINES {
            let x = (i % 2) as f64 * CANVAS as f64;
            let y = (i as f64 / N_LINES as f64) * CANVAS as f64;
            path.line_to((x, y));
        }
        path.close_path();

        try_render_path(
            &mut context,
            device_id,
            &mut renderer,
            &path,
            CANVAS,
            CANVAS,
            "one path, 2_200_000 line_to segments (flatten.rs `lines`-buffer overflow)",
        )
        .await;

        println!();
        println!("Done.");

        Ok::<(), anyhow::Error>(())
    })
}
