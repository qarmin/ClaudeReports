use epaint::{
    ClippedShape, Color32, PathShape, PathStroke, Primitive, Rect, Shape, StrokeKind, TessellationOptions,
    Tessellator, pos2,
};

fn vertex_count(shape: &Shape, clip_rect: Rect, coarse_culling: bool) -> usize {
    let options = TessellationOptions { coarse_tessellation_culling: coarse_culling, feathering: false, ..Default::default() };
    let mut tessellator = Tessellator::new(1.0, options, [1024, 1024], vec![]);
    tessellator
        .tessellate_shapes(vec![ClippedShape::new(clip_rect, shape.clone())])
        .iter()
        .map(|p| if let Primitive::Mesh(mesh) = &p.primitive { mesh.vertices.len() } else { 0 })
        .sum()
}

/// Bounds of the vertices the stroke really covers (no culling, no feathering).
fn drawn_bounds(shape: &Shape) -> Rect {
    let options = TessellationOptions { coarse_tessellation_culling: false, feathering: false, ..Default::default() };
    let mut mesh = epaint::Mesh::default();
    Tessellator::new(1.0, options, [1024, 1024], vec![]).tessellate_shape(shape.clone(), &mut mesh);
    mesh.calc_bounds()
}

fn main() {
    let square = vec![pos2(0.0, 0.0), pos2(10.0, 0.0), pos2(10.0, 10.0), pos2(0.0, 10.0)];
    for kind in [StrokeKind::Middle, StrokeKind::Outside] {
        let mut stroke = PathStroke::new(20.0, Color32::RED);
        stroke.kind = kind;
        let shape = Shape::Path(PathShape { points: square.clone(), closed: true, fill: Color32::TRANSPARENT, stroke });
        println!("{kind:?}: visual_bounding_rect = {:?}", shape.visual_bounding_rect());
        // The clip rect sits in the outer 6 points of the stroke band on the left of the square.
        let clip = Rect::from_min_max(pos2(-18.0, 0.0), pos2(-12.0, 10.0));
        let drawn = drawn_bounds(&shape);
        let culled = vertex_count(&shape, clip, true) == 0;
        println!(
            "  the stroke is really drawn over x {:.0}..{:.0}; clip rect x -18..-12 {} it; the shape is {}",
            drawn.min.x,
            drawn.max.x,
            if drawn.intersects(clip) { "overlaps" } else { "does not overlap" },
            if culled { "CULLED (nothing tessellated)" } else { "tessellated" }
        );
    }
}
