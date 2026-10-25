# timeout in crossbeam_utils::sync::WaitGroup::wait

- crate: https://github.com/crossbeam-rs/crossbeam @ 099d0469efcfacb32527bd9451a4fb62586ae537
- kind: timeout
- last API call: `crossbeam_utils::sync::WaitGroup::wait`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling crossbeam_utils::sync::WaitGroup::wait
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
crossbeam-channel = { git = "https://github.com/crossbeam-rs/crossbeam", rev = "099d0469efcfacb32527bd9451a4fb62586ae537" }
crossbeam-epoch = { git = "https://github.com/crossbeam-rs/crossbeam", rev = "099d0469efcfacb32527bd9451a4fb62586ae537" }
crossbeam-utils = { git = "https://github.com/crossbeam-rs/crossbeam", rev = "099d0469efcfacb32527bd9451a4fb62586ae537" }
```

`src/main.rs`:

```rust
use crossbeam_channel::RecvError;
use crossbeam_channel::RecvTimeoutError;
use crossbeam_utils::sync::WaitGroup;

fn main() {
    let guard = crossbeam_epoch::pin();
    let _ = guard.collector();
    let wait_group = WaitGroup::default();
    wait_group.clone().wait();
    let _ = RecvTimeoutError::from(RecvError::from(RecvError::from(RecvError)));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_crossbeam_utils_sync_waitgroup_wait_430314`).

