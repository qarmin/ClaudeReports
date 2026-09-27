// Copyright 2026 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Standalone benchmark for candidate 013 (VELLO_HYBRID_BILINEAR_IMAGE_SAMPLER).
//!
//! Measures `Scene::draw_image`'s GPU-native bilinear fast path (added by `patch.diff`)
//! against the existing manual-bilinear `set_paint(Image)` path. Renders a grid of small
//! tinted images tiled across a canvas, `ITERS` times, timing wall-clock per frame
//! (device-polled so GPU work is actually waited on, not just submitted).
//!
//! Usage: `cargo run --release -- <manual|fast>`
//!
//! `manual` only exists on the baseline (pre-patch) `vello_hybrid`; `fast` (i.e.
//! `Scene::draw_image`) only exists once `patch.diff` is applied - that's why the justfile
//! builds two separate binaries (`bin_baseline` running `manual`, `bin_patched` running
//! `fast`) rather than passing the mode as a runtime switch against one binary.

use std::sync::Arc;
use std::time::Instant;
use vello_common::color::PremulRgba8;
use vello_common::kurbo::{Affine, Rect};
use vello_common::paint::ImageSource;
use vello_common::pixmap::Pixmap;
use vello_hybrid::{RenderSize, RenderTargetConfig, Renderer, Scene, TextureBindings};

const CANVAS: u16 = 1024;
const IMG_SIZE: u16 = 48;
const GRID: u16 = 16; // GRID x GRID images per frame = 256 draws/frame.
const ITERS: usize = 60;

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "fast".to_string());
    pollster::block_on(run(&mode));
}

fn make_test_pixmap(size: u16) -> Pixmap {
    let mut pixmap = Pixmap::new(size, size);
    for y in 0..size {
        for x in 0..size {
            let r = (x * 255 / size.max(1)) as u8;
            let g = (y * 255 / size.max(1)) as u8;
            // Vary alpha so bilinear sampling near edges is exercised meaningfully.
            let a: u8 = if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
                180
            } else {
                255
            };
            pixmap.set_pixel(
                x,
                y,
                PremulRgba8 {
                    r: (u16::from(r) * u16::from(a) / 255) as u8,
                    g: (u16::from(g) * u16::from(a) / 255) as u8,
                    b: (u16::from(255 - r) * u16::from(a) / 255) as u8,
                    a,
                },
            );
        }
    }
    pixmap
}

async fn run(mode: &str) {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find an adapter");
    eprintln!("Using adapter: {:?}", adapter.get_info());
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
            width: CANVAS.into(),
            height: CANVAS.into(),
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
            width: CANVAS.into(),
            height: CANVAS.into(),
        },
    );
    let render_size = RenderSize {
        width: CANVAS.into(),
        height: CANVAS.into(),
    };
    let depth_texture_view = Renderer::create_depth_texture_view(&device, &render_size);

    let pixmap = Arc::new(make_test_pixmap(IMG_SIZE));
    let image_id = {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let id = renderer.upload_image(&mut resources, &device, &queue, &mut encoder, &pixmap);
        queue.submit([encoder.finish()]);
        id
    };
    let image_source =
        ImageSource::opaque_id_with_transparency_hint(image_id, pixmap.may_have_transparency());

    let build_scene = || {
        let mut scene = Scene::new(CANVAS, CANVAS);
        let step = f64::from(CANVAS) / f64::from(GRID);
        for gy in 0..GRID {
            for gx in 0..GRID {
                // Fractional offset (+ non-uniform scale) so the image-to-device transform
                // is neither an integer translation nor 1:1 scale: `encode_into` downgrades
                // `Medium` (bilinear) to `Low` (nearest-neighbor) for integer-only
                // translations, which would silently make the "manual" mode sample
                // nearest-neighbor instead of the intended manual-bilinear path.
                let x = f64::from(gx) * step + 2.37;
                let y = f64::from(gy) * step + 2.61;
                scene.set_transform(Affine::translate((x, y)) * Affine::scale(1.0625));
                let rect = Rect::new(0.0, 0.0, f64::from(IMG_SIZE), f64::from(IMG_SIZE));
                #[cfg(feature = "fast_path")]
                {
                    scene.draw_image(image_source.clone(), &rect, true);
                }
                #[cfg(not(feature = "fast_path"))]
                {
                    use vello_common::paint::Image;
                    use vello_common::peniko::{Extend, ImageQuality, ImageSampler};
                    scene.set_paint(Image {
                        image: image_source.clone(),
                        sampler: ImageSampler {
                            x_extend: Extend::Pad,
                            y_extend: Extend::Pad,
                            quality: ImageQuality::Medium,
                            alpha: 1.0,
                        },
                    });
                    scene.fill_rect(&rect);
                }
                let _ = mode;
            }
        }
        scene
    };

    let mut durations = Vec::with_capacity(ITERS);
    for i in 0..ITERS {
        let scene = build_scene();
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
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
                &TextureBindings::new(),
            )
            .unwrap();
        queue.submit([encoder.finish()]);
        device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
        let elapsed = start.elapsed();
        if i >= ITERS / 10 {
            durations.push(elapsed);
        }
        std::hint::black_box(&scene);
    }

    durations.sort();
    let n = durations.len();
    let sum: std::time::Duration = durations.iter().sum();
    let mean = sum / n as u32;
    println!("mode={mode} n={n}");
    println!("  min={:?}", durations[0]);
    println!("  p50={:?}", durations[n / 2]);
    println!("  mean={mean:?}");
    println!("  p95={:?}", durations[(n * 95) / 100]);
    println!("  max={:?}", durations[n - 1]);

    // Correctness readback (last frame).
    let bytes_per_row = (u32::from(CANVAS) * 4).next_multiple_of(256);
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Readback"),
        size: u64::from(bytes_per_row) * u64::from(CANVAS),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buf,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: None,
            },
        },
        wgpu::Extent3d {
            width: CANVAS.into(),
            height: CANVAS.into(),
            depth_or_array_layers: 1,
        },
    );
    queue.submit([encoder.finish()]);
    buf.slice(..).map_async(wgpu::MapMode::Read, |r| r.unwrap());
    device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
    let mut img_data = Vec::with_capacity(usize::from(CANVAS) * usize::from(CANVAS) * 4);
    for row in buf
        .slice(..)
        .get_mapped_range()
        .chunks_exact(bytes_per_row as usize)
    {
        img_data.extend_from_slice(&row[0..usize::from(CANVAS) * 4]);
    }
    let out = format!("gpu_image_bench_{mode}.png");
    let file = std::fs::File::create(&out).unwrap();
    let w = std::io::BufWriter::new(file);
    let mut png_encoder = png::Encoder::new(w, CANVAS.into(), CANVAS.into());
    png_encoder.set_color(png::ColorType::Rgba);
    let mut writer = png_encoder.write_header().unwrap();
    writer.write_image_data(&img_data).unwrap();
    println!("  wrote {out}");
}
