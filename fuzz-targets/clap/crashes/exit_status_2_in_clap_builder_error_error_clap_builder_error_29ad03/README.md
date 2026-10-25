# exit status 2 in clap_builder::error::Error<clap_builder::error::KindFormatter>::exit

- crate: https://github.com/clap-rs/clap @ ccc89b83875474399e9d9d70841163f6319d1a41
- kind: exit status 2
- last API call: `clap_builder::error::Error<clap_builder::error::KindFormatter>::exit`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (exit status 2) while calling clap_builder::error::Error<clap_builder::error::KindFormatter>::exit
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
clap = { git = "https://github.com/clap-rs/clap", rev = "ccc89b83875474399e9d9d70841163f6319d1a41" }
clap_builder = { git = "https://github.com/clap-rs/clap", rev = "ccc89b83875474399e9d9d70841163f6319d1a41" }
```

`src/main.rs`:

```rust
use clap_builder::error::ErrorKind;

fn main() {
    let error = <clap_builder::error::Error<clap_builder::error::KindFormatter>>::raw(ErrorKind::ValueValidation, "".to_string());
    let _ = error.exit();
}
```

Run with `cargo run` in this directory (package `repro_exit_status_2_in_clap_builder_error_error_clap_builder_error_29ad03`).

## Output

```
error: 
```
