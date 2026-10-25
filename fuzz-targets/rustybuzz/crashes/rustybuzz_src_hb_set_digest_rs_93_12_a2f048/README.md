# rustybuzz/src/hb/set_digest.rs:93:12

- crate: https://github.com/harfbuzz/rustybuzz @ 9faca967408677f17bc15366dbdc3d2b683d1489
- kind: panic
- location: `rustybuzz/src/hb/set_digest.rs:93:12`
- last API call: `rustybuzz::Face::from_slice`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 13225819301787280044

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rustybuzz = { git = "https://github.com/harfbuzz/rustybuzz", rev = "9faca967408677f17bc15366dbdc3d2b683d1489" }
```

`src/main.rs`:

```rust
use rustybuzz::Face;

fn main() {
    let vec = include_bytes!("data_0.bin").to_vec();
    let _ = Face::from_slice(&vec, 0u32);
}
```

Run with `cargo run` in this directory (package `repro_rustybuzz_src_hb_set_digest_rs_93_12_a2f048`).

## Output

```
thread 'main' (46883) panicked at /home/rafal/.cargo/git/checkouts/rustybuzz-d2dfef5e44134484/9faca96/src/hb/set_digest.rs:93:12:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <rustybuzz::hb::set_digest::hb_set_digest_bits_pattern_t<0> as rustybuzz::hb::set_digest::hb_set_digest_ext>::add_range
             at /home/rafal/.cargo/git/checkouts/rustybuzz-d2dfef5e44134484/9faca96/src/hb/set_digest.rs:93:12
   4: <rustybuzz::hb::set_digest::hb_set_digest_combiner_t<rustybuzz::hb::set_digest::hb_set_digest_bits_pattern_t<0>, rustybuzz::hb::set_digest::hb_set_digest_bits_pattern_t<9>> as rustybuzz::hb::set_digest::hb_set_digest_ext>::add_range
             at /home/rafal/.cargo/git/checkouts/rustybuzz-d2dfef5e44134484/9faca96/src/hb/set_digest.rs:158:31
   5: <rustybuzz::hb::set_digest::hb_set_digest_combiner_t<rustybuzz::hb::set_digest::hb_set_digest_bits_pattern_t<4>, rustybuzz::hb::set_digest::hb_set_digest_combiner_t<rustybuzz::hb::set_digest::hb_set_digest_bits_pattern_t<0>, rustybuzz::hb::set_digest::hb_set_digest_bits_pattern_t<9>>> as rusty... (+51 chars)
             at /home/rafal/.cargo/git/checkouts/rustybuzz-d2dfef5e44134484/9faca96/src/hb/set_digest.rs:159:32
   6: <ttf_parser::ggg::Coverage as rustybuzz::hb::ot_layout_common::CoverageExt>::collect
             at /home/rafal/.cargo/git/checkouts/rustybuzz-d2dfef5e44134484/9faca96/src/hb/ot_layout_common.rs:49:32
   7: <rustybuzz::hb::ot_layout_common::SubstLookup>::parse
             at /home/rafal/.cargo/git/checkouts/rustybuzz-d2dfef5e44134484/9faca96/src/hb/ot_layout_common.rs:116:33
   8: <<rustybuzz::hb::ot_layout_common::SubstLookup>::parse as core::ops::function::FnMut<(ttf_parser::ggg::lookup::Lookup,)>>::call_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:166:5
   9: <&mut <rustybuzz::hb::ot_layout_common::SubstLookup>::parse as core::ops::function::FnOnce<(ttf_parser::ggg::lookup::Lookup,)>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:310:21
  10: <core::option::Option<ttf_parser::ggg::lookup::Lookup>>::map::<rustybuzz::hb::ot_layout_common::SubstLookup, &mut <rustybuzz::hb::ot_layout_common::SubstLookup>::parse>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1163:29
  11: <core::iter::adapters::map::Map<ttf_parser::parser::LazyOffsetArrayIter16<ttf_parser::ggg::lookup::Lookup>, <rustybuzz::hb::ot_layout_common::SubstLookup>::parse> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:107:26
  12: <alloc::vec::Vec<rustybuzz::hb::ot_layout_common::SubstLookup>>::extend_desugared::<core::iter::adapters::map::Map<ttf_parser::parser::LazyOffsetArrayIter16<ttf_parser::ggg::lookup::Lookup>, <rustybuzz::hb::ot_layout_common::SubstLookup>::parse>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4100:44
  13: <alloc::vec::Vec<rustybuzz::hb::ot_layout_common::SubstLookup> as alloc::vec::spec_extend::SpecExtend<rustybuzz::hb::ot_layout_common::SubstLookup, core::iter::adapters::map::Map<ttf_parser::parser::LazyOffsetArrayIter16<ttf_parser::ggg::lookup::Lookup>, <rustybuzz::hb::ot_layout_common::Subst... (+30 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:18:14
  14: <alloc::vec::Vec<rustybuzz::hb::ot_layout_common::SubstLookup> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<rustybuzz::hb::ot_layout_common::SubstLookup, core::iter::adapters::map::Map<ttf_parser::parser::LazyOffsetArrayIter16<ttf_parser::ggg::lookup::Lookup>, <rustybuzz::hb::ot_la... (+46 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter_nested.rs:41:9
  15: <alloc::vec::Vec<rustybuzz::hb::ot_layout_common::SubstLookup> as alloc::vec::spec_from_iter::SpecFromIter<rustybuzz::hb::ot_layout_common::SubstLookup, core::iter::adapters::map::Map<ttf_parser::parser::LazyOffsetArrayIter16<ttf_parser::ggg::lookup::Lookup>, <rustybuzz::hb::ot_layout_common::... (+33 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter.rs:33:9
  16: <alloc::vec::Vec<rustybuzz::hb::ot_layout_common::SubstLookup> as core::iter::traits::collect::FromIterator<rustybuzz::hb::ot_layout_common::SubstLookup>>::from_iter::<core::iter::adapters::map::Map<ttf_parser::parser::LazyOffsetArrayIter16<ttf_parser::ggg::lookup::Lookup>, <rustybuzz::hb::ot_... (+36 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3996:9
  17: <core::iter::adapters::map::Map<ttf_parser::parser::LazyOffsetArrayIter16<ttf_parser::ggg::lookup::Lookup>, <rustybuzz::hb::ot_layout_common::SubstLookup>::parse> as core::iter::traits::iterator::Iterator>::collect::<alloc::vec::Vec<rustybuzz::hb::ot_layout_common::SubstLookup>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  18: <rustybuzz::hb::ot_layout_common::SubstitutionTable>::new
             at /home/rafal/.cargo/git/checkouts/rustybuzz-d2dfef5e44134484/9faca96/src/hb/ot_layout_common.rs:91:73
  19: <<rustybuzz::hb::ot_layout_common::SubstitutionTable>::new as core::ops::function::FnOnce<(ttf_parser::ggg::layout_table::LayoutTable,)>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  20: <core::option::Option<ttf_parser::ggg::layout_table::LayoutTable>>::map::<rustybuzz::hb::ot_layout_common::SubstitutionTable, <rustybuzz::hb::ot_layout_common::SubstitutionTable>::new>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1163:29
  21: <rustybuzz::hb::face::hb_font_t>::from_face
             at /home/rafal/.cargo/git/checkouts/rustybuzz-d2dfef5e44134484/9faca96/src/hb/face.rs:88:38
  22: <rustybuzz::hb::face::hb_font_t>::from_slice
             at /home/rafal/.cargo/git/checkouts/rustybuzz-d2dfef5e44134484/9faca96/src/hb/face.rs:76:14
  23: repro_rustybuzz_src_hb_set_digest_rs_93_12_a2f048::main
             at ./src/main.rs:5:13
  24: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
