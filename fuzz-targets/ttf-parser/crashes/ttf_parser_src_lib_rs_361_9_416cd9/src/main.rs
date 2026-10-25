use ttf_parser::Rect;

fn main() {
    let rect = Rect { x_min: 0i16, y_min: i16::MAX, x_max: 0i16, y_max: -10000i16 };
    let _ = rect.height();
}
