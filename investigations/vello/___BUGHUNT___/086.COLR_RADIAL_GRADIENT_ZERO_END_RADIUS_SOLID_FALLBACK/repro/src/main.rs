// Reproduction for candidate 086: COLR_PAINT_CONVERSION_DIVERGENCE_SWEEP (radial gradient with a
// zero end-radius).
//
// `vello/src/scene.rs`'s `conv_brush` (the `skrifa::color::Brush` -> `peniko::Brush` conversion
// used by classic vello's `Scene::draw_glyphs(..).draw(..)` to render COLRv1 glyphs) passes a
// `skrifa::color::Brush::RadialGradient { c0, r0, c1, r1, .. }` straight through to
// `peniko::Gradient::new_two_point_radial`, unmodified, regardless of `r1`.
//
// `glifo`'s own `ColorPainter::fill` (`glifo/src/colr.rs`, used by `vello_cpu`/`vello_hybrid`) has
// an explicit special case:
//
//     if r1 <= 0.0 || stops.len() == 1 {
//         self.painter.fill_solid(fill_rect, stops[0].color.to_alpha_color());
//         return;
//     }
//
// A radial gradient whose end circle has radius 0 ("focal point" cone gradient - the end circle
// degenerates to a point while the start circle keeps its own real, nonzero radius, and/or a
// different center) is a real, spec-legal COLRv1 `PaintRadialGradient` construct: `radius1` is
// stored as a plain `UFWORD` in the font file, and 0 is a perfectly ordinary encodable value
// (Skia's own two-point-conical-gradient implementation, which classic vello's shader/CPU-shim
// code explicitly ports - see `vello_shaders/shader/draw_leaf.wgsl`'s `if r1 == 0.0 { swap... }`
// and the identical logic in `vello_shaders/src/cpu/draw_leaf.rs` - has a dedicated code path for
// exactly this "one radius is zero" case, it is not treated as invalid input anywhere in that
// pipeline). `skrifa`'s own COLR paint-graph traversal (`skrifa::color::traversal`) has no
// degenerate-radius guard for `PaintRadialGradient` (unlike `PaintLinearGradient`, where skrifa
// itself intercepts `p0 == p1` before ever calling `ColorPainter::fill`) - it forwards `r1 == 0.0`
// straight through to `ColorPainter::fill` unmodified, so this is a real, reachable input via the
// genuine font -> skrifa -> `ColorPainter` pipeline, not a hand-invented one.
//
// This drives the REAL end-to-end path for both renderers, using a real (byte-mutated) COLRv1
// font - `test_glyphs-glyf_colr_1.ttf` (Google/nanoemoji's real COLR test font, already used by
// this project's own `vello_sparse_tests` COLR snapshot tests) with glyph 96's real, existing
// `PaintRadialGradient` (`c0=(400,500) r0=100, c1=(700,500) r1=200` in font units - a real
// two-circle cone gradient with distinct centers and radii, already authored by the font's own
// creators to exercise cone-gradient rendering) byte-patched so its `radius1` field is 0 instead
// of 200 - producing the exact "distinct centers, r0 > 0, r1 == 0" construct described above (see
// `tools/font_patch`, which locates the real field via `skrifa`'s own table-reading API rather
// than a hand-picked byte offset).
//
//   - "reference": `vello_cpu`'s real public `RenderContext::glyph_run(..).fill_glyphs(..)` entry
//     point (CPU rasterizer, uses `glifo` internally, has the `r1 <= 0.0` solid-fallback).
//   - "classic vello (GPU)": `vello::Scene::draw_glyphs(..).draw(..)` to build a real `Scene`,
//     then a real `wgpu` device/queue via `vello::util::RenderContext` and a real
//     `vello::Renderer::render_to_texture` call (the same public API/entry point a real embedding
//     application uses), reading back the real rendered pixels.
//
// This is a single-participant case per CONFIRMATION.md section 11 - a font file is a single
// external artifact handed to each renderer's real public entry point; no second participant is
// faked. Both renders are real, actually executed (no hand-invented pixel/ramp data).
//
// If `glifo`'s `r1 <= 0.0` shortcut is taken, `vello_cpu`'s render of glyph 96 should be a single
// FLAT color (the first color stop) across the whole shape. If classic vello's GPU render instead
// shows real color variation across the shape (an actual radial/cone gradient, as its own shader
// code is designed to render), the two renderers disagree on what this real, spec-legal font
// construct should look like.

use std::iter;
use std::num::NonZeroUsize;
use std::sync::Arc;

const FONT_BYTES: &[u8] = include_bytes!("../assets/patched_radial.ttf");
/// A radial-gradient test glyph in `test_glyphs-glyf_colr_1.ttf`, mutated by `tools/font_patch` so
/// its real `PaintRadialGradient.radius1` field is 0 (was 200), leaving `radius0` (100) and both
/// distinct centers `c0=(400,500)`/`c1=(700,500)` untouched.
const GLYPH_ID: u32 = 96;
const FONT_SIZE: f32 = 200.0;
const CANVAS: u16 = 200;
const ALPHA_THRESHOLD: u8 = 10;

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

/// Among opaque pixels, count how many *distinct* (rounded to reduce antialiasing noise) RGB
/// colors appear. A solid fill produces essentially 1 color; a real gradient produces many.
fn distinct_color_count(pixels: &[[u8; 4]]) -> usize {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for p in pixels {
        if p[3] > ALPHA_THRESHOLD {
            // Round to the nearest 4 to absorb ordinary rounding/dithering noise without masking
            // a real gradient's color sweep.
            set.insert((p[0] / 4, p[1] / 4, p[2] / 4));
        }
    }
    set.len()
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
    let ref_colors = distinct_color_count(&reference_pixels);
    let gpu_colors = distinct_color_count(&classic_vello_pixels);

    println!(
        "radial gradient with r0=100,r1=0 (zero end-radius cone), glyph {GLYPH_ID}: \
         vello_cpu (reference) opaque pixels = {ref_opaque}, distinct colors = {ref_colors} / \
         classic vello (GPU) opaque pixels = {gpu_opaque}, distinct colors = {gpu_colors}"
    );

    // Bug signature: both renderers agree the shape is opaque/filled (so this isn't just a
    // transparency-dropping bug like candidate 081's), but one flattens the whole shape down to
    // essentially a single solid color while the other renders real gradient color variation.
    let ref_is_flat = ref_colors <= 3;
    let gpu_is_flat = gpu_colors <= 3;
    let both_opaque = ref_opaque > 500 && gpu_opaque > 500;

    if both_opaque && ref_is_flat != gpu_is_flat {
        eprintln!(
            "REPRODUCED - for the exact same real, spec-legal COLRv1 PaintRadialGradient (r0=100, \
             r1=0, distinct centers), vello_cpu (reference, via glifo) renders {} \
             ({ref_colors} distinct colors) while classic vello's real GPU render is {} \
             ({gpu_colors} distinct colors) - the two independent conv_brush/ColorPainter COLR \
             paint-graph conversions disagree on what this construct should look like",
            if ref_is_flat { "a flat solid fill" } else { "a real gradient" },
            if gpu_is_flat { "a flat solid fill" } else { "a real gradient" },
        );
        std::process::exit(1);
    }
    println!(
        "did not reproduce - both renderers agree on flat-vs-gradient for this glyph \
         (reference colors={ref_colors}, classic vello (GPU) colors={gpu_colors})"
    );
}
