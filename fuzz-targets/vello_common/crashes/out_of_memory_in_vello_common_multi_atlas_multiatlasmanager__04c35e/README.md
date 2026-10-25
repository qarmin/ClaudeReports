# out of memory in vello_common::multi_atlas::MultiAtlasManager::config

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: out of memory
- last API call: `vello_common::multi_atlas::MultiAtlasManager::config`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 10752973071166565501

## Message

```
process ended abnormally (out of memory) while calling vello_common::multi_atlas::MultiAtlasManager::config
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
use vello_common::multi_atlas::MultiAtlasManager;

fn main() {
    let multi_atlas_manager = MultiAtlasManager::new(AtlasConfig { initial_atlas_count: 1000000000usize, max_atlases: 10000000000000000000usize, atlas_size: (56132u16, 80u16), auto_grow: false, allocation_strategy: AllocationStrategy::RoundRobin });
    let _ = multi_atlas_manager.config();
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_vello_common_multi_atlas_multiatlasmanager__04c35e`).

## Output

```
memory allocation of 32 bytes failed
memory allocation of 512 bytes failed
skipping backtrace printing to avoid potential recursion
```
