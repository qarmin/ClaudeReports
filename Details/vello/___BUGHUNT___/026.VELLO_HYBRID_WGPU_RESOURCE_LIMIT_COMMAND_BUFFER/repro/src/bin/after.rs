// Standalone repro for candidate 026 (VELLO_HYBRID_WGPU_RESOURCE_LIMIT_COMMAND_BUFFER),
// against the *patched* tree.
//
// Identical scenario to `before.rs` (10 uploads of 3000x3000 `Pixmap`s through the real public
// `Renderer::upload_image` entry point), but written against the patched signature, which turns
// atlas exhaustion into a `Result::Err` (`RenderError::AtlasError`) instead of an internal panic.
// A caller (e.g. Blitz) can now catch this and skip/downscale the offending image instead of the
// whole process going down.
//
// Needs a real (possibly software) Vulkan adapter. E.g. SwiftShader:
//   VK_ICD_FILENAMES=/path/to/vk_swiftshader_icd.json cargo run --release --bin after
//
// Exit code convention: 0 = did not reproduce (bug fixed), non-zero/panic = reproduced (still
// panics).

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

    let (mut renderer, mut resources) = Renderer::new(&device, &render_target_config);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("repro encoder"),
    });

    let count = 10;
    let size: u16 = 3000;

    // A fixed build turns the failure into a `Result::Err` a caller can catch instead of an
    // unwind that takes down the whole process - so only an actual panic counts as reproduced.
    let mut saw_graceful_error = false;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        for i in 0..count {
            println!("uploading image {i} ({size}x{size})");
            let pixmap = Pixmap::new(size, size);
            match renderer.upload_image(&mut resources, &device, &queue, &mut encoder, &pixmap) {
                Ok(_image_id) => {}
                Err(e) => {
                    println!("upload_image returned a graceful error instead of panicking: {e}");
                    saw_graceful_error = true;
                    break;
                }
            }
        }
    }));

    match result {
        Ok(()) if saw_graceful_error => {
            println!(
                "did not reproduce - upload_image reported atlas exhaustion as a recoverable \
                 Result::Err instead of panicking"
            );
        }
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
