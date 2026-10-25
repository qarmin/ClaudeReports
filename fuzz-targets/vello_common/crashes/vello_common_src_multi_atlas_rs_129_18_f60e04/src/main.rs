use vello_common::image_cache::ImageCache;
use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;

fn main() {
    let _ = ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 1usize, max_atlases: 0usize, atlas_size: (0u16, 0u16), auto_grow: false, allocation_strategy: AllocationStrategy::LeastUsed });
}
