# rowan/src/cursor.rs:697:9

- crate: https://github.com/rust-analyzer/rowan @ b6f1cb6494a99a2264186d110b1b170989774814
- kind: panic
- location: `rowan/src/cursor.rs:697:9`
- last API call: `rowan::cursor::SyntaxToken::replace_with`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: SyntaxKind(1)
 right: SyntaxKind(0)
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
    let syntax_kind = SyntaxKind(0u16);
    let syntax_kind2 = SyntaxKind(1u16);
    let green_node = GreenNode::from(GreenNode::new::<Vec<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken>>>(syntax_kind, vec![<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken> as ::core::convert::From<rowan::GreenToken>>::from(GreenToken::with_trivia(syntax_kind2, "", vec![], vec![]))]));
    let syntax_node = SyntaxNode::new_root(green_node.clone());
    let syntax_token = syntax_node.first_token().unwrap();
    let _ = syntax_token.replace_with(GreenToken::new(syntax_kind, ""));
}
```

Run with `cargo run` in this directory (package `repro_rowan_src_cursor_rs_697_9_73d6bb`).

## Output

```
thread 'main' (1321579) panicked at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/cursor.rs:697:9:
assertion `left == right` failed
  left: SyntaxKind(1)
 right: SyntaxKind(0)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<rowan::green::SyntaxKind, rowan::green::SyntaxKind>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: <rowan::cursor::SyntaxToken>::replace_with
             at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/cursor.rs:697:9
   5: repro_rowan_src_cursor_rs_697_9_73d6bb::main
             at ./src/main.rs:12:26
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
