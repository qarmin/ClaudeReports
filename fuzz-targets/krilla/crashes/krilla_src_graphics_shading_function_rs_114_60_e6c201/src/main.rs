use krilla::Data;
use krilla::document::Document;
use krilla::geom::Point;
use krilla::geom::Transform;
use krilla::num::NormalizedF32;
use krilla::paint::Fill;
use krilla::paint::FillRule;
use krilla::paint::LinearGradient;
use krilla::paint::Paint;
use krilla::paint::SpreadMethod;
use krilla::text::Font;
use krilla::text::TextDirection;

fn main() {
    let font = Font::new(Data::from(std::sync::Arc::new(include_bytes!("data_0.bin").to_vec())), 0u32).unwrap();
    let mut document = Document::default();
    let mut page = document.start_page();
    let mut surface = page.surface();
    let fill = Fill { paint: Paint::from(LinearGradient { x1: 0.0f32, y1: f32::INFINITY, x2: 0.0f32, y2: 0.0f32, transform: Transform::from_rotate(0.0f32), spread_method: SpreadMethod::Reflect, stops: vec![], anti_alias: false }), opacity: NormalizedF32::ZERO, rule: FillRule::default() };
    surface.set_fill(Some(fill.clone()));
    surface.draw_text(Point::from_xy(0.0f32, 0.0f32), font.clone(), 0.0f32, "a", false, TextDirection::Auto);
}
