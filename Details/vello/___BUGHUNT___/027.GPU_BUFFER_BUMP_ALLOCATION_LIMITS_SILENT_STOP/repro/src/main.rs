// Standalone reproduction for bug-hunt candidate 027 (GPU buffer/bump-allocation limits
// silently exceeded, no error surfaced) - matching
// https://github.com/linebender/vello/issues/720 ("Renderer stops functioning under stress
// without generating error messages").
//
// This renders a real scene through vello's actual, recommended, public entry point
// (`Renderer::render_to_texture`), driven by a real wgpu GPU device (any working wgpu backend;
// see this candidate's `justfile`/README for how this environment selects one without real GPU
// hardware). The scene is deliberately large enough (many small overlapping fills, on a small
// canvas so they all land in the same coarse-rasterization "bin") to exceed vello's fixed-size,
// hand-picked `bin_data` GPU buffer capacity - a real, GPU-observable overflow, not a
// hand-constructed stand-in for one.
//
// What this checks, end-to-end through the real API:
//   - A moderate scene (`N_SMALL` draws) renders successfully and the last-drawn marker
//     rectangle's color is actually present in the output pixels.
//   - A large scene (`N_OVERFLOW` draws) overflows the coarse `bin_data` buffer. On the
//     unpatched code, `render_to_texture` still returns `Ok(())`, and the marker rectangle is
//     missing from the output (the affected bin's content was silently dropped) - the caller has
//     no way to know anything went wrong. On the patched code, `render_to_texture` returns
//     `Err(GpuBufferOverflow(_))` instead.
//
// Exit code convention: 0 = did not reproduce (overflow either wasn't hit, or was properly
// reported as an `Err`), non-zero = reproduced (overflow happened and was silently swallowed as
// `Ok(())`).

use std::num::NonZeroUsize;

use anyhow::{Result, anyhow, bail};
use vello::kurbo::{Affine, Rect};
use vello::peniko::Color;
use vello::util::{RenderContext, block_on_wgpu};
use vello::wgpu::{
    self, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, TexelCopyBufferInfo,
    TextureDescriptor, TextureFormat, TextureUsages,
};
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene};

/// Comfortably below the ~131,000-135,000 draw-count threshold at which this scene/canvas
/// combination overflows vello's `bin_data` buffer (see RESULT.md) - used as a control case to
/// confirm ordinary scenes are unaffected by the fix.
const N_SMALL: u32 = 1_000;

/// Comfortably above that threshold (confirmed to overflow, 3/3 runs) but well below the point
/// (~262,144 draws) at which a *different*, already-recorded bug in this workspace
/// (007.MANY_IMAGES_BINNING_SIZE_UNDERFLOW's `bin_data_start` subtraction) would also start
/// contributing - kept clear of that regime so this candidate's own repro isn't entangled with
/// it.
const N_OVERFLOW: u32 = 200_000;

/// Small canvas so every draw's bounding box lands in the same single coarse "bin" - this is
/// what makes it cheap to exhaust the bin's fixed-size share of `bin_data` with a plain fill
/// scene, no gradients/images/strokes required.
const CANVAS: u32 = 64;

const MARKER_RGBA: [u8; 4] = [1, 254, 3, 255];

fn build_scene(n_background_draws: u32) -> Scene {
    let mut scene = Scene::new();
    for i in 0..n_background_draws {
        let hue = (i as f64 * 0.017_453) % 1.0;
        let color = Color::from_rgba8(((hue * 255.0) as u8).wrapping_add(1), 120, 60, 255);
        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            color,
            None,
            &Rect::from_origin_size((0.0, 0.0), (CANVAS as f64, CANVAS as f64)),
        );
    }
    // Drawn last (on top, covering the whole canvas): if this scene's coarse rasterization
    // overflowed, the affected bin's draws - including this one - never reach the final image.
    scene.fill(
        vello::peniko::Fill::NonZero,
        Affine::IDENTITY,
        Color::from_rgba8(
            MARKER_RGBA[0],
            MARKER_RGBA[1],
            MARKER_RGBA[2],
            MARKER_RGBA[3],
        ),
        None,
        &Rect::from_origin_size((0.0, 0.0), (CANVAS as f64, CANVAS as f64)),
    );
    scene
}

/// Renders `scene` via the real public `Renderer::render_to_texture` entry point and reads back
/// the center pixel of the output texture.
async fn render_and_sample_center(
    scene: &Scene,
) -> Result<(std::result::Result<(), vello::Error>, [u8; 4])> {
    let mut context = RenderContext::new();
    let device_id = context
        .device(None)
        .await
        .ok_or_else(|| anyhow!("No compatible device found"))?;
    let device_handle = &mut context.devices[device_id];
    let device = &device_handle.device;
    let queue = &device_handle.queue;

    let mut renderer = Renderer::new(
        device,
        RendererOptions {
            use_cpu: false,
            num_init_threads: NonZeroUsize::new(1),
            antialiasing_support: std::iter::once(AaConfig::Area).collect(),
            pipeline_cache: None,
        },
    )
    .map_err(|_| anyhow!("Got non-Send/Sync error from creating renderer"))?;

    let params = RenderParams {
        base_color: Color::from_rgba8(0, 0, 0, 255),
        width: CANVAS,
        height: CANVAS,
        antialiasing_method: AaConfig::Area,
    };
    let size = Extent3d {
        width: CANVAS,
        height: CANVAS,
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

    // This is the actual, real result the fix changes - a genuine `Result` from vello's own
    // recommended public rendering API, not a value we construct or infer.
    let render_result = renderer.render_to_texture(device, queue, scene, &view, &params);

    let padded_byte_width = (CANVAS * 4).next_multiple_of(256);
    let buffer_size = padded_byte_width as u64 * CANVAS as u64;
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
        .ok_or_else(|| anyhow!("channel was closed"))??;
    let data = buf_slice.get_mapped_range();
    let cx = (CANVAS / 2) as usize;
    let cy = (CANVAS / 2) as usize;
    let row_start = cy * padded_byte_width as usize;
    let px = row_start + cx * 4;
    let mut center = [0u8; 4];
    center.copy_from_slice(&data[px..px + 4]);

    Ok((render_result, center))
}

fn main() -> Result<()> {
    let small_scene = build_scene(N_SMALL);
    let (small_result, small_pixel) = pollster::block_on(render_and_sample_center(&small_scene))?;
    println!(
        "N_SMALL={N_SMALL}: render_to_texture() = {small_result:?}, center pixel = {small_pixel:?}"
    );

    let overflow_scene = build_scene(N_OVERFLOW);
    let (overflow_result, overflow_pixel) =
        pollster::block_on(render_and_sample_center(&overflow_scene))?;
    println!(
        "N_OVERFLOW={N_OVERFLOW}: render_to_texture() = {overflow_result:?}, center pixel = {overflow_pixel:?}"
    );

    let small_ok = small_result.is_ok() && small_pixel == MARKER_RGBA;
    if !small_ok {
        bail!(
            "control case (N_SMALL) failed unexpectedly - result={small_result:?}, pixel={small_pixel:?}, expected Ok(()) and pixel={MARKER_RGBA:?}"
        );
    }

    let marker_missing = overflow_pixel != MARKER_RGBA;
    let reported_as_error = overflow_result.is_err();

    if marker_missing && !reported_as_error {
        println!(
            "REPRODUCED: the N_OVERFLOW scene's coarse rasterization overflowed a fixed-size \
             GPU buffer (the marker rectangle is missing from the output - pixel={overflow_pixel:?}, \
             expected {MARKER_RGBA:?}), yet render_to_texture() reported Ok(()) - the caller has \
             no way to know the scene was not fully drawn."
        );
        std::process::exit(1);
    }

    if marker_missing && reported_as_error {
        println!(
            "did not reproduce: the overflow happened (marker missing) but was properly reported \
             as {overflow_result:?} instead of being silently swallowed as Ok(())."
        );
    } else {
        println!(
            "did not reproduce: N_OVERFLOW={N_OVERFLOW} draws did not overflow the buffer on this \
             build/environment (marker rectangle was still present)."
        );
    }
    Ok(())
}
