use unicode_segmentation::GraphemeCursor;

fn main() {
    let mut grapheme_cursor = GraphemeCursor::new(1usize, 0usize, true);
    let usize_v = grapheme_cursor.cur_cursor();
    let _ = grapheme_cursor.is_boundary("a", usize_v);
    grapheme_cursor.provide_context("", 1usize);
}
