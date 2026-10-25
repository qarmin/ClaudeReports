use egui::Style;
use egui::text_selection::CCursorRange;
use egui::text_selection::TextCursorState;
use egui_demo_lib::easy_mark::MemoizedEasymarkHighlighter;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;

fn main() {
    let text_cursor_state = TextCursorState::from(CCursorRange::default());
    let _ = text_cursor_state.range(&Fonts::new(TextOptions::default(), FontDefinitions::empty()).layout_uncached(65536.0f32, std::sync::Arc::new(MemoizedEasymarkHighlighter::default().highlight(&Style::default(), "a"))));
}
