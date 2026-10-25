# regex-syntax/src/ast/parse.rs:983:9

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-syntax/src/ast/parse.rs:983:9`
- last API call: `regex_syntax::ast::parse::Parser::parse`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 18085437458194885540

## Message

```
assertion `left == right` failed: parser can only be used once
  left: 1
 right: 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
regex = { git = "https://github.com/rust-lang/regex", rev = "72d650cb0a880a01ab6dc2137c0888e8f89740f7" }
regex-syntax = { git = "https://github.com/rust-lang/regex", rev = "72d650cb0a880a01ab6dc2137c0888e8f89740f7" }
```

`src/main.rs`:

```rust
use regex_syntax::ast::parse::Parser;

fn main() {
    let mut parser = Parser::new();
    let _ = parser.parse_with_comments("a");
    let _ = parser.parse("");
}
```

Run with `cargo run` in this directory (package `repro_regex_syntax_src_ast_parse_rs_983_9_99eb80`).

## Output

```
thread 'main' (358523) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-syntax/src/ast/parse.rs:983:9:
assertion `left == right` failed: parser can only be used once
  left: 1
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <regex_syntax::ast::parse::ParserI<&mut regex_syntax::ast::parse::Parser>>::parse_with_comments
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-syntax/src/ast/parse.rs:983:9
   5: <regex_syntax::ast::parse::ParserI<&mut regex_syntax::ast::parse::Parser>>::parse
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-syntax/src/ast/parse.rs:977:14
   6: <regex_syntax::ast::parse::Parser>::parse
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-syntax/src/ast/parse.rs:363:37
   7: repro_regex_syntax_src_ast_parse_rs_983_9_99eb80::main
             at ./src/main.rs:6:20
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
