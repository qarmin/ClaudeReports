use epaint::TextOptions;
use epaint::TextureAtlas;

fn main() {
    let mut texture_atlas = TextureAtlas::new([1000000usize, 1usize], TextOptions::default());
    let _ = texture_atlas.allocate((0usize, usize::MAX));
}
