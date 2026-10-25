# czkawka_core/src/common/config_cache_path.rs:20:29

- crate: https://github.com/qarmin/czkawka @ eb8b91dbb4d25dff416202674ee0de84ee4b5e5c
- kind: panic
- location: `czkawka_core/src/common/config_cache_path.rs:20:29`
- last API call: `czkawka_core::common::config_cache_path::get_config_cache_path`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Cannot fail if set_config_cache_path was called before
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
czkawka_core = { git = "https://github.com/qarmin/czkawka", rev = "eb8b91dbb4d25dff416202674ee0de84ee4b5e5c" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = czkawka_core::common::config_cache_path::get_config_cache_path();
}
```

Run with `cargo run` in this directory (package `repro_czkawka_core_src_common_config_cache_path_rs_20_29_8cf25d`).

## Output

```
thread 'main' (517044) panicked at /home/rafal/.cargo/git/checkouts/czkawka-c1570a3d7ff29c8c/eb8b91d/czkawka_core/src/common/config_cache_path.rs:20:29:
Cannot fail if set_config_cache_path was called before
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<&core::option::Option<czkawka_core::common::config_cache_path::ConfigCachePath>>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: czkawka_core::common::config_cache_path::get_config_cache_path
             at /home/rafal/.cargo/git/checkouts/czkawka-c1570a3d7ff29c8c/eb8b91d/czkawka_core/src/common/config_cache_path.rs:20:29
   6: repro_czkawka_core_src_common_config_cache_path_rs_20_29_8cf25d::main
             at ./src/main.rs:2:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
