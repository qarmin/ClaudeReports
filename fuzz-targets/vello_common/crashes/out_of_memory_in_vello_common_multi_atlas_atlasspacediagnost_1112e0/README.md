# out of memory in vello_common::multi_atlas::AtlasSpaceDiagnostics::Allocation

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: out of memory
- last API call: `vello_common::multi_atlas::AtlasSpaceDiagnostics::Allocation`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 5342490455167277405

## Message

```
process ended abnormally (out of memory) while calling vello_common::multi_atlas::AtlasSpaceDiagnostics::Allocation
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
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
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_vello_common_multi_atlas_atlasspacediagnost_1112e0`).

## Output

```
memory allocation of 16 bytes failed
memory allocation of 1 bytes failed
skipping backtrace printing to avoid potential recursion
```
