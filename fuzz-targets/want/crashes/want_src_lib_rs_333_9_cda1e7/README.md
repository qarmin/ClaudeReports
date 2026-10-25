# want/src/lib.rs:333:9

- crate: https://github.com/seanmonstar/want @ 322291ce5a87c72ecfd83d383d0d7f5245cf72c6
- kind: panic
- location: `want/src/lib.rs:333:9`
- last API call: `want::Taker::want`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
want called after cancel
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
want = { git = "https://github.com/seanmonstar/want", rev = "322291ce5a87c72ecfd83d383d0d7f5245cf72c6" }
```

`src/main.rs`:

```rust
fn main() {
    let mut taker = { want::new().1 };
    taker.cancel();
    taker.want();
}
```

Run with `cargo run` in this directory (package `repro_want_src_lib_rs_333_9_cda1e7`).

## Output

```
thread 'main' (567316) panicked at /home/rafal/.cargo/git/checkouts/want-a9aac108784238b6/322291c/src/lib.rs:333:9:
want called after cancel
stack backtrace:
   0: std::panicking::begin_panic::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:751:5
   1: <want::Taker>::want
             at /home/rafal/.cargo/git/checkouts/want-a9aac108784238b6/322291c/src/lib.rs:333:9
   2: repro_want_src_lib_rs_333_9_cda1e7::main
             at ./src/main.rs:4:11
   3: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
