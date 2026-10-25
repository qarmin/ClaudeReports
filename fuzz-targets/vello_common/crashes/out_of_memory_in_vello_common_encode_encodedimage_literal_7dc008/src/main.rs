use kurbo::Affine;
use kurbo::Vec2;
use peniko::ImageSampler;
use vello_common::encode::EncodedImage;
use vello_common::paint::ImageSource;
use vello_common::pixmap::Pixmap;
use vello_cpu::Resources;

fn main() {
    let _ = EncodedImage { source: ImageSource::opaque_id(Resources::new().register_image(std::sync::Arc::new(Pixmap::new(32766u16, 16384u16)))), sampler: ImageSampler::default(), may_have_transparency: false, transform: Affine::IDENTITY, x_advance: Vec2::splat(0.0f64), y_advance: Vec2::from_angle(0.0f64), tint: None };
}
