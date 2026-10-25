# gtk4/src/auto/popover_menu.rs:59:9

- crate: https://github.com/tauri-apps/muda @ 33dfdf7c6daceed1dddd73b658e0e4c912685f36
- kind: panic
- location: `gtk4/src/auto/popover_menu.rs:59:9`
- source: https://docs.rs/crate/gtk4/0.11.5/source/src/auto/popover_menu.rs#59
- last API call: `muda::Menu::gtk_context_menu`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
GTK has not been initialized. Call `gtk::init` first.
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
muda = { git = "https://github.com/tauri-apps/muda", rev = "33dfdf7c6daceed1dddd73b658e0e4c912685f36", features = ["common-controls-v6", "gtk3", "gtk4", "serde", "snapshot"] }
```

`src/main.rs`:

```rust
use muda::Menu;

fn main() {
    let _ = <muda::Menu as muda::ContextMenu>::gtk_context_menu(&Menu::new());
}
```

Run with `cargo run` in this directory (package `repro_gtk4_src_auto_popover_menu_rs_59_9_d91926`).

## Output

```
thread 'main' (516983) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gtk4-0.11.5/src/auto/popover_menu.rs:59:9:
GTK has not been initialized. Call `gtk::init` first.
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <gtk4::auto::popover_menu::PopoverMenu>::from_model_full::<gio::auto::menu::Menu>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gtk4-0.11.5/src/rt.rs:32:17
   3: <muda::platform_impl::platform::GtkMenuBar>::new_context
             at /home/rafal/.cargo/git/checkouts/muda-7190a70c65b16524/33dfdf7/src/platform_impl/gtk4/mod.rs:77:22
   4: <muda::platform_impl::platform::PlatformMenu>::ensure_context_menu
             at /home/rafal/.cargo/git/checkouts/muda-7190a70c65b16524/33dfdf7/src/platform_impl/gtk4/mod.rs:365:20
   5: <muda::platform_impl::platform::PlatformMenu>::gtk_context_menu
             at /home/rafal/.cargo/git/checkouts/muda-7190a70c65b16524/33dfdf7/src/platform_impl/gtk4/mod.rs:352:14
   6: <muda::items::menu::Menu as muda::context_menu::ContextMenu>::gtk_context_menu
             at /home/rafal/.cargo/git/checkouts/muda-7190a70c65b16524/33dfdf7/src/items/menu.rs:633:36
   7: repro_gtk4_src_auto_popover_menu_rs_59_9_d91926::main
             at ./src/main.rs:4:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
