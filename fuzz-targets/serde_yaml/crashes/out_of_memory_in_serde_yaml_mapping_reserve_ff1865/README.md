# out of memory in serde_yaml::Mapping::reserve

- crate: https://github.com/dtolnay/serde-yaml @ 2009506d33767dfc88e979d6bc0d53d09f941c94
- kind: out of memory
- last API call: `serde_yaml::Mapping::reserve`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling serde_yaml::Mapping::reserve
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
serde_yaml = { git = "https://github.com/dtolnay/serde-yaml", rev = "2009506d33767dfc88e979d6bc0d53d09f941c94" }
```

`src/main.rs`:

```rust
use serde_yaml::Mapping;

fn main() {
    let mut mapping = Mapping::from(Mapping::with_capacity(0usize));
    mapping.reserve(1000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_serde_yaml_mapping_reserve_ff1865`).

## Output

```
memory allocation of 19327352848 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: <hashbrown::raw::Fallibility>::alloc_err
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:45:40
   5: <hashbrown::raw::RawTableInner>::new_uninitialized::<hashbrown::alloc::inner::Global>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:1597:46
   6: <hashbrown::raw::RawTableInner>::fallible_with_capacity::<hashbrown::alloc::inner::Global>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:1634:21
   7: <hashbrown::raw::RawTableInner>::prepare_resize::<hashbrown::alloc::inner::Global>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:2696:13
   8: <hashbrown::raw::RawTableInner>::resize_inner::<hashbrown::alloc::inner::Global>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:2898:34
   9: <hashbrown::raw::RawTableInner>::reserve_rehash_inner::<hashbrown::alloc::inner::Global>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:2784:22
  10: <hashbrown::raw::RawTable<usize>>::reserve_rehash::<indexmap::inner::get_hash<serde_yaml::value::Value, serde_yaml::value::Value>::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:962:24
  11: <hashbrown::raw::RawTable<usize>>::reserve::<indexmap::inner::get_hash<serde_yaml::value::Value, serde_yaml::value::Value>::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:914:31
  12: <hashbrown::table::HashTable<usize>>::reserve::<indexmap::inner::get_hash<serde_yaml::value::Value, serde_yaml::value::Value>::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/table.rs:846:18
  13: <indexmap::inner::Core<serde_yaml::value::Value, serde_yaml::value::Value>>::reserve
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.14.2/src/inner.rs:233:22
  14: <indexmap::map::IndexMap<serde_yaml::value::Value, serde_yaml::value::Value>>::reserve
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.14.2/src/map.rs:383:19
  15: <serde_yaml::mapping::Mapping>::reserve
             at /home/rafal/.cargo/git/checkouts/serde-yaml-24d0e1a59cfe5308/2009506/src/mapping.rs:42:18
  16: repro_out_of_memory_in_serde_yaml_mapping_reserve_ff1865::main
             at ./src/main.rs:5:13
  17: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
