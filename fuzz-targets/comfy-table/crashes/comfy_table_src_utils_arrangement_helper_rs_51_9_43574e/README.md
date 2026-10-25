# comfy-table/src/utils/arrangement/helper.rs:51:9

- crate: https://github.com/nukesor/comfy-table @ b947cb6bf4a9aef846e4ec00a67f68bd85d41c73
- kind: panic
- location: `comfy-table/src/utils/arrangement/helper.rs:51:9`
- last API call: `comfy_table::utils::arrangement::helper::count_border_columns`
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
comfy-table = { git = "https://github.com/nukesor/comfy-table", rev = "b947cb6bf4a9aef846e4ec00a67f68bd85d41c73", features = ["_debug", "_integration_test", "custom_styling", "reexport_crossterm", "tty"] }
```

`src/main.rs`:

```rust
use comfy_table::Table;

fn main() {
    let _ = comfy_table::utils::arrangement::helper::count_border_columns(&Table::default(), usize::MAX);
}
```

Run with `cargo run` in this directory (package `repro_comfy_table_src_utils_arrangement_helper_rs_51_9_43574e`).

## Output

```
thread 'main' (598830) panicked at /home/rafal/.cargo/git/checkouts/comfy-table-78fe529caf8b1a47/b947cb6/src/utils/arrangement/helper.rs:51:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: comfy_table::utils::arrangement::helper::count_border_columns
             at /home/rafal/.cargo/git/checkouts/comfy-table-78fe529caf8b1a47/b947cb6/src/utils/arrangement/helper.rs:51:9
   4: repro_comfy_table_src_utils_arrangement_helper_rs_51_9_43574e::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
