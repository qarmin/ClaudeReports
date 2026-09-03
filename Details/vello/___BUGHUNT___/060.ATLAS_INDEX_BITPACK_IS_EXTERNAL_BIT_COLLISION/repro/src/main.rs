// Standalone repro for candidate 060: ATLAS_INDEX_BITPACK_IS_EXTERNAL_BIT_COLLISION.
//
// Drives the real, public vello_hybrid GPU rendering pipeline end-to-end: a real wgpu device
// (SwiftShader software Vulkan adapter), a real `Renderer` configured through the public
// `RenderSettings`/`AtlasConfig` API, 257 real `upload_image` calls that force the real
// `MultiAtlasManager` to actually allocate 257 real atlases, and a real `Scene`/`render()` call
// that paints an image from the 257th (0-indexed atlas_id == 256) atlas.
//
// If the bug is present: `atlas_id=256` packed into `image_params` bits 6-13 carries into bit 14
// (the `is_external` bit), so the renderer's own shader believes this atlas-backed image is an
// external texture and samples the always-zero-initialized 1x1 placeholder external texture
// instead - the rendered pixel comes back transparent black instead of the uploaded opaque red.

use vello_common::kurbo::Rect;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::paint::{Image, ImageSource};
use vello_common::peniko::ImageSampler;
use vello_hybrid::{RenderSettings, RenderSize, RenderTargetConfig, Renderer, Scene, TextureBindings};

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Ensure SwiftShader's software Vulkan ICD is discoverable (see
    // ___BUGHUNT___/README.md's "GPU-based tests" row - this is the same real adapter that row
    // documents, just wired up programmatically here instead of via an externally-set env var,
    // so the repro is runnable with a single `cargo run`).
    if std::env::var_os("VK_ICD_FILENAMES").is_none() {
        for candidate in [
            "/home/rafal/Projekty/Obce/vello/tmp/swiftshader_icd/vk_swiftshader_icd.json",
            "/tmp/swiftshader_icd/vk_swiftshader_icd.json",
        ] {
            if std::path::Path::new(candidate).exists() {
                // SAFETY: single-threaded at this point in `main`, before any other thread
                // could plausibly read the environment.
                unsafe {
                    std::env::set_var("VK_ICD_FILENAMES", candidate);
                }
                break;
            }
        }
    }

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("failed to find a Vulkan adapter (is VK_ICD_FILENAMES set to a real ICD?)");
    let info = adapter.get_info();
    println!("adapter: {info:?}");
    let adapter_limits = adapter.limits();
    println!(
        "adapter max_texture_array_layers: {}",
        adapter_limits.max_texture_array_layers
    );
    assert!(
        adapter_limits.max_texture_array_layers > 256,
        "this repro requires an adapter that genuinely supports more than 256 texture array \
         layers - got {}",
        adapter_limits.max_texture_array_layers
    );

    // A real caller asking for the adapter's full reported limits (a completely normal,
    // documented wgpu usage pattern - e.g. wgpu's own `request_adapter` examples) rather than
    // wgpu's conservative default limits (which cap `max_texture_array_layers` at 256 and would
    // otherwise mask this bug entirely, since `MemorySettings::normalize` clamps `max_atlases` to
    // whatever the device limits allow).
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("device"),
            required_features: wgpu::Features::empty(),
            required_limits: adapter_limits.clone(),
            ..Default::default()
        })
        .await
        .expect("failed to create device");
    println!(
        "device max_texture_array_layers: {}",
        device.limits().max_texture_array_layers
    );

    let width = 1_u32;
    let height = 1_u32;

    let target_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("render target"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let target_view = target_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let render_target_config = RenderTargetConfig {
        format: target_texture.format(),
        width,
        height,
    };

    // The real public entry point that reaches `MemorySettings::normalize`/`pack_image_params`:
    // configure the atlas manager with a `max_atlases` far beyond the 8-atlas default and far
    // beyond the 8-bit (255) atlas-index domain the paint-packing format actually supports.
    // `AtlasConfig::max_atlases` is a public, undocumented-as-bounded `usize` field - nothing in
    // its own type stops this.
    let mut settings = RenderSettings::default();
    settings.memory_settings.image_atlas_config = AtlasConfig {
        initial_atlas_count: 0,
        max_atlases: 300,
        atlas_size: (1, 1),
        auto_grow: true,
        allocation_strategy: vello_common::multi_atlas::AllocationStrategy::FirstFit,
    };

    let (mut renderer, mut resources) =
        Renderer::new_with(&device, &render_target_config, settings);

    // Force 257 distinct atlases: each 1x1 image fills its 1x1 atlas completely, so the
    // first-fit allocator can never reuse an earlier atlas and must auto_grow every time.
    //
    // A correctly-bounded implementation must refuse to ever hand out atlas_id 256 (the packed
    // format's 8-bit `atlas_index` field can only address 0..=255) - so on a patched tree this
    // loop is expected to fail with a clean `AtlasLimitReached` (surfaced as a panic through
    // `Renderer::upload_image`'s own internal `.unwrap()`, not something this repro adds) rather
    // than succeeding and silently corrupting the render. Both outcomes are caught below.
    const NUM_IMAGES: u32 = 257;
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("uploads"),
    });

    let upload_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut last_image_id = None;
        for i in 0..NUM_IMAGES {
            let is_last = i == NUM_IMAGES - 1;
            let color: [u8; 4] = if is_last {
                [255, 0, 0, 255] // opaque red - the "real" content of atlas_id == 256's image
            } else {
                [0, 0, 255, 255]
            };

            let src_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("source image"),
                size: wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &src_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &color,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4),
                    rows_per_image: Some(1),
                },
                wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
            );

            let image_id =
                renderer.upload_image(&mut resources, &device, &queue, &mut encoder, &src_texture);
            if is_last {
                last_image_id = Some(image_id);
            }
        }
        (renderer, resources, encoder, last_image_id.expect("uploaded at least one image"))
    }));

    let (mut renderer, mut resources, mut encoder, last_image_id) = match upload_result {
        Ok(v) => v,
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic_payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            if msg.contains("AtlasLimitReached") {
                println!(
                    "did not reproduce - allocating the 257th atlas correctly failed with \
                     AtlasLimitReached instead of silently overflowing atlas_index into the \
                     is_external bit: {msg}"
                );
                std::process::exit(0);
            } else {
                eprintln!("unexpected panic while uploading images: {msg}");
                std::process::exit(2);
            }
        }
    };
    println!("uploaded {NUM_IMAGES} images; last image id = {last_image_id:?} (should be backed by atlas_id 256)");

    let mut scene = Scene::new(width as u16, height as u16);
    scene.set_paint(Image {
        image: ImageSource::opaque_id(last_image_id),
        sampler: ImageSampler::default(),
    });
    scene.fill_rect(&Rect::new(0.0, 0.0, width as f64, height as f64));

    let render_size = RenderSize { width, height };
    let depth_view = Renderer::create_depth_texture_view(&device, &render_size);

    renderer
        .render(
            &scene,
            &mut resources,
            &device,
            &queue,
            &mut encoder,
            &render_size,
            &target_view,
            Some(&depth_view),
            &TextureBindings::new(),
        )
        .unwrap();

    let bytes_per_row = (width * 4).next_multiple_of(256);
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size: u64::from(bytes_per_row) * u64::from(height),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &target_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: None,
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit([encoder.finish()]);

    readback
        .slice(..)
        .map_async(wgpu::MapMode::Read, |result| {
            result.expect("failed to map readback buffer");
        });
    device.poll(wgpu::PollType::wait_indefinitely()).unwrap();

    let data = readback.slice(..).get_mapped_range();
    let pixel = [data[0], data[1], data[2], data[3]];
    drop(data);
    readback.unmap();

    println!("rendered pixel (expected [255, 0, 0, 255] - the opaque red image): {pixel:?}");

    if pixel == [255, 0, 0, 255] {
        println!("did not reproduce - atlas_id 256 sampled correctly from the atlas");
        std::process::exit(0);
    } else {
        eprintln!(
            "REPRODUCED - atlas_id 256's image_params bit-packing collided with the \
             is_external bit; got {pixel:?} instead of the uploaded image's real color. This \
             matches sampling the always-zero placeholder external texture instead of the atlas."
        );
        std::process::exit(1);
    }
}
