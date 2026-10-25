use vello_common::image_cache::ImageCache;
use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;

fn main() {
    let _ = ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 1000000000usize, max_atlases: 10000000000000usize, atlas_size: (1u16, 1u16), auto_grow: false, allocation_strategy: AllocationStrategy::RoundRobin });
}
