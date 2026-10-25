# syn/src/attr.rs:558:39

- crate: https://github.com/dtolnay/syn @ 9ff9287583767e53dcfb70e3b1b350996908bd8b
- kind: panic
- location: `syn/src/attr.rs:558:39`
- last API call: `syn::Meta::require_name_value`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
syn = { git = "https://github.com/dtolnay/syn", rev = "9ff9287583767e53dcfb70e3b1b350996908bd8b" }
```

`src/main.rs`:

```rust
use syn::Meta;
use syn::Path;

fn main() {
    let meta = Meta::from(Path { leading_colon: None, segments: <syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>>::new() });
    let _ = meta.require_name_value();
}
```

Run with `cargo run` in this directory (package `repro_syn_src_attr_rs_558_39_11cde2`).

## Output

```
thread 'main' (471037) panicked at /home/rafal/.cargo/git/checkouts/syn-bf1624cda1931fcf/9ff9287/src/attr.rs:558:39:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<&syn::path::PathSegment>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <syn::attr::Meta>::require_name_value
             at /home/rafal/.cargo/git/checkouts/syn-bf1624cda1931fcf/9ff9287/src/attr.rs:558:39
   6: repro_syn_src_attr_rs_558_39_11cde2::main
             at ./src/main.rs:6:18
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
