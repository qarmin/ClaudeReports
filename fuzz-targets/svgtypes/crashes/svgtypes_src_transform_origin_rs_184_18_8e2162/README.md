# svgtypes/src/transform_origin.rs:184:18

- crate: https://github.com/linebender/svgtypes @ 3ee4db5bcbfe8db817d87b55a3ae4f889bf2d3fa
- kind: panic
- location: `svgtypes/src/transform_origin.rs:184:18`
- last API call: `svgtypes::TransformOrigin::from_str`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
internal error: entered unreachable code
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
svgtypes = { git = "https://github.com/linebender/svgtypes", rev = "3ee4db5bcbfe8db817d87b55a3ae4f889bf2d3fa" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <svgtypes::TransformOrigin as ::core::str::FromStr>::from_str(" ");
}
```

Run with `cargo run` in this directory (package `repro_svgtypes_src_transform_origin_rs_184_18_8e2162`).

## Output

```
thread 'main' (489027) panicked at /home/rafal/.cargo/git/checkouts/svgtypes-da914f7dbc9e8d80/3ee4db5/src/transform_origin.rs:184:18:
internal error: entered unreachable code
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <svgtypes::transform_origin::TransformOrigin as core::str::traits::FromStr>::from_str
             at /home/rafal/.cargo/git/checkouts/svgtypes-da914f7dbc9e8d80/3ee4db5/src/transform_origin.rs:184:18
   4: repro_svgtypes_src_transform_origin_rs_184_18_8e2162::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
