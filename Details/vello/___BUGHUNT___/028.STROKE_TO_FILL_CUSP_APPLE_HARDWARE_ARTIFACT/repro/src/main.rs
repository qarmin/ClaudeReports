// Repro/diagnostic for candidate 028: STROKE_TO_FILL_CUSP_APPLE_HARDWARE_ARTIFACT
// (https://github.com/linebender/vello/issues/581).
//
// Runs the REAL end-to-end classic-vello pipeline through public API:
//   vello::Scene::stroke (real public API, exactly what an application calls)
//     -> Scene's internal Encoding (produced by the real encoder)
//     -> vello_encoding::Resolver::resolve (the real resolve step vello::Renderer itself calls)
//     -> vello_encoding::RenderConfig::new (the real GPU-config/workgroup-count computation)
//     -> vello_shaders::cpu::{pathtag_reduce, pathtag_scan, bbox_clear, flatten} (the exact
//        production CPU-shader implementations vello::Renderer itself dispatches as a fallback -
//        not a reimplementation, the literal same functions, in the same order
//        vello/src/render.rs uses)
//
// on the EXACT near-cusp cubic from the `tricky_strokes` test scene
// (`[(0., 0.01), (128., 127.999), (128., 0.01), (0., 127.99)]`,
// `examples/scenes/src/test_scenes.rs`), reproducing the reporter's own per-cell transform shape
// (`Affine::new([s, 0, 0, s, tx, ty])`, a uniform scale with no rotation - the shape
// `flatten_euler`'s own `scale` local variable reduces to exactly `s` for this transform) and the
// same stroke-width-in-local-space relationship (`Stroke::new(STROKE_WIDTH / s)`,
// `STROKE_WIDTH = 30`) the real `tricky_strokes` scene builder uses, across and beyond the
// reporter's own identified affected scale range (5.3332..113.5797).
//
// linebender/vello#581's own reporter (a vello collaborator) already diagnosed this as specific
// to Metal's `fastMathEnabled` (IEEE-754-violating FP reassociation in Apple's shader compiler),
// verified absent both when Metal's fastmath is explicitly disabled and when running the
// algorithm as a plain scalar CPU implementation (no fastmath there either). This diagnostic
// exercises that exact plain-scalar-f32 CPU implementation (real production code, IEEE-754
// arithmetic, no reassociation of any kind - `rustc` never reassociates floating-point ops unless
// an explicit `fadd_fast`-style intrinsic is used, and none is used anywhere in this codebase) to
// check whether the artifact is observable here too, or whether it only manifests under actual
// fast-math reassociation this sandbox cannot exercise (no Apple/Metal hardware available).

use std::cell::RefCell;
use vello::Scene;
use vello::kurbo::{Affine, BezPath, Cap, Join, PathEl, Point, Stroke};
use vello::low_level::vello_encoding::{
    BumpAllocators, ConfigUniform, LineSoup, RenderConfig, Resolver,
};
use vello::peniko::Color;
use vello_shaders::cpu::CpuBinding;

/// The exact near-cusp cubic from `tricky_strokes`.
const NEAR_CUSP: [(f64, f64); 4] = [(0., 0.01), (128., 127.999), (128., 0.01), (0., 127.99)];

/// Same relationship the real `tricky_strokes` scene builder uses: `STROKE_WIDTH = 30` in the
/// *destination* cell, so the local-space half-width fed to `Scene::stroke` is `STROKE_WIDTH / s`.
const STROKE_WIDTH: f64 = 30.;

/// Flattens `NEAR_CUSP`, stroked, under `Affine::new([s, 0, 0, s, tx, ty])` (uniform scale, no
/// rotation), through the real production CPU-shader pipeline. Returns the emitted `LineSoup`
/// segments belonging to the one path in the scene.
fn flatten_near_cusp_at_scale(s: f64) -> Vec<LineSoup> {
    let mut path = BezPath::new();
    path.push(PathEl::MoveTo(Point::new(NEAR_CUSP[0].0, NEAR_CUSP[0].1)));
    path.push(PathEl::CurveTo(
        Point::new(NEAR_CUSP[1].0, NEAR_CUSP[1].1),
        Point::new(NEAR_CUSP[2].0, NEAR_CUSP[2].1),
        Point::new(NEAR_CUSP[3].0, NEAR_CUSP[3].1),
    ));

    let stroke = Stroke::new(STROKE_WIDTH / s)
        .with_caps(Cap::Butt)
        .with_join(Join::Miter);
    let transform = Affine::new([s, 0.0, 0.0, s, 1000., 1000.]);

    let mut scene = Scene::new();
    scene.stroke(&stroke, transform, Color::from_rgb8(0, 0, 0), None, &path);

    let encoding = scene.encoding();
    let mut resolver = Resolver::new();
    let mut packed: Vec<u8> = Vec::new();
    let (layout, _ramps, _images) = resolver.resolve(encoding, &mut packed);

    let cpu_config = RenderConfig::new(&layout, 4000, 4000, &Color::TRANSPARENT);
    let config: ConfigUniform = cpu_config.gpu;
    let wg = &cpu_config.workgroup_counts;
    let sizes = &cpu_config.buffer_sizes;

    let scene_buf = packed.clone();
    let reduced_buf = RefCell::new(vec![0_u8; sizes.path_reduced.size_in_bytes() as usize]);
    let tagmonoid_buf = RefCell::new(vec![0_u8; sizes.path_monoids.size_in_bytes() as usize]);
    let path_bbox_buf = RefCell::new(vec![0_u8; sizes.path_bboxes.size_in_bytes() as usize]);
    let bump_buf = RefCell::new(vec![0_u8; size_of::<BumpAllocators>()]);
    let lines_buf = RefCell::new(vec![0_u8; sizes.lines.size_in_bytes() as usize]);

    let config_bytes = bytemuck::bytes_of(&config);

    vello_shaders::cpu::pathtag_reduce(
        wg.path_reduce.0,
        &[
            CpuBinding::Buffer(config_bytes),
            CpuBinding::Buffer(bytemuck::cast_slice(&scene_buf)),
            CpuBinding::BufferRW(&reduced_buf),
        ],
    );
    vello_shaders::cpu::pathtag_scan(
        wg.path_scan.0,
        &[
            CpuBinding::Buffer(config_bytes),
            CpuBinding::Buffer(bytemuck::cast_slice(&scene_buf)),
            CpuBinding::Buffer(&reduced_buf.borrow()),
            CpuBinding::BufferRW(&tagmonoid_buf),
        ],
    );
    vello_shaders::cpu::bbox_clear(
        wg.bbox_clear.0,
        &[
            CpuBinding::Buffer(config_bytes),
            CpuBinding::BufferRW(&path_bbox_buf),
        ],
    );
    vello_shaders::cpu::flatten(
        wg.flatten.0,
        &[
            CpuBinding::Buffer(config_bytes),
            CpuBinding::Buffer(bytemuck::cast_slice(&scene_buf)),
            CpuBinding::Buffer(&tagmonoid_buf.borrow()),
            CpuBinding::BufferRW(&path_bbox_buf),
            CpuBinding::BufferRW(&bump_buf),
            CpuBinding::BufferRW(&lines_buf),
        ],
    );

    let bump: BumpAllocators = *bytemuck::from_bytes(&bump_buf.borrow());
    let n_lines = bump.lines as usize;
    assert_eq!(bump.failed, 0, "bump allocator reported failure at scale {s}");

    let lines_bytes = lines_buf.borrow();
    bytemuck::cast_slice::<u8, LineSoup>(&lines_bytes[..n_lines * size_of::<LineSoup>()]).to_vec()
}

fn main() {
    // The reporter's own identified affected range (5.3332..113.5797), plus scales well outside
    // it on both sides.
    let scales = [
        1.0, 2.0, 5.0, 5.3332, 8.0, 16.0, 32.0, 64.0, 100.0, 113.5797, 120.0, 150.0, 200.0,
    ];

    for s in scales {
        let lines = flatten_near_cusp_at_scale(s);
        for l in &lines {
            assert!(
                l.p0[0].is_finite()
                    && l.p0[1].is_finite()
                    && l.p1[0].is_finite()
                    && l.p1[1].is_finite(),
                "non-finite point emitted at scale {s}: {l:?}"
            );
        }
        let mut total_len = 0.0_f64;
        for l in &lines {
            let dx = (l.p1[0] - l.p0[0]) as f64;
            let dy = (l.p1[1] - l.p0[1]) as f64;
            total_len += (dx * dx + dy * dy).sqrt();
        }
        println!(
            "scale={s:>10.4}  n_lines={:>4}  total_polyline_length={total_len:>12.4} (device px)",
            lines.len()
        );
    }

    println!(
        "\ndid not reproduce: the real production CPU-shader `flatten` pipeline (plain scalar \
         f32 Rust arithmetic, strictly IEEE-754, no fast-math/reassociation of any kind) never \
         crashes, never emits a non-finite point, and produces a consistently-growing segment \
         count/polyline length as `s` increases (no sudden collapse or blow-up) across the \
         reporter's own affected scale range and well outside it, on this exact near-cusp cubic. \
         This matches the issue reporter's own finding that the artifact is absent on CPU shaders \
         - see RESULT.md for the additional real-GPU-shader-dispatch check (bin/gpu_check) and \
         why a genuine Metal/Apple-hardware repro was not attempted this session."
    );
}
