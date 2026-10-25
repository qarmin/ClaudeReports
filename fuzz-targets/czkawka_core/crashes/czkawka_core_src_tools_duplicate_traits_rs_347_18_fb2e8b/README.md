# czkawka_core/src/tools/duplicate/traits.rs:347:18

- crate: https://github.com/qarmin/czkawka @ eb8b91dbb4d25dff416202674ee0de84ee4b5e5c
- kind: panic
- location: `czkawka_core/src/tools/duplicate/traits.rs:347:18`
- last API call: `czkawka_core::tools::duplicate::DuplicateFinder::print_results_to_writer`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
explicit panic
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
czkawka_core = { git = "https://github.com/qarmin/czkawka", rev = "eb8b91dbb4d25dff416202674ee0de84ee4b5e5c" }
```

`src/main.rs`:

```rust
use czkawka_core::common::model::CheckingMethod;
use czkawka_core::common::model::HashType;
use czkawka_core::tools::duplicate::DuplicateFinder;
use czkawka_core::tools::duplicate::DuplicateFinderParameters;

fn main() {
    let _ = <czkawka_core::tools::duplicate::DuplicateFinder as czkawka_core::common::traits::PrintResults>::print_results_to_writer::<Vec<u8>>(&DuplicateFinder::new(DuplicateFinderParameters::new(CheckingMethod::VideoAudioContent, HashType::default(), false, 0u64, 0u64, false)), &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_czkawka_core_src_tools_duplicate_traits_rs_347_18_fb2e8b`).

## Output

```
thread 'main' (517268) panicked at /home/rafal/.cargo/git/checkouts/czkawka-c1570a3d7ff29c8c/eb8b91d/czkawka_core/src/tools/duplicate/traits.rs:347:18:
explicit panic
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <czkawka_core::tools::duplicate::DuplicateFinder as czkawka_core::common::traits::PrintResults>::write_results::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/czkawka-c1570a3d7ff29c8c/eb8b91d/czkawka_core/src/tools/duplicate/traits.rs:347:18
   4: <czkawka_core::tools::duplicate::DuplicateFinder as czkawka_core::common::traits::PrintResults>::print_results_to_writer::<alloc::vec::Vec<u8>>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/czkawka-c1570a3d7ff29c8c/eb8b91d/czkawka_core/src/common/traits.rs:78:14
   5: <czkawka_core::tools::duplicate::DuplicateFinder as czkawka_core::common::traits::PrintResults>::print_results_to_writer::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/czkawka-c1570a3d7ff29c8c/eb8b91d/czkawka_core/src/common/traits.rs:76:5
   6: repro_czkawka_core_src_tools_duplicate_traits_rs_347_18_fb2e8b::main
             at ./src/main.rs:7:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
