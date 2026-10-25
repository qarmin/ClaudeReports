use epaint::TextOptions;
use epaint::TextureAtlas;

fn main() {
    let mut texture_atlas = TextureAtlas::new([10000usize, 1usize], TextOptions::default());
    let _ = texture_atlas.allocate((0usize, 1000000000usize));
    let _ = texture_atlas.allocate((0usize, usize::MAX));
}
