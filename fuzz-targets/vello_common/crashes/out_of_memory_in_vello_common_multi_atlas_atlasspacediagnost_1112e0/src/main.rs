use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::multi_atlas::AtlasId;
use vello_common::multi_atlas::AtlasLayerDiagnostics;
use vello_common::multi_atlas::AtlasSpaceDiagnostics;
use vello_common::multi_atlas::MultiAtlasManager;

fn main() {
    let atlas_config = AtlasConfig { initial_atlas_count: usize::MAX, max_atlases: 1125007570538422820usize, atlas_size: (1u16, 1u16), auto_grow: false, allocation_strategy: AllocationStrategy::FirstFit };
    let _ = AtlasSpaceDiagnostics::Allocation { width: 1000u16, height: 256u16, atlas_width: 96u16, atlas_height: 16106u16, max_atlases: 256usize, atlases: vec![AtlasLayerDiagnostics { atlas_id: AtlasId(u32::MAX), total_area: 7726114339975327424u64, free_area: 5385087516234100114u64, free_rectangle_count: 36028797018963966usize, largest_free_width: 255u16, largest_free_height: 39821u16 }, AtlasLayerDiagnostics { atlas_id: AtlasId(65536u32), total_area: u64::MAX, free_area: 18446744073709551614u64, free_rectangle_count: 65535usize, largest_free_width: 11278u16, largest_free_height: 68u16 }, AtlasLayerDiagnostics { atlas_id: AtlasId::new(256u32), total_area: 1u64, free_area: 8u64, free_rectangle_count: 256usize, largest_free_width: 16385u16, largest_free_height: 9u16 }, AtlasLayerDiagnostics { atlas_id: AtlasId::new(3491957232u32), total_area: 6921050401554868828u64, free_area: 0u64, free_rectangle_count: 4usize, largest_free_width: 0u16, largest_free_height: 65529u16 }, AtlasLayerDiagnostics { atlas_id: MultiAtlasManager::new(atlas_config).create_atlas().unwrap(), total_area: 6u64, free_area: 13u64, free_rectangle_count: 0usize, largest_free_width: 256u16, largest_free_height: 31214u16 }] };
}
