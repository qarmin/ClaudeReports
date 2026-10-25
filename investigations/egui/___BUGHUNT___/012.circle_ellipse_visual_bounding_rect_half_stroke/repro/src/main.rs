use epaint::{
    CircleShape, Color32, EllipseShape, Mesh, Shape, Stroke, TessellationOptions, Tessellator, pos2, vec2,
};

fn tessellated_bounds(shape: &Shape) -> epaint::Rect {
    let options = TessellationOptions { coarse_tessellation_culling: false, feathering: false, ..Default::default() };
    let mut tessellator = Tessellator::new(1.0, options, [1024, 1024], vec![]);
    let mut mesh = Mesh::default();
    tessellator.tessellate_shape(shape.clone(), &mut mesh);
    mesh.calc_bounds()
}

fn report(name: &str, shape: Shape) {
    let promised = shape.visual_bounding_rect();
    let drawn = tessellated_bounds(&shape);
    let overshoot = (promised.min.x - drawn.min.x).max(drawn.max.x - promised.max.x);
    println!(
        "{name}: visual_bounding_rect x {:.2}..{:.2}, tessellated stroke x {:.2}..{:.2}, drawn beyond the bounding rect by {overshoot:.2}",
        promised.min.x, promised.max.x, drawn.min.x, drawn.max.x
    );
}

fn main() {
    let stroke = Stroke::new(20.0, Color32::RED);
    report("circle  radius 10, stroke 20", Shape::Circle(CircleShape { center: pos2(0.0, 0.0), radius: 10.0, fill: Color32::TRANSPARENT, stroke }));
    report(
        "ellipse radii 10x20, stroke 20",
        Shape::Ellipse(EllipseShape { center: pos2(0.0, 0.0), radius: vec2(10.0, 20.0), fill: Color32::TRANSPARENT, stroke, angle: 0.0 }),
    );
}
