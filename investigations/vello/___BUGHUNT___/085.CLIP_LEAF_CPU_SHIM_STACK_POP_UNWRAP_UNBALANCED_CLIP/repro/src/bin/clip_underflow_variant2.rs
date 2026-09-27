// Variant of clip_underflow.rs: one real BEGIN_CLIP/END_CLIP pair (balanced), followed by a
// second, dangling END_CLIP with no matching BEGIN_CLIP - exercises the same bug class
// (stack underflow on a *second* excess end-clip after the stack has already been legitimately
// emptied once) rather than the exact same "very first entry is an end-clip" shape used by
// clip_underflow.rs.
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
        scene.push_layer(
            vello::peniko::Fill::NonZero,
            vello::peniko::Compose::SrcOver,
            1.0,
            Affine::IDENTITY,
            &Rect::new(0.0, 0.0, 10.0, 10.0),
        );
        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            Color::from_rgba8(255, 255, 0, 255),
            None,
            &Rect::new(0.0, 0.0, 10.0, 10.0),
        );
        scene.pop_layer();

        // Now append a *second*, dangling END_CLIP - the stack is already empty at this point
        // from the legitimate pop_layer() above, so this one has no matching begin at all.
        {
            let enc = scene.encoding_mut();
            enc.draw_tags.push(DrawTag::END_CLIP);
            enc.path_tags.push(PathTag::PATH);
            enc.n_paths += 1;
            enc.n_clips += 1;
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
