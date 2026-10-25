# glutin/src/config.rs:144:9

- crate: https://github.com/rust-windowing/glutin @ 940a394cc205f152eb12076feff59f1eaaff4f7a
- kind: panic
- location: `glutin/src/config.rs:144:9`
- last API call: `glutin::config::ConfigTemplateBuilder::with_multisampling`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: num_samples.is_power_of_two()
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
glutin = { git = "https://github.com/rust-windowing/glutin", rev = "940a394cc205f152eb12076feff59f1eaaff4f7a" }
```

`src/main.rs`:

```rust
use glutin::config::ConfigTemplateBuilder;

fn main() {
    let config_template_builder = ConfigTemplateBuilder::default();
    let config_template_builder2 = config_template_builder.clone().with_transparency(false);
    let _ = config_template_builder2.clone().with_multisampling(0u8);
}
```

Run with `cargo run` in this directory (package `repro_glutin_src_config_rs_144_9_053d61`).

## Output

```
thread 'main' (595423) panicked at /home/rafal/.cargo/git/checkouts/glutin-174bfcce3014aff8/940a394/glutin/src/config.rs:144:9:
assertion failed: num_samples.is_power_of_two()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <glutin::config::ConfigTemplateBuilder>::with_multisampling
             at /home/rafal/.cargo/git/checkouts/glutin-174bfcce3014aff8/940a394/glutin/src/config.rs:144:9
   4: repro_glutin_src_config_rs_144_9_053d61::main
             at ./src/main.rs:6:46
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
