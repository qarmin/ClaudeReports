# out of memory in fraction::division::divide_to_string[u16]

- crate: https://github.com/dnsl48/fraction @ 53d04cddb4640d1fe7756c289b3ce34be14da504
- kind: out of memory
- last API call: `fraction::division::divide_to_string[u16]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling fraction::division::divide_to_string[u16]
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fraction = { git = "https://github.com/dnsl48/fraction", rev = "53d04cddb4640d1fe7756c289b3ce34be14da504" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = fraction::division::divide_to_string::<u16>(0u16, 0u16, 1000000000000000000usize, false);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_fraction_division_divide_to_string_u16_af1fab`).

## Output

```
memory allocation of 1000000000000000002 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   6: <alloc::raw_vec::RawVec<u8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<u8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <alloc::vec::Vec<u8>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   9: <alloc::string::String>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:494:23
  10: fraction::division::divide_to_string::<u16>
             at /home/rafal/.cargo/git/checkouts/fraction-22db6e5452e03bd8/53d04cd/src/division.rs:448:22
  11: repro_out_of_memory_in_fraction_division_divide_to_string_u16_af1fab::main
             at ./src/main.rs:2:13
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
