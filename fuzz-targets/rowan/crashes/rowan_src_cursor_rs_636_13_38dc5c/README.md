# rowan/src/cursor.rs:636:13

- crate: https://github.com/rust-analyzer/rowan @ b6f1cb6494a99a2264186d110b1b170989774814
- kind: panic
- location: `rowan/src/cursor.rs:636:13`
- last API call: `rowan::cursor::SyntaxNode::covering_element`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Bad range: node range 0..0, range 0..1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rowan = { git = "https://github.com/rust-analyzer/rowan", rev = "b6f1cb6494a99a2264186d110b1b170989774814" }
```

`src/main.rs`:

```rust
use rowan::GreenNode;
use rowan::GreenToken;
use rowan::SyntaxKind;
use rowan::cursor::SyntaxNode;

fn main() {
    let green_token = GreenToken::new(SyntaxKind(0u16), "a");
    let syntax_kind = SyntaxKind(0u16);
    let syntax_node = SyntaxNode::new_root(GreenNode::new::<Vec<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken>>>(SyntaxKind(0u16), vec![<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken> as ::core::convert::From<rowan::GreenNode>>::from(GreenNode::new::<Vec<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken>>>(syntax_kind, vec![])), <rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken> as ::core::convert::From<rowan::GreenToken>>::from(green_token.clone())]));
    let syntax_node2 = syntax_node.last_child().unwrap();
    let _ = syntax_node2.covering_element(syntax_node.text_range());
}
```

Run with `cargo run` in this directory (package `repro_rowan_src_cursor_rs_636_13_38dc5c`).

## Output

```
thread 'main' (1321641) panicked at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/cursor.rs:636:13:
Bad range: node range 0..0, range 0..1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <rowan::cursor::SyntaxNode>::covering_element
             at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/cursor.rs:636:13
   3: repro_rowan_src_cursor_rs_636_13_38dc5c::main
             at ./src/main.rs:11:26
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
