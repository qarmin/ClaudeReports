use krilla::SerializeSettings;
use krilla::destination::NamedDestination;
use krilla::destination::XyzDestination;
use krilla::document::Document;
use krilla::geom::Point;

fn main() {
    let mut document = Document::new_with(SerializeSettings::default());
    let _ = document.register_named_destination(NamedDestination::new("".to_string(), XyzDestination::new(1usize, Point::default())));
    let _ = document.finish();
}
