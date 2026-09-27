//! Standalone repro for candidate 063: `Renderer::render()`'s internal glyph-atlas-caching
//! integration calls `.expect("Failed to render glyphs to atlas")` on a `Result` that
//! `Renderer::render_to_atlas` can legitimately return `Err` for (`RenderError::IntermediateTexture(
//! IntermediateTextureError::TooLarge)`), turning a documented, recoverable configuration outcome
//! ("Setting [`LayersConfig::max_texture_size`] smaller ... to explicitly reject too large layers")
//! into a hard process panic whenever a color-glyph's paint graph needs a nested intermediate
//! layer bigger than that bound - reachable from ordinary text rendering with a real color font
//! and a conservative (but fully supported and documented) `LayersConfig`.
//!
//! This drives vello_hybrid's real, public API end-to-end: a real color-glyph (`COLRv1`) test
//! font (the same file already vendored in this repo and used by its own glyph-rendering test
//! suite), real `vello_common`/`glifo` types (`FontData`, `Glyph`), and a real `wgpu::Device`/
//! `Queue` - no faked participant, no reaching into any crate-private API.

use std::iter;
use std::sync::Arc;

use glifo::Glyph;
use vello_common::kurbo::Affine;
use vello_common::peniko::{Blob, FontData};
use vello_hybrid::{
    AtlasConfig, LayersConfig, MemorySettings, RenderSettings, RenderTargetConfig, Renderer,
    Resources, Scene, SizeU16, TextureBindings,
};

/// The Chromium COLRv1 conformance test font, already vendored in this repo and used by
/// `vello_sparse_tests`' own `glyphs_colr_test_glyphs` test - exercises `PaintComposite` with
/// non-default blend/compose modes, clip groups, and nested transforms, i.e. exactly the paint
/// graph shapes that need a real intermediate GPU layer texture to render.
const TEST_FONT: &[u8] =
    include_bytes!("../../clone/examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf");

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Error)
        .init();
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("Failed to find a Vulkan adapter");
    let info = adapter.get_info();
    println!("Adapter: {} ({:?})", info.name, info.backend);

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("Device"),
            required_features: wgpu::Features::empty(),
            ..Default::default()
        })
        .await
        .expect("Failed to create device");

    // A caller who deliberately keeps intermediate layer textures small, exactly as
    // `LayersConfig::max_texture_size`'s own documentation says is a supported, intended choice
    // ("Setting this to smaller ... to explicitly reject too large layers to limit peak memory
    // consumption"). The image atlas itself is left at its ordinary default.
    let memory_settings = MemorySettings {
        image_atlas_config: AtlasConfig::default(),
        layers_config: LayersConfig {
            max_textures: None,
            min_texture_size: SizeU16::new(8),
            max_texture_size: SizeU16::new(8),
        },
    };
    let level = fearless_simd::Level::try_detect().expect("std feature enables detection");
    let render_settings = RenderSettings {
        level,
        memory_settings,
    };

    let (mut renderer, mut resources) = Renderer::new_with(
        &device,
        &RenderTargetConfig {
            format: wgpu::TextureFormat::Rgba8Unorm,
            width: 64,
            height: 64,
        },
        render_settings,
    );

    let mut scene = Scene::new(64, 64);
    let font = FontData::new(Blob::new(Arc::new(TEST_FONT)), 0);
    // Below the glyph atlas cache's default `max_cached_font_size` (128px), so this glyph is
    // actually eligible for atlas caching (`atlas_cache(true)` below) instead of always falling
    // back to direct, uncached rendering.
    let font_size = 100.0_f32;

    // Glyph 84 in this font (found via a sweep over the font's COLR glyph ids, the same set
    // `vello_sparse_tests`' own `glyphs_colr_test_glyphs` test iterates: 0..8, 161..=165,
    // 170..=176 are non-COLR and excluded there too) needs a 52x76px intermediate layer at
    // this font size - the smallest, single, deterministic repro of this bug class.
    let glyph_id = 84_u32;

    let render_frame = |renderer: &mut Renderer, resources: &mut Resources, scene: &Scene| {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("frame"),
        });
        let target = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Target"),
            size: wgpu::Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = target.create_view(&wgpu::TextureViewDescriptor::default());
        let render_size = vello_hybrid::RenderSize {
            width: 64,
            height: 64,
        };
        let result = renderer.render(
            scene,
            resources,
            &device,
            &queue,
            &mut encoder,
            &render_size,
            &view,
            None,
            &TextureBindings::new(),
        );
        if result.is_ok() {
            queue.submit(Some(encoder.finish()));
        }
        result
    };

    // Fill the glyph (a cache miss, so `fill_glyphs` synchronously allocates it an atlas slot and
    // queues a *pending* atlas-rasterization command for it), then render. `Renderer::render`'s
    // internal glyph-atlas-caching glue (`Resources::before_render`) replays that pending command
    // into the atlas-layer scene before rendering `scene` itself - this is where the bug fires.
    scene.set_transform(Affine::translate((4.0, 60.0)));
    let glyph_iter = iter::once(Glyph {
        id: glyph_id,
        x: 0.0,
        y: 0.0,
    });
    scene
        .glyph_run(&mut resources, &font)
        .font_size(font_size)
        .atlas_cache(true)
        .fill_glyphs(glyph_iter);

    println!("Rendering glyph {glyph_id} at font_size {font_size}...");
    match render_frame(&mut renderer, &mut resources, &scene) {
        Ok(()) => {
            println!("No panic: render succeeded - bug not reproduced with this configuration.");
        }
        Err(err) => println!("Renderer::render returned a graceful Err: {err}"),
    }

    device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
}
