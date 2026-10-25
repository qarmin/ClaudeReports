use egui::Style;
use egui_demo_lib::easy_mark::MemoizedEasymarkHighlighter;
use epaint::Galley;
use epaint::text::cursor::CCursor;

fn main() {
    let galley = Galley::concat(std::sync::Arc::new(MemoizedEasymarkHighlighter::default().highlight(&Style::default(), "")), &[], 0.0f32);
    let _ = galley.cursor_end_of_paragraph(&CCursor::end_of_str(""));
}
