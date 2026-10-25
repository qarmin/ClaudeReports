# timeout in cfg_expr::ParseError::to_string

- crate: https://github.com/EmbarkStudios/cfg-expr @ 2ccf04b84e32345bbb04d21124ecb1c2fd8e42cd
- kind: timeout
- last API call: `cfg_expr::ParseError::to_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling cfg_expr::ParseError::to_string
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
cfg-expr = { git = "https://github.com/EmbarkStudios/cfg-expr", rev = "2ccf04b84e32345bbb04d21124ecb1c2fd8e42cd" }
```

`src/main.rs`:

```rust
use cfg_expr::ParseError;
use cfg_expr::error::Reason;

fn main() {
    let parse_error = ParseError { original: "".to_string(), span: 11usize..11660897532026927812usize, reason: Reason::InvalidNot(18446744073709551614usize) };
    let _ = parse_error.to_string();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_cfg_expr_parseerror_to_string_e6c382`).

