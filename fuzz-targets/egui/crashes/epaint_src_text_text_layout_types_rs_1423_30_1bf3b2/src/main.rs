use egui::Context;
use egui::Style;
use egui_extras::syntax_highlighting::CodeTheme;
use epaint::Galley;
use epaint::text::cursor::CCursor;

fn main() {
    let galley = Galley::concat(std::sync::Arc::new(egui_extras::syntax_highlighting::highlight(&Context::default(), &Style::default(), &CodeTheme::default(), "", "")), &[], 0.0f32);
    let _ = galley.cursor_end_of_row(&CCursor::end_of_str(""));
}
