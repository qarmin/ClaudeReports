// Candidate 090 repro.
//
// Demonstrates that `vello_hybrid`'s wgpu backend panics on a real scene containing enough
// distinct filter layers in a single frame (e.g. many small Gaussian-blur or offset filters),
// purely because `FilterContext::required_filter_data_height` asserts the required filter-data
// texture height against `DeviceLimits::resource_texture_dimension_2d()` (hardcoded 4096 cap,
// see sparse_strips/vello_hybrid/src/render/common.rs) instead of returning an error - the same
// bug class candidate 055 already confirmed for the alpha/encoded-paints/gradient textures, but
// for the filter-data texture, which 055 explicitly left as an open question.
//
// This binary is unchanged by any fix to `common.rs`'s cap: `vello_hybrid::{Scene, Renderer}`'s
// public API doesn't change shape - only internal bookkeeping does - so the same binary can be
// run against a pristine clone (panics once NUM_FILTERS crosses the threshold below) and again
// against a patched clone to demonstrate the fix.
//
// Usage: NUM_FILTERS=<n> cargo run --release
//   NUM_FILTERS=5600000 -> panics against a pristine checkout (exceeds the 4096x4096 texel cap).
//   NUM_FILTERS=5000000 -> succeeds (under the artificial cap).

use vello_common::filter_effects::{Filter, FilterPrimitive};
use vello_common::kurbo::{Affine, Rect};
use vello_hybrid::{RenderSize, RenderTargetConfig, Renderer, Scene, TextureBindings};

fn main() {
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
        .expect("Failed to find a Vulkan adapter (set VK_ICD_FILENAMES to a usable ICD, e.g. a SwiftShader build)");

    let adapter_limits = adapter.limits();
    eprintln!(
        "adapter: {:?}, max_texture_dimension_2d = {}",
        adapter.get_info().name,
        adapter_limits.max_texture_dimension_2d
    );

    let mut required_limits = wgpu::Limits::downlevel_defaults();
    required_limits.max_texture_dimension_2d = adapter_limits.max_texture_dimension_2d;
    required_limits.max_buffer_size = adapter_limits.max_buffer_size;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("Device"),
            required_features: wgpu::Features::empty(),
            required_limits,
            ..Default::default()
        })
        .await
        .expect("Failed to create device");

    let width: u16 = 8;
    let height: u16 = 8;

    let mut scene = Scene::new(width, height);
    scene.set_transform(Affine::IDENTITY);

    let num_filters: usize = std::env::var("NUM_FILTERS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5_600_000);
    eprintln!("Pushing {num_filters} distinct filter layers in a single frame...");

    // Every filter layer gets an `Offset` primitive (the cheapest filter to encode, 48 bytes
    // == 3 texels via `GpuFilterData::SIZE_TEXELS`) with a tiny draw inside it, so
    // `ensure_layer_target` actually allocates a target and calls
    // `FilterContext::push` for every single one of these - exactly the real, public
    // `Scene::push_filter_layer`/`fill_rect`/`pop_layer` call sequence a real caller (an
    // embedder driving many independently-filtered small elements, e.g. from a generated SVG or
    // web page) would make, not a hand-built stand-in for `FilterContext`'s internal state.
    for i in 0..num_filters {
        let dx = (i % 3) as f32 - 1.0;
        let filter = Filter::from_primitive(FilterPrimitive::Offset { dx, dy: 0.0 });
        scene.push_filter_layer(filter);
        scene.fill_rect(&Rect::new(0.0, 0.0, 1.0, 1.0));
        scene.pop_layer();
    }

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render Target"),
        size: wgpu::Extent3d {
            width: width.into(),
            height: height.into(),
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

    let (mut renderer, mut resources) = Renderer::new(
        &device,
        &RenderTargetConfig {
            format: texture.format(),
            width: width.into(),
            height: height.into(),
        },
    );
    let render_size = RenderSize {
        width: width.into(),
        height: height.into(),
    };
    let depth_texture_view = Renderer::create_depth_texture_view(&device, &render_size);
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Vello Render To Buffer"),
    });

    eprintln!("Calling Renderer::render...");
    renderer
        .render(
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
        .unwrap();

    queue.submit([encoder.finish()]);
    device.poll(wgpu::PollType::wait_indefinitely()).unwrap();

    println!("Rendered {num_filters} distinct filter layers without panicking.");
}
