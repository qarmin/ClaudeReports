# Lut::new_with_fn's own BitsStored=32 boundary silently corrupts (release) or panics (debug)

From `IDEAS.md`'s backlog, noticed while working candidate 009 (`009.bits-stored-zero-panic`):
`pixeldata/src/lut.rs`'s `Lut::new_with_fn` computes `let size = (1 << bits_stored as u32) as
usize;` - the shift happens in the untyped literal `1`'s default-inferred type (`i32`) *before*
the `as usize` cast, and `bits_stored == 32` is explicitly within the function's own asserted-valid
range (`assert!(bits_stored != 0 && bits_stored <= 32)`) yet shifts a 32-bit value by 32 bits,
which is out of range for that type. Candidate 009's own fix (restricting the DICOM-file-parsing
`bits_stored()` accessor to `1..=32`) explicitly still let `32` itself through, so this boundary
remained live even after that fix.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Isolated toy reproduction (release build, plain Rust, no project dependencies) | A minimal standalone function reproducing exactly `pixeldata/src/lut.rs`'s shift expression, called with a runtime (non-const, so not caught at compile time) `bits_stored: u16 = 32`. | Confirmed the two-faced symptom directly: **debug build** panics (`attempt to shift left with overflow`); **release build** silently wraps, computing `size = 1` instead of the intended `4294967296` - no error, no panic, just a silently wrong result. |
| Standalone repro (release AND debug builds, `dicom_pixeldata`'s public API: `Lut::new_rescale`, `Lut::get`) | `___BUGHUNT___/010.../repro`. Built `Lut::<u16>::new_rescale(32, false, Rescale::new(1.0, 0.0))` (bits_stored=32, an identity rescale so any two different inputs should legitimately produce two different outputs), then called `.get()` on three widely different sample values (0, 12345, 65535). | Reproduced deterministically in the real project code (not just the toy function): all three calls returned `0` - the LUT's `table.len()` silently wrapped to `1` (matching the toy repro exactly), and `sample_mask` (derived as `table.len() as u32 - 1 = 0`) then masked every input down to index 0 in `Lut::get`, so the LUT is silently degenerate - every distinct pixel sample value collapses to the exact same output, with no error and no panic in a release build. A debug build instead panics with `attempt to shift left with overflow` at construction time. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` (pre-fix) | 3/3 runs reproduced identically - deterministic, no randomness in this code path. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed (also independently
confirmed in a debug build, showing the alternate panic-instead-of-corrupt symptom).

## Class

`logic` (silent, severe data corruption in release builds - the *entire rendered image* collapses
to a single constant pixel value with no error signal at all) and, separately in debug builds,
`crash` (an unguarded integer-shift overflow panic - not the `assert!` a few lines earlier, a
*different* panic from the very next statement, which is even more surprising since it fires
despite passing the function's own stated precondition check).

## Root cause

`Lut::new_with_fn`'s `assert!(bits_stored != 0 && bits_stored <= 32)` and the very next line's
`(1 << bits_stored as u32) as usize` don't actually agree with each other: the assert accepts
`bits_stored == 32`, but the shift computes in `1`'s default-inferred `i32` type (Rust does not
propagate the trailing `as usize` cast backwards into the literal's type), and shifting a 32-bit
integer type by exactly its own bit width is out of range - undefined territory that Rust's own
overflow-checked-arithmetic model treats as an error, checked only in debug builds (`RUSTFLAGS`
default `overflow-checks` is on for `dev`, off for `release`). In release builds, the shift instead
wraps (LLVM's `shl` on out-of-range shift amounts for a fixed-width type is itself technically UB
at the LLVM IR level in unchecked-arithmetic builds, but observably manifests here as the shift
amount being taken modulo the type's bit width - `1i32 << 32` behaves as `1i32 << 0 = 1`),
producing a nonsensical `table` of length 1. `sample_mask` is computed as `table.len() as u32 - 1`
- correctly derived *from the actual (wrongly small) table*, so `Lut::get`'s masking never causes
an out-of-bounds panic downstream (a small mercy: the bug fully self-corrects into "quietly always
returns the one entry" rather than compounding into a second, separate crash), but the LUT is then
completely useless - every input collapses to `table[0]`.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Two complementary changes, layered on top of candidate 009's already-applied fix (this patch
includes both):

1. **`pixeldata/src/lut.rs`** (the actual root cause, protects *every* caller including direct use
   of the public `Lut` API, not just DICOM-object parsing): tightened `new_with_fn`'s own asserted
   bound from `bits_stored <= 32` to `bits_stored <= 16` - a real DICOM image never legitimately
   needs more for this lookup-table-based transform (this only handles `BitsAllocated` of 8 or
   16; floating-point pixel data, which can be 32-bit, goes through a different, non-LUT code path
   entirely) - and simplified the size computation to `1usize << bits_stored`, which can no longer
   overflow for any value the (now-tighter) assert permits.
2. **`pixeldata/src/attribute.rs`** (defense-in-depth for the untrusted-file-parsing path
   specifically, matching candidate 009's own reasoning): tightened `bits_stored()`'s own range
   check from `1..=32` to `1..=16` too, so a malformed file's `BitsStored=32` is rejected with a
   clean `Result::Err` *before* ever reaching `Lut::new_with_fn` at all, rather than relying on
   that function's own (now also fixed, but still panic-based for direct API misuse) assertion.

See `patch.diff` (bundled as the full diff on top of candidate 009's fix - `pixeldata/src/lut.rs`
and `pixeldata/src/attribute.rs`). Verified: `repro/` re-run against the patched tree no longer
silently produces a degenerate table - `Lut::new_rescale(32, ...)` now panics immediately with a
clear, documented assertion message (`bits_stored != 0 && bits_stored <= 16`) instead of silently
succeeding with wrong data; the repro treats this panic as the correct, intentional outcome (this
is the same accepted "documented precondition, panics on direct API misuse" pattern already
established for `VoiLutTransform::new` in candidate 006's investigation) rather than as a residual
bug, since the object-parsing pipeline itself (the actually untrusted-input-facing path) is
separately guarded by the `bits_stored()` accessor fix and never reaches this assert at all for a
malformed file.

- Project test suite: `cargo test -p dicom-pixeldata --lib` - 39 passed / 0 failed / 2 ignored,
  identical both before and after the patch. `cargo test --workspace --lib` - 351 passed / 0
  failed / 2 ignored, identical both before and after (no regressions, no newly-passing tests - no
  existing test constructed a `BitsStored` value at or near this boundary).

## What's not done / open questions

- Only the DICOM-object-parsing path (`bits_stored()`) and the direct `Lut` construction API were
  checked. `pixeldata/src/attribute.rs` has other `u16`-typed attribute accessors
  (`bits_allocated`, `high_bit`, etc.) that weren't audited for similar missing range validation as
  part of this candidate - `bits_stored` was chosen specifically because it's the one directly
  driving `Lut`'s table-size computation.
- The `1..=16` bound chosen for both fixes is based on this LUT-table-based renderer's own
  realistic usage (BitsAllocated of 8 or 16), not on an exhaustive read of every DICOM SOP class's
  own permitted `BitsStored` range across the full standard - if a legitimate DICOM use case
  requiring `BitsStored` between 17 and 32 exists that's still meant to flow through this exact
  code path (as opposed to a genuinely different floating-point/high-bit-depth pixel data
  handling path), this fix would incorrectly reject it. Nothing in this session's reading of
  `pixeldata/src/lib.rs`'s call sites suggested such a case exists for the `Lut`-based renderer
  specifically.
