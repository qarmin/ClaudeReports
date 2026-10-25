# svgtypes/src/error.rs:73:58

- crate: https://github.com/linebender/svgtypes @ 3ee4db5bcbfe8db817d87b55a3ae4f889bf2d3fa
- kind: panic
- location: `svgtypes/src/error.rs:73:58`
- last API call: `svgtypes::Error::to_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [197], error: Utf8Error { valid_up_to: 0, error_len: None } }
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
svgtypes = { git = "https://github.com/linebender/svgtypes", rev = "3ee4db5bcbfe8db817d87b55a3ae4f889bf2d3fa" }
```

`src/main.rs`:

```rust
use svgtypes::Error;

fn main() {
    let error = Error::from(Error::InvalidChar(vec![32u8, 197u8], 0usize));
    let _ = error.to_string();
}
```

Run with `cargo run` in this directory (package `repro_svgtypes_src_error_rs_73_58_7860fa`).

## Output

```
thread 'main' (489114) panicked at /home/rafal/.cargo/git/checkouts/svgtypes-da914f7dbc9e8d80/3ee4db5/src/error.rs:73:58:
called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [197], error: Utf8Error { valid_up_to: 0, error_len: None } }
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<alloc::string::String, alloc::string::FromUtf8Error>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}
             at /home/rafal/.cargo/git/checkouts/svgtypes-da914f7dbc9e8d80/3ee4db5/src/error.rs:73:58
   5: core::iter::adapters::map::map_fold::<&u8, alloc::string::String, (), <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}, core::iter::traits::iterator::Iterator::for_each::call<alloc::string::String, <alloc::vec::Vec<alloc::string::String>>::extend_trusted<core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}>>::{closure#0}>::{closure#0}>::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:88:28
   6: <core::slice::iter::Iter<u8> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<&u8, alloc::string::String, (), <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}, core::iter::traits::iterator::Iterator::for_each::call<alloc::string::String, <alloc::vec::Vec<alloc::string::String>>::extend_trusted<core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}>>::{closure#0}>::{closure#0}>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:274:27
   7: <core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<&u8, alloc::string::String, (), <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}, core::iter::traits::iterator::Iterator::for_each::call<alloc::string::String, <alloc::vec::Vec<alloc::string::String>>::extend_trusted<core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}>>::{closure#0}>::{closure#0}>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/skip.rs:132:19
   8: <core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::traits::iterator::Iterator::for_each::call<alloc::string::String, <alloc::vec::Vec<alloc::string::String>>::extend_trusted<core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}>>::{closure#0}>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:128:19
   9: <core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}> as core::iter::traits::iterator::Iterator>::for_each::<<alloc::vec::Vec<alloc::string::String>>::extend_trusted<core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}>>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:887:14
  10: <alloc::vec::Vec<alloc::string::String>>::extend_trusted::<core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4132:26
  11: <alloc::vec::Vec<alloc::string::String> as alloc::vec::spec_extend::SpecExtend<alloc::string::String, core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}>>>::spec_extend
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:27:14
  12: <alloc::vec::Vec<alloc::string::String> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<alloc::string::String, core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter_nested.rs:60:16
  13: <alloc::vec::Vec<alloc::string::String> as alloc::vec::spec_from_iter::SpecFromIter<alloc::string::String, core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter.rs:33:9
  14: <alloc::vec::Vec<alloc::string::String> as core::iter::traits::collect::FromIterator<alloc::string::String>>::from_iter::<core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3996:9
  15: <core::iter::adapters::map::Map<core::iter::adapters::skip::Skip<core::slice::iter::Iter<u8>>, <svgtypes::error::Error as core::fmt::Display>::fmt::{closure#0}> as core::iter::traits::iterator::Iterator>::collect::<alloc::vec::Vec<alloc::string::String>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  16: <svgtypes::error::Error as core::fmt::Display>::fmt
             at /home/rafal/.cargo/git/checkouts/svgtypes-da914f7dbc9e8d80/3ee4db5/src/error.rs:74:22
  17: <svgtypes::error::Error as alloc::string::SpecToString>::spec_to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2952:9
  18: <svgtypes::error::Error as alloc::string::ToString>::to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2931:9
  19: repro_svgtypes_src_error_rs_73_58_7860fa::main
             at ./src/main.rs:5:19
  20: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
