# TGA_RLE_REPEAT_OVERRUN

`src/codecs/tga/decoder.rs:262-273` (`read_encoded_data`, repeat-packet branch): a repeat packet's
nominal `repeat_count` (attacker-controlled, up to 128) is used to advance
`index += repeat_count * self.raw_bytes_per_pixel` unconditionally, even though the preceding
write (`buf[index..].chunks_exact_mut(bpp).take(repeat_count)`) only ever writes as many whole
chunks as actually fit in the remaining slice. The suspicion (from `IDEAS.md`) was that this lets
`index` overshoot `buf.len()`, ending the `while index < buf.len()` loop early and leaving
stale/unmodified bytes in the tail of the row instead of correct repeated-pixel data.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of the call site (`read_image`, `decoder.rs:403-429`) | Static read | `read_encoded_data` is called exactly **once** per image, over the *entire* raw pixel buffer (`&mut buf[..num_raw_bytes]` where `num_raw_bytes = width*height*raw_bytes_per_pixel`), not once per row. There is no per-row boundary in this decoder at all - RLE packets are free to (and, per spec, legitimately do) straddle scanline boundaries within this single call. |
| Manual boundary-value construction: hand-built minimal RLE-encoded TGA files (public API, `image::load_from_memory_with_format`) with a repeat packet whose nominal count exceeds the pixels actually remaining in the buffer - (a) 1x1 grayscale image, repeat packet claiming 2 repeats into 1 remaining pixel; (b) 3x1 grayscale image, one raw pixel then a repeat packet claiming 5 repeats into 2 remaining pixels | `repro/src/main.rs` cases 1-2, `cargo run --release`, debug-assertions dev build first then release | Both decoded to the **correct** pixel values (`0xAB` for case 1; `[0x11, 0xCC, 0xCC]` for case 2) - no stale bytes, no wrong data. |
| Randomized boundary sweep: 500 trials of randomly generated raw/repeat packet sequences (widths 1-9, grayscale) where roughly half of all repeat packets are deliberately given an inflated nominal count that overruns the pixels remaining, each trial's expected output computed by clamping repeat/raw counts to the actual remaining pixels (mirroring what a correct decoder must produce) and compared against the real decoder's output | `repro/src/main.rs` case 3, same binary, fixed-seed xorshift64, `cargo run --release` | **0/500 mismatches.** |

## Status

**Not reproducible.**

Ran 3 escalating approaches (code reading of the call site, two hand-built minimal instances, then
a 500-trial randomized boundary sweep specifically targeting the overrun condition) - all consistent,
0 mismatches across every trial including the two hand-built ones. Re-ran the full sweep twice
(fixed seed, deterministic) with identical results both times.

**Fully reproducible**: N/A (Not reproducible - the suspected defect does not manifest; there is
nothing to reproduce).

## Class

N/A (not reproduced).

## Root cause

The candidate's premise (index overshoot -> stale bytes left behind) does not hold for this
decoder because of an invariant the candidate's analysis missed: `read_encoded_data` is called
**once** for the whole `width*height*raw_bytes_per_pixel`-byte plane, which is by construction
always an exact multiple of `raw_bytes_per_pixel` (`bpp`). This invariant is preserved by every
iteration of the decode loop:

- Raw-byte branch: `num_raw_bytes = ((run_packet+1)*bpp).min(buf.len()-index)` - either adds an
  exact multiple of `bpp` to `index`, or (when it's the last packet needed) clamps `index` to
  exactly `buf.len()`.
- Repeat branch: `buf[index..]` therefore always has a length that is an exact multiple of `bpp`
  *before* this branch runs. `chunks_exact_mut(bpp)` on a slice whose length is an exact multiple
  of `bpp` has **no remainder** - it always exposes exactly `remaining_len / bpp` complete chunks,
  and nothing less. So when `repeat_count` (via `.take(repeat_count)`) exceeds `remaining_len /
  bpp`, `chunks_exact_mut` iterates *all* `remaining_len / bpp` chunks anyway, writing the repeat
  value across the **entire** remaining tail of the buffer - correctly, since that tail is exactly
  what the RLE run was going to occupy right up to the end of the image. Only *after* the buffer is
  already fully and correctly filled does `index += repeat_count * bpp` push `index` past
  `buf.len()`, which simply ends the `while index < buf.len()` loop on its very next check - by
  which point there is nothing left to write anyway.

In short: the "overshoot" always happens on the *last* packet needed to complete the buffer, and by
construction that packet's write always exactly reaches (or, when clamped by `chunks_exact_mut`,
exactly matches) the buffer's true end. There is no reachable state where a partial chunk gets
silently skipped, because the remaining length is never anything other than a whole multiple of
`bpp` at the point the repeat branch runs. (A genuinely truncated/malformed *reader* stream - too
few bytes before EOF - is caught separately by `self.r.read_exact(...)` returning an `io::Error`,
unrelated to this index-arithmetic path.)

## The fix

No fix applied - nothing to fix; the suspected defect is not present given the current call
pattern (single call over the whole image plane, not per-row).

- Project test suite: not run (no source change made; nothing to verify against).

## What's not done / open questions

- This decoder currently ignores scanline boundaries entirely when applying RLE runs (a single
  `read_encoded_data` call spans the whole image, not per-row) - this is actually *why* the
  candidate's proposed failure mode can't occur, but it does mean a non-conformant encoder that
  relies on scanline-bounded RLE semantics (per the original TGA spec, packets should not cross
  scanline boundaries, though most real-world encoders/decoders including this one don't enforce
  it) would still decode "successfully" here with different pixel placement than a strictly
  spec-conformant decoder; not investigated further since it's a spec-conformance question, not a
  memory-safety or stale-data bug.
- Did not check other RLE-encoded decoders in this codebase (e.g. any other formats with a similar
  `chunks_exact_mut(bpp).take(count)` + separate `index +=` pattern) for the same *shape* of
  index-vs-actual-write mismatch in a context where the multiple-of-`bpp` invariant might not hold
  (e.g. a call site that passes a buffer whose length isn't guaranteed to be a `bpp` multiple) -
  worth a quick grep-based sweep (`chunks_exact_mut` + separate manual index/counter advance) if a
  next session wants to generalize this check.
