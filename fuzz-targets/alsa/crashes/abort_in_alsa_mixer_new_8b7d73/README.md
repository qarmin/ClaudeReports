# abort in alsa::Mixer::new

- crate: https://github.com/diwic/alsa-rs @ ffc02b01730d2f78d2fdebd26e8ef492bae9f276
- kind: abort
- last API call: `alsa::Mixer::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (abort) while calling alsa::Mixer::new
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
alsa = { git = "https://github.com/diwic/alsa-rs", rev = "ffc02b01730d2f78d2fdebd26e8ef492bae9f276" }
```

`src/main.rs`:

```rust
use alsa::Mixer;
use alsa::Output;

fn main() {
    let _ = Output::local_error_handler();
    let _ = Mixer::new("(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\)(\\", false);
}
```

Run with `cargo run` in this directory (package `repro_abort_in_alsa_mixer_new_8b7d73`).

## Output

```
repro_abort_in_alsa_mixer_new_8b7d73: output.c:293: snd_output_buffer_print: Assertion `result == (int)size' failed.
```
