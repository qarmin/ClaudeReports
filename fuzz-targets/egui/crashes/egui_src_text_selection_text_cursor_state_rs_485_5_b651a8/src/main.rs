use epaint::text::CharIndex;

fn main() {
    let _ = egui::text_selection::text_cursor_state::slice_char_range("", CharIndex::default().saturating_add(1usize)..CharIndex::default());
}
