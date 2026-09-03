// Reproduction for candidate 081: COLR_SWEEP_GRADIENT_DEGENERATE_ANGLE_TRANSPARENT.
//
// `vello/src/scene.rs`'s `conv_brush` (the `skrifa::color::Brush` -> `peniko::Brush` conversion
// used by classic vello's `Scene::draw_glyphs(..).draw(..)` to render COLRv1 glyphs) constructs a
// `peniko::Gradient::new_sweep` directly from skrifa's `start_angle`/`end_angle`, with no handling
// for the case where the two are equal.
//
// skrifa's own COLR paint-graph traversal (`skrifa::color::traversal`) explicitly forwards this
// degenerate case (`PaintSweepGradient.startAngle == endAngle` with `ColorLine.extend == Pad`) to
// `ColorPainter::fill` - it's a real, spec-permitted COLRv1 construct, not a font error (see the
// OpenType COLR spec's "sweep gradients" section: "everything before this spot is painted with
// the first color, everything after this spot is painted with the last color"). `glifo`'s own
// `ColorPainter` implementation (used by `vello_cpu`/`vello_hybrid`) already works around this by
// nudging the end angle by a small artificial offset when it sees `start_angle == end_angle`
// (`glifo/src/colr.rs`, comment: "Vello doesn't accept sweep gradient with same start and end
// angle, so add an artificial, small offset"). Classic vello's `conv_brush` has no such
// workaround, so the degenerate sweep gradient falls straight through to
// `vello_encoding::Encoding::encode_sweep_gradient`'s own `(t0 - t1).abs() <
// SKIA_DEGENERATE_THRESHOLD` check, which encodes the paint as fully transparent instead of a
// solid color - silently dropping the entire color layer.
//
// This drives the REAL end-to-end path for both renderers, using a real (mutated) COLRv1 font -
// `test_glyphs-glyf_colr_1.ttf` (Google/nanoemoji's real COLR test font, already used by this
// project's own `vello_sparse_tests` COLR snapshot tests) with glyph 12's
// (`sweep_0_360_pad_narrow`) real `PaintSweepGradient.endAngle` field byte-patched to equal its
// `startAngle` field (see `tools/font_patch`, which locates the real paint table via `skrifa`'s
// own table-reading API rather than hand-picking a byte offset) - producing exactly the
// `startAngle == endAngle` + `Extend::Pad` construct skrifa's traversal code above shows is a
// real, reachable, spec-legal case:
//
//   - "reference": `vello_cpu`'s real public `RenderContext::glyph_run(..).fill_glyphs(..)`
//     entry point (CPU rasterizer, uses `glifo` internally, has the degenerate-angle workaround).
//   - "classic vello (GPU)": `vello::Scene::draw_glyphs(..).draw(..)` to build a real `Scene`,
//     then a real `wgpu` device/queue via `vello::util::RenderContext` and a real
//     `vello::Renderer::render_to_texture` call (the same public API/entry point a real
//     embedding application uses), reading back the real rendered pixels.
//
// This is a single-participant case per CONFIRMATION.md section 11 - a font file is a single
// external artifact handed to each renderer's real public entry point; no second participant is
// faked. Both renders are real, actually executed (no hand-invented pixel/ramp data).
//
// vello_cpu (reference) renders a large solid disc (glyph 12 is a filled circle). Classic vello's
// GPU render should be pixel-identical (same font, same glyph) but instead renders fully
// transparent - dropping the entire glyph.

use std::iter;
use std::num::NonZeroUsize;
use std::sync::Arc;

const FONT_BYTES: &[u8] = include_bytes!("../assets/degenerate_sweep.ttf");
/// `sweep_0_360_pad_narrow` in `test_glyphs-glyf_colr_1.ttf`, mutated by `tools/font_patch` so its
/// `PaintSweepGradient.endAngle` byte-for-byte equals its `startAngle` (both raw F2Dot14 -1.0,
/// which after skrifa's own "+180" shift become startAngle == endAngle == 0 degrees), with
/// `Extend::Pad` left untouched.
const GLYPH_ID: u32 = 12;
const FONT_SIZE: f32 = 200.0;
const CANVAS: u16 = 200;
/// A pixel counts as "opaque" if its alpha exceeds this - well above rounding noise, well below
/// the fully-opaque disc's alpha.
const ALPHA_THRESHOLD: u8 = 10;
/// Observed: 15552 opaque pixels for vello_cpu's reference render (the solid disc), 0 for
/// classic vello's real GPU render of the same glyph. A generous quarter of the reference area
/// counts as "reproduced" for the GPU-side check.
const MIN_REFERENCE_OPAQUE: usize = 1000;
const MAX_BUGGY_OPAQUE: usize = 50;

fn render_reference_vello_cpu() -> Vec<[u8; 4]> {
    use vello_common::kurbo::Affine;
    use vello_common::pixmap::Pixmap;
    use vello_cpu::peniko::{Blob, FontData};
    use vello_cpu::{Glyph, RasterizerSettings, RenderContext, RenderMode, Resources};

    let font = FontData::new(Blob::new(Arc::new(FONT_BYTES.to_vec())), 0);

    let mut ctx = RenderContext::new(CANVAS, CANVAS);
    let mut resources = Resources::new();

    ctx.set_transform(Affine::translate((0.0, CANVAS as f64)));
    ctx.glyph_run(&mut resources, &font)
        .font_size(FONT_SIZE)
        .fill_glyphs(iter::once(Glyph {
            id: GLYPH_ID,
            x: 0.0,
            y: 0.0,
        }));
    ctx.flush();

    let mut pixmap = Pixmap::new(CANVAS, CANVAS);
    ctx.render_with(
        &mut pixmap,
        &mut resources,
        RasterizerSettings {
            render_mode: RenderMode::OptimizeQuality,
            ..Default::default()
        },
    );

    (0..CANVAS)
        .flat_map(|y| (0..CANVAS).map(move |x| (x, y)))
        .map(|(x, y)| {
            let p = pixmap.sample(x, y);
            [p.r, p.g, p.b, p.a]
        })
        .collect()
}

async fn render_classic_vello_gpu() -> Vec<[u8; 4]> {
    use vello::kurbo::Affine;
    use vello::peniko::{Blob, Fill, FontData, color::palette};
    use vello::util::{RenderContext, block_on_wgpu};
    use vello::wgpu::{
        self, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d,
        TexelCopyBufferInfo, TextureDescriptor, TextureFormat, TextureUsages,
    };
    use vello::{AaConfig, Glyph, RenderParams, Renderer, RendererOptions, Scene};

    let font = FontData::new(Blob::new(Arc::new(FONT_BYTES.to_vec())), 0);

    let mut scene = Scene::new();
    scene
        .draw_glyphs(&font)
        .font_size(FONT_SIZE)
        .transform(Affine::translate((0.0, CANVAS as f64)))
        .draw(
            Fill::NonZero,
            iter::once(Glyph {
                id: GLYPH_ID,
                x: 0.0,
                y: 0.0,
            }),
        );

    let mut context = RenderContext::new();
    let device_id = context
        .device(None)
        .await
        .expect("no compatible wgpu device found in this environment");
    let device_handle = &mut context.devices[device_id];
    let device = &device_handle.device;
    let queue = &device_handle.queue;
    let mut renderer = Renderer::new(
        device,
        RendererOptions {
            use_cpu: false,
            num_init_threads: NonZeroUsize::new(1),
            antialiasing_support: iter::once(AaConfig::Area).collect(),
            pipeline_cache: None,
        },
    )
    .expect("failed to create renderer");

    let render_params = RenderParams {
        base_color: palette::css::TRANSPARENT,
        width: CANVAS as u32,
        height: CANVAS as u32,
        antialiasing_method: AaConfig::Area,
    };
    let size = Extent3d {
        width: CANVAS as u32,
        height: CANVAS as u32,
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
    renderer
        .render_to_texture(device, queue, &scene, &view, &render_params)
        .expect("render_to_texture failed");

    let padded_byte_width = (CANVAS as u32 * 4).next_multiple_of(256);
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
        .expect("channel closed before map_async completed")
        .expect("map_async failed");
    let data = buf_slice.get_mapped_range();
    let mut out = Vec::with_capacity((CANVAS as usize) * (CANVAS as usize));
    for row in 0..CANVAS as u32 {
        let start = (row * padded_byte_width) as usize;
        for col in 0..CANVAS as u32 {
            let base = start + col as usize * 4;
            out.push([data[base], data[base + 1], data[base + 2], data[base + 3]]);
        }
    }
    out
}

fn write_png(path: &str, pixels: &[[u8; 4]]) {
    let mut data = Vec::with_capacity(pixels.len() * 4);
    for p in pixels {
        data.extend_from_slice(p);
    }
    let mut encoder = png::Encoder::new(
        std::fs::File::create(path).unwrap(),
        CANVAS as u32,
        CANVAS as u32,
    );
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
}

fn opaque_pixel_count(pixels: &[[u8; 4]]) -> usize {
    pixels.iter().filter(|p| p[3] > ALPHA_THRESHOLD).count()
}

fn main() {
    let reference_pixels = render_reference_vello_cpu();
    let classic_vello_pixels = pollster::block_on(render_classic_vello_gpu());

    write_png("reference_vello_cpu.png", &reference_pixels);
    write_png("classic_vello_gpu.png", &classic_vello_pixels);

    let ref_opaque = opaque_pixel_count(&reference_pixels);
    let gpu_opaque = opaque_pixel_count(&classic_vello_pixels);

    println!(
        "degenerate sweep gradient (start_angle == end_angle, Extend::Pad), glyph {GLYPH_ID}: \
         vello_cpu (reference) opaque pixels = {ref_opaque} / {}, classic vello (GPU) opaque \
         pixels = {gpu_opaque} / {}",
        reference_pixels.len(),
        classic_vello_pixels.len()
    );

    if ref_opaque > MIN_REFERENCE_OPAQUE && gpu_opaque < MAX_BUGGY_OPAQUE {
        eprintln!(
            "REPRODUCED - classic vello's real GPU render of a degenerate COLRv1 sweep-gradient \
             glyph (start_angle == end_angle, Extend::Pad) is almost entirely transparent \
             ({gpu_opaque} opaque pixels) while vello_cpu's real render of the exact same glyph \
             is a large solid disc ({ref_opaque} opaque pixels) - consistent with \
             vello/src/scene.rs's conv_brush lacking glifo's degenerate-angle workaround, so the \
             degenerate sweep gradient hits vello_encoding's t0==t1 fallback to fully-transparent \
             instead of a visible color"
        );
        std::process::exit(1);
    }
    println!(
        "did not reproduce - both renderers show a similar number of opaque pixels \
         (reference={ref_opaque}, classic vello (GPU)={gpu_opaque})"
    );
}
