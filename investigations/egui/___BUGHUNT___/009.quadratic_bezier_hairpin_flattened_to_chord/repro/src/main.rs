use epaint::{Color32, QuadraticBezierShape, Shape, Stroke, TessellationOptions, Tessellator, ClippedShape, Primitive, Rect, pos2, Pos2};

fn x_range(points: &[Pos2]) -> (f32, f32) {
    points.iter().fold((f32::MAX, f32::MIN), |(lo, hi), p| (lo.min(p.x), hi.max(p.x)))
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn coord(&mut self) -> f32 {
        ((self.next() % 200_001) as f32 - 100_000.0) / 1000.0
    }
}

fn max_deviation(q: &QuadraticBezierShape, polyline: &[Pos2]) -> f32 {
    (0..=200)
        .map(|i| {
            let p = q.sample(i as f32 / 200.0).to_vec2();
            polyline
                .windows(2)
                .map(|w| {
                    let (a, b) = (w[0].to_vec2(), w[1].to_vec2());
                    let ab = b - a;
                    let t = if ab.length_sq() > 0.0 { ((p - a).dot(ab) / ab.length_sq()).clamp(0.0, 1.0) } else { 0.0 };
                    (a + ab * t - p).length()
                })
                .fold(f32::MAX, f32::min)
        })
        .fold(0.0, f32::max)
}

fn main() {
    // Random quadratics with coordinates in [-100, 100]: how far is the true curve from the flattened polyline?
    let (tolerance, n) = (0.1_f32, 200_000);
    let mut rng = Rng(12345);
    let (mut violations, mut worst) = (0, 0.0_f32);
    for _ in 0..n {
        let pts = [pos2(rng.coord(), rng.coord()), pos2(rng.coord(), rng.coord()), pos2(rng.coord(), rng.coord())];
        let q = QuadraticBezierShape::from_points_stroke(pts, false, Color32::TRANSPARENT, Stroke::NONE);
        let deviation = max_deviation(&q, &q.flatten(Some(tolerance)));
        if deviation > 2.0 * tolerance {
            violations += 1;
            worst = worst.max(deviation);
        }
    }
    println!("random quadratics, tolerance {tolerance}: {violations}/{n} deviate by more than 2x tolerance (worst deviation {worst:.2})");

    // A quadratic whose three control points are collinear, with the control point beyond the end point:
    // the curve runs out to x ~ 5.26 and then comes back to x = 1.
    let q = QuadraticBezierShape::from_points_stroke(
        [pos2(0.0, 0.0), pos2(10.0, 0.0), pos2(1.0, 0.0)],
        false,
        Color32::TRANSPARENT,
        Stroke::NONE,
    );
    let (_, true_max) = x_range(&(0..=1000).map(|i| q.sample(i as f32 / 1000.0)).collect::<Vec<_>>());
    let flat = q.flatten(Some(0.1));
    println!("quadratic: true max x = {true_max:.3}, flatten(Some(0.1)) = {flat:?} (max x {})", x_range(&flat).1);

    // The same through the real tessellator (what egui paints): the stroke only covers x in [0, 1].
    let shape = Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke(
        [pos2(0.0, 0.0), pos2(10.0, 0.0), pos2(1.0, 0.0)],
        false,
        Color32::TRANSPARENT,
        Stroke::new(2.0, Color32::RED),
    ));
    let mut tessellator = Tessellator::new(1.0, TessellationOptions::default(), [1024, 1024], vec![]);
    let prims = tessellator.tessellate_shapes(vec![ClippedShape::new(Rect::EVERYTHING, shape)]);
    let mut max_x = f32::MIN;
    for prim in &prims {
        if let Primitive::Mesh(mesh) = &prim.primitive {
            max_x = mesh.vertices.iter().map(|v| v.pos.x).fold(max_x, f32::max);
        }
    }
    println!("tessellated stroke: max vertex x = {max_x:.3} (expected about {true_max:.3} + stroke half width)");

    // A control: bending the control point off the line by a tiny amount makes the overshoot appear.
    let q2 = QuadraticBezierShape::from_points_stroke(
        [pos2(0.0, 0.0), pos2(10.0, 0.01), pos2(1.0, 0.0)],
        false,
        Color32::TRANSPARENT,
        Stroke::NONE,
    );
    println!("control (control point 0.01 off the line): flatten max x = {:.3}", x_range(&q2.flatten(Some(0.1))).1);
}
