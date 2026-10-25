# seahash/src/buffer.rs:205:9

- crate: https://gitlab.redox-os.org/redox-os/seahash @ 74c02182146d1edd7c91a9e6eddefc7390682a70
- kind: panic
- location: `seahash/src/buffer.rs:205:9`
- last API call: `seahash::State::pop`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
seahash = { git = "https://gitlab.redox-os.org/redox-os/seahash", rev = "74c02182146d1edd7c91a9e6eddefc7390682a70" }
```

`src/main.rs`:

```rust
use seahash::State;

fn main() {
    let u64_v = seahash::reference::hash(&vec![]);
    let mut state = State::new(0u64, 0u64, 0u64, u64_v);
    state.pop(0u64);
}
```

Run with `cargo run` in this directory (package `repro_seahash_src_buffer_rs_205_9_4308db`).

## Output

```
thread 'main' (574154) panicked at /home/rafal/.cargo/git/checkouts/seahash-39c74a3aeb9b7433/74c0218/src/buffer.rs:205:9:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <seahash::buffer::State>::pop
             at /home/rafal/.cargo/git/checkouts/seahash-39c74a3aeb9b7433/74c0218/src/buffer.rs:205:9
   4: repro_seahash_src_buffer_rs_205_9_4308db::main
             at ./src/main.rs:6:11
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
