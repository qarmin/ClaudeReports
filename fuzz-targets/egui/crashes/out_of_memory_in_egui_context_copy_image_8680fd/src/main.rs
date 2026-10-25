use ecolor::Color32;
use egui::Context;
use epaint::ColorImage;

fn main() {
    let context = Context::default();
    context.copy_image(ColorImage::filled([1usize, 10000000000usize], Color32::from_rgba_unmultiplied_const(0u8, 0u8, 0u8, 0u8)));
}
