use krilla::Data;
use krilla::SerializeSettings;
use krilla::document::Document;
use krilla::geom::Point;
use krilla::geom::Size;
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::Fill;
use krilla::paint::FillRule;
use krilla::paint::Paint;
use krilla::paint::SpreadMethod;
use krilla::paint::SweepGradient;
use krilla::text::Font;
use krilla::text::TextDirection;

fn main() {
    let spread_method = SpreadMethod::Pad;
    let mut document = Document::new_with(SerializeSettings::default());
    let mut page = document.start_page_with(PageSettings::new(Size::from_wh(1.0f32, 1.0f32).unwrap()));
    let mut surface = page.surface();
    let transform = surface.cur_transform();
    surface.set_fill(Some(Fill { paint: Paint::from(SweepGradient { cx: 0.0f32, cy: 1.0f32, start_angle: 0.0f32, end_angle: 1.0f32, transform: transform, spread_method: spread_method, stops: vec![], anti_alias: false }), opacity: NormalizedF32::ZERO, rule: FillRule::default() }));
    surface.draw_text(Point::default(), Font::new(Data::from(std::sync::Arc::new(include_bytes!("data_0.bin").to_vec())), 0u32).unwrap(), 0.0f32, "Z", false, TextDirection::LeftToRight);
}
