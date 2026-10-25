# timeout in console::Term::move_cursor_right

- crate: https://github.com/console-rs/console @ 4329b774d2e7117df706193636b4734b147e6554
- kind: timeout
- last API call: `console::Term::move_cursor_right`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling console::Term::move_cursor_right
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
    let term = Term::stderr();
    let _ = term.move_cursor_right(1usize);
    let _ = term.clear_last_lines(4294967295usize);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_console_term_move_cursor_right_e7643b`).

## Output

```
\x1b[1C\x1b[4294967295A\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1... (+20 chars)
```
