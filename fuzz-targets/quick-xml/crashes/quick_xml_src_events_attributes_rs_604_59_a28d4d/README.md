# quick-xml/src/events/attributes.rs:604:59

- crate: https://github.com/tafia/quick-xml @ 2eaa844b4b40117fb7ed4d1f512ede918125aeab
- kind: panic
- location: `quick-xml/src/events/attributes.rs:604:59`
- last API call: `quick_xml::events::attributes::Attributes::has_nil`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
start byte index 1 is not a char boundary; it is inside '\u{200b}' (bytes 0..3 of string)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
quick-xml = { git = "https://github.com/tafia/quick-xml", rev = "2eaa844b4b40117fb7ed4d1f512ede918125aeab", features = ["arbitrary", "async-tokio", "document-features", "encoding", "encoding_rs", "escape-html", "overlapped-lists", "serde", "serde-types", "serialize", "tokio"] }
```

`src/main.rs`:

```rust
use quick_xml::SeError;
use quick_xml::events::attributes::Attributes;
use quick_xml::name::NamespaceResolver;

fn main() {
    let se_error = SeError::Custom("\u{200b}".to_string());
    let string = se_error.to_string();
    let namespace_resolver = NamespaceResolver::default();
    let mut attributes = Attributes::html(&string, 1usize);
    let _ = attributes.has_nil(&namespace_resolver);
}
```

Run with `cargo run` in this directory (package `repro_quick_xml_src_events_attributes_rs_604_59_a28d4d`).

## Output

```
thread 'main' (578369) panicked at /home/rafal/.cargo/git/checkouts/quick-xml-1a0f2c88fad65209/2eaa844/src/events/attributes.rs:604:59:
start byte index 1 is not a char boundary; it is inside '\u{200b}' (bytes 0..3 of string)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::str::slice_error_fail_rt
   3: core::str::slice_error_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/str/mod.rs:70:5
   4: <core::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:335:21
   5: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:248:34
   6: <str as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:15
   7: <quick_xml::events::attributes::Attributes as core::iter::traits::iterator::Iterator>::next::{closure#0}
             at /home/rafal/.cargo/git/checkouts/quick-xml-1a0f2c88fad65209/2eaa844/src/events/attributes.rs:604:59
   8: <quick_xml::events::attributes::Attr<core::ops::range::Range<usize>>>::map::<&str, <quick_xml::events::attributes::Attributes as core::iter::traits::iterator::Iterator>::next::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/quick-xml-1a0f2c88fad65209/2eaa844/src/events/attributes.rs:780:45
   9: <quick_xml::events::attributes::Attributes as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.cargo/git/checkouts/quick-xml-1a0f2c88fad65209/2eaa844/src/events/attributes.rs:604:38
  10: <quick_xml::events::attributes::Attributes as core::iter::traits::iterator::Iterator>::try_fold::<(), core::iter::traits::iterator::Iterator::any::check<core::result::Result<quick_xml::events::attributes::Attribute, quick_xml::events::attributes::AttrError>, <quick_xml::events::attributes::Attributes>::has_nil::{closure#0}>::{closure#0}, core::ops::control_flow::ControlFlow<()>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2489:34
  11: <quick_xml::events::attributes::Attributes as core::iter::traits::iterator::Iterator>::any::<<quick_xml::events::attributes::Attributes>::has_nil::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2893:14
  12: <quick_xml::events::attributes::Attributes>::has_nil
             at /home/rafal/.cargo/git/checkouts/quick-xml-1a0f2c88fad65209/2eaa844/src/events/attributes.rs:572:14
  13: repro_quick_xml_src_events_attributes_rs_604_59_a28d4d::main
             at ./src/main.rs:10:24
  14: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
