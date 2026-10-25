use vello_common::image_cache::ImageCache;
use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::paint::NoOpImageResolver;

fn main() {
    let _ = <vello_common::paint::NoOpImageResolver as vello_common::paint::ImageResolver>::resolve(&NoOpImageResolver, ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 10000000000000000000usize, max_atlases: 17380519893440117345usize, atlas_size: (1u16, 256u16), auto_grow: true, allocation_strategy: AllocationStrategy::LeastUsed }).allocate(0u16, 65518u16, 26046u16).unwrap());
}
