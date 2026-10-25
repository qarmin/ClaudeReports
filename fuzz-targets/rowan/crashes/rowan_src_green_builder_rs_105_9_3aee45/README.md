# rowan/src/green/builder.rs:105:9

- crate: https://github.com/rust-analyzer/rowan @ b6f1cb6494a99a2264186d110b1b170989774814
- kind: panic
- location: `rowan/src/green/builder.rs:105:9`
- last API call: `rowan::GreenNodeBuilder::start_node_at`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
checkpoint no longer valid, was finish_node called early?
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rowan = { git = "https://github.com/rust-analyzer/rowan", rev = "b6f1cb6494a99a2264186d110b1b170989774814" }
```

`src/main.rs`:

```rust
use rowan::GreenNodeBuilder;
use rowan::GreenToken;
use rowan::SyntaxKind;

fn main() {
    let syntax_kind = SyntaxKind(0u16);
    let green_token = GreenToken::new(syntax_kind, "");
    let string = green_token.to_string();
    let mut green_node_builder = GreenNodeBuilder::new();
    green_node_builder.token(syntax_kind, &string);
    let checkpoint = green_node_builder.checkpoint();
    let mut green_node_builder2 = GreenNodeBuilder::new();
    green_node_builder2.start_node_at(checkpoint, syntax_kind);
}
```

Run with `cargo run` in this directory (package `repro_rowan_src_green_builder_rs_105_9_3aee45`).

## Output

```
thread 'main' (1321363) panicked at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/green/builder.rs:105:9:
checkpoint no longer valid, was finish_node called early?
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <rowan::green::builder::GreenNodeBuilder>::start_node_at
             at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/green/builder.rs:105:9
   3: repro_rowan_src_green_builder_rs_105_9_3aee45::main
             at ./src/main.rs:13:25
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
