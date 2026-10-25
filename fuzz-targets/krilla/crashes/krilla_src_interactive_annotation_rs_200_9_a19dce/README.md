# krilla/src/interactive/annotation.rs:200:9

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/interactive/annotation.rs:200:9`
- last API call: `krilla::annotation::LinkAnnotation::new_with_quad_points`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: !quad_points.is_empty()
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::action::Action;
use krilla::action::ResetFormAction;
use krilla::annotation::LinkAnnotation;
use krilla::annotation::Target;

fn main() {
    let _ = LinkAnnotation::new_with_quad_points(vec![], Target::Action(Action::ResetForm(ResetFormAction::Include(vec![]))));
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_interactive_annotation_rs_200_9_a19dce`).

## Output

```
thread 'main' (250401) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/annotation.rs:200:9:
assertion failed: !quad_points.is_empty()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <krilla::interactive::annotation::LinkAnnotation>::new_with_quad_points
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/annotation.rs:200:9
   4: repro_krilla_src_interactive_annotation_rs_200_9_a19dce::main
             at ./src/main.rs:7:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
