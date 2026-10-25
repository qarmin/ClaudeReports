use epaint::FontColorTransferFunction;
use epaint::TextOptions;
use epaint::TextureAtlas;

fn main() {
    let texture_atlas = TextureAtlas::new([10000000000000000usize, 1usize], TextOptions { max_texture_side: 0usize, color_transfer_function: FontColorTransferFunction::Off, font_hinting: false, subpixel_binning: false });
    let _ = texture_atlas.options();
}
