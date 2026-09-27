# What does resetting the zlib decompressor cost per APNG frame? (measurement only, no patch here)

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

`ZlibStream::reset()` (`src/decoder/zlib.rs:81-84`) does `*self.state = Decompressor::new()`,
called once per frame from `parse_fctl` - each PNG/APNG frame is an independent zlib stream with no
cross-frame dictionary, so *some* reset is semantically required. The open question (from IDEAS.md)
was what `Decompressor::new()` itself costs, and whether `fdeflate` exposes a cheaper way to get
the same effect.

## What was tried

Standalone bench (`bench/` - depends on both `png` and `fdeflate` directly, matching the exact
`fdeflate = "0.3.3"` requirement in `png`'s own `Cargo.toml` so the resolved version is identical):

1. Called `fdeflate::Decompressor::new()` in a tight loop (100,000 calls), counting allocations and
   wall time directly - no `png` code involved, isolates the constructor's own cost.
2. Built and fully decoded real N-frame APNGs (via the crate's own public encoder/decoder API, not
   synthetic) for `frames` in `{1, 2, 4, 8, 16, 32}`, counting total decoder allocations for each.

| Measurement | Result |
|---|---|
| `Decompressor::new()` allocations | **exactly 2 per call** (100,000/100,000 samples) |
| `Decompressor::new()` time | ~175 ns/call |
| APNG decode allocations, frames=1/2/4/8/16/32 | 8 / 10 / 14 / 22 / 38 / 70 |
| Allocation delta per additional frame | **exactly +2**, every step (2, 4, 8, 16, 32 frame deltas: 2/2, 4/2=2, 8/4=2, 16/8=2, 32/16=2) |

The per-frame allocation growth (+2/frame) matches `Decompressor::new()`'s own allocation count
(2/call) exactly - strong evidence this one constructor call explains effectively all of the
per-frame allocation growth seen in real APNG decoding, not just a plausible contributor.

Source of the 2 allocations (read from `fdeflate` 0.3.7's own source,
`~/.cargo/registry/.../fdeflate-0.3.7/src/decompress.rs:120-130`): `Decompressor::new()`
constructs `litlen_table: Box::new([0; 4096])` and `dist_table: Box::new([0; 512])` - two boxed
lookup tables rebuilt via `build_tables` (`:398`) from each block's Huffman header before being
read, so their *previous contents* are provably not needed - only fresh, correctly-sized
allocations are.

## Result

**Allocation count.** Real, precisely-quantified, and directly attributable: 2 allocations added to
every reset, matching real-world APNG decode allocation growth exactly. For a typical 8-32 frame
animation this is 16-64 allocations attributable to this one call path alone.

**Time.** Small in absolute terms - ~175 ns/reset means even a 32-frame animation pays only
~5.4 μs total for 31 resets, against a measured ~139 μs total decode time for that same animation
(~4%). Real, but modest relative to total decode cost; the win here is squarely allocation-shaped,
not time-shaped (consistent with `___OPTIMIZATIONS___`'s stance that a pure allocation win, without
a matching time win, is still a legitimate, reportable result).

## Does it change behavior/output?

- N/A - measurement only, no patch. Nothing in `png`'s or `fdeflate`'s shipped source was changed.
- Correctness of the measurement itself: the APNG decode loop in the bench decodes every frame of
  every constructed animation to completion via the crate's own public API with no errors, for
  every frame count tested.
- Project test suite: not run - no source file was changed.

## Why the result looks the way it does

`fdeflate::Decompressor` has **no public `reset()` method** - only `new()` (checked directly against
`fdeflate` 0.3.7's public API surface, `decompress.rs:118-1031`). `png`'s `ZlibStream::reset()`
therefore has no cheaper option available from the outside; a real fix requires `fdeflate` itself to
expose a reset that clears the scalar/enum state fields (`buffer`, `nbits`, `header`,
`uncompressed_bytes_left`, `queued_rle`, `queued_backref`, `last_block`, `fixed_table`, `state`,
`checksum`, `ignore_adler32`) while reusing the two already-allocated boxed tables in place (their
contents don't need clearing, per the `build_tables`-always-rebuilds observation above, only their
allocations need to survive). This is a genuine **vendoring candidate**, per this skill's own
guidance for when profiling points inside a dependency - not something fixable from `png`'s side
alone.

## What's not done / open questions

**What happens next, stated explicitly (per SKILL.md's vendoring section):** this is a
**documented finding, not an applied fix**. The concrete next step for a future session is either
(a) clone `fdeflate` at the pinned version this project uses, add a `pub fn reset(&mut self)` that
reuses the two boxed tables in place, measure it in `fdeflate`'s own `___OPTIMIZATIONS___/`
workspace, and open an upstream PR/issue against `fdeflate` if it measures a real win with no
correctness regression, or (b) a local `[patch]` override in `png`'s own `Cargo.toml` pointing at a
patched fork, which is a heavier commitment (deviates from the published dependency) that should be
a deliberate choice by the `png` maintainers, not something this workspace decides unilaterally.
Neither was attempted this session - the risk of introducing a subtle correctness bug in an
unfamiliar crate's internal Huffman-table state (the same class of mistake this workspace's own
candidate 002 made and caught, see that `RESULT.md`) was judged not worth taking on without the
extra scrutiny a dedicated vendoring session would need.
- Not measured: whether the ~175 ns/call figure is representative of a release build with a
  different allocator (this machine uses glibc malloc; jemalloc/mimalloc could have different
  small-allocation overhead) - noted as a caveat, not chased further.
