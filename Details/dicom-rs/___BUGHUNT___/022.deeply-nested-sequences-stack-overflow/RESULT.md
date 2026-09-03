# Unbounded recursion depth in InMemDicomObject's data set builder: stack overflow crash on deeply nested sequences

Found during a fresh discovery round (no seeded lead): after a run of numeric/parsing-domain
bugs (i32 overflow in `021`, off-by-one truncation in `020`), I switched to a structurally
different question - not "is a computed value wrong", but "is there anything in this codebase
whose resource usage (stack, not heap) scales with a value the input file fully controls, with
no cap". `object/src/mem.rs`'s `InMemDicomObject::build_object` and `build_sequence` are mutually
recursive: `build_object` calls `build_sequence` on `DataToken::SequenceStart`, and
`build_sequence` calls `build_object` again on `DataToken::ItemStart`, once per nested data set
item. Nothing bounds how many times this can recurse - it is driven entirely by how many
`SQ`-within-`Item`-within-`SQ`... levels the input data set actually contains, which an attacker
fully controls. A data set with enough nesting overflows the native call stack.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `object/src/mem.rs`'s `build_object` (SequenceStart branch calls `Self::build_sequence(...)`) and `build_sequence` (ItemStart branch calls `Self::build_object(...)`), both called from every top-level entry point (`read_dataset_with_dict_ts_cs`, `from_reader`/`open_file`'s codec paths, etc.) with no depth parameter or limit anywhere in the call chain. | Identified the unbounded mutual recursion; no depth check anywhere in `object`, `parser`, or `encoding` crates that would reject a data set for being too deeply nested. |
| Standalone repro (release build, public API: `dicom_object::InMemDicomObject::read_dataset_with_ts`) | `___BUGHUNT___/022.../repro`. Programmatically built an Explicit VR Little Endian dataset consisting of *N* levels of `SQ -> Item -> SQ -> Item -> ... -> (empty item)` (20 bytes of overhead per level: a 12-byte SQ element header wrapping an 8-byte item header), then parsed it. | With `N = 20000` (a 400,000-byte file), the process reliably aborts: `thread 'main' has overflowed its stack / fatal runtime error: stack overflow, aborting` (exit code 134). Binary-searched the actual threshold on this machine: `N = 5000` did not crash, `N = 8000` already did - so ~6000-8000 levels is enough on an 8 MiB default stack; `N = 20000` was chosen for the repro to give comfortable margin over stack-size variance across machines/CI. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` (`N = 20000` default) | 3/3 runs reproduced identically (deterministic - the same fixed input always recurses to the same depth). |
| gdb | `gdb -batch -ex run -ex "bt 20" --args ./target/release/repro 20000` | Confirmed the crash is a `SIGSEGV` hit while executing `<InMemDicomObject>::build_object`, with the backtrace showing the same frame (`build_object`) repeating identically - i.e., genuine unbounded recursion, not some unrelated issue coincidentally triggered by a large input. See `backtrace.txt`. |
| Reachability check | Every public entry point that parses a data set (`InMemDicomObject::read_dataset*`, and transitively `dicom_object::open_file`/`from_reader`, and `storescp`'s association handling which calls `InMemDicomObject::read_dataset_with_ts` on received P-Data) funnels through `build_object`. | N/A (confirms this is reachable from ordinary file loading *and* from a DICOM network listener processing an incoming C-STORE, i.e. this is remotely triggerable, not just a local file-parsing concern). |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, plus an independent gdb-backed backtrace
confirming the exact recursive call site.

## Class

`crash` (unbounded stack depth driven by untrusted input structure - a classic recursive-descent-
parser DoS, reachable both by loading a crafted local file and by any DICOM service (e.g.
`storescp`, or any application built on this crate acting as an SCP) receiving a crafted
C-STORE/C-FIND/C-GET response data set over the network).

## Root cause

`InMemDicomObject::build_object` (data set / item level) and `build_sequence` (sequence level) in
`object/src/mem.rs` recurse into each other once per level of `SQ`/item nesting in the data set
being parsed, with no depth counter or limit anywhere in the call chain. Real-world DICOM data
sets rarely nest more than a handful of levels deep (Structured Reports being the deepest common
case, and even those are typically under 20 levels), so this was never exercised by realistic
input - but nothing in the format or the parser enforces that assumption, and a data set is
trivially crafted (20 bytes per extra level of nesting) to recurse however deep an attacker likes.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Threaded an explicit `depth: u32` counter through the recursive `build_object`/`build_sequence`
pair (via a new internal `build_object_impl`, keeping the original `build_object` signature/call
sites as a thin `depth = 0` entry point so no external or test call site needed to change), and
added a `MAX_SEQUENCE_DEPTH = 1024` constant checked at the top of `build_object_impl`. Exceeding
the limit now returns a new `ReadError::NestingTooDeep` (via a new snafu context selector,
`object/src/lib.rs`) instead of recursing further. 1024 levels is far beyond any legitimate DICOM
data set's nesting depth while still comfortably bounding stack usage. See `patch.diff`
(`object/src/lib.rs`, `object/src/mem.rs`).

Verified:
- `repro/` re-run against the patched tree no longer crashes: `Data set sequences are nested too
  deeply (limit is 1024)`, exit code 0.
- Boundary check: 1024 levels still parses successfully (no false positive at the limit itself);
  1025 levels is rejected gracefully with the same error.
- 10-level input (typical/realistic nesting) still parses successfully - no regression for normal
  DICOM data.
- Project test suite: `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored, identical
  both before and after the patch (no regressions; no existing test data set nests anywhere close
  to 1024 levels).

## What's not done / open questions

- Only the `InMemDicomObject` builder path (`object` crate) was fixed. The lower-level
  `dicom-parser` `DataSetReader`/`LazyDataSetReader` token stream itself is iterative (it
  maintains its own nesting state as a `Vec` of `SeqToken`, not via native recursion), so a
  consumer that processes the token stream directly (rather than going through
  `InMemDicomObject::build_object`) would not hit this particular stack-overflow mechanism - but
  such a consumer could still choose to build its own recursive structure over an unbounded
  number of tokens, which is a design concern for API consumers rather than a bug in this crate.
  This was not separately investigated as a distinct candidate.
- Did not check whether `dicom-json`'s DICOM+JSON deserialization path (which also has to handle
  nested sequences, from JSON input) has an analogous unbounded-recursion concern; JSON's own
  nesting is typically bounded by `serde_json`'s recursion limit, so this is lower-priority but
  unconfirmed either way.
- Not fuzzed - found by directly reasoning about resource-usage classes (stack vs. heap) that
  scale with attacker-controlled *structure* rather than attacker-controlled *values*, a category
  not covered by any of this session's other candidates (which were all about the size/magnitude
  of values, not the shape of the token stream).
