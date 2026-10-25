use krilla::Data;
use krilla::document::Document;
use krilla::geom::Point;
use krilla::geom::Transform;
use krilla::num::NormalizedF32;
use krilla::paint::LineCap;
use krilla::paint::LineJoin;
use krilla::paint::LinearGradient;
use krilla::paint::Paint;
use krilla::paint::SpreadMethod;
use krilla::paint::Stroke;
use krilla::text::Font;
use krilla::text::TextDirection;

fn main() {
    let paint = Paint::from(LinearGradient { x1: 0.0f32, y1: 0.0f32, x2: 0.0f32, y2: 0.0f32, transform: Transform::identity(), spread_method: SpreadMethod::Pad, stops: vec![], anti_alias: false });
    let mut document = Document::new();
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.set_stroke(Some(Stroke { paint: paint.clone(), width: 1.0f32, miter_limit: 0.0f32, line_cap: LineCap::Square, line_join: LineJoin::Bevel, opacity: NormalizedF32::new(0.0f32).unwrap(), dash: None }));
    surface.draw_text(Point::from_xy(0.0f32, 0.0f32), Font::new(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32).unwrap(), 0.0f32, "1", false, TextDirection::BottomToTop);
}
