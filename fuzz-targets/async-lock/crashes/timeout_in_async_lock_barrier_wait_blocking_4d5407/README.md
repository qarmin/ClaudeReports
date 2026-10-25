# timeout in async_lock::Barrier::wait_blocking

- crate: https://github.com/smol-rs/async-lock @ 23e181d5cfb91a59382e5719c682caa58ff43ace
- kind: timeout
- last API call: `async_lock::Barrier::wait_blocking`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling async_lock::Barrier::wait_blocking
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
async-lock = { git = "https://github.com/smol-rs/async-lock", rev = "23e181d5cfb91a59382e5719c682caa58ff43ace" }
```

`src/main.rs`:

```rust
use async_lock::Barrier;
use async_lock::Semaphore;

fn main() {
    let semaphore_guard_arc = std::sync::Arc::new(Semaphore::new(4999147007719037987usize)).acquire_arc_blocking();
    semaphore_guard_arc.forget();
    let barrier = Barrier::new(usize::MAX);
    let _ = barrier.wait_blocking();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_async_lock_barrier_wait_blocking_4d5407`).

