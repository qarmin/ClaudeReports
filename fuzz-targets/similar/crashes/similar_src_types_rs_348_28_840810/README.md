# similar/src/types.rs:348:28

- crate: https://github.com/mitsuhiko/similar @ 96993d92851eff9a484ea157614de30f80dfdc73
- kind: panic
- location: `similar/src/types.rs:348:28`
- last API call: `similar::udiff::UnifiedHunkHeader::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
similar = { git = "https://github.com/mitsuhiko/similar", rev = "96993d92851eff9a484ea157614de30f80dfdc73" }
```

`src/main.rs`:

```rust
use similar::DiffOp;
use similar::udiff::UnifiedHunkHeader;

fn main() {
    let _ = UnifiedHunkHeader::new(&[DiffOp::Insert { old_index: 1usize, new_index: usize::MAX, new_len: 1usize }]);
}
```

Run with `cargo run` in this directory (package `repro_similar_src_types_rs_348_28_840810`).

## Output

```
thread 'main' (1407932) panicked at /home/rafal/.cargo/git/checkouts/similar-1382b3d2b482dd2e/96993d9/src/types.rs:348:28:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <similar::types::DiffOp>::as_tag_tuple
             at /home/rafal/.cargo/git/checkouts/similar-1382b3d2b482dd2e/96993d9/src/types.rs:348:28
   4: <similar::types::DiffOp>::old_range
             at /home/rafal/.cargo/git/checkouts/similar-1382b3d2b482dd2e/96993d9/src/types.rs:304:14
   5: <similar::udiff::UnifiedHunkHeader>::new
             at /home/rafal/.cargo/git/checkouts/similar-1382b3d2b482dd2e/96993d9/src/udiff.rs:90:31
   6: repro_similar_src_types_rs_348_28_840810::main
             at ./src/main.rs:5:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
