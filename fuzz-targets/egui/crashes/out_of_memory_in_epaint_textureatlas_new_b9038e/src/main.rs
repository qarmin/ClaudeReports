use epaint::FontColorTransferFunction;
use epaint::TextOptions;
use epaint::TextureAtlas;

fn main() {
    let _ = TextureAtlas::new([1000000000usize, 1usize], TextOptions { max_texture_side: 1usize, color_transfer_function: FontColorTransferFunction::TwoCoverageMinusCoverageSq, font_hinting: false, subpixel_binning: false });
}
