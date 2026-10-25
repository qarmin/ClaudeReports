use epaint::TextureId;
use epaint::TextureManager;

fn main() {
    let mut texture_manager = TextureManager::default();
    texture_manager.retain(TextureId::default());
}
