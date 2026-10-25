use ecolor::Color32;
use egui::Event;
use epaint::ColorImage;

fn main() {
    let _ = Event::PasteImage(std::sync::Arc::new(ColorImage::filled([1usize, 1000000000usize], Color32::GOLD)));
}
