# epaint/src/text/text_layout_types.rs:255:13

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/text/text_layout_types.rs:255:13`
- last API call: `epaint::text::LayoutJob::debug_sanity_check`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 3294265860786362477

## Message

```
assertion `left == right` failed: First LayoutSection must start at byte 0
  left: ByteIndex(1)
 right: ByteIndex(0)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use emath::Align;
use epaint::text::ByteIndex;
use epaint::text::LayoutJob;
use epaint::text::LayoutSection;
use epaint::text::TextFormat;
use epaint::text::TextWrapping;

fn main() {
    let layout_job = LayoutJob { text: "".to_string(), sections: vec![LayoutSection { leading_space: 0.0f32, byte_range: ByteIndex::from(1usize)..ByteIndex::ZERO, format: TextFormat::default() }], wrap: TextWrapping::truncate_at_width(0.0f32), first_row_min_height: 0.0f32, break_on_newline: false, halign: Align::Center, justify: false, round_output_to_gui: false, keep_trailing_whitespace: false };
    layout_job.debug_sanity_check();
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_text_text_layout_types_rs_255_13_9d81c7`).

## Output

```
thread 'main' (337910) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout_types.rs:255:13:
assertion `left == right` failed: First LayoutSection must start at byte 0
  left: ByteIndex(1)
 right: ByteIndex(0)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<epaint::text::index::ByteIndex, epaint::text::index::ByteIndex>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: <epaint::text::text_layout_types::LayoutJob>::debug_sanity_check
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout_types.rs:255:13
   5: repro_epaint_src_text_text_layout_types_rs_255_13_9d81c7::main
             at ./src/main.rs:10:16
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
