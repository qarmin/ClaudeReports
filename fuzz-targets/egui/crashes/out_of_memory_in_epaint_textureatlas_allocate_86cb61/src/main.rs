use epaint::FontColorTransferFunction;
use epaint::TextOptions;
use epaint::TextureAtlas;

fn main() {
    let mut texture_atlas = TextureAtlas::new([65536usize, 1usize], TextOptions { max_texture_side: 0usize, color_transfer_function: FontColorTransferFunction::Gamma(0.0f32), font_hinting: false, subpixel_binning: false });
    let _ = texture_atlas.allocate((1usize, 10000usize));
}
