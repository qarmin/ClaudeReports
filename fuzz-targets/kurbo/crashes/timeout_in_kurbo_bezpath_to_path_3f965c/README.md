# timeout in kurbo::BezPath::to_path

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::BezPath::to_path`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 15083805307320109301

## Message

```
process ended abnormally (timeout) while calling kurbo::BezPath::to_path
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::Axis;
use kurbo::CircleSegment;

fn main() {
    let axis = Axis::Horizontal;
    let point = axis.pack_point(0.0f64, 0.0f64);
    let circle_segment = CircleSegment::new(point, 0.0f64, 0.0f64, 0.0f64, 1.3270698809889517e78f64);
    let arc = circle_segment.inner_arc();
    let _ = <kurbo::BezPath as kurbo::Shape>::to_path(&<kurbo::Arc as kurbo::Shape>::into_path(arc, 0.0f64), 0.0f64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_bezpath_to_path_3f965c`).

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
   9: <alloc::vec::Vec<kurbo::bezpath::PathEl>>::extend_desugared::<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, kurbo::arc::ArcAppendIter>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4104:22
  10: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_extend::SpecExtend<kurbo::bezpath::PathEl, core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, kurbo::arc::ArcAppendIter>>>::spec_extend
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:18:14
  11: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<kurbo::bezpath::PathEl, core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, kurbo::arc::ArcAppendIter>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter_nested.rs:41:9
  12: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_from_iter::SpecFromIter<kurbo::bezpath::PathEl, core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, kurbo::arc::ArcAppendIter>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter.rs:33:9
  13: <alloc::vec::Vec<kurbo::bezpath::PathEl> as core::iter::traits::collect::FromIterator<kurbo::bezpath::PathEl>>::from_iter::<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, kurbo::arc::ArcAppendIter>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3996:9
  14: <core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, kurbo::arc::ArcAppendIter> as core::iter::traits::iterator::Iterator>::collect::<alloc::vec::Vec<kurbo::bezpath::PathEl>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  15: <kurbo::bezpath::BezPath as core::iter::traits::collect::FromIterator<kurbo::bezpath::PathEl>>::from_iter::<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, kurbo::arc::ArcAppendIter>>
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:509:47
  16: <core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, kurbo::arc::ArcAppendIter> as core::iter::traits::iterator::Iterator>::collect::<kurbo::bezpath::BezPath>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  17: <kurbo::arc::Arc as kurbo::shape::Shape>::to_path
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/shape.rs:68:39
  18: <kurbo::arc::Arc as kurbo::shape::Shape>::into_path
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/shape.rs:83:14
  19: repro_timeout_in_kurbo_bezpath_to_path_3f965c::main
             at ./src/main.rs:9:56
  20: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
