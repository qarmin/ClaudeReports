# out of memory in toml::map::Map<String, toml::Value>::keys

- crate: toml 1.1.8+spec-1.1.0
- kind: out of memory
- last API call: `toml::map::Map<String, toml::Value>::keys`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling toml::map::Map<String, toml::Value>::keys
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
toml = { version = "=1.1.8+spec-1.1.0", features = ["debug", "display", "fast_hash", "parse", "preserve_order", "serde", "std", "unbounded"] }
```

`src/main.rs`:

```rust
fn main() {
    let map = <toml::map::Map<String, toml::Value>>::with_capacity(1000000000usize);
    let _ = map.keys();
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_toml_map_map_string_toml_value_keys_aa96eb`).

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
   7: <hashbrown::raw::RawTableInner>::with_capacity::<hashbrown::alloc::inner::Global>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:1661:13
   8: <hashbrown::raw::RawTable<usize>>::with_capacity_in
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:648:20
   9: <hashbrown::raw::RawTable<usize>>::with_capacity
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:601:9
  10: <hashbrown::table::HashTable<usize>>::with_capacity
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/table.rs:92:18
  11: <indexmap::inner::Core<alloc::string::String, toml::value::Value>>::with_capacity
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.14.2/src/inner.rs:122:22
  12: <indexmap::map::IndexMap<alloc::string::String, toml::value::Value, foldhash::fast::RandomState>>::with_capacity_and_hasher
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.14.2/src/map.rs:171:23
  13: <toml::map::Map<alloc::string::String, toml::value::Value>>::with_capacity
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/toml-1.1.8+spec-1.1.0/src/map.rs:77:18
  14: repro_out_of_memory_in_toml_map_map_string_toml_value_keys_aa96eb::main
             at ./src/main.rs:2:15
  15: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
