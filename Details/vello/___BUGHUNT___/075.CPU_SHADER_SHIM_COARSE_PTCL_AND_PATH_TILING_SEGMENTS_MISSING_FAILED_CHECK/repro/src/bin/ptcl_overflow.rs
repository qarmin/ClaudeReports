// Standalone reproduction for bug-hunt candidate 075
// (CPU_SHADER_SHIM_COARSE_PTCL_AND_PATH_TILING_SEGMENTS_MISSING_FAILED_CHECK).
//
// Drives vello's real, public entry point (`vello::Renderer::render_to_texture` with
// `RendererOptions::use_cpu: true`), fed with a real (tiny, one-draw) `Scene` rendered at a large
// enough canvas through a real (software) `wgpu` device, exactly the same way a real application
// using `use_cpu: true` would. `vello_encoding::config::BufferSizes::new` hardcodes the `ptcl`
// buffer's capacity at `1 << 23` (8,388,608 `u32`s) *independent of canvas size* - but
// `vello_shaders/src/cpu/coarse.rs`'s `TileState::new` seeds every tile's initial ptcl cursor at
// `tile_ix * PTCL_INITIAL_ALLOC` (`PTCL_INITIAL_ALLOC = 64`), so a canvas with more than
// 8,388,608 / 64 = 131,072 tiles overflows this fixed buffer just from iterating over ordinary
// in-canvas tiles - with no scene content required at all to also overflow tile_alloc.rs's,
// path_count.rs's, or binning.rs's (all much smaller, 1<<21/1<<18) buffers first.

use std::num::NonZeroUsize;
use std::panic::AssertUnwindSafe;

use anyhow::{Result, anyhow};
use vello::kurbo::{Affine, Rect};
use vello::peniko::Color;
use vello::util::RenderContext;
use vello::wgpu::{self, Extent3d, TextureDescriptor, TextureFormat, TextureUsages};
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene};

fn main() -> Result<()> {
    pollster::block_on(async {
        // 364*364 = 132,496 tiles > 131,072 -> the fixed-size `ptcl` buffer (8,388,608 u32s)
        // overflows purely from tile iteration, with plenty of margin.
        const CANVAS: u32 = 364 * 16; // 5824, well under the SwiftShader adapter's
        // max_texture_dimension_2d (8192).

        let mut context = RenderContext::new();
        let device_id = context
            .device(None)
            .await
            .ok_or_else(|| anyhow!("No compatible device found"))?;
        let mut renderer = Renderer::new(
            &context.devices[device_id].device,
            RendererOptions {
                use_cpu: true,
                num_init_threads: NonZeroUsize::new(1),
                antialiasing_support: std::iter::once(AaConfig::Area).collect(),
                pipeline_cache: None,
            },
        )
        .map_err(|_| anyhow!("Got non-Send/Sync error from creating renderer"))?;

        // A single tiny draw object, nowhere near overflowing tile_alloc.rs/path_count.rs/
        // binning.rs's own (much smaller) fixed-size buffers - included so the scene isn't
        // literally empty (matching realistic application usage), not because the bug needs any
        // scene content at all.
        let mut scene = Scene::new();
        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            Color::from_rgba8(255, 255, 0, 255),
            None,
            &Rect::new(0.0, 0.0, 4.0, 4.0),
        );

        let device_handle = &mut context.devices[device_id];
        let device = &device_handle.device;
        let queue = &device_handle.queue;

        let params = RenderParams {
            base_color: Color::from_rgba8(0, 0, 0, 255),
            width: CANVAS,
            height: CANVAS,
            antialiasing_method: AaConfig::Area,
        };
        let size = Extent3d {
            width: CANVAS,
            height: CANVAS,
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

        println!(
            "Rendering a {CANVAS}x{CANVAS} canvas (one tiny 4x4px fill) via use_cpu:true ..."
        );
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            renderer.render_to_texture(device, queue, &scene, &view, &params)
        }));
        match result {
            Ok(Ok(())) => println!("rendered OK, no panic"),
            Ok(Err(e)) => println!("rendered with Err({e:?}), no panic"),
            Err(_) => println!("PANICKED (see stderr above for the real backtrace)"),
        }

        // See ptcl_dynamic.rs for why teardown is skipped here.
        std::process::exit(0);
    })
}
