use ecolor::Color32;
use egui::load::ImagePoll;
use epaint::ColorImage;

fn main() {
    let _ = ImagePoll::Ready { image: std::sync::Arc::new(ColorImage::filled([1usize, 1000000000usize], Color32::from_rgba_unmultiplied_const(0u8, 1u8, 0u8, 0u8))) };
}
