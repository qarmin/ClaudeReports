# hashbrown/src/raw.rs:36:40

- crate: toml 1.1.8+spec-1.1.0
- kind: panic
- location: `hashbrown/src/raw.rs:36:40`
- source: https://docs.rs/crate/hashbrown/0.17.1/source/src/raw.rs#36
- last API call: `toml::map::Map<String, toml::Value>::with_capacity`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Hash table capacity overflow
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
    let _ = <toml::map::Map<String, toml::Value>>::with_capacity(10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_hashbrown_src_raw_rs_36_40_79d5a8`).

## Output

```
thread 'main' (375201) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:36:40:
Hash table capacity overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <hashbrown::raw::Fallibility>::capacity_overflow
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:36:40
   3: <hashbrown::raw::RawTableInner>::fallible_with_capacity::<hashbrown::alloc::inner::Global>::{closure#0}
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:1631:48
   4: <core::option::Option<usize>>::ok_or_else::<hashbrown::TryReserveError, <hashbrown::raw::RawTableInner>::fallible_with_capacity<hashbrown::alloc::inner::Global>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1365:25
   5: <hashbrown::raw::RawTableInner>::fallible_with_capacity::<hashbrown::alloc::inner::Global>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:1631:22
   6: <hashbrown::raw::RawTableInner>::with_capacity::<hashbrown::alloc::inner::Global>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:1661:13
   7: <hashbrown::raw::RawTable<usize>>::with_capacity_in
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:648:20
   8: <hashbrown::raw::RawTable<usize>>::with_capacity
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs:601:9
   9: <hashbrown::table::HashTable<usize>>::with_capacity
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/table.rs:92:18
  10: <indexmap::inner::Core<alloc::string::String, toml::value::Value>>::with_capacity
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.14.2/src/inner.rs:122:22
  11: <indexmap::map::IndexMap<alloc::string::String, toml::value::Value, foldhash::fast::RandomState>>::with_capacity_and_hasher
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.14.2/src/map.rs:171:23
  12: <toml::map::Map<alloc::string::String, toml::value::Value>>::with_capacity
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/toml-1.1.8+spec-1.1.0/src/map.rs:77:18
  13: repro_hashbrown_src_raw_rs_36_40_79d5a8::main
             at ./src/main.rs:2:13
  14: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
