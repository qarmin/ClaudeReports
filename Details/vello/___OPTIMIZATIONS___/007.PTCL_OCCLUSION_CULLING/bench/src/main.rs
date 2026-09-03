// Benchmark for ___OPTIMIZATIONS___/007.PTCL_OCCLUSION_CULLING.
//
// Builds a scene of many fully-opaque, tile-aligned rectangles stacked on top of
// each other (the exact "many draws stack in the same small screen area" shape
// described in https://github.com/linebender/vello/issues/409), renders it
// repeatedly through the real `vello` GPU pipeline, and reports wall-clock time.
// Also writes a hash of the rendered pixels so a caller can diff patched vs.
// baseline output for a bit-exactness check.

use std::num::NonZeroUsize;
use std::time::Instant;

use vello::kurbo::{Affine, Rect};
use vello::peniko::color::palette;
use vello::peniko::Fill;
use vello::util::{block_on_wgpu, RenderContext};
use vello::wgpu::{
    self, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d,
    TexelCopyBufferInfo, TextureDescriptor, TextureFormat, TextureUsages,
};
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const LAYERS: u32 = 400;
const ITERATIONS: u32 = 30;
const WARMUP: u32 = 5;

fn build_scene(layers: u32) -> Scene {
    let mut scene = Scene::new();
    // Every layer fully covers the canvas, tile-aligned (16px tiles), with full
    // opacity - the worst case for issue #409: only the last layer is ever visible,
    // but a naive PTCL still carries every earlier one.
    for i in 0..layers {
        let color = if i % 2 == 0 {
            palette::css::RED
        } else {
            palette::css::BLUE
        };
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            color,
            None,
            &Rect::new(0.0, 0.0, WIDTH as f64, HEIGHT as f64),
        );
    }
    scene
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    pollster::block_on(run())
}

async fn run() -> anyhow::Result<()> {
    let use_cpu = std::env::args().any(|a| a == "--use-cpu");
    let mut context = RenderContext::new();
    let device_id = context
        .device(None)
        .await
        .ok_or_else(|| anyhow::anyhow!("No compatible device found"))?;
    let device_handle = &mut context.devices[device_id];
    let device = &device_handle.device;
    let queue = &device_handle.queue;

    let mut renderer = Renderer::new(
        device,
        RendererOptions {
            use_cpu,
            num_init_threads: NonZeroUsize::new(1),
            antialiasing_support: vello::AaSupport::area_only(),
            ..Default::default()
        },
    )
    .map_err(|_| anyhow::anyhow!("Got non-Send/Sync error from creating renderer"))?;

    let scene = build_scene(LAYERS);
    let render_params = RenderParams {
        base_color: palette::css::BLACK,
        width: WIDTH,
        height: HEIGHT,
        antialiasing_method: AaConfig::Area,
    };

    let size = Extent3d {
        width: WIDTH,
        height: HEIGHT,
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

    let padded_byte_width = (WIDTH * 4).next_multiple_of(256);
    let buffer_size = padded_byte_width as u64 * HEIGHT as u64;

    let mut durations = Vec::with_capacity(ITERATIONS as usize);
    let mut last_pixels: Vec<u8> = Vec::new();
    for i in 0..(WARMUP + ITERATIONS) {
        let start = Instant::now();
        renderer
            .render_to_texture(device, queue, &scene, &view, &render_params)
            .map_err(|_| anyhow::anyhow!("Got non-Send/Sync error from rendering"))?;

        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("readback"),
            size: buffer_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Copy out buffer"),
        });
        encoder.copy_texture_to_buffer(
            target.as_image_copy(),
            TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_byte_width),
                    rows_per_image: None,
                },
            },
            size,
        );
        queue.submit([encoder.finish()]);
        let buf_slice = buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buf_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        block_on_wgpu(device, receiver.receive())
            .ok_or_else(|| anyhow::anyhow!("channel was closed"))??;
        let elapsed = start.elapsed();
        {
            let data = buf_slice.get_mapped_range();
            if i == WARMUP + ITERATIONS - 1 {
                let mut result_unpadded = Vec::with_capacity((WIDTH * HEIGHT * 4) as usize);
                for row in 0..HEIGHT {
                    let s = (row * padded_byte_width) as usize;
                    result_unpadded.extend(&data[s..s + (WIDTH * 4) as usize]);
                }
                last_pixels = result_unpadded;
            }
        }
        buffer.unmap();

        if i >= WARMUP {
            durations.push(elapsed);
        }
    }

    durations.sort();
    let min = durations[0];
    let max = durations[durations.len() - 1];
    let mean: std::time::Duration = durations.iter().sum::<std::time::Duration>() / durations.len() as u32;
    let p50 = durations[durations.len() / 2];

    // Cheap non-cryptographic hash of the rendered pixels, for a before/after
    // bit-exactness comparison without shipping a whole PNG.
    let mut hash: u64 = 1469598103934665603;
    for b in &last_pixels {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(1099511628211);
    }

    println!("layers={LAYERS} size={WIDTH}x{HEIGHT} use_cpu={use_cpu}");
    println!(
        "min={:?} p50={:?} mean={:?} max={:?}",
        min, p50, mean, max
    );
    println!("pixel_hash_fnv1a={hash:016x}");

    Ok(())
}
