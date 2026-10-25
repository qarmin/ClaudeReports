# alsa-rs/src/seq.rs:695:9

- crate: https://github.com/diwic/alsa-rs @ ffc02b01730d2f78d2fdebd26e8ef492bae9f276
- kind: panic
- location: `alsa-rs/src/seq.rs:695:9`
- last API call: `alsa::seq::Event::new[()]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: D::has_data(t)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
alsa = { git = "https://github.com/diwic/alsa-rs", rev = "ffc02b01730d2f78d2fdebd26e8ef492bae9f276" }
```

`src/main.rs`:

```rust
use alsa::seq::Event;
use alsa::seq::EventType;

fn main() {
    let v = ();
    let _ = Event::new::<()>(EventType::Pgmchange, &v);
}
```

Run with `cargo run` in this directory (package `repro_alsa_rs_src_seq_rs_695_9_390b6d`).

## Output

```
thread 'main' (1299959) panicked at /home/rafal/.cargo/git/checkouts/alsa-rs-ff4182d1fcf76b3b/ffc02b0/src/seq.rs:695:9:
assertion failed: D::has_data(t)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <alsa::seq::Event>::new::<()>
             at /home/rafal/.cargo/git/checkouts/alsa-rs-ff4182d1fcf76b3b/ffc02b0/src/seq.rs:695:9
   4: repro_alsa_rs_src_seq_rs_695_9_390b6d::main
             at ./src/main.rs:6:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
