# rust-cssparser/src/tokenizer.rs:298:9

- crate: https://github.com/servo/rust-cssparser @ 9f3ceddf398f5ce4ab76fdad1b42ccb6c578a926
- kind: panic
- location: `rust-cssparser/src/tokenizer.rs:298:9`
- last API call: `cssparser::Parser::slice`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: self.input.is_char_boundary(range.end.0)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
cssparser = { git = "https://github.com/servo/rust-cssparser", rev = "9f3ceddf398f5ce4ab76fdad1b42ccb6c578a926" }
```

`src/main.rs`:

```rust
use cssparser::Parser;

fn main() {
    let string = "".to_string();
    let parser = Parser::new(&string);
    let parser_state = parser.state();
    let string2 = "Y".to_string();
    let mut parser2 = Parser::new(&string2);
    let _ = cssparser::parse_nth(&mut parser2);
    let _ = parser.slice(parser_state.position()..parser2.position());
}
```

Run with `cargo run` in this directory (package `repro_rust_cssparser_src_tokenizer_rs_298_9_730318`).

## Output

```
thread 'main' (1399914) panicked at /home/rafal/.cargo/git/checkouts/rust-cssparser-c679796d37363c82/9f3cedd/src/tokenizer.rs:298:9:
assertion failed: self.input.is_char_boundary(range.end.0)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <cssparser::parser::Parser>::slice
             at /home/rafal/.cargo/git/checkouts/rust-cssparser-c679796d37363c82/9f3cedd/src/tokenizer.rs:298:9
   4: repro_rust_cssparser_src_tokenizer_rs_298_9_730318::main
             at ./src/main.rs:10:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
