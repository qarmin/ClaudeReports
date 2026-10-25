use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::multi_atlas::MultiAtlasManager;

fn main() {
    let _ = MultiAtlasManager::new(AtlasConfig { initial_atlas_count: 1000000000usize, max_atlases: 1000000000usize, atlas_size: (1u16, 1u16), auto_grow: false, allocation_strategy: AllocationStrategy::LeastUsed });
}
