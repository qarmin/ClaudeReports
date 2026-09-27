// Reproduction for candidate 064: COLR_SWEEP_GRADIENT_MISSING_Y_FLIP.
//
// `vello::scene::conv_brush` (the `skrifa::color::Brush` -> `peniko::Brush` conversion used by
// classic vello's `Scene::draw_glyphs(..).draw(..)` to render COLRv1 glyphs) constructs a
// `peniko::Gradient::new_sweep` directly from the `skrifa`-provided center/angles, with a
// standing `// TODO: This is upside-down, see https://github.com/linebender/vello/pull/1221`
// comment directly above it. PR #1221 fixed this exact issue for `vello_cpu`/`vello_hybrid` (via
// `glifo`'s `ColorPainter` impl, then named `sparse_strips/vello_common/src/colr.rs`) by negating
// the gradient center's y-coordinate and composing an extra `Affine::scale_non_uniform(1.0,
// -1.0)` into the paint transform - "to bridge the gap between peniko and COLR" - but never
// applied the matching fix to classic vello's own, separate `DrawColorGlyphs` implementation in
// `vello/src/scene.rs`.
//
// This drives the REAL end-to-end path for both renderers, using the exact same real COLRv1 test
// font (Google/nanoemoji's `test_glyphs-glyf_colr_1.ttf`, already used by this project's own
// `vello_sparse_tests` COLR snapshot tests) and the exact same glyph
// (`sweep_0_360_pad_narrow`, glyph id 12 - a real font-authored `PaintSweepGradient` with
// `Extend::Pad` and stops narrowed to a sub-range of the full circle, chosen specifically by the
// test font's authors to exercise sweep-gradient direction/extend edge cases):
//
//   - "reference": `vello_cpu`'s real public `RenderContext::glyph_run(..).fill_glyphs(..)`
//     entry point (CPU rasterizer, uses `glifo` internally, includes PR #1221's fix).
//   - "classic vello (GPU)": `vello::Scene::draw_glyphs(..).draw(..)` to build a real `Scene`,
//     then a real `wgpu` device/queue via `vello::util::RenderContext` and a real
//     `vello::Renderer::render_to_texture` call (the same public API/entry point a real
//     embedding application uses), reading back the real rendered pixels.
//
// This is a single-participant case per CONFIRMATION.md section 11 - a font file is a single
// external artifact handed to each renderer's real public entry point; no second participant is
// faked. Both renders are real, actually executed (no hand-invented pixel/ramp data).
//
// The two images should be pixel-identical (up to ordinary anti-aliasing/rounding noise between
// two independent rasterizers). Instead, on the buggy `vello/src/scene.rs`, classic vello's
// render is the exact vertical mirror image of the correct one.

use std::iter;
use std::num::NonZeroUsize;
use std::sync::Arc;

const FONT_BYTES: &[u8] = include_bytes!("../assets/test_glyphs-glyf_colr_1.ttf");
/// `sweep_0_360_pad_narrow` in `test_glyphs-glyf_colr_1.ttf` - a `PaintSweepGradient` covering the
/// full 0-360 degree circle with `Extend::Pad` and color stops narrowed to the sub-range
/// [0.25, 0.75], found via `fontTools` introspection of the font's `COLR` table (see RESULT.md).
const GLYPH_ID: u32 = 12;
const FONT_SIZE: f32 = 200.0;
const CANVAS: u16 = 200;
/// Pixels whose max per-channel difference exceeds this are considered "different" - well above
/// ordinary anti-aliasing noise between two independent rasterizers (observed max ~2-3 near the
/// circle's outer edge once the bug is fixed), well below the near-total-mismatch this bug causes
/// across a large fraction of the gradient's visible area.
const DIFF_THRESHOLD: i32 = 10;
/// Number of differing pixels (out of 200*200 = 40000) above which the render is considered
/// reproducing the bug. Observed: ~15447 on the buggy code, ~632 (edge anti-aliasing only) once
/// fixed.
const REPRODUCED_DIFF_COUNT: usize = 5000;

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

fn main() {
    let reference_pixels = render_reference_vello_cpu();
    let classic_vello_pixels = pollster::block_on(render_classic_vello_gpu());

    write_png("reference_vello_cpu.png", &reference_pixels);
    write_png("classic_vello_gpu.png", &classic_vello_pixels);

    let mut n_diff = 0usize;
    let mut max_diff = 0i32;
    for (r, c) in reference_pixels.iter().zip(classic_vello_pixels.iter()) {
        let d = r
            .iter()
            .zip(c.iter())
            .map(|(a, b)| (*a as i32 - *b as i32).abs())
            .max()
            .unwrap();
        if d > DIFF_THRESHOLD {
            n_diff += 1;
        }
        max_diff = max_diff.max(d);
    }

    println!(
        "vello_cpu (reference) vs classic vello (GPU): {n_diff} / {} pixels differ by more than \
         {DIFF_THRESHOLD}/255 in some channel (max_diff={max_diff}); images written to \
         reference_vello_cpu.png / classic_vello_gpu.png",
        reference_pixels.len()
    );

    if n_diff >= REPRODUCED_DIFF_COUNT {
        eprintln!(
            "REPRODUCED - classic vello's real GPU render of a COLRv1 sweep-gradient glyph \
             diverges from vello_cpu's real render of the exact same glyph in {n_diff} of \
             {} pixels (>= threshold {REPRODUCED_DIFF_COUNT}) - consistent with the missing \
             y-flip in vello::scene::conv_brush's SweepGradient handling",
            reference_pixels.len()
        );
        std::process::exit(1);
    }
    println!(
        "did not reproduce - only {n_diff} pixels differ (< threshold {REPRODUCED_DIFF_COUNT}), \
         consistent with ordinary anti-aliasing noise between two independent rasterizers"
    );
}
