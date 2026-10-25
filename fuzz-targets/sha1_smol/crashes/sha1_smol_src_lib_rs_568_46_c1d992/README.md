# sha1-smol/src/lib.rs:568:46

- crate: https://github.com/mitsuhiko/sha1-smol @ f9d88bd70e9bd577c7c3afdc6acd0313af605324
- kind: panic
- location: `sha1-smol/src/lib.rs:568:46`
- last API call: `sha1_smol::Digest::from_str`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
end byte index 8 is not a char boundary; it is inside '\u{d4f98}' (bytes 5..9 of string)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
sha1_smol = { git = "https://github.com/mitsuhiko/sha1-smol", rev = "f9d88bd70e9bd577c7c3afdc6acd0313af605324" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <sha1_smol::Digest as ::core::str::FromStr>::from_str("\u{97a59}x\u{d4f98} 㚺\u{102a1e}D/FkC*A\u{9ccaa}ÿ\u{52441}cN;[#S");
}
```

Run with `cargo run` in this directory (package `repro_sha1_smol_src_lib_rs_568_46_c1d992`).

## Output

```
thread 'main' (1424528) panicked at /home/rafal/.cargo/git/checkouts/sha1-smol-8f26a8ce69bb70a7/f9d88bd/src/lib.rs:568:46:
end byte index 8 is not a char boundary; it is inside '\u{d4f98}' (bytes 5..9 of string)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::str::slice_error_fail_rt
   3: core::str::slice_error_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/str/mod.rs:70:5
   4: <core::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:335:21
   5: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:248:34
   6: <str as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:15
   7: <sha1_smol::Digest as core::str::traits::FromStr>::from_str
             at /home/rafal/.cargo/git/checkouts/sha1-smol-8f26a8ce69bb70a7/f9d88bd/src/lib.rs:568:46
   8: repro_sha1_smol_src_lib_rs_568_46_c1d992::main
             at ./src/main.rs:2:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
