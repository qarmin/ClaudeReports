use epaint::TextOptions;
use epaint::TextureAtlas;

fn main() {
    let mut texture_atlas = TextureAtlas::new([10000usize, 1usize], TextOptions::default());
    let _ = texture_atlas.allocate((65536usize, 0usize));
}
