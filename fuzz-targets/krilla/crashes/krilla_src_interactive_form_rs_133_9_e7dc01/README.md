# krilla/src/interactive/form.rs:133:9

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/interactive/form.rs:133:9`
- last API call: `krilla::document::Document::finish`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
field group name cannot contain a period
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::document::Document;
use krilla::form::FieldGroup;
use krilla::form::FieldTree;
use krilla::form::Node;

fn main() {
    let mut document = Document::default();
    document.set_field_tree(FieldTree { fields: vec![Node::from(FieldGroup { name: ".".to_string(), fields: vec![] })] });
    let _ = document.finish();
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_interactive_form_rs_133_9_e7dc01`).

## Output

```
thread 'main' (251073) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/form.rs:133:9:
field group name cannot contain a period
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <krilla::interactive::form::FieldGroup>::serialize_group
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/form.rs:133:9
   3: <krilla::interactive::form::Node>::serialize_node
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/form.rs:176:29
   4: <krilla::interactive::form::AcroForm>::serialize::{closure#0}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/form.rs:59:34
   5: <&mut <krilla::interactive::form::AcroForm>::serialize::{closure#0} as core::ops::function::FnOnce<(&krilla::interactive::form::Node,)>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:310:21
   6: <core::option::Option<&krilla::interactive::form::Node>>::map::<pdf_writer::object::Ref, &mut <krilla::interactive::form::AcroForm>::serialize::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1163:29
   7: <core::iter::adapters::map::Map<core::slice::iter::Iter<krilla::interactive::form::Node>, <krilla::interactive::form::AcroForm>::serialize::{closure#0}> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:107:26
   8: <pdf_writer::object::Array>::items::<pdf_writer::object::Ref, core::iter::adapters::map::Map<core::slice::iter::Iter<krilla::interactive::form::Node>, <krilla::interactive::form::AcroForm>::serialize::{closure#0}>>
             at /home/rafal/.cargo/git/checkouts/pdf-writer-6ef6756ec5e515e5/fdf0901/src/object.rs:831:22
   9: <pdf_writer::forms::Form>::fields::<core::iter::adapters::map::Map<core::slice::iter::Iter<krilla::interactive::form::Node>, <krilla::interactive::form::AcroForm>::serialize::{closure#0}>>
             at /home/rafal/.cargo/git/checkouts/pdf-writer-6ef6756ec5e515e5/fdf0901/src/forms.rs:18:46
  10: <krilla::interactive::form::AcroForm>::serialize
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/form.rs:61:18
  11: <krilla::serialize::SerializeContext>::serialize_forms
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:838:22
  12: <krilla::serialize::SerializeContext>::finish
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:475:14
  13: <krilla::document::Document>::finish
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/document.rs:160:31
  14: repro_krilla_src_interactive_form_rs_133_9_e7dc01::main
             at ./src/main.rs:9:22
  15: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
