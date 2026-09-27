# encoding-uninhabited-adapter-unreachable-safe

A fresh `grep -rn "unreachable!()" --include="*.rs" .` sweep of the whole workspace (per
`projects/dicom-rs.md`'s note that the original 23-occurrence inventory from the 2026-08-24
session was fully exhausted and a new sweep should be run from scratch, since this repo merges
multiple times a day) found 32 occurrences, 3 files of which were not covered by either session:
`core/src/header.rs` (1 site), `encoding/src/adapters.rs` (8 sites), `encoding/src/
transfer_syntax/mod.rs` (2 sites) - 11 sites total, all checked together in this candidate since
they share the exact same root cause and verification method.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning: read each site's enclosing `impl` block and the type it's implemented for | Static code reading, no build needed | All 11 sites are trait method implementations (`PixelDataReader`/`PixelDataWriter` for `encoding/src/adapters.rs`'s `NeverPixelAdapter` and re-exported `crate::transfer_syntax::NeverAdapter`; `DataRWAdapter` for `encoding/src/transfer_syntax/mod.rs`'s `NeverAdapter`; `HasLength` for `core/src/header.rs`'s `EmptyObject`) implemented for a **field-less, variant-less `enum` type** (`pub enum NeverPixelAdapter {}`, `pub enum NeverAdapter {}`, `pub enum EmptyObject {}`) - an uninhabited type in Rust's type system, which by construction can never have a value instantiated. Every method taking `&self` on such a type is therefore permanently unreachable: there is no way to call it, since no caller can ever produce a `Self` to call it on. This is the exact "uninhabited-type construction... used as a 'this can never be instantiated' marker" pattern already documented as safe in `projects/dicom-rs.md` from the first session's own `unreachable!()` sweep, now confirmed to extend to three more files under the same reasoning |

## Status

**Not reproducible** - ruled out by type-system reasoning alone (no build needed): each site's
enclosing type is uninhabited, so the method can never be called by any well-typed Rust program,
safe or unsafe (barring `unsafe` transmutation tricks to conjure a value out of thin air, which is
UB regardless and not a "confirmed bug" in this crate's own code).

## Class

n/a

## Root cause

Not applicable (no bug) - these are deliberate, idiomatic uses of Rust's uninhabited-type pattern
(`enum Foo {}`) to mark "this adapter is a compile-time placeholder, never a real value" for
transfer syntaxes that don't need a pixel data codec or a reader/writer adapter. The `unreachable!()`
bodies exist only to satisfy the trait's required method signatures; they document (and would
catch, via a controlled panic rather than silent UB) a violation of the type system's own
guarantee if one were ever somehow introduced (e.g. by an unsound `unsafe` change elsewhere), but
under normal, safe Rust usage they can never execute.

## The fix

N/A - no bug found, no fix needed.

## What's not done / open questions

- This exhausts the *new* files found by this session's fresh `unreachable!()` sweep beyond what
  candidates `027`-`031` already covered. The full 32-occurrence current inventory (`json/src/
  de/mod.rs`, `transfer-syntax-registry/src/adapters/jpegxl.rs`, `object/src/collector.rs`,
  `core/src/header.rs`, `object/src/meta.rs`, `object/src/mem.rs`, `core/src/value/range.rs`,
  `parser/src/stateful/encode.rs`, `parser/src/dataset/mod.rs`, `parser/src/stateful/decode.rs`,
  `findscu/src/main.rs`, `encoding/src/adapters.rs`, `encoding/src/transfer_syntax/mod.rs`) is now
  believed fully accounted for across both sessions: `json/de/mod.rs`, `jpegxl.rs`, `collector.rs`,
  `range.rs`, `decode.rs` were candidates `027`-`031`; `object/src/meta.rs` and `object/src/mem.rs`
  and `parser/src/stateful/encode.rs` and `parser/src/dataset/mod.rs` were candidates `023`-`026`
  from the 2026-08-24 session; `findscu/src/main.rs`'s two sites were confirmed CLI-argument-driven
  (not remote/untrusted-data-driven) by a quick read during this session's CLI-tools discovery pass,
  not written up as their own candidate since they're trivially local-only and outside this skill's
  untrusted-input focus. A *future* fresh grep should still be re-run rather than trusting this
  list indefinitely, per the same reasoning noted in `projects/dicom-rs.md` already.
