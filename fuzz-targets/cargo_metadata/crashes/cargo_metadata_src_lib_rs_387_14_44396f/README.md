# cargo_metadata/src/lib.rs:387:14

- crate: https://github.com/oli-obk/cargo_metadata @ 7cc507b09ac0d87bb83de45f61c17dbfb006f51b
- kind: panic
- location: `cargo_metadata/src/lib.rs:387:14`
- last API call: `cargo_metadata::WorkspaceDefaultMembers::get`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
WorkspaceDefaultMembers should only be dereferenced on Cargo versions >= 1.71
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
cargo_metadata = { git = "https://github.com/oli-obk/cargo_metadata", rev = "7cc507b09ac0d87bb83de45f61c17dbfb006f51b" }
```

`src/main.rs`:

```rust
use cargo_metadata::WorkspaceDefaultMembers;

fn main() {
    let workspace_default_members = WorkspaceDefaultMembers::default();
    let _ = workspace_default_members.get(0usize);
}
```

Run with `cargo run` in this directory (package `repro_cargo_metadata_src_lib_rs_387_14_44396f`).

## Output

```
thread 'main' (619533) panicked at /home/rafal/.cargo/git/checkouts/cargo_metadata-cff8f4a622a4ba4e/7cc507b/src/lib.rs:387:14:
WorkspaceDefaultMembers should only be dereferenced on Cargo versions >= 1.71
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<&alloc::vec::Vec<cargo_metadata::PackageId>>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <cargo_metadata::WorkspaceDefaultMembers as core::ops::deref::Deref>::deref
             at /home/rafal/.cargo/git/checkouts/cargo_metadata-cff8f4a622a4ba4e/7cc507b/src/lib.rs:387:14
   6: repro_cargo_metadata_src_lib_rs_387_14_44396f::main
             at ./src/main.rs:5:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
