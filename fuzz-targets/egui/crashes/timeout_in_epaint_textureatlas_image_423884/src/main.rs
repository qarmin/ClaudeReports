use epaint::FontColorTransferFunction;
use epaint::TextOptions;
use epaint::TextureAtlas;

fn main() {
    let texture_atlas = TextureAtlas::new([10000usize, 0usize], TextOptions { max_texture_side: 0usize, color_transfer_function: FontColorTransferFunction::LIGHT_MODE_DEFAULT, font_hinting: false, subpixel_binning: false });
    let _ = texture_atlas.image();
}
