use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::Atlas;
use vello_common::multi_atlas::AtlasAllocation;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::multi_atlas::AtlasId;
use vello_common::multi_atlas::MultiAtlasManager;

fn main() {
    let _ = AtlasAllocation { atlas_id: MultiAtlasManager::new(AtlasConfig { initial_atlas_count: usize::MAX, max_atlases: 10392699432343005273usize, atlas_size: (30005u16, 17u16), auto_grow: false, allocation_strategy: AllocationStrategy::default() }).create_atlas().unwrap(), allocation: Atlas::new(AtlasId(1073741824u32), 37347u16, 1u16).allocate(1u16, 1u16).unwrap() };
}
