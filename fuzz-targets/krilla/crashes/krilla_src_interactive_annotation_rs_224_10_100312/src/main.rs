use krilla::annotation::LinkAnnotation;
use krilla::annotation::Target;
use krilla::destination::Destination;
use krilla::destination::XyzDestination;
use krilla::geom::Point;
use krilla::geom::Quadrilateral;

fn main() {
    let _ = LinkAnnotation::new_with_quad_points(vec![Quadrilateral([Point::from_xy(0.0f32, 0.0f32), Point { x: 0.0f32, y: 0.0f32 }, Point::from_xy(0.0f32, 0.0f32), Point { x: 0.0f32, y: f32::MIN }])], Target::Destination(Destination::Xyz(XyzDestination::new(0usize, Point::from_xy(0.0f32, 0.0f32)))));
}
