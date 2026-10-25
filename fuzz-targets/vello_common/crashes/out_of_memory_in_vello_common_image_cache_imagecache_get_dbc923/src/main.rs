use vello_common::image_cache::ImageCache;
use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;

fn main() {
    let image_cache = ImageCache::new_dummy();
    let _ = image_cache.get(ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 10000000000000000000usize, max_atlases: 11628016252581910041usize, atlas_size: (u16::MAX, 1u16), auto_grow: false, allocation_strategy: AllocationStrategy::RoundRobin }).allocate(u16::MAX, 127u16, u16::MAX).unwrap());
}
