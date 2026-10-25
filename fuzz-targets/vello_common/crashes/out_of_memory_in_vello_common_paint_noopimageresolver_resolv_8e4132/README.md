# out of memory in vello_common::paint::NoOpImageResolver::resolve

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: out of memory
- last API call: `vello_common::paint::NoOpImageResolver::resolve`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 8687117081231645880

## Message

```
process ended abnormally (out of memory) while calling vello_common::paint::NoOpImageResolver::resolve
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
use vello_common::paint::NoOpImageResolver;

fn main() {
    let _ = <vello_common::paint::NoOpImageResolver as vello_common::paint::ImageResolver>::resolve(&NoOpImageResolver, ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 10000000000000000000usize, max_atlases: 17380519893440117345usize, atlas_size: (1u16, 256u16), auto_grow: true, allocation_strategy: AllocationStrategy::LeastUsed }).allocate(0u16, 65518u16, 26046u16).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_vello_common_paint_noopimageresolver_resolv_8e4132`).

## Output

```
memory allocation of 32 bytes failed
memory allocation of 512 bytes failed
skipping backtrace printing to avoid potential recursion
```
