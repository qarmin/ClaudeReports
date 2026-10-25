use krilla::destination::NamedDestination;
use krilla::destination::XyzDestination;
use krilla::document::Document;
use krilla::geom::Point;

fn main() {
    let mut document = Document::new();
    let _ = document.register_named_destination(NamedDestination::new("".to_string(), XyzDestination::new(usize::MAX, Point { x: 0.0f32, y: 0.0f32 })));
    let _ = document.finish();
}
