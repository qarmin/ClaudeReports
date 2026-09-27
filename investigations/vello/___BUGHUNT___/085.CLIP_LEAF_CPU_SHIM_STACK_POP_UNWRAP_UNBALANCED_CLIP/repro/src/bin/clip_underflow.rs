use std::num::NonZeroUsize;
use std::panic::AssertUnwindSafe;

use anyhow::{Result, anyhow};
use vello::kurbo::{Affine, Rect};
use vello::peniko::Color;
use vello::util::RenderContext;
use vello::wgpu::{self, Extent3d, TextureDescriptor, TextureFormat, TextureUsages};
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene};
use vello_encoding::{DrawTag, PathTag};

fn main() -> Result<()> {
    pollster::block_on(async {
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
        // A normal, valid draw object first (satisfies the "at least one transform/style before
        // any draw object" invariant, and gives clip_leaf a valid draw_monoids[..] entry it
        // never actually touches for this bug).
        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            Color::from_rgba8(255, 255, 0, 255),
            None,
            &Rect::new(0.0, 0.0, 10.0, 10.0),
        );

        // Now use `Scene::encoding_mut` (the project's own public, documented "this can be used
        // to more easily create invalid scenes" escape hatch) to append a raw END_CLIP draw
        // object with no matching BEGIN_CLIP ever encoded - exactly mirroring what
        // `Encoding::encode_end_clip` does internally, MINUS its `n_open_clips > 0` guard.
        {
            let enc = scene.encoding_mut();
            enc.draw_tags.push(DrawTag::END_CLIP);
            enc.path_tags.push(PathTag::PATH);
            enc.n_paths += 1;
            enc.n_clips += 1;
            // n_open_clips deliberately left at 0: there was never a matching begin_clip, so
            // there is nothing to "close" and resolve() should not append any synthetic END_CLIP.
        }

        let device_handle = &mut context.devices[device_id];
        let device = &device_handle.device;
        let queue = &device_handle.queue;
        let params = RenderParams {
            base_color: Color::from_rgba8(0, 0, 0, 255),
            width: 16,
            height: 16,
            antialiasing_method: AaConfig::Area,
        };
        let target = device.create_texture(&TextureDescriptor {
            label: Some("Target texture"),
            size: Extent3d {
                width: 16,
                height: 16,
                depth_or_array_layers: 1,
            },
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
            Ok(Ok(())) => println!("rendered OK, no panic"),
            Ok(Err(e)) => println!("rendered with Err({e:?}), no panic"),
            Err(_) => println!("PANICKED (see stderr above for the real backtrace)"),
        }
        Ok(())
    })
}
