// Standalone reproduction for bug-hunt candidate 066 (CPU_SHADER_SHIM_BUMP_ALLOCATOR_MISSING_FAILED_CHECK).
//
// Drives vello's real, public entry point (`vello::Renderer::render_to_texture` with
// `RendererOptions::use_cpu: true`), fed with real `Scene`/`BezPath`/stroke geometry through a
// real (software) `wgpu` device, exactly the same way a real application using `use_cpu: true`
// would. This demonstrates three of the CPU-shader-shim's bump-allocator buffers panicking with
// an index-out-of-bounds when a real scene's demand exceeds the buffer's fixed hand-picked
// capacity (`vello_encoding::config::BufferSizes::new`) - a condition the real GPU path handles
// safely via `BumpAllocators::failed` + an offset-reset-to-0 clamp in the WGSL shaders
// themselves.

use std::num::NonZeroUsize;
use std::panic::AssertUnwindSafe;

use anyhow::{Result, anyhow};
use vello::kurbo::{Affine, BezPath, Line, Stroke};
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
    try_render_scene(context, device_id, renderer, &scene, width, height, label).await;
}

async fn try_render_scene(
    context: &mut RenderContext,
    device_id: usize,
    renderer: &mut Renderer,
    scene: &Scene,
    width: u32,
    height: u32,
    label: &str,
) {
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
        renderer.render_to_texture(device, queue, scene, &view, &params)
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
        println!("=== 1: tile_alloc.rs `tiles` buffer overflow (many overlapping full-canvas diagonals) ===");
        {
            const CANVAS_A: u32 = 2048;
            let mut context = RenderContext::new();
            let (device_id, mut renderer) = make_renderer(&mut context).await?;
            let mut scene = Scene::new();
            let stroke = Stroke::new(1.0);
            for i in 0..1_000u32 {
                let jitter = (i % 7) as f64 * 0.01;
                let line = Line::new(
                    (0.0 + jitter, 0.0),
                    (CANVAS_A as f64, CANVAS_A as f64 - jitter),
                );
                scene.stroke(
                    &stroke,
                    Affine::IDENTITY,
                    Color::from_rgba8(255, 255, 0, 255),
                    None,
                    &line,
                );
            }
            try_render_scene(
                &mut context,
                device_id,
                &mut renderer,
                &scene,
                CANVAS_A,
                CANVAS_A,
                "1: 1000 full-canvas diagonals (tile_alloc.rs tiles-buffer overflow)",
            )
            .await;
        }

        println!();
        println!("=== 2: path_count.rs `seg_counts` buffer overflow (one path, many full-diagonal lines) ===");
        {
            const CANVAS_B: u32 = 8000;
            let mut context = RenderContext::new();
            let (device_id, mut renderer) = make_renderer(&mut context).await?;
            let mut path = BezPath::new();
            path.move_to((0.0, 0.0));
            let n_lines = 2_500u32;
            for i in 0..n_lines {
                let t = i as f64 / n_lines as f64;
                let jitter = (i % 97) as f64 * 1e-3;
                if i % 2 == 0 {
                    path.line_to((CANVAS_B as f64, CANVAS_B as f64 - jitter));
                } else {
                    path.line_to((0.0, jitter + t * 0.001));
                }
            }
            path.close_path();
            try_render_path(
                &mut context,
                device_id,
                &mut renderer,
                &path,
                CANVAS_B,
                CANVAS_B,
                "2: one path, 2500 full-diagonal lines (path_count.rs seg_counts-buffer overflow)",
            )
            .await;
        }

        println!();
        println!(
            "=== 3: binning.rs `bin_data` buffer overflow (many tiny draws each straddling a \
             4-bin corner) ==="
        );
        {
            // bin_data's capacity (1<<18 = 262144) is much smaller, relative to how many
            // (draw, bin) pairs a real scene can cheaply generate, than tiles'/seg_counts'
            // (1<<21). A tiny (2x2 pixel) stroked square exactly straddling a point where 4
            // bins meet (bin = 256x256px) contributes 4 (draw, bin) pairs to `bump.binning`
            // while only needing 4 tiles from `bump.tile` (well within the 1<<21 tile budget) -
            // so many repeats of the *same* tiny draw overflow `bin_data` long before `tiles` or
            // `seg_counts`.
            const CANVAS_C: u32 = 512; // 2x2 bins, 32x32 tiles - plenty for every draw to fit
            const N_DRAWS: u32 = 90_000; // 90_000 * 4 = 360_000 > 262_144 (bin_data capacity)
            let mut context = RenderContext::new();
            let (device_id, mut renderer) = make_renderer(&mut context).await?;
            let mut scene = Scene::new();
            let stroke = Stroke::new(1.0);
            // Straddles the bin corner at pixel (256, 256): a 2x2 px square from (255,255) to
            // (257,257).
            let mut square = BezPath::new();
            square.move_to((255.0, 255.0));
            square.line_to((257.0, 255.0));
            square.line_to((257.0, 257.0));
            square.line_to((255.0, 257.0));
            square.close_path();
            for _ in 0..N_DRAWS {
                scene.stroke(
                    &stroke,
                    Affine::IDENTITY,
                    Color::from_rgba8(255, 255, 0, 255),
                    None,
                    &square,
                );
            }
            try_render_scene(
                &mut context,
                device_id,
                &mut renderer,
                &scene,
                CANVAS_C,
                CANVAS_C,
                "3: 90000 bin-corner-straddling squares (binning.rs bin_data-buffer overflow)",
            )
            .await;
        }

        println!();
        println!("Done.");

        Ok::<(), anyhow::Error>(())
    })
}
