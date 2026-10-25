# alsa-rs/src/seq.rs:691:9

- crate: https://github.com/diwic/alsa-rs @ ffc02b01730d2f78d2fdebd26e8ef492bae9f276
- kind: panic
- location: `alsa-rs/src/seq.rs:691:9`
- last API call: `alsa::seq::Event::new[()]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
event type must not carry variable-length data
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
    let _ = Event::new::<()>(EventType::Bounce, &v);
}
```

Run with `cargo run` in this directory (package `repro_alsa_rs_src_seq_rs_691_9_95e2ad`).

## Output

```
thread 'main' (1299907) panicked at /home/rafal/.cargo/git/checkouts/alsa-rs-ff4182d1fcf76b3b/ffc02b0/src/seq.rs:691:9:
event type must not carry variable-length data
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <alsa::seq::Event>::new::<()>
             at /home/rafal/.cargo/git/checkouts/alsa-rs-ff4182d1fcf76b3b/ffc02b0/src/seq.rs:691:9
   3: repro_alsa_rs_src_seq_rs_691_9_95e2ad::main
             at ./src/main.rs:6:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
