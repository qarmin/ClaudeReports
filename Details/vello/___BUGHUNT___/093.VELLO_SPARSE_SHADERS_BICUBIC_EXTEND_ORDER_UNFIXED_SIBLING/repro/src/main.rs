// Copyright 2026 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Standalone differential reproduction for the sibling bug named in candidate 009's own
//! "What's not done" section: `vello_hybrid`'s GPU fragment shader still applies `Extend` once,
//! upfront, to a single sample position before dispatching to the bicubic (`ImageQuality::High`)
//! sampling helper (`bicubic_sample` in `helpers/image.wesl`, and
//! `external_bicubic_sample` in `helpers/external_texture.wesl`), instead of extending each of the
//! 16 taps independently the way `vello_cpu`'s `extend_x!`/`extend_y!` macros do
//! (`vello_cpu/src/fine/common/image.rs`). This is expected to produce a wrong,
//! duplicated-edge-texel blend at the wrap boundary for `Extend::Repeat` images under
//! `ImageQuality::High`.
//!
//! This drives the real, public rendering entry points of both `vello_cpu` (CPU rasterizer) and
//! `vello_hybrid` (real wgpu GPU pipeline, via a real `wgpu::Device`/`wgpu::Queue`) with the exact
//! same scene, and diffs the resulting pixel buffers directly - no bytes/state are hand-constructed
//! to stand in for a peer, both backends are driven through their genuine public
//! `fill_rect`/`render` APIs exactly as a real caller (or the project's own snapshot tests) would.
//!
//! Needs a working wgpu backend (Vulkan/Metal/DX12) to create a device. On a machine with a real
//! GPU this works out of the box; in a headless/CI-like environment without one, point
//! `VK_ICD_FILENAMES` at a software Vulkan ICD (e.g. swiftshader or lavapipe) before running.

use std::sync::Arc;

use vello_common::kurbo::{Affine, Rect};
use vello_common::paint::{Image, ImageSource};
use vello_common::peniko::{Extend, ImageAlphaType, ImageQuality, ImageSampler};
use vello_common::pixmap::{PixelMetadata, Pixmap};
use vello_cpu::{Level, RenderContext, RenderSettings};

const CANVAS_W: u16 = 64;
const CANVAS_H: u16 = 10;
const IMG_W: u16 = 8;
const IMG_H: u16 = 8;

/// A width-8 image whose columns are maximally distinct, so any wrong blend at the
/// `Extend::Repeat` wrap boundary (between the last column and the first column of the next
/// period) is trivially visible in the destination pixels.
fn make_probe_image() -> Arc<Pixmap> {
    let cols: [[u8; 4]; 8] = [
        [255, 0, 0, 255],   // 0: red
        [0, 255, 0, 255],   // 1: green
        [255, 255, 255, 255], // 2: white
        [255, 255, 255, 255], // 3: white
        [255, 255, 255, 255], // 4: white
        [255, 255, 255, 255], // 5: white
        [0, 0, 255, 255],   // 6: blue
        [255, 255, 0, 255], // 7: yellow (last column, adjacent to wrap back to column 0)
    ];
    let mut data = Vec::with_capacity(usize::from(IMG_W) * usize::from(IMG_H) * 4);
    for _y in 0..IMG_H {
        for x in 0..IMG_W {
            data.extend_from_slice(&cols[usize::from(x)]);
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
            x_extend: Extend::Repeat,
            y_extend: Extend::Pad,
            quality: ImageQuality::High,
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

/// The project's own `vello_sparse_tests` snapshot harness tolerates a small per-channel u8
/// difference (`hybrid_tolerance`/`cpu_u8_tolerance`, up to 3) between CPU and Hybrid output
/// specifically for bicubic (`ImageQuality::High`) interpolation, due to ordinary floating-point
/// evaluation-order differences between the two backends' cubic-weight math - see e.g.
/// `vello_sparse_tests/tests/glyph.rs`'s `cpu_u8_tolerance = 3` cases with the comment "likely due
/// to having to use bicubic interpolation". A tolerance of 2 here distinguishes that expected
/// noise from the much larger, structurally-wrong mismatch this candidate is about.
const TOLERANCE: i32 = 2;

fn within_tolerance(a: [u8; 4], b: [u8; 4]) -> bool {
    a.iter()
        .zip(b.iter())
        .all(|(&x, &y)| (i32::from(x) - i32::from(y)).abs() <= TOLERANCE)
}

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // A fractional (non-integer) translation avoids vello_common's own "integer-translation ->
    // downgrade to nearest-neighbor" fast-path optimization, so both backends genuinely do
    // bicubic sampling with sub-pixel taps straddling the Extend::Repeat wrap boundary
    // (the image is 8px wide, so the wrap repeats every 8 destination pixels).
    let transform = Affine::translate((0.5, 0.5));

    let cpu_out = render_cpu(transform);
    let hybrid_out = render_hybrid(transform).await;

    let probe_y = 4;
    println!("x   cpu_rgba          hybrid_rgba        mismatch");
    let mut any_mismatch = false;
    let mut mismatch_count = 0usize;
    for x in 0u16..CANVAS_W {
        let cpu_px = pixel(&cpu_out, x, probe_y);
        let hybrid_px = pixel(&hybrid_out, x, probe_y);
        let mismatch = !within_tolerance(cpu_px, hybrid_px);
        any_mismatch |= mismatch;
        if mismatch {
            mismatch_count += 1;
        }
        // Print everything near each period boundary (multiples of IMG_W), plus any mismatch
        // anywhere else too.
        let near_boundary = (x % IMG_W) >= IMG_W - 3 || (x % IMG_W) <= 2;
        if near_boundary || mismatch {
            println!(
                "{x:<3} {cpu_px:?}   {hybrid_px:?}   {}",
                if mismatch { "MISMATCH" } else { "match" }
            );
        }
    }

    println!();
    println!(
        "Any CPU/Hybrid mismatch beyond tolerance {TOLERANCE} found: {any_mismatch} ({mismatch_count}/{CANVAS_W} pixels differ on probe row)"
    );

    if any_mismatch {
        println!(
            "RESULT: REPRODUCED (CPU and Hybrid disagree beyond the project's own bicubic FP tolerance under Extend::Repeat, ImageQuality::High)"
        );
        std::process::exit(1);
    } else {
        println!("RESULT: NOT REPRODUCED (CPU and Hybrid agree within tolerance {TOLERANCE})");
    }
}
