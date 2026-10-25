# out of memory in vello_common::multi_atlas::AtlasAllocation {literal}

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: out of memory
- last API call: `vello_common::multi_atlas::AtlasAllocation {literal}`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 2223730525772288216

## Message

```
process ended abnormally (out of memory) while calling vello_common::multi_atlas::AtlasAllocation {literal}
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
use vello_common::multi_atlas::Atlas;
use vello_common::multi_atlas::AtlasAllocation;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::multi_atlas::AtlasId;
use vello_common::multi_atlas::MultiAtlasManager;

fn main() {
    let _ = AtlasAllocation { atlas_id: MultiAtlasManager::new(AtlasConfig { initial_atlas_count: usize::MAX, max_atlases: 10392699432343005273usize, atlas_size: (30005u16, 17u16), auto_grow: false, allocation_strategy: AllocationStrategy::default() }).create_atlas().unwrap(), allocation: Atlas::new(AtlasId(1073741824u32), 37347u16, 1u16).allocate(1u16, 1u16).unwrap() };
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_vello_common_multi_atlas_atlasallocation_li_c9ef3c`).

## Output

```
memory allocation of 32 bytes failed
memory allocation of 512 bytes failed
skipping backtrace printing to avoid potential recursion
```
