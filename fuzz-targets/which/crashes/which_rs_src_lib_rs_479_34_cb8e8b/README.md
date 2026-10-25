# which-rs/src/lib.rs:479:34

- crate: https://github.com/harryfei/which-rs @ 8e84cb67761b6e19d2cfa512c5d55cbe5fd9e8bc
- kind: panic
- location: `which-rs/src/lib.rs:479:34`
- last API call: `which::WhichConfig<which::sys::RealSys, which::Noop>::all_results`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
binary_name not set! You must set binary_name or regex before searching!
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
which = { git = "https://github.com/harryfei/which-rs", rev = "8e84cb67761b6e19d2cfa512c5d55cbe5fd9e8bc" }
```

`src/main.rs`:

```rust
use which::sys::RealSys;

fn main() {
    let which_config = <which::WhichConfig<which::sys::RealSys, which::Noop>>::new_with_sys(RealSys);
    let which_config2 = which_config.system_cwd(false);
    let which_config3 = which_config2.system_path_list();
    let _ = which_config3.all_results();
}
```

Run with `cargo run` in this directory (package `repro_which_rs_src_lib_rs_479_34_cb8e8b`).

## Output

```
thread 'main' (1418972) panicked at /home/rafal/.cargo/git/checkouts/which-rs-e5d6a995f4251a7b/8e84cb6/src/lib.rs:479:34:
binary_name not set! You must set binary_name or regex before searching!
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<std::ffi::os_str::OsString>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <which::WhichConfig<which::sys::RealSys>>::all_results
             at /home/rafal/.cargo/git/checkouts/which-rs-e5d6a995f4251a7b/8e84cb6/src/lib.rs:479:34
   6: repro_which_rs_src_lib_rs_479_34_cb8e8b::main
             at ./src/main.rs:7:27
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
