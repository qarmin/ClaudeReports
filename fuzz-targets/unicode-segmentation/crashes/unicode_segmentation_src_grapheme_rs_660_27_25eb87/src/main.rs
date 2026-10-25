use unicode_segmentation::GraphemeCursor;

fn main() {
    let mut grapheme_cursor = GraphemeCursor::new(1usize, 0usize, false);
    let _ = grapheme_cursor.is_boundary("\u{1c30b}", 0usize);
}
