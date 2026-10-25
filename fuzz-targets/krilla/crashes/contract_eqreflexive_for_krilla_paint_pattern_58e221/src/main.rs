use krilla::document::Document;
use krilla::geom::Transform;
use krilla::paint::Pattern;

fn main() {
    let mut document = Document::new();
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.push_transform(&Transform::from_scale(0.0f32, 0.0f32).invert().unwrap());
    let transform = surface.ctm();
    let mut stream_builder = surface.stream_builder();
    let mut surface2 = stream_builder.surface();
    let stream_builder2 = surface2.stream_builder();
    let stream = stream_builder2.finish();
    let pattern = Pattern { stream: stream.clone(), transform: transform, width: 0.0f32, height: 0.0f32 };
    assert!(pattern == pattern, "Eq is not reflexive");
}
