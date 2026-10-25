use ecolor::Color32;
use epaint::ColorImage;
use epaint::FontColorTransferFunction;

fn main() {
    let _ = ColorImage::new([1000000000usize, 10000000000000000000usize], vec![FontColorTransferFunction::TwoCoverageMinusCoverageSq.color_from_coverage(65536.0f32), Color32::DARK_RED, Color32::from_rgba_unmultiplied(255u8, 0u8, 8u8, 162u8)]);
}
