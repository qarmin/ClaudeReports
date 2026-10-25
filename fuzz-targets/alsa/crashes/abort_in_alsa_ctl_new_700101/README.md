# abort in alsa::Ctl::new

- crate: https://github.com/diwic/alsa-rs @ ffc02b01730d2f78d2fdebd26e8ef492bae9f276
- kind: abort
- last API call: `alsa::Ctl::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (abort) while calling alsa::Ctl::new
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
alsa = { git = "https://github.com/diwic/alsa-rs", rev = "ffc02b01730d2f78d2fdebd26e8ef492bae9f276" }
```

`src/main.rs`:

```rust
use alsa::Ctl;
use alsa::Output;

fn main() {
    let _ = Output::local_error_handler();
    let _ = Ctl::new("a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}a\u{301}\u{301}\u{301}", false);
}
```

Run with `cargo run` in this directory (package `repro_abort_in_alsa_ctl_new_700101`).

## Output

```
repro_abort_in_alsa_ctl_new_700101: output.c:293: snd_output_buffer_print: Assertion `result == (int)size' failed.
```
