// Standalone repro for candidate 071 (VELLO_HYBRID_ATLAS_INDEX_ENCODING_OVERFLOW), against the
// *patched* tree (after `patch.diff` is applied).
//
// Same setup as `before.rs` (real device, `max_atlases` requested at 2048, which the device's own
// `max_texture_array_layers` genuinely allows), but now `MemorySettings::normalize` also clamps
// `max_atlases` to `MAX_ATLASES_SUPPORTED_BY_ENCODING` (256) - the actual limit of the 8-bit
// `atlas_index` field in `pack_image_params` - regardless of what the device allows. This repro
// therefore only uploads 256 images (atlas ids 0..255, the full range the encoding can address)
// and confirms `render()` completes without panicking for the highest addressable atlas id (255) -
// i.e. the fix keeps `atlas_index` within its 8-bit budget instead of ever handing it a value the
// encoding can't represent.
//
// (Uploading a 257th image after the fix now hits a *different*, already independently reported
// and confirmed bug - candidate 026's `image_cache.allocate(..).unwrap()` panicking on
// `AtlasError::MaxAtlasesReached` - which is expected and outside this candidate's scope: the fix
// here is only responsible for making sure `atlas_index` never silently overflows its 8-bit field,
// not for how atlas exhaustion itself is surfaced to callers.)
//
// Needs a real (possibly software) Vulkan adapter. E.g. SwiftShader:
//   VK_ICD_FILENAMES=/path/to/vk_swiftshader_icd.json cargo run --bin after
//
// Exit code convention: 0 = did not reproduce (fix holds), non-zero/panic = still broken.

use vello_common::kurbo::Rect;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::paint::{Image, ImageSource};
use vello_common::peniko::ImageSampler;
use vello_common::pixmap::Pixmap;
use vello_hybrid::{RenderSettings, RenderSize, RenderTargetConfig, Renderer, Scene, TextureBindings};

fn main() {
    env_logger::init();
    pollster::block_on(run());
}

async fn run() {
    let mut desc = wgpu::InstanceDescriptor::new_without_display_handle();
    desc.backends = wgpu::Backends::VULKAN;
    let instance = wgpu::Instance::new(desc);

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("no adapter available - set VK_ICD_FILENAMES to a usable Vulkan ICD");
    println!("Using adapter: {:?}", adapter.get_info());
    let adapter_limits = adapter.limits();
    assert!(
        adapter_limits.max_texture_array_layers > 256,
        "this repro needs a real adapter that reports more than 256 texture array layers"
    );

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            required_limits: adapter_limits.clone(),
            ..Default::default()
        })
        .await
        .expect("failed to create device");

    let render_target_config = RenderTargetConfig {
        format: wgpu::TextureFormat::Rgba8Unorm,
        width: 64,
        height: 64,
    };

    let mut settings = RenderSettings::default();
    // Same caller-requested value as before.rs (2048) - the fix must clamp this down to 256
    // internally, not require the caller to already know about the encoding's own limit.
    settings.memory_settings.image_atlas_config = AtlasConfig {
        initial_atlas_count: 0,
        max_atlases: 2048,
        atlas_size: (1, 1),
        auto_grow: true,
        ..AtlasConfig::default()
    };

    let (mut renderer, mut resources) = Renderer::new_with(&device, &render_target_config, settings);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("repro encoder"),
    });

    // Exactly the number of atlases the encoding can address (0..255) - should all succeed.
    let count: u32 = 256;
    let mut last_image_id = None;
    for i in 0..count {
        let pixmap = Pixmap::new(1, 1);
        let image_id = renderer.upload_image(&mut resources, &device, &queue, &mut encoder, &pixmap);
        if i % 50 == 0 {
            println!("uploaded image {i} (expected atlas_id = {i})");
        }
        last_image_id = Some(image_id);
    }
    queue.submit(Some(encoder.finish()));

    let last_image_id = last_image_id.unwrap();
    let last_atlas_id = count - 1;
    println!("last image's expected atlas_id = {last_atlas_id} (max addressable by the 8-bit encoding)");

    let mut scene = Scene::new(64, 64);
    scene.set_paint(Image {
        image: ImageSource::opaque_id(last_image_id),
        sampler: ImageSampler::default(),
    });
    scene.fill_rect(&Rect::new(0.0, 0.0, 4.0, 4.0));

    let target_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("repro render target"),
        size: wgpu::Extent3d {
            width: 64,
            height: 64,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let view = target_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut render_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("repro render encoder"),
    });

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        renderer
            .render(
                &scene,
                &mut resources,
                &device,
                &queue,
                &mut render_encoder,
                &RenderSize { width: 64, height: 64 },
                &view,
                None,
                &TextureBindings::new(),
            )
            .expect("render() returned an error");
    }));

    match result {
        Ok(()) => {
            queue.submit(Some(render_encoder.finish()));
            println!(
                "did not reproduce - render() completed without panicking for atlas_id {last_atlas_id} \
                 (max_atlases was correctly clamped to 256, so atlas_index never overflowed its 8-bit field)"
            );
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic_payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("STILL BROKEN - render() panicked while encoding atlas_id {last_atlas_id}: {msg}");
            std::process::exit(1);
        }
    }
}
