// Standalone repro for candidate 071 (VELLO_HYBRID_ATLAS_INDEX_ENCODING_OVERFLOW), against the
// *pristine* (unpatched) tree.
//
// Drives vello_hybrid's real public wgpu-backend API end-to-end - no second participant is being
// faked here, everything below is the crate's own public surface exercised the way a real
// application would:
//   - a real wgpu::Device/Queue, requesting the *adapter's own reported* limits (a completely
//     ordinary thing for a native app that wants to use the full capability of the device it's
//     running on - not an adversarial/unusual choice). Needs a real (possibly software) Vulkan
//     adapter that reports more than 256 `max_texture_array_layers` - real hardware commonly
//     does (this repro's SwiftShader software adapter reports 2048; wgpu's own conservative
//     *default* Limits is exactly 256, which is why this must be tested against a real adapter's
//     real limits, not defaults).
//   - `Renderer::new_with` with an explicit `AtlasConfig::max_atlases` set well above 256 - a
//     value the device's own reported `max_texture_array_layers` genuinely allows, not an
//     out-of-range/malformed one.
//   - `Renderer::upload_image` (its own public entry point for registering a decoded image) is
//     called repeatedly with distinct 1x1 `Pixmap`s (the crate's own public CPU-side image type)
//     until more than 256 atlases have actually been allocated (a 1x1 atlas layer can only ever
//     hold one 1x1 image, so this allocates exactly one new atlas per uploaded image).
//   - the resulting `ImageId` is then drawn via `Scene::set_paint(Image { .. })` +
//     `Scene::fill_rect`, and `Renderer::render()` (the real paint-encoding path) is invoked for
//     real, exercising `encode_image_paint` -> `pack_image_params` with an `atlas_index >= 256`.
//
// `pack_image_params`'s own doc comment says atlas_index is packed into 8 bits (bits 6-13,
// "supports up to 256 atlases"), and it carries a `debug_assert!(atlas_index <= 255, ..)` - but
// `MemorySettings::normalize` (the function whose entire job is clamping a caller-supplied
// `AtlasConfig` to what the backend can actually support) only clamped `max_atlases` to the
// device's reported `max_texture_array_layers`, never to this independent 256 wire-format cap.
// In a debug build this manifests as the `debug_assert!` panic below; in a release build
// (`debug_assertions` off) it instead silently corrupts the packed `image_params` u32 (bit 14,
// `is_external`, gets OR'd with the overflowed atlas_index bits - see RESULT.md).
//
// Needs a real (possibly software) Vulkan adapter. E.g. SwiftShader:
//   VK_ICD_FILENAMES=/path/to/vk_swiftshader_icd.json cargo run --bin before
// (run WITHOUT --release - this repro relies on the debug_assert to demonstrate the bug; see
// RESULT.md for the separate release-mode silent-corruption evidence.)
//
// Exit code convention: 0 = did not reproduce, non-zero/panic = reproduced.

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
    println!(
        "adapter max_texture_array_layers = {}",
        adapter_limits.max_texture_array_layers
    );
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
    println!(
        "device.limits().max_texture_array_layers = {}",
        device.limits().max_texture_array_layers
    );

    let render_target_config = RenderTargetConfig {
        format: wgpu::TextureFormat::Rgba8Unorm,
        width: 64,
        height: 64,
    };

    let mut settings = RenderSettings::default();
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

    // Each 1x1 image forces its own brand-new 1x1 atlas, so uploading N distinct images
    // deterministically allocates exactly N atlases (atlas ids 0..N-1, in allocation order).
    let count: u32 = 300;
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
    println!("last image's expected atlas_id = {last_atlas_id} (>= 256: {})", last_atlas_id >= 256);

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
            println!("did not reproduce - render() completed without panicking for atlas_id {last_atlas_id}");
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic_payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("REPRODUCED - render() panicked while encoding atlas_id {last_atlas_id}: {msg}");
            std::process::exit(1);
        }
    }
}
