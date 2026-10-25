use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::Atlas;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::multi_atlas::MultiAtlasManager;

fn main() {
    let _ = Atlas::new(MultiAtlasManager::new(AtlasConfig { initial_atlas_count: 1000000000usize, max_atlases: usize::MAX, atlas_size: (2049u16, 16384u16), auto_grow: false, allocation_strategy: AllocationStrategy::default() }).create_atlas().unwrap(), u16::MAX, 16384u16);
}
