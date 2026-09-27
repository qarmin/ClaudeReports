// Benchmark for ___OPTIMIZATIONS___/018.VELLO_CLASSIC_LARGE_IMAGE_ATLAS_CLIFF.
//
// Investigates linebender/vello#1545: a full-screen image whose height crosses a
// power-of-two atlas-size boundary (4096 -> 4097) was reported to cause a sharp,
// *sustained* frame-rate cliff on iPad Pro. This drives the real `vello::Renderer`
// (real WGSL compute dispatch, not the CPU-only echo path) through two scenarios:
//
//   --mode cliff   Renders one full-canvas image repeatedly. Reports the first
//                  ("cold", atlas created + image uploaded) frame separately from
//                  the steady-state frames (image already resident, nothing to
//                  upload) so a per-frame cliff tied to atlas size can be told
//                  apart from a one-time atlas-creation cost.
//
//   --mode growth  Establishes several small images resident in the atlas at a
//                  stable size, runs steady-state frames, then introduces one more
//                  large image that forces the atlas to grow. Reports the growth
//                  frame's cost against the steady frames around it, to quantify
//                  the cost of forcing every already-resident image to be marked
//                  dirty (and re-uploaded) when `ImageCache::bump_size` repacks.
//
// Also prints a pixel hash every run for a before/after bit-exactness check.

use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{Duration, Instant};

use vello::kurbo::Affine;
use vello::peniko::{Blob, ImageAlphaType, ImageBrush, ImageData, ImageFormat};
use vello::peniko::color::palette;
use vello::util::{block_on_wgpu, RenderContext};
use vello::wgpu::{
    self, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d,
    TexelCopyBufferInfo, TextureDescriptor, TextureFormat, TextureUsages,
};
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene};

const CANVAS_W: u32 = 1024;
const CANVAS_H: u32 = 1024;

fn make_image(width: u32, height: u32, fill_byte: u8) -> ImageData {
    let len = (width as usize) * (height as usize) * 4;
    ImageData {
        data: Blob::new(Arc::new(vec![fill_byte; len])),
        format: ImageFormat::Rgba8,
        alpha_type: ImageAlphaType::AlphaPremultiplied,
        width,
        height,
    }
}

fn full_canvas_image_scene(image: &ImageData) -> Scene {
    let mut scene = Scene::new();
    let brush = ImageBrush::new(image.clone());
    let sx = CANVAS_W as f64 / image.width as f64;
    let sy = CANVAS_H as f64 / image.height as f64;
    scene.draw_image(&brush, Affine::scale_non_uniform(sx, sy));
    scene
}

/// A scene with `n_small` small tile-aligned images (always resident once placed)
/// plus, if `big` is `Some`, one additional full-canvas image.
fn tiled_scene(smalls: &[ImageData], big: Option<&ImageData>) -> Scene {
    let mut scene = Scene::new();
    let cols = 4;
    let tile_w = CANVAS_W as f64 / cols as f64;
    let tile_h = CANVAS_H as f64 / cols as f64;
    for (i, image) in smalls.iter().enumerate() {
        let col = (i % cols) as f64;
        let row = (i / cols) as f64;
        let brush = ImageBrush::new(image.clone());
        let sx = tile_w / image.width as f64;
        let sy = tile_h / image.height as f64;
        scene.draw_image(
            &brush,
            Affine::translate((col * tile_w, row * tile_h)).pre_scale_non_uniform(sx, sy),
        );
    }
    if let Some(image) = big {
        let brush = ImageBrush::new(image.clone());
        let sx = CANVAS_W as f64 / image.width as f64;
        let sy = CANVAS_H as f64 / image.height as f64;
        scene.draw_image(&brush, Affine::scale_non_uniform(sx, sy));
    }
    scene
}

struct Harness {
    context: RenderContext,
    device_id: usize,
    renderer: Renderer,
    target: wgpu::Texture,
    view: wgpu::TextureView,
    padded_byte_width: u32,
    buffer_size: u64,
}

impl Harness {
    async fn new() -> anyhow::Result<Self> {
        let mut context = RenderContext::new();
        let device_id = context
            .device(None)
            .await
            .ok_or_else(|| anyhow::anyhow!("No compatible device found"))?;
        let device_handle = &context.devices[device_id];
        let device = &device_handle.device;

        let renderer = Renderer::new(
            device,
            RendererOptions {
                use_cpu: false,
                num_init_threads: NonZeroUsize::new(1),
                antialiasing_support: vello::AaSupport::area_only(),
                ..Default::default()
            },
        )
        .map_err(|_| anyhow::anyhow!("Got non-Send/Sync error from creating renderer"))?;

        let size = Extent3d {
            width: CANVAS_W,
            height: CANVAS_H,
            depth_or_array_layers: 1,
        };
        let target = device.create_texture(&TextureDescriptor {
            label: Some("Target texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = target.create_view(&wgpu::TextureViewDescriptor::default());

        let padded_byte_width = (CANVAS_W * 4).next_multiple_of(256);
        let buffer_size = padded_byte_width as u64 * CANVAS_H as u64;

        Ok(Self {
            context,
            device_id,
            renderer,
            target,
            view,
            padded_byte_width,
            buffer_size,
        })
    }

    /// Renders `scene` once, returns (elapsed, pixel hash).
    async fn render_once(&mut self, scene: &Scene) -> anyhow::Result<(Duration, u64)> {
        let device_handle = &self.context.devices[self.device_id];
        let device = &device_handle.device;
        let queue = &device_handle.queue;

        let render_params = RenderParams {
            base_color: palette::css::BLACK,
            width: CANVAS_W,
            height: CANVAS_H,
            antialiasing_method: AaConfig::Area,
        };

        let start = Instant::now();
        self.renderer
            .render_to_texture(device, queue, scene, &self.view, &render_params)
            .map_err(|_| anyhow::anyhow!("Got non-Send/Sync error from rendering"))?;

        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("readback"),
            size: self.buffer_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Copy out buffer"),
        });
        encoder.copy_texture_to_buffer(
            self.target.as_image_copy(),
            TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(self.padded_byte_width),
                    rows_per_image: None,
                },
            },
            Extent3d {
                width: CANVAS_W,
                height: CANVAS_H,
                depth_or_array_layers: 1,
            },
        );
        queue.submit([encoder.finish()]);
        let buf_slice = buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buf_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        block_on_wgpu(device, receiver.receive())
            .ok_or_else(|| anyhow::anyhow!("channel was closed"))??;
        let elapsed = start.elapsed();

        let mut hash: u64 = 1469598103934665603;
        {
            let data = buf_slice.get_mapped_range();
            for row in 0..CANVAS_H {
                let s = (row * self.padded_byte_width) as usize;
                for b in &data[s..s + (CANVAS_W * 4) as usize] {
                    hash ^= *b as u64;
                    hash = hash.wrapping_mul(1099511628211);
                }
            }
        }
        buffer.unmap();

        Ok((elapsed, hash))
    }
}

fn summarize(label: &str, durations: &[Duration]) {
    let mut sorted = durations.to_vec();
    sorted.sort();
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    let mean: Duration = sorted.iter().sum::<Duration>() / sorted.len() as u32;
    let p50 = sorted[sorted.len() / 2];
    println!("{label}: n={} min={min:?} p50={p50:?} mean={mean:?} max={max:?}", sorted.len());
}

async fn run_cliff(height: u32, iterations: u32) -> anyhow::Result<()> {
    // Aspect roughly matching the reported 1835x8192 image.
    let width = (height as f64 * (1835.0 / 8192.0)).round().max(1.0) as u32;
    let image = make_image(width, height, 0x7f);
    let scene = full_canvas_image_scene(&image);

    let mut harness = Harness::new().await?;

    let (first_elapsed, first_hash) = harness.render_once(&scene).await?;
    println!("mode=cliff height={height} width={width} canvas={CANVAS_W}x{CANVAS_H}");
    println!("first_frame={first_elapsed:?} pixel_hash_fnv1a={first_hash:016x}");

    let mut steady = Vec::with_capacity(iterations as usize);
    let mut last_hash = first_hash;
    for _ in 0..iterations {
        let (elapsed, hash) = harness.render_once(&scene).await?;
        steady.push(elapsed);
        last_hash = hash;
    }
    summarize("steady", &steady);
    println!("steady_pixel_hash_fnv1a={last_hash:016x}");
    Ok(())
}

async fn run_growth(iterations_before: u32, iterations_after: u32) -> anyhow::Result<()> {
    // 8 small images comfortably resident in the default 1024x1024 atlas.
    let smalls: Vec<ImageData> = (0..8u8)
        .map(|i| make_image(128, 128, 0x10 + i))
        .collect();
    let steady_scene = tiled_scene(&smalls, None);

    let mut harness = Harness::new().await?;

    // Warm up: place the small images and reach a stable, all-clean atlas.
    for _ in 0..3 {
        harness.render_once(&steady_scene).await?;
    }

    let mut before = Vec::with_capacity(iterations_before as usize);
    for _ in 0..iterations_before {
        let (elapsed, _hash) = harness.render_once(&steady_scene).await?;
        before.push(elapsed);
    }
    summarize("steady_before_growth", &before);

    // One large image that does not fit in the current atlas size, forcing
    // `ImageCache::bump_size` to repack (and mark every existing resident dirty).
    let big = make_image(920, 4096, 0xaa);
    let growth_scene = tiled_scene(&smalls, Some(&big));
    let (growth_elapsed, _hash) = harness.render_once(&growth_scene).await?;
    println!("growth_frame={growth_elapsed:?}");

    let mut after = Vec::with_capacity(iterations_after as usize);
    for _ in 0..iterations_after {
        let (elapsed, hash) = harness.render_once(&growth_scene).await?;
        after.push(elapsed);
        let _ = hash;
    }
    summarize("steady_after_growth", &after);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    let mode = args
        .iter()
        .position(|a| a == "--mode")
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
        .unwrap_or("cliff");
    let height: u32 = args
        .iter()
        .position(|a| a == "--height")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(4096);
    let iterations: u32 = args
        .iter()
        .position(|a| a == "--iterations")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);

    match mode {
        "cliff" => pollster::block_on(run_cliff(height, iterations)),
        "growth" => pollster::block_on(run_growth(iterations, iterations)),
        other => Err(anyhow::anyhow!("unknown --mode {other}")),
    }
}
