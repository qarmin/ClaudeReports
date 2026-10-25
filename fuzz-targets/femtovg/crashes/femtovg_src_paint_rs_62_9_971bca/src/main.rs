use femtovg::Paint;

fn main() {
    let paint = Paint::elliptical_gradient_stops(0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, vec![]);
    let paint2 = paint.clone().with_font_weight(0.0f32);
    let paint3 = paint2.clone().with_font_variation(&[1u8, 0u8, 0u8, 0u8], 0.0f32);
    let paint4 = paint3.clone().with_font_italic(false);
    let _ = paint4.clone().with_font_variation(&[0u8, 0u8, 0u8, 1u8], 0.0f32);
}
