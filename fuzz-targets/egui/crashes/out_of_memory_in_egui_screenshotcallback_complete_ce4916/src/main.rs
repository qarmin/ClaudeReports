use ecolor::Color32;
use ecolor::Hsva;
use egui::ScreenshotCallback;
use epaint::ColorImage;

fn main() {
    let screenshot_callback = ScreenshotCallback::new(|_| ());
    screenshot_callback.clone().complete(std::sync::Arc::new(ColorImage::filled([1usize, 1000000000usize], Color32::from(Hsva::from_srgba_premultiplied([0u8, 0u8, 0u8, 0u8])))));
}
