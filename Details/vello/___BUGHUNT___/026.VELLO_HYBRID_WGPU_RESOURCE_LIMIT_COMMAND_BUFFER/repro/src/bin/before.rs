// Standalone repro for candidate 026 (VELLO_HYBRID_WGPU_RESOURCE_LIMIT_COMMAND_BUFFER),
// against the *pristine* (unpatched) tree.
//
// Drives vello_hybrid's real public wgpu-backend API (`Renderer::upload_image`) with a handful
// of large (3000x3000px) `Pixmap`s - the same shape of input a real image decoder (Blitz/Servo)
// would hand it after decoding a handful of large source images (linebender/vello#1381: "5-10
// images with dimensions in the 1000-3000px range"). No second participant is being faked here:
// `Pixmap` is the crate's own public CPU-side image representation and `upload_image` is its own
// public entry point for registering decoded image data with the GPU atlas cache - this *is* the
// real end-to-end trigger path.
//
// On the pristine tree, `Renderer::upload_image` returns a bare `ImageId` and panics internally
// (`image_cache.allocate(..).unwrap()`) once the image atlas cache runs out of room. This binary
// only compiles against that pre-fix signature - see `after.rs` for the patched signature.
//
// Needs a real (possibly software) Vulkan adapter. E.g. SwiftShader:
//   VK_ICD_FILENAMES=/path/to/vk_swiftshader_icd.json cargo run --release --bin before
//
// Exit code convention: 0 = did not reproduce, non-zero/panic = reproduced.

use vello_common::pixmap::Pixmap;
use vello_hybrid::{RenderTargetConfig, Renderer};

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

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("failed to create device");

    let render_target_config = RenderTargetConfig {
        format: wgpu::TextureFormat::Rgba8Unorm,
        width: 64,
        height: 64,
    };

    // RenderSettings::default() -> AtlasConfig { max_atlases: 8, atlas_size: (4096, 4096), ... }
    let (mut renderer, mut resources) = Renderer::new(&device, &render_target_config);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("repro encoder"),
    });

    // A handful of large source images - the same shape of report as linebender/vello#1381
    // ("5-10 images with dimensions in the 1000-3000px range"). Each 3000x3000 image, once
    // padded, cannot share a 4096x4096 atlas layer with another image of similar size, so with
    // the default max_atlases = 8 the image cache's atlas space is exhausted well before 10
    // uploads complete.
    let count = 10;
    let size: u16 = 3000;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        for i in 0..count {
            println!("uploading image {i} ({size}x{size})");
            let pixmap = Pixmap::new(size, size);
            let _image_id =
                renderer.upload_image(&mut resources, &device, &queue, &mut encoder, &pixmap);
        }
    }));

    match result {
        Ok(()) => {
            queue.submit(Some(encoder.finish()));
            println!("did not reproduce - all {count} uploads of {size}x{size} images succeeded");
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic_payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("REPRODUCED - upload_image panicked: {msg}");
            std::process::exit(1);
        }
    }
}
