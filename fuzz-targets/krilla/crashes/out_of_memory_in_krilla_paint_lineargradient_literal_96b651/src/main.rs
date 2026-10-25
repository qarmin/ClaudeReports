use krilla::Data;
use krilla::geom::Transform;
use krilla::image::Image;
use krilla::paint::LinearGradient;
use krilla::paint::SpreadMethod;

fn main() {
    let transform = Transform::from_scale(0.0f32, 0.0f32);
    let f32_v = transform.tx();
    let _ = Image::from_webp(Data::from(include_bytes!("data_0.bin").to_vec()), false);
    let _ = LinearGradient { x1: 0.0f32, y1: 0.0f32, x2: f32_v, y2: f32::NAN, transform: transform, spread_method: SpreadMethod::Pad, stops: vec![], anti_alias: false };
}
