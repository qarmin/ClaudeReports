# rav1e/src/util/kmeans.rs:35:10

- crate: https://github.com/xiph/rav1e @ 31435de9d76fddd38f6dcc31d4014574cebb2092
- kind: panic
- location: `rav1e/src/util/kmeans.rs:35:10`
- last API call: `rav1e::bench::util::kmeans[i64, 4]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 311817353808402421

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rav1e = { git = "https://github.com/xiph/rav1e", rev = "31435de9d76fddd38f6dcc31d4014574cebb2092", features = ["aom-sys", "asm", "av-metrics", "backtrace", "bench", "binaries", "byteorder", "capi", "cc", "channel-api", "check_asm", "clap", "clap_complete", "console", "crossbeam", "dav1d-sys", "decode_test", "decode_test_dav1d", "dump_ivf", "dump_lookahead_data", "fern", "git_version", "image", "ivf", "nasm-rs", "nom", "quick_test", "scan_fmt", "serde", "serde-big-array", "serialize", "signal-hook", "signal_support", "threading", "toml", "tracing", "tracing-chrome", "tracing-subscriber", "unstable", "wasm", "wasm-bindgen", "y4m"] }
```

`src/main.rs`:

```rust
fn main() {
    let _ = rav1e::bench::util::kmeans::<i64, 4>(&[1i64, 9223372036854775806i64]);
}
```

Run with `cargo run` in this directory (package `repro_rav1e_src_util_kmeans_rs_35_10_16fd5b`).

## Output

```
thread 'main' (565478) panicked at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/util/kmeans.rs:35:10:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: rav1e::util::kmeans::kmeans::<i64, 4>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/util/kmeans.rs:35:10
   4: <core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::skip::Skip<core::slice::iter::Iter<i64>>, core::slice::iter::Iter<i64>>, rav1e::util::kmeans::kmeans<i64, 4>::{closure#1}> as core::iter::traits::iterator::Iterator>::__iterator_get_unchecked
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:138:18
   5: <core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::skip::Skip<core::slice::iter::Iter<i64>>, core::slice::iter::Iter<i64>>, rav1e::util::kmeans::kmeans<i64, 4>::{closure#1}>, core::iter::adapters::zip::Zip<core::iter::adapters::s... (+470 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/zip.rs:307:30
   6: <core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::skip::Skip<core::slice::iter::Iter<i64>>, core::slice::iter::Iter<i64>>, rav1e::util::kmeans::kmeans<i64, 4>::{closure#1}>, core::iter::adapters::zip::Zip<core::iter::adapters::s... (+130 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/zip.rs:83:9
   7: <core::iter::adapters::enumerate::Enumerate<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::skip::Skip<core::slice::iter::Iter<i64>>, core::slice::iter::Iter<i64>>, rav1e::util::kmeans::kmeans<i64, 4>::{closure#1}>, core::iter:... (+174 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/enumerate.rs:80:27
   8: rav1e::util::kmeans::kmeans::<i64, 4>
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/util/kmeans.rs:33:42
   9: repro_rav1e_src_util_kmeans_rs_35_10_16fd5b::main
             at ./src/main.rs:2:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
