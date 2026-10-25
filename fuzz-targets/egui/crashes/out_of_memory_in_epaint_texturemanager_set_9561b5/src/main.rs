use epaint::TextOptions;
use epaint::TextureAtlas;
use epaint::TextureId;
use epaint::TextureManager;

fn main() {
    let mut texture_manager = TextureManager::default();
    texture_manager.set(TextureId::default(), TextureAtlas::new([100000000000usize, 1usize], TextOptions::default()).take_delta().unwrap());
}
