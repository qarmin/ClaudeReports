// Exploratory: attempt to trigger coarse.rs's *other* ptcl call site (line 68, the dynamic
// alloc_cmd/chunk-jump path) independently of the static per-tile-cursor overflow (line 382)
// that ptcl_overflow.rs already confirms. Uses a canvas just under the 131,072-tile threshold
// (leaving a small dynamic-growth margin) plus many tiny same-tile draws to force many
// alloc_cmd chunk jumps concentrated on one tile.
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
        const CANVAS: u32 = 361 * 16; // 5776, 361*361=130321 tiles < 131072 threshold
        const N_DRAWS: u32 = 60_000;

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

        let mut scene = Scene::new();
        for _ in 0..N_DRAWS {
            scene.fill(
                vello::peniko::Fill::NonZero,
                Affine::IDENTITY,
                Color::from_rgba8(255, 255, 0, 255),
                None,
                &Rect::new(0.0, 0.0, 16.0, 16.0),
            );
        }

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
            "Rendering a {CANVAS}x{CANVAS} canvas ({N_DRAWS} tiny same-tile fills) via use_cpu:true ..."
        );
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            renderer.render_to_texture(device, queue, &scene, &view, &params)
        }));
        match result {
            Ok(Ok(())) => println!("rendered OK, no panic"),
            Ok(Err(e)) => println!("rendered with Err({e:?}), no panic"),
            Err(_) => println!("PANICKED (see stderr above for the real backtrace)"),
        }

        // Skip wgpu/RenderContext teardown: on this sandbox's slow software (SwiftShader)
        // adapter, dropping the queue after a very heavy CPU-shim render can hit an unrelated
        // "timed out waiting on last successful submission" panic during cleanup - after the
        // actual result above was already captured, so it isn't part of what's being measured.
        std::process::exit(0);
    })
}
