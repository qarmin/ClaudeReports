# rowan/src/cursor.rs:601:9

- crate: https://github.com/rust-analyzer/rowan @ b6f1cb6494a99a2264186d110b1b170989774814
- kind: panic
- location: `rowan/src/cursor.rs:601:9`
- last API call: `rowan::cursor::SyntaxNode::token_at_offset`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Bad offset: range 0..0 offset 1
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
use rowan::SyntaxKind;
use rowan::cursor::SyntaxNode;

fn main() {
    let mut green_node_builder = GreenNodeBuilder::new();
    let checkpoint = green_node_builder.checkpoint();
    let syntax_kind = SyntaxKind(0u16);
    green_node_builder.start_node(syntax_kind);
    green_node_builder.start_node_at(checkpoint, SyntaxKind(0u16));
    green_node_builder.finish_node();
    green_node_builder.token(syntax_kind, "a");
    green_node_builder.finish_node();
    let green_node = green_node_builder.finish();
    let syntax_node = SyntaxNode::from(SyntaxNode::new_root(green_node.clone()));
    let syntax_node2 = syntax_node.tree_top();
    let syntax_node3 = syntax_node2.first_child().unwrap();
    let _ = syntax_node3.token_at_offset(syntax_node.text_without_outer_trivia().len());
}
```

Run with `cargo run` in this directory (package `repro_rowan_src_cursor_rs_601_9_cd648e`).

## Output

```
thread 'main' (1321521) panicked at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/cursor.rs:601:9:
Bad offset: range 0..0 offset 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <rowan::cursor::SyntaxNode>::token_at_offset
             at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/cursor.rs:601:9
   3: repro_rowan_src_cursor_rs_601_9_cd648e::main
             at ./src/main.rs:18:26
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
