// Copyright 2026 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Perf-optimize candidate 017 scratch bench: patched build exercising
//! `RenderRegion::Rects` (damage-region rendering, PR #1737) and `Scene::push_clip_rect`
//! (integer rect-clip fast path, PR #1786).

use std::time::Instant;
use vello_common::geometry::RectU16;
use vello_common::kurbo::{Affine, BezPath, Rect};
use vello_common::peniko::Color;
use vello_hybrid::{ClearSettings, RenderRegion, Scene};

const W: u16 = 1920;
const H: u16 = 1080;
const ROWS: usize = 400;
const ROW_H: f64 = 24.0;
const ITERS: usize = 300;

fn build_list_scene() -> Scene {
    let mut scene = Scene::new(W, H);
    scene.set_transform(Affine::IDENTITY);
    scene.set_paint(Color::new([0.09, 0.09, 0.11, 1.0]));
    scene.fill_rect(&Rect::new(0.0, 0.0, W as f64, H as f64));
    for i in 0..ROWS {
        let y = (i as f64 * ROW_H) % (H as f64);
        let shade: f32 = 0.2 + 0.6 * ((i % 7) as f32 / 7.0);
        scene.set_paint(Color::new([shade, shade * 0.6, 1.0 - shade, 1.0]));
        scene.fill_rect(&Rect::new(8.0, y + 2.0, W as f64 - 8.0, y + ROW_H - 2.0));
        scene.set_paint(Color::new([1.0, 1.0, 1.0, 0.85]));
        scene.fill_rect(&Rect::new(16.0, y + 4.0, 16.0 + 120.0, y + ROW_H - 4.0));
    }
    scene
}

fn build_clipped_scene(use_rect_clip: bool) -> Scene {
    let mut scene = Scene::new(W, H);
    let clip = Rect::new(200.0, 100.0, 1720.0, 980.0);
    if use_rect_clip {
        scene.push_clip_rect(&clip);
    } else {
        let mut clip_path = BezPath::new();
        clip_path.move_to((clip.x0, clip.y0));
        clip_path.line_to((clip.x1, clip.y0));
        clip_path.line_to((clip.x1, clip.y1));
        clip_path.line_to((clip.x0, clip.y1));
        clip_path.close_path();
        scene.push_clip_path(&clip_path);
    }
    for i in 0..ROWS {
        let y = 100.0 + (i as f64 * ROW_H) % 900.0;
        let shade: f32 = 0.2 + 0.6 * ((i % 7) as f32 / 7.0);
        scene.set_paint(Color::new([shade, shade * 0.6, 1.0 - shade, 1.0]));
        scene.fill_rect(&Rect::new(150.0, y, 1770.0, y + ROW_H - 4.0));
    }
    scene
}

async fn setup() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("Failed to find an appropriate adapter");
    adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("Device"),
            required_features: wgpu::Features::empty(),
            ..Default::default()
        })
        .await
        .expect("Failed to create device")
}

fn make_target(device: &wgpu::Device) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render Target"),
        size: wgpu::Extent3d {
            width: W.into(),
            height: H.into(),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "full".into());
    let (device, queue) = setup().await;
    let render_size = vello_hybrid::RenderSize {
        width: W.into(),
        height: H.into(),
    };
    let (texture, view) = make_target(&device);
    let (mut renderer, mut resources) = vello_hybrid::Renderer::new(
        &device,
        &vello_hybrid::RenderTargetConfig {
            format: texture.format(),
            width: W.into(),
            height: H.into(),
        },
    );

    // A single thin "one new row scrolled into view" damage rect - the concrete
    // scrolling/clipped-viewport UI redraw shape from the candidate description.
    let damage_rect = RectU16::new(0, 0, W, ROW_H as u16 + 4);

    let (scene, region, clear) = match mode.as_str() {
        "full" => (
            build_list_scene(),
            RenderRegion::Full,
            ClearSettings::Viewport {
                color: vello_common::peniko::color::AlphaColor::TRANSPARENT,
            },
        ),
        "partial" => (
            build_list_scene(),
            RenderRegion::Rects(std::slice::from_ref(&damage_rect)),
            ClearSettings::Rects {
                color: vello_common::peniko::color::AlphaColor::TRANSPARENT,
                rects: std::slice::from_ref(&damage_rect),
            },
        ),
        "clip" => (
            build_clipped_scene(false),
            RenderRegion::Full,
            ClearSettings::Viewport {
                color: vello_common::peniko::color::AlphaColor::TRANSPARENT,
            },
        ),
        "clip_rect" => (
            build_clipped_scene(true),
            RenderRegion::Full,
            ClearSettings::Viewport {
                color: vello_common::peniko::color::AlphaColor::TRANSPARENT,
            },
        ),
        other => panic!("unknown mode {other}"),
    };

    let mut durations = Vec::with_capacity(ITERS);
    for i in 0..ITERS {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("bench encoder"),
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
                &view,
                &vello_hybrid::TextureBindings::new(),
                clear,
                region,
            )
            .unwrap();
        queue.submit([encoder.finish()]);
        device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
        let elapsed = start.elapsed();
        if i >= 30 {
            durations.push(elapsed);
        }
        std::hint::black_box(&texture);
    }
    durations.sort();
    let n = durations.len();
    let min = durations[0];
    let p50 = durations[n / 2];
    let mean: f64 = durations.iter().map(|d| d.as_secs_f64()).sum::<f64>() / n as f64;
    let max = durations[n - 1];
    println!(
        "mode={mode} n={n} min={:.4}ms p50={:.4}ms mean={:.4}ms max={:.4}ms partial_renders={} culled_strips={}",
        min.as_secs_f64() * 1000.0,
        p50.as_secs_f64() * 1000.0,
        mean * 1000.0,
        max.as_secs_f64() * 1000.0,
        renderer.partial_renders(),
        renderer.culled_strips(),
    );
}
