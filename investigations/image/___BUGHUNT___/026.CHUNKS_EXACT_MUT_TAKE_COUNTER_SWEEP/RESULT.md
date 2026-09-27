# CHUNKS_EXACT_MUT_TAKE_COUNTER_SWEEP

Follow-up from candidate 004 (`TGA_RLE_REPEAT_OVERRUN`, Not reproducible): a grep-based sweep of
the whole codebase for the same shape - `chunks_exact_mut(n).take(count)` (or an equivalent
`.next()`-in-a-loop pattern) paired with a separately-tracked index/counter advanced by the
*nominal* (unclamped) count rather than the actual number of chunks written - to see whether any
other decoder repeats it in a context where the "buffer length is a multiple of `n`" invariant
might not hold (e.g. a call site passing a genuinely partial/row-sized buffer instead of a
whole-plane buffer, unlike TGA's one call site).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Grep sweep of `src/` for `chunks_exact_mut` (49 call sites) cross-referenced against every `.take(` call site in the same directories | `grep -rn` over `src/codecs/`, `src/imageops/`, `src/images/` | Found exactly one other instance of the shape besides TGA: `src/codecs/bmp/decoder.rs:2298`, `read_rle_data`'s RLE8 encoded-run branch - `(&mut pixel_iter).take(n_pixels).for_each(...)` followed by `x += n_pixels as u32;` at line 2332, where `n_pixels` is the raw (unclamped) RLE control byte (0-255) and `x` is never reconciled against the actual number of pixels written. `filter_1d.rs:545`'s `chunks_exact_mut(...).take(half_kernel)` does not match - no separate counter is advanced afterward from the nominal `take` count. |
| Code reading of `read_rle_data` and its helpers (`set_8bit_pixel_run`/`set_4bit_pixel_run`/`set_2bit_pixel_run`) and of `x`'s only two use sites | Manual trace | `row` passed to `pixel_iter = row.chunks_exact_mut(num_channels)` is one row from `self.rows(buf)`, whose stride (`width * num_channels`) is always an exact multiple of `num_channels`, so the multiple-of-`n` invariant TGA relied on does hold here too (unlike the hypothesized "genuinely partial/row-sized buffer" case this sweep was looking for). More importantly, unlike a raw-indexing bug, every write in this path goes through a bounds-checked `Iterator::next()` (returning `Option`, explicitly handled with `if let Some(..) else { false / break / return }`) or a `.take()`-wrapped iterator that self-limits to the remaining chunks - `x` itself is never used to index any raw slice/array directly, only to reposition a safe iterator via `.nth()` (also bounds-checked, silently stops at the end) or to bound a `for _ in 0..x { pixel_iter.next() ... }` loop that already checks `Option` on every iteration. `palette[*idx as usize]`/`palette[$i as usize]` indexing is separately safe regardless of `x`: `read_palette` always allocates exactly 256 entries even when the declared palette is smaller (`decoder.rs:1795-1801`), so a raw `u8` index (0-255, or a 4-bit/2-bit masked sub-index) can never be out of bounds. |
| Debug-assertions + overflow-checks build, manual boundary-value construction: two hand-built minimal `BI_RLE8` BMP files (`RUSTFLAGS="-C debug-assertions=on -C overflow-checks=on"`), each with an encoded-run control byte (0xFF / 0xFE, i.e. `n_pixels` = 255/254) against a 4-pixel-wide row, one plain and one combined with a subsequent delta-escape (exercising the `for _ in 0..x` skip loop with an inflated `x`) | `cargo test --test scratch_bmp_rle8_overrun` (temporary test file, deleted after the run, not committed) | Both cases decoded successfully with no panic and the expected 4x2 / 4x3 dimensions - confirms the code-reading conclusion empirically. |

## Status

**Not reproducible.** Grep-swept every `chunks_exact_mut` call site in `src/` (49 total) and cross-referenced against every `.take(` call site in the same directories; the only other instance of the exact shape (`chunks_exact_mut(n).take(count)` + a separately-advanced nominal counter) besides TGA's own (already ruled out as candidate 004) is BMP's `read_rle_data` RLE8 branch, and it's safe for a different, stronger reason than TGA's "call site invariant happens to hold": the inflated/unclamped counter (`x`) is never used to directly index a raw slice at all - both of its use sites re-derive position through the standard library's own bounds-checked iterator methods (`Iterator::nth`/`Iterator::next`, both `Option`-returning and already `if let Some(..) else { .. }`-guarded at every call site). No signal of any kind (crash, wrong output, sanitizer complaint) was found in code reading or in a debug-assertions build exercising boundary values designed to maximize the mismatch between the nominal run length and the actual row width.

**Fully reproducible**: n/a (not Confirmed).

## Class

n/a (not Confirmed).

## Root cause

n/a - the suspected bug shape does not manifest anywhere else in the codebase in a way that causes an out-of-bounds access or incorrect state. In every instance found (TGA candidate 004's `read_encoded_data`, and this candidate's BMP `read_rle_data`), either the buffer-length-is-a-multiple-of-`n` invariant holds at the one call site (TGA), or - as an independent, stronger safety net (BMP) - the mismatched counter is never used as a raw index in the first place, only to drive further bounds-checked iterator calls that themselves silently stop at the end of the underlying slice.

## The fix

Not applicable - nothing to fix.

- Project test suite: not run (no source change was made; the only file added was a temporary, uncommitted scratch test under `tests/`, deleted after use, and `git status` was confirmed clean before finishing).

## What's not done / open questions

- This sweep only searched for the literal `chunks_exact_mut(...).take(...)` textual shape plus a separately-advanced counter. A structurally equivalent but textually different pattern (e.g. `chunks_mut(n)` instead of `chunks_exact_mut`, or a hand-rolled loop with raw slice indexing instead of an iterator) was not swept - `chunks_mut` in particular behaves differently at the tail (its last chunk can be shorter than `n`) and was not checked for a similar counter-mismatch shape anywhere in the codebase.
- `set_2bit_pixel_run` (2bpp RLE, if reachable at all from a real BMP RLE variant) was read but not independently exercised with its own boundary-value test the way the 8bpp path was - by the same code-reading argument (identical `Option`-checked `.next()` shape) it's expected to be equally safe, but wasn't empirically probed this session.
