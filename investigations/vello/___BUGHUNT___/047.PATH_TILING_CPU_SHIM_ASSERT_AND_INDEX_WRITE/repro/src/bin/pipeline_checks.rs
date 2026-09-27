// Standalone reproduction for bug-hunt candidate 047
// (vello_shaders/src/cpu/path_tiling.rs:160-164's `assert!`s + the unchecked
// `segments[(seg_start + seg_within_slice) as usize] = segment` write).
//
// This drives vello's real, public, recommended entry point (`vello::Renderer::render_to_texture`
// with `RendererOptions::use_cpu: true`, so the *actual* production CPU-shader fallback runs -
// `vello_shaders::cpu::path_tiling` is not reimplemented, it is the literal function
// `vello::Renderer` calls whenever a GPU is missing a feature or CPU shaders were requested), fed
// with real `Scene`/`BezPath` geometry through a real (software) `wgpu` device.
//
// What this checks, in order:
//   1. The exact minimal-repro line segments from two *historical* CPU-shim assertion-failure
//      reports (github.com/linebender/vello#378, #379-adjacent #775) - to confirm the
//      `ROBUST_EPSILON`/`ONE_MINUS_ULP` fix that closed #378/#379 is still effective in the
//      current tree, and that #775 (still open upstream, a *visual* artifact report, not a
//      crash report) does not additionally trip path_tiling.rs's own assert.
//   2. A "many draws, full-canvas bbox" scene that overflows vello_encoding's fixed-size `tiles`
//      buffer (config.rs: `1 << 21`) in `tile_alloc.rs`, and a "single path, many full-diagonal
//      lines" scene designed to overflow the `segments`/`seg_counts` buffers specifically - both
//      demonstrate that *other*, earlier CPU-shim pipeline stages (`tile_alloc.rs`, `binning.rs`,
//      `coarse.rs`, `path_count.rs`) have the identical "unchecked write into an upstream
//      bump-allocator-sized buffer" bug class, and structurally always panic *before*
//      `path_tiling.rs`'s own equivalent write is ever reached with an out-of-range index (see
//      RESULT.md's "Root cause" section for why `seg_counts`/`segments` can't be decoupled).
//
// Exit code convention: this program does not "reproduce" anything for path_tiling.rs itself
// (see RESULT.md, Status: Not reproducible) - it prints PASS/FAIL-style status lines and always
// exits 0; the point is to show the real, captured behavior, not to gate a CI-style pass/fail on
// a bug that wasn't found.

use std::num::NonZeroUsize;
use std::panic::AssertUnwindSafe;

use anyhow::{Result, anyhow};
use vello::kurbo::{Affine, BezPath, Line, Stroke};
use vello::peniko::Color;
use vello::util::RenderContext;
use vello::wgpu::{self, Extent3d, TextureDescriptor, TextureFormat, TextureUsages};
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene};

/// Renders `path` (filled) through the real public API with `use_cpu: true`, catching any panic
/// from inside the CPU-shader dispatch (which runs synchronously on this thread, so a panic from
/// e.g. `path_tiling.rs`'s `assert!`s or its final unchecked slice index propagates straight up
/// through `render_to_texture`).
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
        println!(
            "=== Part 1: historical CPU-shim assertion-failure reports (vello#378, vello#775) ==="
        );
        {
            let mut context = RenderContext::new();
            let (device_id, mut renderer) = make_renderer(&mut context).await?;

            // vello#378 (fixed by the ONE_MINUS_ULP/ROBUST_EPSILON logic present in the current
            // tree): a near-horizontal line with a huge span, not exactly grid-aligned. Original
            // report: "assertion failed: xy0.x >= tile_xy.x && xy0.x <= tile_xy1.x".
            let mut p378 = BezPath::new();
            p378.move_to((-0.36201912, 1.7544264e-13));
            p378.line_to((1354.9857, 1.7544264e-13));
            p378.line_to((100.0, 100.0));
            p378.close_path();
            try_render_path(
                &mut context,
                device_id,
                &mut renderer,
                &p378,
                1400,
                200,
                "vello#378 near-horizontal-huge-span",
            )
            .await;

            // vello#775 (still open upstream as of this candidate; reported as a *visual*
            // winding/fill artifact from tile-boundary-snapping, not a crash): a near-vertical
            // sliver straddling x=0.
            let mut p775 = BezPath::new();
            p775.move_to((0.0, 0.0));
            p775.line_to((1000.0, 0.0));
            p775.line_to((1000.0, 1000.0));
            p775.line_to((0.0, 1000.0));
            p775.line_to((-1.87016292e-06, 923.848633));
            p775.line_to((1.87016292e-06, 156.151367));
            p775.close_path();
            try_render_path(
                &mut context,
                device_id,
                &mut renderer,
                &p775,
                1000,
                1000,
                "vello#775 tile-boundary-snap",
            )
            .await;
        }

        println!();
        println!(
            "=== Part 2: sibling bump-allocator overflow demonstration (why path_tiling.rs's own \
             write is masked) ==="
        );
        {
            // 2a. "Many overlapping full-canvas diagonals" - each draw's bbox covers almost the
            // whole (small) canvas, so `tile_alloc.rs`'s per-draw tile-rectangle allocation
            // (`(ux1-ux0)*(ux1-uy0)` tiles, no bounds check of its own) overflows the `tiles`
            // buffer (1<<21 entries) after only a few hundred draws - long before path_tiling.rs
            // would ever run.
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
                "2a: 1000 full-canvas diagonals (tile_alloc.rs tiles-buffer overflow)",
            )
            .await;
        }
        {
            // 2b. "One path, many full-diagonal lines" - the path's bbox (hence its tile_alloc
            // cost) is paid exactly once (the whole canvas), but each of the many nearly-parallel
            // diagonal LineSoup entries independently crosses ~(width_in_tiles+height_in_tiles)
            // tiles, so path_count.rs's own unchecked write into the (same-sized, 1<<21)
            // `seg_counts` buffer overflows first - one pipeline stage before path_tiling.rs's
            // `segments` write would need to.
            const CANVAS_B: u32 = 8000; // multiple of 16 -> 500x500 = 250,000 tiles, well under 1<<21
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
                "2b: one path, 2500 full-diagonal lines (path_count.rs seg_counts-buffer overflow)",
            )
            .await;
        }

        println!();
        println!(
            "Done. See RESULT.md for why these panics land in tile_alloc.rs/path_count.rs and not \
             in path_tiling.rs itself, and `search` (this same folder's other binary) for the \
             offline float-precision search over path_tiling.rs's own assert invariant."
        );

        Ok::<(), anyhow::Error>(())
    })
}
