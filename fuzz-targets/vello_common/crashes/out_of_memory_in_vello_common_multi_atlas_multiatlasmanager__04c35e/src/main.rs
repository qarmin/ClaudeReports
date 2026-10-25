use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::multi_atlas::MultiAtlasManager;

fn main() {
    let multi_atlas_manager = MultiAtlasManager::new(AtlasConfig { initial_atlas_count: 1000000000usize, max_atlases: 10000000000000000000usize, atlas_size: (56132u16, 80u16), auto_grow: false, allocation_strategy: AllocationStrategy::RoundRobin });
    let _ = multi_atlas_manager.config();
}
