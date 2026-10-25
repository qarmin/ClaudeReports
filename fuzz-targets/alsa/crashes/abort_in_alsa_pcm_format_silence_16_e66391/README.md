# abort in alsa::pcm::Format::silence_16

- crate: https://github.com/diwic/alsa-rs @ ffc02b01730d2f78d2fdebd26e8ef492bae9f276
- kind: abort
- last API call: `alsa::pcm::Format::silence_16`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (abort) while calling alsa::pcm::Format::silence_16
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
alsa = { git = "https://github.com/diwic/alsa-rs", rev = "ffc02b01730d2f78d2fdebd26e8ef492bae9f276" }
```

`src/main.rs`:

```rust
use alsa::pcm::Format;

fn main() {
    let format = Format::dsd_u32();
    let _ = format.silence_16();
}
```

Run with `cargo run` in this directory (package `repro_abort_in_alsa_pcm_format_silence_16_e66391`).

## Output

```
repro_abort_in_alsa_pcm_format_silence_16_e66391: pcm_misc.c:585: snd_pcm_format_silence_16: Assertion `snd_pcm_format_physical_width(format) <= 16' failed.
```
