# term/src/terminfo/parm.rs:295:36

- crate: https://github.com/Stebalien/term @ 5325c789f24aa4e9378733b8432befed964462d0
- kind: panic
- location: `term/src/terminfo/parm.rs:295:36`
- last API call: `term::terminfo::parm::expand`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
term = { git = "https://github.com/Stebalien/term", rev = "5325c789f24aa4e9378733b8432befed964462d0" }
```

`src/main.rs`:

```rust
use term::terminfo::parm::Variables;

fn main() {
    let mut variables = Variables::new();
    let _ = term::terminfo::parm::expand(&vec![37u8, 112u8, 48u8], &[], &mut variables);
}
```

Run with `cargo run` in this directory (package `repro_term_src_terminfo_parm_rs_295_36_ddd080`).

## Output

```
thread 'main' (529305) panicked at /home/rafal/.cargo/git/checkouts/term-87b7c925214b2495/5325c78/src/terminfo/parm.rs:295:36:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: term::terminfo::parm::expand
             at /home/rafal/.cargo/git/checkouts/term-87b7c925214b2495/5325c78/src/terminfo/parm.rs:295:36
   4: repro_term_src_terminfo_parm_rs_295_36_ddd080::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
