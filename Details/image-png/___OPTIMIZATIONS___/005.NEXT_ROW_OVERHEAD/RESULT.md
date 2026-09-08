# How much overhead does `next_row`/`next_interlaced_row` add over `read_row`? (measurement only, no patch)

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

`Reader::read_row` writes directly into a caller-provided buffer - no extra copy. `next_row`/
`next_interlaced_row` (`src/decoder/mod.rs:524-541`) already use the efficient `mem::take` +
`resize` + put-back pattern for their own internal `scratch_buffer` (reuses capacity, no
allocation once warmed up) and hand back a borrowed slice - so the IDEAS.md backlog entry (citing
issue #417's complaint about an extra copy in `image::codecs::png::PngReader::read`) framed the
real open question as: given `read_row` already exists as the zero-copy alternative, what does the
remaining `next_row`/`next_interlaced_row` path cost *on top of* `read_row` itself? This candidate
answers that with no code change - `read_row` is already public API.

## What was tried

Standalone bench (`bench/` - no existing harness compares these two APIs directly) decodes a real
file three ways - `next_frame` (bulk, for reference), a `read_row` loop, and a `next_row` loop -
verifying all three produce byte-identical pixel output, then times all three, **rotating which
one runs first every iteration** (3-way analogue of `tools/paired.sh`'s alternation, done within one
process to avoid the cross-process timing confound flagged in candidate 001).

| File | rows | next_frame p50 | read_row p50 | next_row p50 | next_row vs read_row |
|---|---|---|---|---|---|
| kodim02.png (768x512 RGB) | 512 | 2.0596ms | 2.0646ms | 2.0663ms | +0.1% |
| Lohengrin...png (2812x3982 grayscale) | 3982 | 13.9556ms | 13.9418ms | 14.0009ms | +0.4% |
| tango-icon-...-16.png (16x16 RGBA) | 16 | 0.0044ms | 0.0044ms | 0.0047ms | **+5.9%** |

## Result

**Time.** `next_row`'s overhead over `read_row` is negligible in absolute terms across the board -
sub-microsecond per call - but it's real and it scales with **row count**, not image size: two
large images (512 and 3982 rows) show +0.1% and +0.4%, while a 16-row image shows +5.9%. This is
consistent with the overhead being a small fixed per-call cost (the `mem::take`/put-back dance plus
an extra `Option`/struct wrapping layer) that only becomes visible in relative terms when there's
very little actual decode work per row to amortize it against. `next_frame` (bulk decode via a
single call) tracks `read_row` closely in all three cases, as expected since `next_frame` is
implemented as a `read_row` loop internally.

## Does it change behavior/output?

- N/A - measurement only, no patch. `Reader::read_row` is already public, documented, shipped API.
- Correctness of the measurement itself: `next_frame`/`read_row`/`next_row` decoded pixel buffers
  are asserted byte-identical for every file before any timing is recorded (`verify=OK` on all 3
  files tested).
- Project test suite: not run - no source file was changed by this candidate.

## Why the result looks the way it does

`next_row` and `next_interlaced_row` are thin wrappers around the same `next_interlaced_row_impl`/
`read_row` machinery `read_row` itself calls, adding one `mem::take`+`resize`+put-back cycle (cheap
after the first row, since capacity is reused) and an `Option<InterlacedRow>` wrapper per call.
There's no hidden extra copy of the decoded row data itself - the transform function writes
directly into `self.scratch_buffer`, the same buffer that gets borrowed out. So the API-level
"copy" issue issue #417 describes is not inside this crate's `next_row`/`next_interlaced_row` -
it's whatever the *caller* (e.g. the `image` crate's `PngReader::read`) does with the borrowed
slice it gets back (typically copying it into their own output buffer, because they can't hold a
borrow across their own API boundary). That caller-side copy is out of scope for this crate to fix;
`read_row` already is the answer for a caller that can restructure to use it.

## What's not done / open questions

- This doesn't measure or attempt to fix anything on the `image` crate's own integration side
  (issue #417 is filed against `png`, but the actual extra copy it describes lives in a downstream
  consumer) - out of scope for this workspace.
- Only tested non-interlaced files; Adam7-interlaced images call `next_interlaced_row_impl` more
  times per output row in the early passes (partial rows), which could change the per-call-overhead
  ratio - not measured here (ties into the broader "no interlaced files in the bench corpus" gap
  already noted in IDEAS.md).
