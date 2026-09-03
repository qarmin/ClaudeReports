// Candidate 097 repro.
//
// Demonstrates that `vello_hybrid`'s wgpu backend crashes on a real, non-adversarial scene (many
// small non-overlapping opaque rects drawn in a single frame - e.g. a densely-packed grid/mosaic
// or many independently-styled tiny UI elements) whenever the resulting `GpuStrip` buffer for a
// single draw pass exceeds the device's `max_buffer_size`
// (`sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`'s `create_strips_buffer`/
// `upload_strip_pair`, called from `strip_pass_inner`). Unlike the atlas/gradient/filter-texture
// resize functions (candidates 026/055/090), this call site has *no* size check at all before
// calling `Device::create_buffer` - the strips buffer is created directly from the frame's real
// strip count with no clamp, no assert, and no `Result` path. wgpu's own default "uncaptured
// error" handler turns the resulting validation error into a hard process panic.
//
// Every axis-aligned, pixel-boundary-aligned `fill_rect` call produces exactly ONE `GpuStrip`
// (24 bytes) regardless of the rect's size (`sparse_strips/vello_hybrid/src/rect.rs`'s
// "rectangle fast path" - see `splitter_keeps_large_aligned_rect_as_single_main_rect`), so a
// scene with enough distinct rects reliably crosses `wgpu::Limits::default()`'s spec-mandated
// `max_buffer_size` of 256 MiB (11,184,811 rects/strips is the exact byte-level boundary; this
// repro defaults to 12,000,000, comfortably over it). Each rect uses a distinct solid color so
// that no two adjacent strips are coalesced into a single wider run by strip generation, which
// would otherwise silently shrink the real strip count below what the draw-call count implies.
//
// This binary is unchanged by the patch: `vello_hybrid::{Scene, Renderer}`'s public API doesn't
// change shape before/after the fix - only `render()`'s returned `Result` gains a new `Err`
// variant - so the same binary can be run against a pristine clone (panics deep inside wgpu's own
// validation, never returning a `Result` at all) and again against a patched clone (returns
// `Err(RenderError::StripsBufferTooLarge)` gracefully instead) to demonstrate the fix.
//
// Usage:
//   COLS=4000 ROWS=3000 cargo run --release
//     -> 12,000,000 rects (288,000,000 bytes): panics against a pristine checkout (wgpu
//        validation panic on `Device::create_buffer`, label "Strips Buffer"); against a patched
//        checkout, returns `Err(RenderError::StripsBufferTooLarge { required: 288000000, max:
//        268435456 })` gracefully instead.
//   COLS=3000 ROWS=3000 cargo run --release
//     -> 9,000,000 rects (216,000,000 bytes, under the 256 MiB default limit): renders
//        successfully either way (before and after the fix).
//   HIGH_LIMITS=1 COLS=4000 ROWS=3000 cargo run --release
//     -> requests the adapter's own (higher) real `max_buffer_size` instead of the WebGPU-spec
//        default. On an adapter reporting >= 288,000,000 bytes (e.g. this environment's
//        SwiftShader software Vulkan adapter, which reports 1 GiB), the same 12M-rect scene
//        renders successfully both before AND after the fix - demonstrating the fix tracks the
//        device's real capacity rather than rejecting large scenes unconditionally.

use std::time::Instant;
use vello_common::kurbo::{Affine, Rect};
use vello_common::peniko::Color;
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
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find a Vulkan adapter (set VK_ICD_FILENAMES to a usable ICD, e.g. a SwiftShader build)");
    eprintln!("adapter: {:?}", adapter.get_info());

    // Explicitly request the WebGPU-spec-mandated default limits, matching what a typical
    // embedder requests unless it deliberately opts into higher limits. Set HIGH_LIMITS=1 to
    // instead request the adapter's own (higher) real limits - see the module doc comment above.
    let required_limits = if std::env::var("HIGH_LIMITS").is_ok() {
        adapter.limits()
    } else {
        wgpu::Limits::default()
    };
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("dev"),
            required_features: wgpu::Features::empty(),
            required_limits,
            ..Default::default()
        })
        .await
        .expect("Failed to create device");
    eprintln!("max_buffer_size = {}", device.limits().max_buffer_size);

    let cols: u32 = std::env::var("COLS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(4000);
    let rows: u32 = std::env::var("ROWS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3000);
    let width: u16 = u16::try_from(cols).unwrap();
    let height: u16 = u16::try_from(rows).unwrap();

    eprintln!(
        "Building scene with {cols} x {rows} = {} distinct 1x1 opaque rects...",
        u64::from(cols) * u64::from(rows)
    );
    let t0 = Instant::now();
    let mut scene = Scene::new(width, height);
    scene.set_transform(Affine::IDENTITY);

    let mut i: u32 = 0;
    for y in 0..rows {
        for x in 0..cols {
            // Distinct color per rect guarantees a distinct paint per strip, so adjacent
            // strips are never coalesced into a single wider run during strip generation - this
            // is the real, public Scene::set_paint/fill_rect path a real caller uses, not a
            // hand-built stand-in for internal strip state.
            let r = (i & 0xFF) as u8;
            let g = ((i >> 8) & 0xFF) as u8;
            let b = ((i >> 16) & 0xFF) as u8;
            scene.set_paint(Color::from_rgba8(r, g, b, 255));
            scene.fill_rect(&Rect::new(
                f64::from(x),
                f64::from(y),
                f64::from(x) + 1.0,
                f64::from(y) + 1.0,
            ));
            i = i.wrapping_add(1);
        }
    }
    eprintln!("Scene built in {:?}", t0.elapsed());

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
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Vello Render To Buffer"),
    });

    let t1 = Instant::now();
    let result = renderer.render(
        &scene,
        &mut resources,
        &device,
        &queue,
        &mut encoder,
        &render_size,
        &texture_view,
        None,
        &TextureBindings::new(),
    );
    eprintln!("render() returned in {:?}", t1.elapsed());

    // A real caller handles the `Result` instead of unwrapping it - this is exactly the
    // graceful-degradation path the fix enables. Before the fix, `render()` never returns at
    // all for this input: `Device::create_buffer` panics deep inside wgpu's own validation
    // before this code ever sees a `Result`.
    match result {
        Ok(()) => {
            queue.submit([encoder.finish()]);
            device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
            println!(
                "Rendered {} rects without panicking.",
                u64::from(cols) * u64::from(rows)
            );
        }
        Err(err) => {
            println!("render() returned a graceful error instead of panicking: {err}");
        }
    }
}
