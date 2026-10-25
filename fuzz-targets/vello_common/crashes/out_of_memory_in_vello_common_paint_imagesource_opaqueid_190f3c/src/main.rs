use vello_common::image_cache::ImageCache;
use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::paint::ImageSource;

fn main() {
    let _ = ImageSource::OpaqueId { id: ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 1000000000usize, max_atlases: 1000000000usize, atlas_size: (1u16, 1u16), auto_grow: false, allocation_strategy: AllocationStrategy::BestFit }).allocate(0u16, 0u16, 0u16).unwrap(), may_have_transparency: false };
}
