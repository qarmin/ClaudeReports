# rust/src/entry/poentry.rs:99:45

- crate: https://github.com/mondeja/rspolib @ 12343ca23f0c9cbe6d7a2cdcfbf5881a6cb1fbe8
- kind: panic
- location: `rust/src/entry/poentry.rs:99:45`
- last API call: `rspolib::POEntry::to_string_with_wrapwidth`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rspolib = { git = "https://github.com/mondeja/rspolib", rev = "12343ca23f0c9cbe6d7a2cdcfbf5881a6cb1fbe8" }
```

`src/main.rs`:

```rust
use rspolib::POEntry;

fn main() {
    let p_o_entry = POEntry { msgid: "".to_string(), msgstr: None, msgid_plural: None, msgstr_plural: vec![], msgctxt: None, obsolete: false, comment: Some("a".to_string()), tcomment: None, occurrences: vec![], flags: vec![], previous_msgid: None, previous_msgid_plural: None, previous_msgctxt: None, linenum: 0usize };
    let _ = p_o_entry.to_string_with_wrapwidth(0usize);
}
```

Run with `cargo run` in this directory (package `repro_rust_src_entry_poentry_rs_99_45_58ce14`).

## Output

```
thread 'main' (1364865) panicked at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/entry/poentry.rs:99:45:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <rspolib::entry::poentry::POEntry>::format_comment_inplace
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/entry/poentry.rs:99:45
   4: <rspolib::entry::poentry::POEntry>::to_string_with_wrapwidth
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/entry/poentry.rs:124:18
   5: repro_rust_src_entry_poentry_rs_99_45_58ce14::main
             at ./src/main.rs:5:23
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
