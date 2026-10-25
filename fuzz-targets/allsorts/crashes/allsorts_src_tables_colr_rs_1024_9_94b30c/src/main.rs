use allsorts::tables::colr::ClipBox;

fn main() {
    let clip_box = ClipBox { x_min: i16::MIN, y_min: 0i16, x_max: 0i16, y_max: 0i16, var_index_base: None };
    let _ = clip_box.width();
}
