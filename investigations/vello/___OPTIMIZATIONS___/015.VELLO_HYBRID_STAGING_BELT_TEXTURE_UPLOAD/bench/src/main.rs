//! Standalone GPU bench for vello_hybrid's wgpu backend: renders the Ghostscript Tiger scene
//! repeatedly (like open PR #1532's own reported methodology) and times the CPU-side cost of
//! `Renderer::render()` (encoding + any per-call staging allocation/copy), not GPU execution time.
//!
//! Every iteration's rendered output is read back and hashed; the hash of the final frame is
//! printed so a baseline vs. patched run can be compared for bit-exactness.

use sha2::{Digest, Sha256};
use std::time::Instant;
use vello_common::kurbo::{Affine, Stroke};
use vello_common::pico_svg::{Item, PicoSvg};
use vello_common::pixmap::PixelMetadata;
use vello_hybrid::{DimensionConstraints, Scene};

fn render_svg(ctx: &mut Scene, items: &[Item], transform: Affine) {
    ctx.set_transform(transform);
    for item in items {
        match item {
            Item::Fill(fill_item) => {
                ctx.set_paint(fill_item.color);
                ctx.fill_path(&fill_item.path);
            }
            Item::Stroke(stroke_item) => {
                let style = Stroke::new(stroke_item.width);
                ctx.set_stroke(style);
                ctx.set_paint(stroke_item.color);
                ctx.stroke_path(&stroke_item.path);
            }
            Item::Group(group_item) => {
                render_svg(ctx, &group_item.children, transform * group_item.affine);
                ctx.set_transform(transform);
            }
        }
    }
}

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let mut args = std::env::args().skip(1);
    let frames: usize = args
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1200);

    let svg = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/Ghostscript_Tiger.svg"),
    )
    .expect("error reading tiger svg");
    let render_scale = 5.0;
    let parsed = PicoSvg::load(&svg, 1.0).expect("error parsing SVG");

    let constraints = DimensionConstraints::default();
    let svg_width = parsed.size.width * render_scale;
    let svg_height = parsed.size.height * render_scale;
    let (width, height) = constraints.calculate_dimensions(svg_width, svg_height);
    let width = DimensionConstraints::convert_dimension(width);
    let height = DimensionConstraints::convert_dimension(height);

    let mut scene = Scene::new(width, height);
    render_svg(&mut scene, &parsed.items, Affine::scale(render_scale));

    let backends = std::env::var("WGPU_BACKEND")
        .ok()
        .and_then(|s| match s.to_lowercase().as_str() {
            "vulkan" => Some(wgpu::Backends::VULKAN),
            "gl" | "opengl" => Some(wgpu::Backends::GL),
            _ => None,
        })
        .unwrap_or(wgpu::Backends::VULKAN);
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("Failed to find an appropriate adapter");
    eprintln!("adapter: {:?}", adapter.get_info());
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("Device"),
            required_features: wgpu::Features::empty(),
            ..Default::default()
        })
        .await
        .expect("Failed to create device");

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

    let (mut renderer, mut resources) = vello_hybrid::Renderer::new(
        &device,
        &vello_hybrid::RenderTargetConfig {
            format: texture.format(),
            width: width.into(),
            height: height.into(),
        },
    );
    let render_size = vello_hybrid::RenderSize {
        width: width.into(),
        height: height.into(),
    };
    let depth_texture_view = vello_hybrid::Renderer::create_depth_texture_view(&device, &render_size);

    let bytes_per_row = (u32::from(width) * 4).next_multiple_of(256);
    let texture_copy_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: u64::from(bytes_per_row) * u64::from(height),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let warmup = frames / 10;
    let mut durations = Vec::with_capacity(frames);
    let mut last_hash = String::new();

    for i in 0..frames {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Vello Render To Buffer"),
        });

        let start = Instant::now();
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
                &vello_hybrid::TextureBindings::new(),
            )
            .unwrap();
        queue.submit([encoder.finish()]);
        let elapsed = start.elapsed();
        if i >= warmup {
            durations.push(elapsed);
        }

        // A real windowed app is frame-paced (vsync/present), so the CPU never gets more than
        // ~1 frame ahead of the GPU. Block until this frame's submission completes so pending
        // `map_async` callbacks (staging-belt recall, in the patched build) actually fire and
        // buffers get recycled - without this, an unthrottled loop against a slow (here,
        // software) backend lets frames queue up faster than the GPU drains them, which starves
        // the belt of `recall()`-able chunks and defeats the whole point of reusing them.
        device.poll(wgpu::PollType::wait_indefinitely()).unwrap();

        // Hash only the final frame's readback (readback every frame would dominate the timing
        // and isn't part of what this bench is measuring).
        if i == frames - 1 {
            let mut readback_encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Readback"),
                });
            readback_encoder.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyBufferInfo {
                    buffer: &texture_copy_buffer,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(bytes_per_row),
                        rows_per_image: None,
                    },
                },
                wgpu::Extent3d {
                    width: width.into(),
                    height: height.into(),
                    depth_or_array_layers: 1,
                },
            );
            queue.submit([readback_encoder.finish()]);

            texture_copy_buffer
                .slice(..)
                .map_async(wgpu::MapMode::Read, move |result| {
                    if result.is_err() {
                        panic!("Failed to map texture for reading");
                    }
                });
            device.poll(wgpu::PollType::wait_indefinitely()).unwrap();

            let mut img_data = Vec::with_capacity(usize::from(width) * usize::from(height) * 4);
            for row in texture_copy_buffer
                .slice(..)
                .get_mapped_range()
                .chunks_exact(bytes_per_row as usize)
            {
                img_data.extend_from_slice(&row[0..usize::from(width) * 4]);
            }
            texture_copy_buffer.unmap();

            let mut hasher = Sha256::new();
            hasher.update(&img_data);
            last_hash = format!("{:x}", hasher.finalize());

            // Silence "unused" for PixelMetadata import in case future edits drop the usage.
            let _ = PixelMetadata::default();
        }
    }

    durations.sort();
    let n = durations.len();
    let min = durations[0];
    let p50 = durations[n / 2];
    let mean: f64 = durations.iter().map(|d| d.as_secs_f64()).sum::<f64>() / n as f64;
    let p95 = durations[(n * 95) / 100];
    let max = durations[n - 1];
    let total: f64 = durations.iter().map(|d| d.as_secs_f64()).sum();

    println!(
        "frames={frames} measured={n} total_render_ms={:.3} min_us={:.1} p50_us={:.1} mean_us={:.1} p95_us={:.1} max_us={:.1} hash={last_hash}",
        total * 1000.0,
        min.as_secs_f64() * 1e6,
        p50.as_secs_f64() * 1e6,
        mean * 1e6,
        p95.as_secs_f64() * 1e6,
        max.as_secs_f64() * 1e6,
    );
}
