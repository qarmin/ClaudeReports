# out of memory in vello_common::image_cache::ImageCache::get

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: out of memory
- last API call: `vello_common::image_cache::ImageCache::get`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 11402582335931325982

## Message

```
process ended abnormally (out of memory) while calling vello_common::image_cache::ImageCache::get
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
    let image_cache = ImageCache::new_dummy();
    let _ = image_cache.get(ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 10000000000000000000usize, max_atlases: 11628016252581910041usize, atlas_size: (u16::MAX, 1u16), auto_grow: false, allocation_strategy: AllocationStrategy::RoundRobin }).allocate(u16::MAX, 127u16, u16::MAX).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_vello_common_image_cache_imagecache_get_dbc923`).

## Output

```
memory allocation of 1 bytes failed
... (previous line repeated 1 more times)
skipping backtrace printing to avoid potential recursion
```
