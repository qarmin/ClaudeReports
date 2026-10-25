use egui::load::SizedTexture;
use epaint::TextureHandle;
use epaint::TextureId;

fn main() {
    let _ = SizedTexture::from_handle(&TextureHandle::new(std::sync::Arc::new(<epaint::mutex::RwLock<epaint::TextureManager> as ::core::default::Default>::default()), TextureId::default()));
}
