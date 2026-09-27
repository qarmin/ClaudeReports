# core-fragments-float-division-truncation

`core/src/value/fragments.rs`'s `Fragments::new` (used to split a frame's raw pixel data into
DICOM encapsulated-pixel-data fragments of a given size) computes the number of fragments needed
using an `f32` division:

```rust
let number_of_fragments = (data.len() as f32 / fragment_size as f32).ceil() as u32;
```

`f32` has 24 bits of mantissa precision and cannot exactly represent every `usize`/`u32` value
once the input exceeds `2^24` (16,777,216) - for `data.len()` values near or past that
threshold, `.ceil()` can round to a result **one fragment short** of the correct integer
ceiling division. Since the crate then does `data.chunks_exact(fragment_size as usize)` to
produce the fragments, and `chunks_exact` silently discards any trailing bytes that don't form a
complete chunk, the shortfall isn't caught - it manifests as **silent, irrecoverable truncation
of the tail of the pixel data**, with no error, panic, or warning of any kind.

A ~16MB single frame (e.g. one slice of a 4096x4096 16-bit-per-pixel image, which alone is 32MB)
is an entirely ordinary size for real-world DICOM pixel data, not an extreme edge case.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-066): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/fragments-new-float-division` (commit `d2b7f8f7`) fixing this exact computation | Confirmed unmerged |
| Wrote a small standalone Rust program (`/tmp`, not part of this candidate's deliverable) to brute-force search for the smallest `data.len()` where the f32-based ceiling division diverges from the correct integer `div_ceil`, for `fragment_size = 2` | `rustc -O`, linear search from 16,000,000 | Found `len = 16_777_217` (`2^24 + 1`): correct fragment count is `8,388,609`, f32 computes `8,388,608` - exactly one fragment short |
| Temporary in-tree `#[test]` in `core/src/value/fragments.rs` (reverted before write-up): call `Fragments::new` with a 16,777,217-byte buffer and `fragment_size = 2`, sum the byte lengths of all produced fragments | `cargo test -p dicom-core fragments_new_large_data_truncation_repro --release` | Fragments cover only 16,777,216 bytes of the 16,777,217-byte input - the last byte is silently dropped, confirmed via `chunks_exact` discarding the incomplete trailing chunk since the (wrongly small) computed `encapsulated_size` was less than `data.len()`, so the usual zero-padding step never ran |
| Standalone `repro/` (path-dependency on a pristine clone of `df2b9e42`, via `just test`) | `cargo run --release` (release mode needed for the ~32MB of `Vec<u8>` allocations to run quickly) | Reproduces the same truncation deterministically; see `backtrace.txt` |

## Status

**Confirmed** - deterministic, reproduces every run for the specific input size identified (not
a probabilistic/fuzzed find - the f32 rounding behavior for this exact input is fully
deterministic per IEEE 754).

## Class

`logic` (silent data corruption/loss - not a crash - in a data path that real-world image data
routinely exercises; large multi-megapixel or multi-frame studies are common in DICOM, well
within reach of the `2^24`-byte threshold where `f32` precision breaks down).

## Root cause

`f32`'s 24-bit mantissa cannot exactly represent all integers once they exceed `2^24`; dividing
and rounding in `f32` for `data.len()` values near or past that threshold can produce a
fragment count that undercounts by one, and nothing downstream catches or corrects for this -
`chunks_exact` silently drops whatever doesn't fit evenly into the (wrongly small) fragment
count's worth of data.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Replace the `f32`-based ceiling division with exact integer ceiling division (`usize::div_ceil`,
falling back to `u32::MAX` in the (practically unreachable) case where the fragment count itself
would overflow `u32`):

```rust
let number_of_fragments =
    u32::try_from(data.len().div_ceil(fragment_size as usize)).unwrap_or(u32::MAX);
```

See `patch.diff` for the exact diff (scoped to `core/src/value/fragments.rs`; includes the
regression test kept in the `tests` module).

- `cargo build -p dicom-core`: builds cleanly with the fix applied.
- `cargo test -p dicom-core --lib --release` (against the patched pristine clone): **68 passed /
  0 failed** (67 pre-existing + this candidate's own new regression test), confirming zero
  regressions.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.
- `repro/` (`just test`): fails its own data-completeness assertion before the patch (1 byte
  silently dropped); passes cleanly (all bytes accounted for) after.
