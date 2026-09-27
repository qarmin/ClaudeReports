// Copyright 2026 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Standalone differential reproduction for
//! <https://github.com/linebender/vello/issues/1590>: vello_hybrid computes the wrong
//! sample coordinates for bilinear (`ImageQuality::Medium`) image sampling combined with
//! `Extend::Pad`, disagreeing with vello_cpu's (correct) output.
//!
//! This drives the real, public rendering entry points of both `vello_cpu` (CPU rasterizer)
//! and `vello_hybrid` (real wgpu GPU pipeline, via a real `wgpu::Device`/`wgpu::Queue`) with
//! the exact same scene, and diffs the resulting pixel buffers directly. No bytes/state are
//! hand-constructed to stand in for a peer - both backends are driven through their genuine
//! public `fill_rect`/`render` APIs, exactly as a real caller (or the project's own snapshot
//! tests) would use them.
//!
//! Needs a working wgpu backend (Vulkan/Metal/DX12) to create a device. On a machine with a
//! real GPU this works out of the box; in a headless/CI-like environment without one, point
//! `VK_ICD_FILENAMES` at a software Vulkan ICD (e.g. swiftshader or lavapipe) before running.

use std::sync::Arc;

use vello_common::kurbo::{Affine, Rect};
use vello_common::paint::{Image, ImageSource};
use vello_common::peniko::{Extend, ImageAlphaType, ImageQuality, ImageSampler};
use vello_common::pixmap::{PixelMetadata, Pixmap};
use vello_cpu::{Level, RenderContext, RenderSettings};

const CANVAS_W: u16 = 100;
const CANVAS_H: u16 = 20;
const IMG_W: u16 = 10;
const IMG_H: u16 = 10;

/// A width-10 image where every row is: 8 white columns, then green, then red.
/// This makes the two rightmost texels maximally different, so any bilinear blending
/// between them (instead of `Extend::Pad` clamping fully to the last one) is trivially
/// visible in the output pixels.
fn make_probe_image() -> Arc<Pixmap> {
    let mut data = Vec::with_capacity(usize::from(IMG_W) * usize::from(IMG_H) * 4);
    for _y in 0..IMG_H {
        for x in 0..IMG_W {
            let px: [u8; 4] = if x == IMG_W - 1 {
                [255, 0, 0, 255] // red: last column
            } else if x == IMG_W - 2 {
                [0, 255, 0, 255] // green: second-to-last column
            } else {
                [255, 255, 255, 255] // white: everything else
            };
            data.extend_from_slice(&px);
        }
    }
    Arc::new(Pixmap::from_parts(
        data,
        IMG_W,
        IMG_H,
        PixelMetadata::new(ImageAlphaType::AlphaPremultiplied, false),
    ))
}

fn make_image_paint(image: ImageSource) -> Image {
    Image {
        image,
        sampler: ImageSampler {
            x_extend: Extend::Pad,
            y_extend: Extend::Pad,
            quality: ImageQuality::Medium,
            alpha: 1.0,
        },
    }
}

fn render_cpu(transform: Affine) -> Pixmap {
    let settings = RenderSettings {
        level: Level::baseline(),
        num_threads: 0,
    };
    let mut ctx = RenderContext::new_with(CANVAS_W, CANVAS_H, settings);
    let mut resources = vello_cpu::Resources::new();
    let image_id = resources.register_image(make_probe_image());
    let source = ImageSource::opaque_id_with_transparency_hint(image_id, false);

    ctx.set_paint_transform(transform);
    ctx.set_paint(make_image_paint(source));
    ctx.fill_rect(&Rect::new(0.0, 0.0, f64::from(CANVAS_W), f64::from(CANVAS_H)));
    ctx.flush();

    let mut out = Pixmap::new(CANVAS_W, CANVAS_H);
    ctx.render(&mut out, &mut resources);
    out
}

async fn render_hybrid(transform: Affine) -> Pixmap {
    let mut scene = vello_hybrid::Scene::new_with(CANVAS_W, CANVAS_H, Level::baseline());

    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("Failed to find an appropriate adapter");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("Device"),
            required_features: wgpu::Features::empty(),
            ..Default::default()
        })
        .await
        .expect("Failed to create device");

    eprintln!("wgpu adapter: {:?}", adapter.get_info());

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render Target"),
        size: wgpu::Extent3d {
            width: CANVAS_W.into(),
            height: CANVAS_H.into(),
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
            width: CANVAS_W.into(),
            height: CANVAS_H.into(),
        },
    );

    let mut upload_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Upload image"),
    });
    let image_id = renderer.upload_image(
        &mut resources,
        &device,
        &queue,
        &mut upload_encoder,
        &make_probe_image(),
    );
    queue.submit([upload_encoder.finish()]);
    let source = ImageSource::opaque_id_with_transparency_hint(image_id, false);

    scene.set_paint_transform(transform);
    scene.set_paint(make_image_paint(source));
    scene.fill_rect(&Rect::new(0.0, 0.0, f64::from(CANVAS_W), f64::from(CANVAS_H)));

    let render_size = vello_hybrid::RenderSize {
        width: CANVAS_W.into(),
        height: CANVAS_H.into(),
    };
    let depth_texture_view = vello_hybrid::Renderer::create_depth_texture_view(&device, &render_size);

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
            &vello_hybrid::TextureBindings::new(),
        )
        .unwrap();

    let bytes_per_row = (u32::from(CANVAS_W) * 4).next_multiple_of(256);
    let texture_copy_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: u64::from(bytes_per_row) * u64::from(CANVAS_H),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    encoder.copy_texture_to_buffer(
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
            width: CANVAS_W.into(),
            height: CANVAS_H.into(),
            depth_or_array_layers: 1,
        },
    );
    queue.submit([encoder.finish()]);

    texture_copy_buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            result.expect("Failed to map texture for reading");
        });
    device.poll(wgpu::PollType::wait_indefinitely()).unwrap();

    let mut img_data = Vec::with_capacity(usize::from(CANVAS_W) * usize::from(CANVAS_H) * 4);
    for row in texture_copy_buffer
        .slice(..)
        .get_mapped_range()
        .chunks_exact(bytes_per_row as usize)
    {
        img_data.extend_from_slice(&row[0..usize::from(CANVAS_W) * 4]);
    }
    texture_copy_buffer.unmap();

    Pixmap::from_parts(
        img_data,
        CANVAS_W,
        CANVAS_H,
        PixelMetadata::new(ImageAlphaType::AlphaPremultiplied, false),
    )
}

fn pixel(pixmap: &Pixmap, x: u16, y: u16) -> [u8; 4] {
    let buf = pixmap.data();
    let idx = usize::from(y) * usize::from(pixmap.width()) + usize::from(x);
    let p = buf[idx];
    [p.r, p.g, p.b, p.a]
}

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // A fractional (non-integer) translation avoids vello_common's own "integer-translation ->
    // downgrade to nearest-neighbor" fast-path optimization for `ImageQuality::Medium`, so both
    // backends genuinely do bilinear sampling.
    let transform = Affine::translate((0.5, 0.5));

    let cpu_out = render_cpu(transform);
    let hybrid_out = render_hybrid(transform).await;

    // Image is 10px wide; under Extend::Pad, everything at or beyond destination x=10 should
    // render as solid red (255,0,0,255) forever - the rightmost source texel, repeated.
    let probe_y = 5;
    println!("x   cpu_rgba          hybrid_rgba        mismatch");
    let mut any_mismatch = false;
    for x in 0u16..CANVAS_W {
        let cpu_px = pixel(&cpu_out, x, probe_y);
        let hybrid_px = pixel(&hybrid_out, x, probe_y);
        let mismatch = cpu_px != hybrid_px;
        any_mismatch |= mismatch;
        if x < 20 || mismatch {
            println!(
                "{x:<3} {cpu_px:?}   {hybrid_px:?}   {}",
                if mismatch { "MISMATCH" } else { "match" }
            );
        }
    }

    println!();
    println!("Any CPU/Hybrid mismatch found: {any_mismatch}");

    if any_mismatch {
        println!("RESULT: REPRODUCED (CPU and Hybrid disagree under Extend::Pad bilinear sampling)");
        std::process::exit(1);
    } else {
        println!("RESULT: NOT REPRODUCED (CPU and Hybrid agree)");
    }
}
