# out of memory in kurbo::BezPath::as_line

- crate: kurbo 0.13.1
- kind: out of memory
- last API call: `kurbo::BezPath::as_line`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling kurbo::BezPath::as_line
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { version = "=0.13.1" }
```

`src/main.rs`:

```rust
use kurbo::Triangle;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::as_line(&<kurbo::CircleSegment as kurbo::Shape>::to_path(&Triangle::from_coords((0.0f64, 0.0f64), (0.0f64, 0.0f64), (0.0f64, 0.0f64)).inscribed_circle().segment(1.0f64, 1.0f64, 1.0f64), 0.0f64));
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_kurbo_bezpath_as_line_c8cc2a`).

## Output

```
memory allocation of 3758096384 bytes failed
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
   5: <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
   6: <alloc::raw_vec::RawVecInner>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:672:17
   7: <alloc::raw_vec::RawVec<kurbo::bezpath::PathEl>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:341:29
   8: <alloc::vec::Vec<kurbo::bezpath::PathEl>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1461:18
   9: <alloc::vec::Vec<kurbo::bezpath::PathEl>>::extend_desugared::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4104:22
  10: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_extend::SpecExtend<kurbo::bezpath::PathEl, core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>>>::spec_extend
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:18:14
  11: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<kurbo::bezpath::PathEl, core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter_nested.rs:41:9
  12: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_from_iter::SpecFromIter<kurbo::bezpath::PathEl, core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter.rs:33:9
  13: <alloc::vec::Vec<kurbo::bezpath::PathEl> as core::iter::traits::collect::FromIterator<kurbo::bezpath::PathEl>>::from_iter::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3996:9
  14: <core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter> as core::iter::traits::iterator::Iterator>::collect::<alloc::vec::Vec<kurbo::bezpath::PathEl>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  15: <kurbo::bezpath::BezPath as core::iter::traits::collect::FromIterator<kurbo::bezpath::PathEl>>::from_iter::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/kurbo-0.13.1/src/bezpath.rs:509:47
  16: <core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter> as core::iter::traits::iterator::Iterator>::collect::<kurbo::bezpath::BezPath>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  17: <kurbo::circle::CircleSegment as kurbo::shape::Shape>::to_path
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/kurbo-0.13.1/src/shape.rs:68:39
  18: repro_out_of_memory_in_kurbo_bezpath_as_line_c8cc2a::main
             at ./src/main.rs:4:56
  19: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
