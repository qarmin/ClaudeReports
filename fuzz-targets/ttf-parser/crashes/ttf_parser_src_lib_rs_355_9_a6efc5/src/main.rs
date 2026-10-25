use ttf_parser::Rect;

fn main() {
    let rect = Rect { x_min: i16::MIN, y_min: 0i16, x_max: 0i16, y_max: 0i16 };
    let _ = rect.width();
}
