# idna/src/deprecated.rs:212:9

- crate: https://github.com/servo/rust-url @ 00a6ce58d02f4e0d43c5ca0702c0bedb8b1ebf3a
- kind: panic
- location: `idna/src/deprecated.rs:212:9`
- last API call: `idna::Config::use_idna_2008_rules`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
IDNA 2008 rules are no longer supported
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
url = { git = "https://github.com/servo/rust-url", rev = "00a6ce58d02f4e0d43c5ca0702c0bedb8b1ebf3a" }
idna = { git = "https://github.com/servo/rust-url", rev = "00a6ce58d02f4e0d43c5ca0702c0bedb8b1ebf3a" }
```

`src/main.rs`:

```rust
use idna::Config;

fn main() {
    let _ = Config::default().use_idna_2008_rules(true);
}
```

Run with `cargo run` in this directory (package `repro_idna_src_deprecated_rs_212_9_993350`).

## Output

```
thread 'main' (524974) panicked at /home/rafal/.cargo/git/checkouts/rust-url-f0546b39189470a0/00a6ce5/idna/src/deprecated.rs:212:9:
IDNA 2008 rules are no longer supported
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <idna::deprecated::Config>::use_idna_2008_rules
             at /home/rafal/.cargo/git/checkouts/rust-url-f0546b39189470a0/00a6ce5/idna/src/deprecated.rs:212:9
   4: repro_idna_src_deprecated_rs_212_9_993350::main
             at ./src/main.rs:4:31
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
