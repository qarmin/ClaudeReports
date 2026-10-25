use krilla::document::Document;
use krilla::geom::PathBuilder;
use krilla::paint::FillRule;

fn main() {
    let mut path_builder = PathBuilder::new();
    path_builder.quad_to(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    path_builder.close();
    let fill_rule = FillRule::NonZero;
    let mut document = Document::default();
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.push_clip_path(&path_builder.finish().unwrap(), &fill_rule);
}
