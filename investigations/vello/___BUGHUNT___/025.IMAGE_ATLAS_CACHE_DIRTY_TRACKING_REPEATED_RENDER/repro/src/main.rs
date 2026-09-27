// Standalone reproduction for bug-hunt candidate 025 (vello issue #1809, root-caused via the
// upstream (still-open, unmerged as of this session) PR #1664 "fix: preserve image atlas across
// patchless resolves"):
//
// `Resolver::resolve` has an early-return fast path for a "patchless" encoding (one with no
// gradients/images/glyph-runs at all, e.g. a scene that only fills solid-color paths):
//
//     if resources.patches.is_empty() {
//         let layout = resolve_solid_paths_only(encoding, packed);
//         return (layout, Ramps::default(), Images::default());
//     }
//
// This returns `Images::default()` (a 0x0 atlas, no uploads) WITHOUT ever calling
// `ImageCache::begin_resolve`/`finish_resolve` - the image-residency cache is not touched at all
// for that render. `vello::render::render_encoding_coarse` reads `images.width`/`images.height` to
// decide whether the renderer's persistent GPU atlas texture proxy needs to be resized; seeing a
// 0x0 atlas after a previous frame had a real, populated atlas, it frees the old (correctly
// populated) GPU atlas texture and replaces it with a fresh, empty 1x1 one.
//
// The next render that draws an image again *does* have patches, so it takes the normal path:
// `ImageCache::get_or_insert` finds the image's atlas entry still resident (the cache was never
// touched by the intervening patchless render) with `dirty == false` (it was already uploaded and
// marked clean by an earlier render) - so it is NOT re-queued for upload, even though the GPU
// texture that's supposed to contain it was just discarded and replaced with a brand new,
// zero-initialized one. The result: the image is missing from the render, silently, with no error
// anywhere - exactly the reported symptom, and exactly reproducible with three real
// `Renderer::render_to_texture` calls on the same `Renderer` (a real second/third "render" acting
// as the two participants whose interaction causes this - no hand-constructed internal state is
// used anywhere below, only vello's own public `Scene`/`Renderer` API):
//
//   1. render_to_texture(scene with an image)       -> image atlas created & populated, correct
//   2. render_to_texture(scene with NO patches)      -> atlas freed & replaced with an empty 1x1
//   3. render_to_texture(scene with the SAME image)  -> atlas resized back, but never re-uploaded
//                                                        (cache still thinks it's clean) -> BLANK
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (render 3 lost the image
// after an intervening patchless render freed and replaced the GPU atlas texture).
//
// This environment has no real GPU; set `WGPU_BACKEND=gl` (and, if needed,
// `LIBGL_ALWAYS_SOFTWARE=1`) to use Mesa's llvmpipe software rasterizer over the GL backend - see
// this candidate's `justfile`/README for the exact invocation. Any working wgpu backend
// (Vulkan/Metal/DX12/GL) exercises the exact same real atlas/resolve code.

use std::num::NonZeroUsize;
use std::sync::Arc;

use anyhow::{Result, anyhow, bail};
use vello::kurbo::{Affine, Rect};
use vello::peniko::color::palette;
use vello::peniko::{Blob, Color, Fill, ImageAlphaType, ImageData, ImageFormat};
use vello::util::{RenderContext, block_on_wgpu};
use vello::wgpu::{
    self, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, TexelCopyBufferInfo,
    TextureDescriptor, TextureFormat, TextureUsages,
};
use vello::{RendererOptions, Scene};

fn main() -> Result<()> {
    env_logger::init();
    pollster::block_on(run())
}

async fn readback(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    target: &wgpu::Texture,
    width: u32,
    height: u32,
) -> Result<Vec<u8>> {
    let padded_byte_width = (width * 4).next_multiple_of(256);
    let buffer_size = padded_byte_width as u64 * height as u64;
    let buffer = device.create_buffer(&BufferDescriptor {
        label: Some("val"),
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
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit([encoder.finish()]);
    let buf_slice = buffer.slice(..);

    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buf_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    if let Some(recv_result) = block_on_wgpu(device, receiver.receive()) {
        recv_result?;
    } else {
        bail!("channel was closed");
    }

    let data = buf_slice.get_mapped_range();
    let mut result_unpadded = Vec::<u8>::with_capacity((width * height * 4) as usize);
    for row in 0..height {
        let start = (row * padded_byte_width) as usize;
        result_unpadded.extend(&data[start..start + (width * 4) as usize]);
    }
    drop(data);
    buffer.unmap();
    Ok(result_unpadded)
}

fn center_pixel(buf: &[u8], width: u32, height: u32) -> [u8; 4] {
    let x = width / 2;
    let y = height / 2;
    let idx = ((y * width + x) * 4) as usize;
    [buf[idx], buf[idx + 1], buf[idx + 2], buf[idx + 3]]
}

async fn run() -> Result<()> {
    let mut context = RenderContext::new();
    let device_id = context
        .device(None)
        .await
        .ok_or_else(|| anyhow!("No compatible device found"))?;
    let device_handle = &mut context.devices[device_id];
    let device = &device_handle.device;
    let queue = &device_handle.queue;
    println!("Using adapter: {:?}", device_handle.adapter().get_info());

    let mut renderer = vello::Renderer::new(
        device,
        RendererOptions {
            use_cpu: false,
            num_init_threads: NonZeroUsize::new(1),
            antialiasing_support: vello::AaSupport::area_only(),
            ..Default::default()
        },
    )
    .or_else(|_| bail!("Got non-Send/Sync error from creating renderer"))?;

    // A small, fully opaque solid-red image - anything other than the scene's transparent
    // background is unambiguous evidence the image was actually sampled.
    const IMG_W: u32 = 16;
    const IMG_H: u32 = 16;
    let mut pixels = Vec::with_capacity((IMG_W * IMG_H * 4) as usize);
    for _ in 0..(IMG_W * IMG_H) {
        pixels.extend_from_slice(&[255_u8, 0, 0, 255]);
    }
    let image_data = ImageData {
        data: Blob::new(Arc::new(pixels)),
        format: ImageFormat::Rgba8,
        width: IMG_W,
        height: IMG_H,
        alpha_type: ImageAlphaType::Alpha,
    };

    let width = IMG_W;
    let height = IMG_H;
    let render_params = vello::RenderParams {
        base_color: palette::css::TRANSPARENT,
        width,
        height,
        antialiasing_method: vello::AaConfig::Area,
    };
    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let make_target = |label: &'static str| {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        (texture, view)
    };

    // --- Render 1: a scene that draws the image. ---
    let mut scene = Scene::new();
    scene.draw_image(&image_data, Affine::IDENTITY);
    let (target1, view1) = make_target("Target texture 1 (image)");
    renderer
        .render_to_texture(device, queue, &scene, &view1, &render_params)
        .or_else(|_| bail!("Got non-Send/Sync error from rendering (1st call)"))?;
    let buf1 = readback(device, queue, &target1, width, height).await?;
    let px1 = center_pixel(&buf1, width, height);

    // --- Render 2: an intervening *patchless* scene (a plain solid-color fill, no
    // images/gradients/glyphs at all) - this is the "Resolver::resolve" early-return path that
    // never touches the image residency cache. ---
    let mut solid_scene = Scene::new();
    solid_scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        Color::from_rgba8(0, 255, 0, 255),
        None,
        &Rect::new(0.0, 0.0, width as f64, height as f64),
    );
    let (_target_solid, view_solid) = make_target("Target texture (solid, patchless)");
    renderer
        .render_to_texture(device, queue, &solid_scene, &view_solid, &render_params)
        .or_else(|_| bail!("Got non-Send/Sync error from rendering (patchless call)"))?;

    // --- Render 3: the SAME image again, in a freshly built scene - this is the render that,
    // per PR #1664's own description, silently samples the just-recreated, never-reuploaded
    // atlas texture. ---
    let mut scene2 = Scene::new();
    scene2.draw_image(&image_data, Affine::IDENTITY);
    let (target3, view3) = make_target("Target texture 3 (image again)");
    renderer
        .render_to_texture(device, queue, &scene2, &view3, &render_params)
        .or_else(|_| bail!("Got non-Send/Sync error from rendering (3rd call)"))?;
    let buf3 = readback(device, queue, &target3, width, height).await?;
    let px3 = center_pixel(&buf3, width, height);

    println!("render 1 (image)            center pixel (expected opaque red [255,0,0,255]): {px1:?}");
    println!("render 3 (image, after a patchless render in between) center pixel: {px3:?}");

    let expected = [255_u8, 0, 0, 255];
    let render1_ok = px1 == expected;
    let render3_ok = px3 == expected;

    if render1_ok && !render3_ok {
        eprintln!(
            "REPRODUCED: the first image render was correct ({px1:?}), but after an intervening \
             patchless render, drawing the exact same ImageData again produced {px3:?} instead of \
             {expected:?} - the image was silently dropped, no error/panic anywhere."
        );
        std::process::exit(1);
    } else if !render1_ok {
        eprintln!(
            "did not reproduce as expected: the FIRST render already failed to draw the image \
             ({px1:?}, expected {expected:?}) - this is a different bug than the one this \
             candidate targets."
        );
        std::process::exit(2);
    } else {
        println!("did not reproduce - the image survived the intervening patchless render.");
    }
    Ok(())
}
