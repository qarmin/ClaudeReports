# out of memory in serde_json::Value::get[String]

- crate: https://github.com/serde-rs/json @ 8c535a260ce561eaab4111aafc29038eb9f77ff3
- kind: out of memory
- last API call: `serde_json::Value::get[String]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 2929847218188459181

## Message

```
process ended abnormally (out of memory) while calling serde_json::Value::get[String]
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
serde_json = { git = "https://github.com/serde-rs/json", rev = "8c535a260ce561eaab4111aafc29038eb9f77ff3", features = ["alloc", "arbitrary_precision", "float_roundtrip", "indexmap", "preserve_order", "raw_value", "std", "unbounded_depth"] }
```

`src/main.rs`:

```rust
use serde_json::Value;

fn main() {
    let value = Value::from(<serde_json::Map<String, serde_json::Value>>::with_capacity(1000000000usize));
    let _ = value.get::<String>("".to_string());
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_serde_json_value_get_string_b65a76`).

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
  11: <indexmap::inner::Core<alloc::string::String, serde_json::value::Value>>::with_capacity
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.14.2/src/inner.rs:122:22
  12: <indexmap::map::IndexMap<alloc::string::String, serde_json::value::Value, std::hash::random::RandomState>>::with_capacity_and_hasher
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.14.2/src/map.rs:171:23
  13: <serde_json::map::Map<alloc::string::String, serde_json::value::Value>>::with_capacity
             at /home/rafal/.cargo/git/checkouts/json-b4d7793b2759afed/8c535a2/src/map.rs:68:18
  14: repro_out_of_memory_in_serde_json_value_get_string_b65a76::main
             at ./src/main.rs:4:29
  15: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
