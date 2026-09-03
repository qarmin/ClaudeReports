// Candidate 055 repro.
//
// Demonstrates that `vello_hybrid`'s wgpu backend panics on a real, non-adversarial scene (many
// distinct gradients drawn in a single frame) well below what the *real* GPU device it's running
// on actually supports, purely because `DeviceLimits::resource_texture_dimension_2d` hardcodes an
// artificial 4096 cap (see sparse_strips/vello_hybrid/src/render/common.rs) regardless of the
// device's own `max_texture_dimension_2d`.
//
// This binary is unchanged by the patch: `vello_hybrid::{Scene, Renderer}`'s public API doesn't
// change shape before/after the fix - only internal bookkeeping does - so the same binary can be
// run against a pristine clone (panics once NUM_GRADIENTS crosses the threshold below) and again
// against a patched clone (no longer panics at the same count, only once the *real* device budget
// is exceeded) to demonstrate the fix.
//
// Usage: NUM_GRADIENTS=<n> cargo run --release
//   NUM_GRADIENTS=4200 -> panics against a pristine checkout, succeeds against a patched one.
//   NUM_GRADIENTS=4000 -> succeeds either way (under the artificial cap).
//   NUM_GRADIENTS=8500 -> panics either way (exceeds this adapter's real 8192 limit).

use vello_common::color::DynamicColor;
use vello_common::kurbo::{Affine, Point, Rect};
use vello_common::peniko::{Color, ColorStop, ColorStops, Gradient, LinearGradientPosition};
use vello_hybrid::{RenderSize, RenderTargetConfig, Renderer, Scene, TextureBindings};

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Use a real Vulkan adapter (SwiftShader software implementation works fine here) that
    // reports a max_texture_dimension_2d well above the hardcoded 4096 cap in vello_hybrid, so
    // any panic below is due to the artificial cap, not the device's real limit.
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
    assert!(
        adapter_limits.max_texture_dimension_2d > 4096,
        "this repro requires a device whose real limit exceeds vello_hybrid's artificial 4096 cap"
    );

    let mut required_limits = wgpu::Limits::downlevel_defaults();
    required_limits.max_texture_dimension_2d = adapter_limits.max_texture_dimension_2d;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("Device"),
            required_features: wgpu::Features::empty(),
            required_limits,
            ..Default::default()
        })
        .await
        .expect("Failed to create device");

    let width: u16 = 64;
    let height: u16 = 64;

    let mut scene = Scene::new(width, height);
    scene.set_transform(Affine::IDENTITY);

    // Fill the same tiny rect over and over, each time with a distinct gradient (distinct color
    // per draw => distinct GradientCacheKey => guaranteed cache miss) whose stops are spaced
    // closely enough to force the maximum 4096-wide LUT (see
    // vello_common::encode::determine_lut_size). All of this happens within a single
    // frame/Scene, before GradientRampCache::maintain() ever runs (it only runs at the very end
    // of Renderer::render), so nothing here dedups or evicts mid-frame - this is exactly the
    // real, public Scene::set_paint/fill_rect path a real caller uses, not a hand-built stand-in
    // for internal state.
    let num_gradients: usize = std::env::var("NUM_GRADIENTS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(4200);
    eprintln!("Drawing {num_gradients} distinct max-width gradients in a single frame...");

    for i in 0..num_gradients {
        let g = u32::try_from(i).unwrap_or(u32::MAX);
        let color_mid = Color::new([
            f32::from((g >> 16) as u8) / 255.0,
            f32::from((g >> 8) as u8) / 255.0,
            f32::from(g as u8) / 255.0,
            1.0,
        ]);
        // Offset stays within the band that forces determine_lut_size() to pick the maximum
        // 4096-wide LUT, while still being unique per iteration so every gradient is a distinct
        // cache entry.
        let offset = 0.0003 + (i as f32) * 2e-8;
        let gradient = Gradient {
            kind: LinearGradientPosition {
                start: Point::new(0.0, 0.0),
                end: Point::new(f64::from(width), 0.0),
            }
            .into(),
            stops: ColorStops(
                vec![
                    ColorStop {
                        offset: 0.0,
                        color: DynamicColor::from_alpha_color(color_mid),
                    },
                    ColorStop {
                        offset,
                        color: DynamicColor::from_alpha_color(Color::new([1.0, 1.0, 1.0, 1.0])),
                    },
                    ColorStop {
                        offset: 1.0,
                        color: DynamicColor::from_alpha_color(color_mid),
                    },
                ]
                .into(),
            ),
            ..Default::default()
        };

        scene.set_paint(gradient);
        scene.fill_rect(&Rect::new(0.0, 0.0, f64::from(width), f64::from(height)));
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

    println!("Rendered {num_gradients} distinct gradients without panicking.");
}
