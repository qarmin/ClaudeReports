// Standalone repro for candidate 069 (VELLO_HYBRID_SCHEDULER_ROUND_DEADLOCK_PANIC).
//
// Drives vello_hybrid's real public API end-to-end: a `Scene` built purely with the crate's own
// public recording methods (`push_layer`/`pop_layer`/`set_paint`/`fill_rect`) - a real, deeply and
// widely nested layer graph, the same shape of input a real application (e.g. an SVG/PDF renderer
// hitting deeply nested groups/clips, or a procedurally generated scene) would hand to the crate -
// is fed straight into the real `Renderer::render` wgpu entry point. No second participant is
// being faked here: `Scene` is the crate's own public scene-recording API and `Renderer::render`
// is its own public rendering entry point; this *is* the real end-to-end trigger path, the same
// one `examples/render_to_file.rs` uses.
//
// The scene is a ternary tree of plain (no clip/blend/filter) layers, 3^11-1)/2 = 88,573 layers
// deep/wide, each layer's content confined to the same tiny 8x8 region. Combined with a small
// `min_texture_size` (8x8, a legitimate, supported configuration point per `LayersConfig`'s own
// docs), this defeats the scheduler's page-reuse batching enough that it must place operations
// into tens of thousands of distinct "rounds" (see `schedule/round.rs`) - on the pristine tree,
// this blows past the scheduler's internal `MAX_ROUNDS = 40_000` safety guard and panics with
// "possible deadlock in scheduler detected" instead of returning a normal `RenderError`. On the
// patched tree, `Renderer::render` instead returns `Err(RenderError::SchedulingLimitExceeded)`.
//
// Needs a real (possibly software) Vulkan adapter. E.g. SwiftShader:
//   VK_ICD_FILENAMES=/path/to/vk_swiftshader_icd.json cargo run --release
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (panic on pristine tree).

use vello_common::kurbo::Rect;
use vello_common::peniko::color::palette::css::BLUE;
use vello_hybrid::{LayersConfig, MemorySettings, RenderSettings, RenderSize, RenderTargetConfig};
use vello_hybrid::{Renderer, Scene, SizeU16, TextureBindings};

/// Depth of the ternary layer tree. (3^11 - 1) / 2 = 88,573 layers - enough to exceed the
/// scheduler's `MAX_ROUNDS = 40_000` guard once every layer needs its own scheduling round.
const DEPTH: usize = 11;
const CHILDREN: usize = 3;

fn add_tree(scene: &mut Scene, depth: usize) {
    scene.push_layer(None, None, None, None, None);
    if depth == 1 {
        scene.set_paint(BLUE.with_alpha(0.5));
        scene.fill_rect(&Rect::new(0.0, 0.0, 8.0, 8.0));
    } else {
        for _ in 0..CHILDREN {
            add_tree(scene, depth - 1);
        }
    }
    scene.pop_layer();
}

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let mut scene = Scene::new(8, 8);
    add_tree(&mut scene, DEPTH);

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
        width: 8,
        height: 8,
    };

    // A tiny minimum intermediate texture size - a legitimate, documented configuration point
    // (see `LayersConfig::min_texture_size`'s own docs about mobile GPUs preferring smaller
    // intermediate textures) - is what forces the scheduler to place almost every layer into its
    // own round instead of batching many of them into a shared atlas page.
    let settings = RenderSettings {
        memory_settings: MemorySettings {
            layers_config: LayersConfig {
                max_textures: None,
                min_texture_size: SizeU16::new(8),
                max_texture_size: SizeU16::new(4096),
            },
            ..Default::default()
        },
        ..Default::default()
    };
    let (mut renderer, mut resources) = Renderer::new_with(&device, &render_target_config, settings);

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render Target"),
        size: wgpu::Extent3d {
            width: 8,
            height: 8,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let render_size = RenderSize {
        width: 8,
        height: 8,
    };
    let depth_texture_view = Renderer::create_depth_texture_view(&device, &render_size);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("repro encoder"),
    });

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        renderer.render(
            &scene,
            &mut resources,
            &device,
            &queue,
            &mut encoder,
            &render_size,
            &texture_view,
            Some(&depth_texture_view),
            &TextureBindings::new(),
        )
    }));

    match result {
        Ok(Ok(())) => {
            println!("did not reproduce - render() completed successfully");
        }
        Ok(Err(err)) => {
            println!(
                "did not reproduce - render() returned a clean error instead of panicking: {err}"
            );
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic_payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("REPRODUCED - render() panicked: {msg}");
            std::process::exit(1);
        }
    }
}
