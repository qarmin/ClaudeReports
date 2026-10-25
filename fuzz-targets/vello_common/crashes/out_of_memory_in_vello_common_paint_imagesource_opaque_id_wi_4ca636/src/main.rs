use vello_common::image_cache::ImageCache;
use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::paint::ImageSource;

fn main() {
    let _ = ImageSource::opaque_id_with_transparency_hint(ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 1000000000000000000usize, max_atlases: 10000000000000000000usize, atlas_size: (1u16, 8330u16), auto_grow: false, allocation_strategy: AllocationStrategy::BestFit }).allocate(u16::MAX, 60u16, 34647u16).unwrap(), true);
}
