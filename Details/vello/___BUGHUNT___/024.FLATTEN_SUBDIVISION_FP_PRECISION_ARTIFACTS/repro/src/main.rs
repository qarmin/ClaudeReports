// Repro for candidate 024: FLATTEN_SUBDIVISION_FP_PRECISION_ARTIFACTS.
//
// Runs the REAL end-to-end classic-vello pipeline through public API:
//   vello::Scene::stroke (real public API, exactly what an application calls)
//     -> Scene's internal Encoding (produced by the real encoder)
//     -> vello_encoding::Resolver::resolve (the real resolve step vello::Renderer itself calls)
//     -> vello_encoding::RenderConfig::new (the real GPU-config/workgroup-count computation)
//     -> vello_shaders::cpu::{pathtag_reduce, pathtag_scan, bbox_clear, flatten} (the exact
//        production CPU-shader implementations that vello::Renderer itself dispatches as a
//        fallback for a GPU that's missing a required feature - not a reimplementation we wrote,
//        the literal same functions, exercised in the same order vello/src/render.rs uses)
//
// No GPU/wgpu is involved (this environment has none available), but every function invoked here
// is the real production code that a real vello::Renderer invocation would also call for these
// specific pipeline stages - nothing about the encoded scene or the flatten algorithm is faked.
//
// Reproduces https://github.com/linebender/vello/issues/1139 : a short, real-world DWG-derived
// line segment stroked under a transform with large translation exhibits large flattening error
// because vello's GPU-stroke path flattens stroke offset curves in *local* (pre-transform)
// coordinates using f32, and the CAD-derived local coordinates here are large enough that f32
// precision loss corrupts the offset-curve tangent/curvature computation.

use std::cell::RefCell;
use vello::Scene;
use vello::kurbo::{Affine, Line, Stroke};
use vello::low_level::vello_encoding::{
    BumpAllocators, ConfigUniform, LineSoup, RenderConfig, Resolver,
};
use vello::peniko::Color;
use vello_shaders::cpu::CpuBinding;

fn main() {
    // Exact repro from linebender/vello#1139.
    let stroke = Stroke::new(2.0).with_caps(vello::kurbo::Cap::Butt);
    let stroke_color = Color::new([1., 0.0, 0.0, 1.]);
    let line = Line::new((41550.906, 4632.2827), (41550.926, 4632.273));
    let transform = Affine::new([853.3334, 0.0, 0.0, -1360.7974, -35455741.0, 6302581.5]);

    let mut scene = Scene::new();
    scene.stroke(&stroke, transform, stroke_color, None, &line);

    let encoding = scene.encoding();
    let mut resolver = Resolver::new();
    let mut packed: Vec<u8> = Vec::new();
    let (layout, _ramps, _images) = resolver.resolve(encoding, &mut packed);

    let cpu_config = RenderConfig::new(&layout, 64, 64, &Color::TRANSPARENT);
    let config: ConfigUniform = cpu_config.gpu;
    let wg = &cpu_config.workgroup_counts;
    let sizes = &cpu_config.buffer_sizes;

    // Buffers, all zero-initialized exactly as `vello::low_level::Render` does (via
    // `recording.clear_all`/fresh GPU buffer allocation) before the real dispatch sequence.
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
    println!("bump.failed = {}, n_lines = {}", bump.failed, n_lines);

    let lines_bytes = lines_buf.borrow();
    let lines: &[LineSoup] = bytemuck::cast_slice(&lines_bytes[..n_lines * size_of::<LineSoup>()]);

    // Ideal (exact, f64) device-space geometry: for a straight line stroked at half-width
    // `offset`, each offset edge is *exactly* a straight segment parallel to the original line,
    // at perpendicular distance `offset` in *local* space, then transformed to device space.
    let (p0, p1) = (line.p0, line.p1);
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    let len = (dx * dx + dy * dy).sqrt();
    let (nx, ny) = (-dy / len, dx / len); // unit normal, local space
    let offset = stroke.width / 2.0;

    let to_device = |x: f64, y: f64| transform * vello::kurbo::Point::new(x, y);

    let ideal_top0 = to_device(p0.x + nx * offset, p0.y + ny * offset);
    let ideal_top1 = to_device(p1.x + nx * offset, p1.y + ny * offset);
    let ideal_bot0 = to_device(p0.x - nx * offset, p0.y - ny * offset);
    let ideal_bot1 = to_device(p1.x - nx * offset, p1.y - ny * offset);

    // Perpendicular device-space distance of point `p` from the infinite line through `a`-`b`.
    fn perp_dist(a: vello::kurbo::Point, b: vello::kurbo::Point, p: (f32, f32)) -> f64 {
        let (adx, ady) = (b.x - a.x, b.y - a.y);
        let len = (adx * adx + ady * ady).sqrt();
        let (px, py) = (p.0 as f64 - a.x, p.1 as f64 - a.y);
        ((adx * py - ady * px) / len).abs()
    }

    let mut max_dev = 0.0_f64;
    for l in lines {
        let d0 = perp_dist(ideal_top0, ideal_top1, (l.p0[0], l.p0[1]))
            .min(perp_dist(ideal_bot0, ideal_bot1, (l.p0[0], l.p0[1])));
        let d1 = perp_dist(ideal_top0, ideal_top1, (l.p1[0], l.p1[1]))
            .min(perp_dist(ideal_bot0, ideal_bot1, (l.p1[0], l.p1[1])));
        println!(
            "line p0={:?} p1={:?}  dev0={:.4} dev1={:.4}",
            l.p0, l.p1, d0, d1
        );
        max_dev = max_dev.max(d0).max(d1);
    }

    // The flattening algorithm's own documented tolerance is 0.25 device pixels (`tol` in
    // flatten_euler). A correct implementation should keep every emitted point within a small
    // constant factor of that. A precision-collapse bug blows this budget by orders of magnitude.
    const TOL: f64 = 0.25;
    println!("max deviation from ideal straight offset edge: {max_dev:.4} device px (tol={TOL})");
    if max_dev > TOL * 10.0 {
        eprintln!(
            "REPRODUCED: flattened stroke offset points deviate {max_dev:.4} device px from the \
             true straight line, far exceeding the algorithm's own {TOL} px tolerance budget"
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - flattened points stayed within tolerance");
    }
}
