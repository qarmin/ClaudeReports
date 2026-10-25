# read-fonts/src/ps/cff/stack.rs:207:19

- crate: https://github.com/googlefonts/fontations @ 5c480698f43ab4ff5b304de5a9204c1b5e720aec
- kind: panic
- location: `read-fonts/src/ps/cff/stack.rs:207:19`
- last API call: `read_fonts::ps::cff::stack::Stack::fixed_array[4]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 14312769313960914331

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
read-fonts = { git = "https://github.com/googlefonts/fontations", rev = "5c480698f43ab4ff5b304de5a9204c1b5e720aec" }
```

`src/main.rs`:

```rust
use read_fonts::ps::cff::stack::Stack;

fn main() {
    let stack = Stack::default();
    let _ = stack.fixed_array::<4>(usize::MAX);
}
```

Run with `cargo run` in this directory (package `repro_read_fonts_src_ps_cff_stack_rs_207_19_b08bcf`).

## Output

```
thread 'main' (249631) panicked at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/read-fonts/src/ps/cff/stack.rs:207:19:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <read_fonts::ps::cff::stack::Stack>::fixed_array::<4>
             at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/read-fonts/src/ps/cff/stack.rs:207:19
   4: repro_read_fonts_src_ps_cff_stack_rs_207_19_b08bcf::main
             at ./src/main.rs:5:19
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
