use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::multi_atlas::AtlasLayerDiagnostics;
use vello_common::multi_atlas::MultiAtlasManager;

fn main() {
    let _ = AtlasLayerDiagnostics { atlas_id: MultiAtlasManager::new(AtlasConfig { initial_atlas_count: 1000000000usize, max_atlases: 1000000000usize, atlas_size: (1u16, 1u16), auto_grow: false, allocation_strategy: AllocationStrategy::RoundRobin }).create_atlas().unwrap(), total_area: 0u64, free_area: 0u64, free_rectangle_count: 0usize, largest_free_width: 0u16, largest_free_height: 0u16 };
}
