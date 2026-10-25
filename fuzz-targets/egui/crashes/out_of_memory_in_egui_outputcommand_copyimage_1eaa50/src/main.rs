use ecolor::Color32;
use egui::OutputCommand;
use epaint::ColorImage;

fn main() {
    let _ = OutputCommand::CopyImage(ColorImage::filled([1000000000usize, 1usize], Color32::GRAY));
}
