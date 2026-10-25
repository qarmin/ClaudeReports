# krilla/src/interactive/form.rs:444:9

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/interactive/form.rs:444:9`
- last API call: `krilla::form::FormField<krilla::form::kind::Radio>::radio`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
field name cannot contain a period
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <krilla::form::FormField<krilla::form::kind::Radio>>::radio(".".to_string(), None);
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_interactive_form_rs_444_9_057fb8`).

## Output

```
thread 'main' (250487) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/form.rs:444:9:
field name cannot contain a period
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <krilla::interactive::form::FormField<krilla::interactive::form::kind::Radio>>::radio
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/form.rs:444:9
   3: repro_krilla_src_interactive_form_rs_444_9_057fb8::main
             at ./src/main.rs:2:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
