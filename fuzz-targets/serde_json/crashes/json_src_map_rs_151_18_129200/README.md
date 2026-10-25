# json/src/map.rs:151:18

- crate: https://github.com/serde-rs/json @ 8c535a260ce561eaab4111aafc29038eb9f77ff3
- kind: panic
- location: `json/src/map.rs:151:18`
- last API call: `serde_json::Map<String, serde_json::Value>::shift_insert`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17330015691815902329

## Message

```
index out of bounds: the len is 0 but the index is 1. Expected index <= len
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
    let mut map = <serde_json::Map<String, serde_json::Value> as ::core::default::Default>::default();
    let _ = map.shift_insert(1usize, "".to_string(), Value::Object(<serde_json::Map<String, serde_json::Value>>::with_capacity(0usize)));
}
```

Run with `cargo run` in this directory (package `repro_json_src_map_rs_151_18_129200`).

## Output

```
thread 'main' (158504) panicked at /home/rafal/.cargo/git/checkouts/json-b4d7793b2759afed/8c535a2/src/map.rs:151:18:
index out of bounds: the len is 0 but the index is 1. Expected index <= len
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: indexmap::util::assert_index_le
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.14.2/src/util.rs:19:5
   3: <indexmap::map::IndexMap<alloc::string::String, serde_json::value::Value, std::hash::random::RandomState>>::shift_insert
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.14.2/src/map.rs:687:17
   4: <serde_json::map::Map<alloc::string::String, serde_json::value::Value>>::shift_insert
             at /home/rafal/.cargo/git/checkouts/json-b4d7793b2759afed/8c535a2/src/map.rs:151:18
   5: repro_json_src_map_rs_151_18_129200::main
             at ./src/main.rs:5:17
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
