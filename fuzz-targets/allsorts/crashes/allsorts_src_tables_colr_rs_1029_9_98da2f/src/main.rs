use allsorts::tables::colr::ClipBox;

fn main() {
    let clip_box = ClipBox { x_min: 0i16, y_min: i16::MAX, x_max: 0i16, y_max: -10000i16, var_index_base: None };
    let _ = clip_box.height();
}
