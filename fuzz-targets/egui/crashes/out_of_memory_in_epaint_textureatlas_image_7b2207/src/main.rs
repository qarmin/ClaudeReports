use epaint::TextOptions;
use epaint::TextureAtlas;

fn main() {
    let texture_atlas = TextureAtlas::new([1000000000usize, 1usize], TextOptions::default());
    let _ = texture_atlas.image();
}
