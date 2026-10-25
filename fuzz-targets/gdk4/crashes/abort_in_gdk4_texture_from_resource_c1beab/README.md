# abort in gdk4::Texture::from_resource

- crate: gdk4 0.11.5
- kind: abort
- last API call: `gdk4::Texture::from_resource`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (abort) while calling gdk4::Texture::from_resource
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gdk4 = { version = "=0.11.5" }
```

`src/main.rs`:

```rust
use gdk4::Texture;

fn main() {
    let _ = Texture::from_resource("");
}
```

Run with `cargo run` in this directory (package `repro_abort_in_gdk4_texture_from_resource_c1beab`).

## Output

```
(process:3323398): Gdk-ERROR **: 17:26:46.019: Resource path  is not a valid image: The resource at “” does not exist
```
