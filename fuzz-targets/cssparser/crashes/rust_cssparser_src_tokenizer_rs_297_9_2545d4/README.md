# rust-cssparser/src/tokenizer.rs:297:9

- crate: https://github.com/servo/rust-cssparser @ 9f3ceddf398f5ce4ab76fdad1b42ccb6c578a926
- kind: panic
- location: `rust-cssparser/src/tokenizer.rs:297:9`
- last API call: `cssparser::Parser::slice_from`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: self.input.is_char_boundary(range.start.0)
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
    let string = "a".to_string();
    let mut parser = Parser::new(&string);
    let _ = cssparser::parse_nth(&mut parser);
    let string2 = "".to_string();
    let parser2 = Parser::new(&string2);
    let source_position = parser.position();
    let _ = parser2.slice_from(source_position);
}
```

Run with `cargo run` in this directory (package `repro_rust_cssparser_src_tokenizer_rs_297_9_2545d4`).

## Output

```
thread 'main' (1399990) panicked at /home/rafal/.cargo/git/checkouts/rust-cssparser-c679796d37363c82/9f3cedd/src/tokenizer.rs:297:9:
assertion failed: self.input.is_char_boundary(range.start.0)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <cssparser::parser::Parser>::slice
             at /home/rafal/.cargo/git/checkouts/rust-cssparser-c679796d37363c82/9f3cedd/src/tokenizer.rs:297:9
   4: <cssparser::parser::Parser>::slice_from
             at /home/rafal/.cargo/git/checkouts/rust-cssparser-c679796d37363c82/9f3cedd/src/tokenizer.rs:291:14
   5: repro_rust_cssparser_src_tokenizer_rs_297_9_2545d4::main
             at ./src/main.rs:10:21
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
