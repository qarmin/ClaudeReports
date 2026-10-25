# kurbo/src/bezpath.rs:902:42

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: panic
- location: `kurbo/src/bezpath.rs:902:42`
- last API call: `kurbo::simplify::SimplifyBezPath::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Can't start a segment on a ClosePath
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::PathEl;
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let path_el = PathEl::ClosePath;
    let _ = SimplifyBezPath::new(vec![path_el]);
}
```

Run with `cargo run` in this directory (package `repro_kurbo_src_bezpath_rs_902_42_6d59e5`).

## Output

```
thread 'main' (411481) panicked at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:902:42:
Can't start a segment on a ClosePath
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <kurbo::bezpath::Segments<alloc::vec::into_iter::IntoIter<kurbo::bezpath::PathEl>> as core::iter::traits::iterator::Iterator>::next::{closure#0}
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:902:42
   3: <core::option::Option<(kurbo::point::Point, kurbo::point::Point)>>::get_or_insert_with::<<kurbo::bezpath::Segments<alloc::vec::into_iter::IntoIter<kurbo::bezpath::PathEl>> as core::iter::traits::iterator::Iterator>::next::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1826:49
   4: <kurbo::bezpath::Segments<alloc::vec::into_iter::IntoIter<kurbo::bezpath::PathEl>> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:896:49
   5: <core::iter::adapters::map::Map<kurbo::bezpath::Segments<alloc::vec::into_iter::IntoIter<kurbo::bezpath::PathEl>>, <kurbo::simplify::SimplifyBezPath>::new<alloc::vec::Vec<kurbo::bezpath::PathEl>>::{closure#0}> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:107:19
   6: <alloc::vec::Vec<kurbo::simplify::SimplifyCubic> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<kurbo::simplify::SimplifyCubic, core::iter::adapters::map::Map<kurbo::bezpath::Segments<alloc::vec::into_iter::IntoIter<kurbo::bezpath::PathEl>>, <kurbo::simplify::SimplifyBezPath>::new<alloc::vec::Vec<kurbo::bezpath::PathEl>>::{closure#0}>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter_nested.rs:24:41
   7: <alloc::vec::Vec<kurbo::simplify::SimplifyCubic> as alloc::vec::spec_from_iter::SpecFromIter<kurbo::simplify::SimplifyCubic, core::iter::adapters::map::Map<kurbo::bezpath::Segments<alloc::vec::into_iter::IntoIter<kurbo::bezpath::PathEl>>, <kurbo::simplify::SimplifyBezPath>::new<alloc::vec::Vec<kurbo::bezpath::PathEl>>::{closure#0}>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter.rs:33:9
   8: <alloc::vec::Vec<kurbo::simplify::SimplifyCubic> as core::iter::traits::collect::FromIterator<kurbo::simplify::SimplifyCubic>>::from_iter::<core::iter::adapters::map::Map<kurbo::bezpath::Segments<alloc::vec::into_iter::IntoIter<kurbo::bezpath::PathEl>>, <kurbo::simplify::SimplifyBezPath>::new<alloc::vec::Vec<kurbo::bezpath::PathEl>>::{closure#0}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3996:9
   9: <core::iter::adapters::map::Map<kurbo::bezpath::Segments<alloc::vec::into_iter::IntoIter<kurbo::bezpath::PathEl>>, <kurbo::simplify::SimplifyBezPath>::new<alloc::vec::Vec<kurbo::bezpath::PathEl>>::{closure#0}> as core::iter::traits::iterator::Iterator>::collect::<alloc::vec::Vec<kurbo::simplify::SimplifyCubic>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  10: <kurbo::simplify::SimplifyBezPath>::new::<alloc::vec::Vec<kurbo::bezpath::PathEl>>
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/simplify.rs:168:14
  11: repro_kurbo_src_bezpath_rs_902_42_6d59e5::main
             at ./src/main.rs:6:13
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
