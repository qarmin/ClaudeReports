use epaint::FontColorTransferFunction;
use epaint::TextOptions;
use epaint::TextureAtlas;

fn main() {
    let _ = TextureAtlas::new([10000000000000000000usize, 0usize], TextOptions { max_texture_side: 0usize, color_transfer_function: FontColorTransferFunction::Off, font_hinting: false, subpixel_binning: false });
}
