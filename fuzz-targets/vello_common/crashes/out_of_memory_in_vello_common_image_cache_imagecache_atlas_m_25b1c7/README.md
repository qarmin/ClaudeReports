# out of memory in vello_common::image_cache::ImageCache::atlas_manager

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: out of memory
- last API call: `vello_common::image_cache::ImageCache::atlas_manager`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 5188780382741660453

## Message

```
process ended abnormally (out of memory) while calling vello_common::image_cache::ImageCache::atlas_manager
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use vello_common::image_cache::ImageCache;
use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;

fn main() {
    let image_cache = ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 1000000000usize, max_atlases: 1000000000usize, atlas_size: (1u16, 1u16), auto_grow: false, allocation_strategy: AllocationStrategy::LeastUsed });
    let _ = image_cache.atlas_manager();
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_vello_common_image_cache_imagecache_atlas_m_25b1c7`).

## Output

```
memory allocation of 32 bytes failed
memory allocation of 512 bytes failed
skipping backtrace printing to avoid potential recursion
```
