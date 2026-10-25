# syn/src/expr.rs:1140:9

- crate: https://github.com/dtolnay/syn @ 9ff9287583767e53dcfb70e3b1b350996908bd8b
- kind: panic
- location: `syn/src/expr.rs:1140:9`
- last API call: `syn::Member::from<usize>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: index < u32::MAX as usize
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
syn = { git = "https://github.com/dtolnay/syn", rev = "9ff9287583767e53dcfb70e3b1b350996908bd8b" }
```

`src/main.rs`:

```rust
use syn::Member;

fn main() {
    let _ = Member::from(4294967296usize);
}
```

Run with `cargo run` in this directory (package `repro_syn_src_expr_rs_1140_9_58da46`).

## Output

```
thread 'main' (471168) panicked at /home/rafal/.cargo/git/checkouts/syn-bf1624cda1931fcf/9ff9287/src/expr.rs:1140:9:
assertion failed: index < u32::MAX as usize
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <syn::expr::Index as core::convert::From<usize>>::from
             at /home/rafal/.cargo/git/checkouts/syn-bf1624cda1931fcf/9ff9287/src/expr.rs:1140:9
   4: <syn::expr::Member as core::convert::From<usize>>::from
             at /home/rafal/.cargo/git/checkouts/syn-bf1624cda1931fcf/9ff9287/src/expr.rs:1077:25
   5: repro_syn_src_expr_rs_1140_9_58da46::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
