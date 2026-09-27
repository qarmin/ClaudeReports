# MinEntropy's 4x256-u32 counts array: cut zeroing cost by dropping the 4-way scatter

`src/filter/mod.rs`'s `entropy()` (used only by `Filter::MinEntropy`, confirmed by candidate 009 to
be a real opt-in use case worth optimizing) zeroes a `[[u32; 256]; 4]` (4 KiB) array on every call,
and `adaptive_filter` calls it once per candidate row filter (4x per row: Up/Sub/Avg/Paeth). For a
1024-wide RGBA row (4096 B of actual pixel data), that's 4 KiB of zeroing plus a 256-iteration
consolidation loop and a 256-iteration `ilog2i` loop, run 4 times per row, against 4 KiB of real
work. The candidate: replace the 4-way scatter (`[[u32; 256]; 4]`, described in-code as
"reduce contention") with a single `[u32; 256]` array - a straight sequential increment per byte,
no consolidation step needed since there is only one array to read back from.

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

## What was tried

Harness: `___OPTIMIZATIONS___/harness/encoder_bench --mode filter --only MinEntropy`
(`FdeflateUltraFast` compression, matching candidate 009's methodology), copied into the worktree
and pointed at the local `png` checkout. Two repeat runs per configuration to establish the noise
floor, `taskset -c 0` for single-core, unpinned for multi-core.

| File | Variant | single-core p50 (run1 / run2) | multi-core p50 (run1 / run2) | output bytes |
|---|---|---|---|---|
| synthetic 1024x1024 RGBA8 (`photo` pattern) | baseline (4-way scatter) | 11.939ms / 10.051ms | 9.676ms / 9.659ms | 3,355,304 |
| synthetic 1024x1024 RGBA8 | patched (single array) | 10.091ms / 10.005ms | 10.163ms | 3,355,304 |
| `tests/benches/paletted-zune.png` (2048x2253 indexed8) | baseline | 14.179ms / 14.185ms | 14.157ms / 14.249ms | 1,747,469 |
| `tests/benches/paletted-zune.png` | patched | 16.181ms / 16.271ms | 16.237ms / 16.280ms | 1,747,469 |
| `tests/benches/kodim02.png` (768x512 RGB8) | baseline | 3.264ms / 3.830ms | (not re-run, high baseline noise) | 666,988 |
| `tests/benches/kodim02.png` | patched | 3.656ms / 3.642ms | (not re-run) | 666,988 |

## Result

**Time: mixed, and net negative on the file where it matters most.** On the synthetic 1024x1024
RGBA image (the exact case the candidate description targeted: wide row, lots of actual pixel
data relative to the 4 KiB counts array), the change is within the noise floor - baseline's own
two runs (11.939ms vs 10.051ms, ~16% apart) already bracket the patched result (10.091ms /
10.005ms), so no real delta is demonstrated there either way.

On `paletted-zune.png` - the indexed/paletted file where candidate 009 found `MinEntropy`'s
*best* size win (-3.90% vs `Adaptive`) - the patched version is **consistently ~14% slower**
across 4 independent runs (2 single-core + 2 multi-core), 14.16-14.25ms baseline vs
16.14-16.28ms patched, a gap far larger than either configuration's own run-to-run spread
(<1% each). `kodim02.png` (RGB8, no palette) sits in between and is inconclusive given the
baseline's own ~17% run-to-run spread on that file.

The regression on the indexed file is consistent with the original code's own comment ("scatter
the counts into 4 separate arrays to reduce contention"): indexed/palette data repeats a small
set of byte values far more than RGBA/RGB photo data, so sequential `counts[byte] += 1` on a
single array hits the same array slot back-to-back much more often, creating a read-after-write
dependency chain the CPU can't hide - exactly the situation the original 4-way scatter was
designed to avoid. The zeroing savings (4 KiB -> 1 KiB, plus dropping the 256-iteration
consolidation loop) does not make up for that lost instruction-level parallelism on this file.

## Does it change behavior/output?

- **Bit-exact?** yes. Encoded output size is identical before/after on every file tested
  (3,355,304 / 1,747,469 / 666,988 bytes respectively) - the arithmetic is the same total per byte
  value, just accumulated into one array instead of four then summed, so `RowFilter` choice and
  final bytes are unaffected.
- **Is this a breaking change?** No - private helper function, no public API change.
- Project test suite (`cargo test --release`, worktree at base commit `05ccdb9c`):
  - Baseline: 107 passed (91 + 1 + 4 + 3 + 8 across the 5 test binaries/doctests), 1 ignored,
    0 failed.
  - Patched: identical - 107 passed, 1 ignored, 0 failed.

## Why the result looks the way it does

The candidate's premise (4 KiB of zeroing is expensive relative to a ~4 KiB row) is true, but the
original 4-way scatter isn't purely wasted work being traded for that zeroing cost - it also buys
real instruction-level parallelism for the counting loop itself, and that benefit turns out to
dominate on exactly the color type (indexed/paletted) that candidate 009 identified as
`MinEntropy`'s best use case. A synthetic RGBA image with more evenly-distributed byte values
doesn't exercise that dependency-chain cost as much, which is why it showed no clear delta either
way there.

## What's not done / open questions

- Not tried: a 2-array (halfway) variant, or a design that keeps the 4-way scatter for counting
  but avoids re-zeroing across calls (e.g. a persistent thread-local buffer with a touched-index
  list to reset only touched slots) - either could plausibly capture some of the zeroing saving
  without losing the ILP benefit, but adds real complexity (thread-local state, extra
  bookkeeping per increment) that wasn't judged worth prototyping blind without a specific
  follow-up. Left as an `IDEAS.md` entry for whoever picks this up next.
- Not measured: allocation count/bytes or peak RSS - `entropy()`'s arrays are stack-allocated in
  both variants, no heap allocation in this function either way, so this dimension isn't
  applicable to this specific change.
- Corpus-wide timing (all 15 files, not just 3) was not re-run for the patched variant - the
  3-file spread (flat-to-regression) was judged clear enough to not need it to reach a real
  conclusion here.
