# types/src/types.rs:38:9

- crate: https://github.com/zakarumych/gpu-descriptor @ 21e6b7526642ebf9abd48548434ba4599bf2190f
- kind: panic
- location: `types/src/types.rs:38:9`
- last API call: `gpu_descriptor_types::DescriptorTotalCount::total`
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
gpu-descriptor = { git = "https://github.com/zakarumych/gpu-descriptor", rev = "21e6b7526642ebf9abd48548434ba4599bf2190f" }
gpu-descriptor-types = { git = "https://github.com/zakarumych/gpu-descriptor", rev = "21e6b7526642ebf9abd48548434ba4599bf2190f" }
```

`src/main.rs`:

```rust
use gpu_descriptor_types::DescriptorTotalCount;

fn main() {
    let descriptor_total_count = DescriptorTotalCount { sampler: 0u32, combined_image_sampler: 0u32, sampled_image: 0u32, storage_image: 0u32, uniform_texel_buffer: 0u32, storage_texel_buffer: 0u32, uniform_buffer: 0u32, storage_buffer: 0u32, uniform_buffer_dynamic: 0u32, storage_buffer_dynamic: 0u32, input_attachment: 0u32, acceleration_structure: 0u32, inline_uniform_block_bytes: 1u32, inline_uniform_block_bindings: u32::MAX };
    let _ = descriptor_total_count.total();
}
```

Run with `cargo run` in this directory (package `repro_types_src_types_rs_38_9_91a66d`).

## Output

```
thread 'main' (1438487) panicked at /home/rafal/.cargo/git/checkouts/gpu-descriptor-e34fa4a3ed93a823/21e6b75/types/src/types.rs:38:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <gpu_descriptor_types::types::DescriptorTotalCount>::total
             at /home/rafal/.cargo/git/checkouts/gpu-descriptor-e34fa4a3ed93a823/21e6b75/types/src/types.rs:38:9
   4: repro_types_src_types_rs_38_9_91a66d::main
             at ./src/main.rs:5:36
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
