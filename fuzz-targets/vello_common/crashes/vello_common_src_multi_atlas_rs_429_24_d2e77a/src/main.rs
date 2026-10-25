use vello_common::multi_atlas::AtlasError;
use vello_common::multi_atlas::AtlasId;
use vello_common::multi_atlas::AtlasLayerDiagnostics;
use vello_common::multi_atlas::AtlasSpaceDiagnostics;

fn main() {
    let atlas_error = AtlasError::AtlasLimitReached { max_atlases: 0usize, diagnostics: AtlasSpaceDiagnostics::Allocation { width: 0u16, height: 0u16, atlas_width: 0u16, atlas_height: 0u16, max_atlases: 0usize, atlases: vec![AtlasLayerDiagnostics { atlas_id: AtlasId::new(0u32), total_area: 0u64, free_area: u64::MAX, free_rectangle_count: 255usize, largest_free_width: 7u16, largest_free_height: 63490u16 }, AtlasLayerDiagnostics { atlas_id: AtlasId(1u32), total_area: 76u64, free_area: 18446744073709551614u64, free_rectangle_count: 11usize, largest_free_width: 32266u16, largest_free_height: 65534u16 }] } };
    let _ = atlas_error.to_string();
}
