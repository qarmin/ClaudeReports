use gdk4::Cursor;

fn main() {
    let cursor_builder = Cursor::builder();
    let cursor_builder2 = cursor_builder.hotspot_y(-1000000000i32);
    let _ = cursor_builder2.build();
}
