# rowan/src/syntax_text.rs:52:33

- crate: https://github.com/rust-analyzer/rowan @ b6f1cb6494a99a2264186d110b1b170989774814
- kind: panic
- location: `rowan/src/syntax_text.rs:52:33`
- last API call: `rowan::SyntaxText::char_at`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
start byte index 1 is not a char boundary; it is inside '\u{feff}' (bytes 0..3 of string)
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
    let syntax_kind2 = SyntaxKind(0u16);
    let green_node = GreenNode::new::<Vec<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken>>>(syntax_kind, vec![<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken> as ::core::convert::From<rowan::GreenToken>>::from(GreenToken::with_trivia(syntax_kind2, "\u{feff}", vec![], vec![]))]);
    let green_token = GreenToken::from(GreenToken::with_trivia(syntax_kind2, "1", vec![], vec![]));
    let syntax_node = SyntaxNode::new_root(green_node.clone());
    let syntax_text = syntax_node.text_without_outer_trivia();
    let _ = syntax_text.char_at(<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken> as ::core::convert::From<rowan::GreenToken>>::from(green_token.clone()).text_len());
}
```

Run with `cargo run` in this directory (package `repro_rowan_src_syntax_text_rs_52_33_0c2633`).

## Output

```
thread 'main' (1321700) panicked at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/syntax_text.rs:52:33:
start byte index 1 is not a char boundary; it is inside '\u{feff}' (bytes 0..3 of string)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::str::slice_error_fail_rt
   3: core::str::slice_error_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/str/mod.rs:70:5
   4: <core::ops::range::RangeFrom<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:528:21
   5: <str as core::ops::index::Index<core::ops::range::RangeFrom<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:15
   6: <rowan::syntax_text::SyntaxText>::char_at::{closure#0}
             at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/syntax_text.rs:52:33
   7: <rowan::syntax_text::SyntaxText>::try_for_each_chunk::<<rowan::syntax_text::SyntaxText>::char_at::{closure#0}, char>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/syntax_text.rs:95:51
   8: <rowan::syntax_text::SyntaxText>::try_fold_chunks::<(), <rowan::syntax_text::SyntaxText>::try_for_each_chunk<<rowan::syntax_text::SyntaxText>::char_at::{closure#0}, char>::{closure#0}, char>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/syntax_text.rs:88:56
   9: core::iter::adapters::filter_map::filter_map_try_fold::<rowan::cursor::SyntaxToken, (rowan::cursor::SyntaxToken, text_size::range::TextRange), (), core::result::Result<(), char>, <rowan::syntax_text::SyntaxText>::tokens_with_ranges::{closure#2}, <rowan::syntax_text::SyntaxText>::try_fold_chunk... (+167 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/filter_map.rs:50:20
  10: <&mut core::iter::adapters::filter_map::filter_map_try_fold<rowan::cursor::SyntaxToken, (rowan::cursor::SyntaxToken, text_size::range::TextRange), (), core::result::Result<(), char>, <rowan::syntax_text::SyntaxText>::tokens_with_ranges::{closure#2}, <rowan::syntax_text::SyntaxText>::try_fold_c... (+246 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:298:21
  11: <&mut &mut core::iter::adapters::filter_map::filter_map_try_fold<rowan::cursor::SyntaxToken, (rowan::cursor::SyntaxToken, text_size::range::TextRange), (), core::result::Result<(), char>, <rowan::syntax_text::SyntaxText>::tokens_with_ranges::{closure#2}, <rowan::syntax_text::SyntaxText>::try_f... (+251 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:298:21
  12: <core::iter::sources::once::Once<rowan::cursor::SyntaxToken> as core::iter::traits::iterator::Iterator>::try_fold::<(), &mut &mut core::iter::adapters::filter_map::filter_map_try_fold<rowan::cursor::SyntaxToken, (rowan::cursor::SyntaxToken, text_size::range::TextRange), (), core::result::Resul... (+328 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2490:21
  13: <core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rowan::cursor::SyntaxToken>::leading_trivia::{closure#0}>, core::iter::sources::once::Once<rowan::cursor::SyntaxToken>> as core::iter::traits::iterator::Iterator>::try_fold::<(), &mut &mut core:... (+487 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:110:21
  14: <core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rowan::cursor::SyntaxToken>::leading_trivia::{closure#0}>, core::iter::sources::once::Once<rowan::cursor::SyntaxToken>>, core::iter::adapters::map::Map<core::o... (+642 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:106:21
  15: <core::iter::adapters::flatten::FlattenCompat<_, _> as core::iter::traits::iterator::Iterator>::try_fold::flatten::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rowan::cursor::SyntaxToken>::leading_trivia:... (+711 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/flatten.rs:566:35
  16: <core::iter::adapters::flatten::FlattenCompat<_, _>>::iter_try_fold::flatten::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rowan::cursor::SyntaxToken>::leading_trivia::{closure#0}>, core::iter::sources::o... (+1218 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/flatten.rs:423:30
  17: core::iter::adapters::map::map_try_fold::<rowan::cursor::SyntaxToken, core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rowan::cursor::SyntaxToken>::leading_trivia::{closure#0}>, core::iter::sources::once::Once... (+1783 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:95:21
  18: core::iter::adapters::filter_map::filter_map_try_fold::<rowan::utility_types::NodeOrToken<rowan::cursor::SyntaxNode, rowan::cursor::SyntaxToken>, rowan::cursor::SyntaxToken, (), core::result::Result<(), char>, <rowan::syntax_text::SyntaxText>::tokens_with_ranges::{closure#0}, core::iter::adapt... (+2072 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/filter_map.rs:50:20
  19: core::iter::adapters::filter_map::filter_map_try_fold::<rowan::utility_types::WalkEvent<rowan::utility_types::NodeOrToken<rowan::cursor::SyntaxNode, rowan::cursor::SyntaxToken>>, rowan::utility_types::NodeOrToken<rowan::cursor::SyntaxNode, rowan::cursor::SyntaxToken>, (), core::result::Result<... (+2456 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/filter_map.rs:50:20
  20: <rowan::cursor::PreorderWithTokens as core::iter::traits::iterator::Iterator>::try_fold::<(), core::iter::adapters::filter_map::filter_map_try_fold<rowan::utility_types::WalkEvent<rowan::utility_types::NodeOrToken<rowan::cursor::SyntaxNode, rowan::cursor::SyntaxToken>>, rowan::utility_types::N... (+2581 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2490:21
  21: <core::iter::adapters::filter_map::FilterMap<rowan::cursor::PreorderWithTokens, <rowan::cursor::SyntaxNode>::descendants_with_tokens::{closure#0}> as core::iter::traits::iterator::Iterator>::try_fold::<(), core::iter::adapters::filter_map::filter_map_try_fold<rowan::utility_types::NodeOrToken<... (+2309 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/filter_map.rs:138:19
  22: <core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter_map::FilterMap<rowan::cursor::PreorderWithTokens, <rowan::cursor::SyntaxNode>::descendants_with_tokens::{closure#0}>, <rowan::syntax_text::SyntaxText>::tokens_with_ranges::{closure#0}> as core::iter::traits::iterator::It... (+2132 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/filter_map.rs:138:19
  23: <core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter_map::FilterMap<rowan::cursor::PreorderWithTokens, <rowan::cursor::SyntaxNode>::descendants_with_tokens::{closure#0}>, <rowan::syntax_text::SyntaxText>::tokens_with_ranges::{closure#0}>, <ro... (+1666 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:121:19
  24: <core::iter::adapters::fuse::Fuse<core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter_map::FilterMap<rowan::cursor::PreorderWithTokens, <rowan::cursor::SyntaxNode>::descendants_with_tokens::{closure#0}>, <rowan::syntax_text::SyntaxText>::toke... (+2056 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/fuse.rs:339:24
  25: <core::iter::adapters::fuse::Fuse<core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter_map::FilterMap<rowan::cursor::PreorderWithTokens, <rowan::cursor::SyntaxNode>::descendants_with_tokens::{closure#0}>, <rowan::syntax_text::SyntaxText>::toke... (+1700 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/fuse.rs:94:9
  26: <core::iter::adapters::flatten::FlattenCompat<core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter_map::FilterMap<rowan::cursor::PreorderWithTokens, <rowan::cursor::SyntaxNode>::descendants_with_tokens::{closure#0}>, <rowan::syntax_text::Synta... (+1548 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/flatten.rs:431:25
  27: <core::iter::adapters::flatten::FlattenCompat<core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter_map::FilterMap<rowan::cursor::PreorderWithTokens, <rowan::cursor::SyntaxNode>::descendants_with_tokens::{closure#0}>, <rowan::syntax_text::Synta... (+1041 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/flatten.rs:569:14
  28: <core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter_map::FilterMap<rowan::cursor::PreorderWithTokens, <rowan::cursor::SyntaxNode>::descendants_with_tokens::{closure#0}>, <rowan::syntax_text::SyntaxText>::tokens_with_ranges::{closure#... (+1003 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/flatten.rs:79:20
  29: <core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter_map::FilterMap<rowan::cursor::PreorderWithTokens, <rowan::cursor::SyntaxNode>::descendants_with_tokens::{closure#0}>, <rowan::syntax_text... (+857 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/filter_map.rs:138:19
  30: <rowan::syntax_text::SyntaxText>::try_fold_chunks::<(), <rowan::syntax_text::SyntaxText>::try_for_each_chunk<<rowan::syntax_text::SyntaxText>::char_at::{closure#0}, char>::{closure#0}, char>
             at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/syntax_text.rs:88:14
  31: <rowan::syntax_text::SyntaxText>::try_for_each_chunk::<<rowan::syntax_text::SyntaxText>::char_at::{closure#0}, char>
             at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/syntax_text.rs:95:14
  32: <rowan::syntax_text::SyntaxText>::char_at
             at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/syntax_text.rs:48:24
  33: repro_rowan_src_syntax_text_rs_52_33_0c2633::main
             at ./src/main.rs:13:25
  34: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
