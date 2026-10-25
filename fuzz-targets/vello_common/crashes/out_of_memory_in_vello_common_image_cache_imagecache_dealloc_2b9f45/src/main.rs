use vello_common::image_cache::ImageCache;
use vello_common::pixmap::Pixmap;
use vello_cpu::Resources;

fn main() {
    let mut image_cache = ImageCache::new_dummy();
    let _ = image_cache.deallocate(Resources::default().register_image(std::sync::Arc::new(Pixmap::new(10000u16, u16::MAX))));
}
