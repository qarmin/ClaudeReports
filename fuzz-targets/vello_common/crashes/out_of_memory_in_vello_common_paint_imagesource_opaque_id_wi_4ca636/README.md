# out of memory in vello_common::paint::ImageSource::opaque_id_with_transparency_hint

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: out of memory
- last API call: `vello_common::paint::ImageSource::opaque_id_with_transparency_hint`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 2225816699640448237

## Message

```
process ended abnormally (out of memory) while calling vello_common::paint::ImageSource::opaque_id_with_transparency_hint
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
use vello_common::paint::ImageSource;

fn main() {
    let _ = ImageSource::opaque_id_with_transparency_hint(ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 1000000000000000000usize, max_atlases: 10000000000000000000usize, atlas_size: (1u16, 8330u16), auto_grow: false, allocation_strategy: AllocationStrategy::BestFit }).allocate(u16::MAX, 60u16, 34647u16).unwrap(), true);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_vello_common_paint_imagesource_opaque_id_wi_4ca636`).

## Output

```
memory allocation of 32 bytes failed
memory allocation of 512 bytes failed
skipping backtrace printing to avoid potential recursion
```
