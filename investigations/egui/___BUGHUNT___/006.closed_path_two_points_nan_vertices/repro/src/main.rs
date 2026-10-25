use epaint::{
    ClippedPrimitive, ClippedShape, Color32, Primitive, Rect, Shape, Stroke,
    TessellationOptions, Tessellator, pos2,
};

fn non_finite_vertices(shape: Shape) -> (usize, usize) {
    let mut tessellator = Tessellator::new(1.0, TessellationOptions::default(), [1024, 1024], vec![]);
    let prims: Vec<ClippedPrimitive> = tessellator.tessellate_shapes(vec![ClippedShape::new(Rect::EVERYTHING, shape)]);
    let mut total = 0;
    let mut bad = 0;
    for prim in &prims {
        if let Primitive::Mesh(mesh) = &prim.primitive {
            total += mesh.vertices.len();
            bad += mesh.vertices.iter().filter(|v| !v.pos.x.is_finite() || !v.pos.y.is_finite()).count();
        }
    }
    (total, bad)
}

fn main() {
    let stroke = Stroke::new(4.0, Color32::RED);
    let a = pos2(0.0, 0.0);
    let b = pos2(10.0, 0.0);
    let cases: Vec<(&str, Shape)> = vec![
        ("closed_line, 3 points (triangle)", Shape::closed_line(vec![a, b, pos2(10.0, 10.0)], stroke)),
        ("closed_line, 2 points", Shape::closed_line(vec![a, b], stroke)),
        ("closed_line, 3 collinear points", Shape::closed_line(vec![a, b, pos2(20.0, 0.0)], stroke)),
        ("closed_line, repeated first point", Shape::closed_line(vec![a, a, b], stroke)),
        ("convex_polygon, 2 points + fill", Shape::convex_polygon(vec![a, b], Color32::GREEN, stroke)),
        ("open line, a-b-a (control)", Shape::line(vec![a, b, a], stroke)),
    ];
    for (name, shape) in cases {
        let (total, bad) = non_finite_vertices(shape);
        println!("{name:40} vertices={total:3} non-finite={bad}");
    }
}
