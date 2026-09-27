# CIFF (Canon CRW) entry parsing panics on an out-of-range attacker-controlled data slice

`rawler::formats::ciff::CiffEntry::new` slices the whole file buffer at a `data_offset..data_offset
+ bytesize` range where both bounds are read directly from the CRW file's own directory-entry
bytes, with no bounds check - `HEURISTICS.md`'s "index/slice arithmetic near a boundary" pattern,
on the CIFF/CRW container parser (`formats::ciff`), used by every `.crw` (older Canon raw) file
via `rawler::decoders::crw::CrwDecoder`. Root-caused from a fuzzer-found crash on a completely
empty (0-byte) input against the existing `fuzz_ciff_parse` target's own low-level entry point
(`CiffIFD::new(data, 0, data.len(), 0)`, which bypasses the real magic-byte gate `is_ciff`
enforces) - that specific 0-byte crash is **not** independently reachable through the real
`rawler::get_decoder` path (see "What was tried"), but reading the code around it turned up a
second, closely related, and genuinely real-path-reachable instance of the same unchecked-slice
pattern one level down, in `CiffEntry::new` - that's the one this candidate fixes and demonstrates
end-to-end.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo fuzz run fuzz_ciff_parse` (existing harness in this repo) | ~25 min, no seed corpus (same run that found candidates 003/004) | Found a 0-byte crashing input against `CiffIFD::new` called directly - artifact saved at `rawler/fuzz/artifacts/fuzz_ciff_parse/crash-da39a3ee5e6b4b0d3255bfef95601890afd80709` |
| Code reading of `CiffIFD::new`/`CiffEntry::new` (`formats/ciff/mod.rs:55-126`) and how `get_decoder`/`CrwDecoder`/`CiffIFD::new_file` chain into it | - | The fuzzer's own 0-byte crash (`end - 4` underflow in `CiffIFD::new`, `mod.rs:59`) requires `end < 4`; `new_file` always calls `CiffIFD::new` with `end = data.len()`, and that's only reached after `is_ciff` already confirmed the file is >= 14 bytes with the `HEAPCCDR` magic - so the *0-byte* crash specifically is not reachable via the real decode path, only via calling the public-but-unguarded `CiffIFD::new` directly (a genuine public API misuse surface, but not demonstrable as "a real file breaks a real caller"). `CiffEntry::new`'s `&buf[data_offset..data_offset + bytesize]` (`mod.rs:115`), reached from the same directory-parsing loop for *every* entry (not just the 0-byte edge case), has the identical missing-bounds-check shape and **is** reachable with a normal-sized, magic-byte-valid file |
| Standalone repro: a hand-built, genuinely minimal-but-valid 30-byte CRW file (real `HEAPCCDR` magic, one directory entry with an absurd declared `bytesize`), opened via the real, public, top-level `rawler::get_decoder` | debug build, 3 runs; release build, 1 run | Deterministic panic, 3/3 debug + 1/1 release, identical panic location/message each time; full backtrace confirms the real path `get_decoder -> CrwDecoder::new -> CiffIFD::new_file -> CiffIFD::new -> CiffEntry::new` |
| `cargo test -p rawler --lib` before and after the fix | debug build, 66 tests | 66 passed / 0 failed, both before and after |

## Status

**Confirmed.** Reproduces 3/3 debug runs and 1/1 release run, fully deterministic. Not gated by
`overflow-checks`/build profile - a plain out-of-range slice index, same class as candidates
003/004.

**Fully reproducible**: **Yes**. `repro/` writes a real, hand-built-but-genuinely-`HEAPCCDR`-magic-
valid CRW file to disk (30 bytes: a minimal header, one malicious directory entry) and opens it
through the unmodified, public, top-level `rawler::get_decoder` - the exact function every
`dnglab`/rawler-based caller invokes first. The full backtrace (see `output.txt`) confirms this
goes through the real `CrwDecoder::new` / `CiffIFD::new_file` chain, not a bypassed low-level call.
Single-participant case, no second participant to fake, per `CONFIRMATION.md` section 11.

## Class

`crash`.

## Root cause

`CiffEntry::new` (`formats/ciff/mod.rs:102-126`):

```rust
let (bytesize, data_offset) = match datalocation {
  0x0000 => (LEu32(buf, offset + 2) as usize, LEu32(buf, offset + 6) as usize + value_data),
  0x4000 => (8, offset + 2),
  val => return Err(...),
};
let data = &buf[data_offset..data_offset + bytesize];
```

For the (common) `datalocation == 0x0000` case, both `bytesize` and `data_offset` are read
directly from the entry's own 10 bytes in the file with no validation, then used to slice the
whole file buffer. Any entry whose declared size/offset exceed the actual file length - trivial to
craft, and not even unusual for a truncated/corrupted real-world CRW file - panics immediately.
`CiffIFD::new`'s directory-header parsing (`mod.rs:55-63`) has the same missing-bounds-check shape
one layer up (`end - 4` underflow if `end < 4`, and an unchecked `LEu16(buf, start +
valuedata_size)` read for the entry count) - not independently confirmed reachable via a real file
this session (see "What was tried"), but fixed defensively anyway since it's the exact same
pattern in the same tiny function.

## The fix

`CiffEntry::new`: replaced the unchecked `&buf[data_offset..data_offset + bytesize]` with
`data_offset.checked_add(bytesize).and_then(|end| buf.get(data_offset..end)).ok_or_else(...)?`,
turning both an out-of-range range and a `usize` addition overflow into the function's existing
`Result` error path instead of a panic.

`CiffIFD::new`: added `if end < 4 || end > buf.len() { return Err(...) }` before the `end - 4`
subtraction, and replaced the unchecked `start + valuedata_size` (used as an `LEu16` read offset)
with a `checked_add` + explicit length check before reading. Neither of these two was
independently confirmed reachable via a real file this session, but they're the identical
unchecked-offset pattern in the same function `CiffEntry::new`'s fix targets, immediately upstream
of it in the same directory-parsing loop.

Verified: `repro/`'s 30-byte crafted CRW file no longer panics through the real `get_decoder`
path and returns a descriptive error instead, in both debug and release. Also re-verified the
original fuzzer-found 0-byte input against the low-level `CiffIFD::new` call directly (3/3 runs,
no panic, clean error) - confirming the defensive `end < 4` guard added alongside the main fix
also closes that entry point, even though it wasn't independently confirmed reachable through a
real file.

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after (`cargo test -p
  rawler --lib`). No new failures; no existing test exercises CIFF/CRW parsing with malformed
  input, so this fix is additive coverage, not a regression risk.

## What's not done / open questions

- `dircount_offset`'s bounds check (`.filter(|&o| o + 2 <= buf.len())`) uses a plain `+ 2` rather
  than `checked_add`, so it has a purely theoretical residual overflow if `o` were within 2 of
  `usize::MAX` - not practically reachable (would require a `buf.len()` of a comparable magnitude,
  impossible for any real file) and not worth the extra churn to re-verify given the rest of this
  session's remaining time, but noted for completeness.
- `CiffIFD::new`'s recursive subifd call (`mod.rs:69`, `CiffIFD::new(buf, e.data_offset,
  e.data_offset + e.bytesize, depth + 1)`) computes `e.data_offset + e.bytesize` without a
  `checked_add` either - now protected transitively by the new `end > buf.len()` guard at the top
  of `CiffIFD::new` (an overflowed/wrapped sum would almost certainly fail that check, though a
  contrived wraparound landing back in-range isn't formally ruled out) - not independently
  stress-tested this session.
- Same file-format-parser-layer bug class as candidate 003 (X3F) - both are container/directory
  parsers for older, TIFF-adjacent-but-not-quite-TIFF raw formats with hand-rolled offset/size
  arithmetic and no shared bounds-checking helper. A worthwhile future architectural note (not
  pursued this session): most of candidates 003/005's fixes are the same `buf.get(range).ok_or_else`
  pattern repeated by hand - a small shared helper in this crate (e.g. `checked_slice(buf, range,
  context) -> Result<&[u8]>`) would make this fix shape easy to apply consistently to any other
  decoder found doing the same thing, rather than re-deriving the error-message wording each time.
