# abort in gdk4::DisplayManager::get

- crate: gdk4 0.11.5
- kind: abort
- last API call: `gdk4::DisplayManager::get`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (abort) while calling gdk4::DisplayManager::get
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gdk4 = { version = "=0.11.5" }
```

`src/main.rs`:

```rust
use gdk4::DisplayManager;

fn main() {
    let _ = DisplayManager::get();
}
```

Run with `cargo run` in this directory (package `repro_abort_in_gdk4_displaymanager_get_4aa5f9`).

## Output

```
(process:3330731): Gdk-ERROR **: 17:27:04.342: gdk_display_manager_get() was called before gtk_init()
```
