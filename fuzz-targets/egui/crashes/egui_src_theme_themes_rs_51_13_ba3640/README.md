# egui/src/theme/themes.rs:51:13

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/theme/themes.rs:51:13`
- last API call: `egui::theme::Themes::get[SeparatorStyle]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 9362975339415790505

## Message

```
A style should be set for "egui::widget_style::SeparatorStyle"
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec", features = ["_override_unity", "bytemuck", "callstack", "cint", "color-hex", "default_fonts", "document-features", "experimental", "mint", "monochrome_emoji_fonts", "persistence", "rayon", "ron", "serde", "unity", "web_file"] }
```

`src/main.rs`:

```rust
use egui::theme::Themes;

fn main() {
    let themes = Themes::default();
    let _ = themes.get::<egui::widget_style::SeparatorStyle>();
}
```

Run with `cargo run` in this directory (package `repro_egui_src_theme_themes_rs_51_13_ba3640`).

## Output

```
thread 'main' (82975) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/theme/themes.rs:51:13:
A style should be set for "egui::widget_style::SeparatorStyle"
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::theme::themes::Themes>::get::<egui::widget_style::SeparatorStyle>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/theme/themes.rs:51:13
   3: <core::option::Option<alloc::sync::Arc<epaint::mutex::Mutex<alloc::boxed::Box<dyn egui::theme::style_provider::StyleProvider<egui::widget_style::SeparatorStyle> + core::marker::Send + core::marker::Sync>>>>>::unwrap_or_else::<<egui::theme::themes::Themes>::get<egui::widget_style::SeparatorStyl... (+16 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1065:21
   4: <egui::theme::themes::Themes>::get::<egui::widget_style::SeparatorStyle>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/theme/themes.rs:50:11
   5: repro_egui_src_theme_themes_rs_51_13_ba3640::main
             at ./src/main.rs:5:20
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
