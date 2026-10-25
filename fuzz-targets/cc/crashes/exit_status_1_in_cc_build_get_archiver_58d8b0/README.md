# exit status 1 in cc::Build::get_archiver

- crate: https://github.com/rust-lang/cc-rs @ 7c27a1ab65e9f5909ed48eabf6bb4483876b9218
- kind: exit status 1
- last API call: `cc::Build::get_archiver`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (exit status 1) while calling cc::Build::get_archiver
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
cc = { git = "https://github.com/rust-lang/cc-rs", rev = "7c27a1ab65e9f5909ed48eabf6bb4483876b9218" }
```

`src/main.rs`:

```rust
use cc::Build;

fn main() {
    let build = Build::new();
    let _ = build.get_archiver();
}
```

Run with `cargo run` in this directory (package `repro_exit_status_1_in_cc_build_get_archiver_58d8b0`).

## Output

```
error occurred in cc-rs: environment variable TARGET not defined
```
