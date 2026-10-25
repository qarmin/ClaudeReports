# timeout in console::Term::clear_last_lines

- crate: https://github.com/console-rs/console @ 4329b774d2e7117df706193636b4734b147e6554
- kind: timeout
- last API call: `console::Term::clear_last_lines`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling console::Term::clear_last_lines
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
console = { git = "https://github.com/console-rs/console", rev = "4329b774d2e7117df706193636b4734b147e6554" }
```

`src/main.rs`:

```rust
use console::Term;

fn main() {
    let term = Term::buffered_stdout();
    let _ = term.clear_last_lines(1000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_console_term_clear_last_lines_566c85`).

