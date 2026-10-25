# rowan/src/green/token.rs:170:9

- crate: https://github.com/rust-analyzer/rowan @ b6f1cb6494a99a2264186d110b1b170989774814
- kind: panic
- location: `rowan/src/green/token.rs:170:9`
- last API call: `rowan::GreenToken::with_trivia`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
trivia tokens cannot themselves carry trivia
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rowan = { git = "https://github.com/rust-analyzer/rowan", rev = "b6f1cb6494a99a2264186d110b1b170989774814" }
```

`src/main.rs`:

```rust
use rowan::GreenToken;
use rowan::SyntaxKind;

fn main() {
    let green_token = GreenToken::new(SyntaxKind(0u16), "");
    let syntax_kind = SyntaxKind(0u16);
    let syntax_kind2 = SyntaxKind(0u16);
    let _ = GreenToken::with_trivia(syntax_kind2, "", vec![GreenToken::with_trivia(syntax_kind, "", vec![], vec![green_token.clone()])], vec![]);
}
```

Run with `cargo run` in this directory (package `repro_rowan_src_green_token_rs_170_9_a2e72c`).

## Output

```
thread 'main' (1321302) panicked at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/green/token.rs:170:9:
trivia tokens cannot themselves carry trivia
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <rowan::green::token::GreenToken>::with_trivia
             at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/green/token.rs:170:9
   3: repro_rowan_src_green_token_rs_170_9_a2e72c::main
             at ./src/main.rs:8:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
