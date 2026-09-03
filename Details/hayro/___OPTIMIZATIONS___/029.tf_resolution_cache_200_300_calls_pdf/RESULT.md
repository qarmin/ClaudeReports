# `Tf`-resolution-cache fix (candidate 007) at 218 `Tf` calls/page, a real PDF in the 200-300 bracket

Backlog entry (candidate 019, top of the backlog): narrow the "above 147 (no change), at or below
326 (`img.pdf`, strong win)" bracket for candidate 007's `Tf`-resolution-cache fix by finding a
real, non-synthetic PDF with roughly 200-300 `Tf` calls/page reusing a small set of fonts.
`hayro-tests/pdfs/custom/` (279 PDFs present in this sandbox) was surveyed with `grep -c ' Tf'`
first (a cheap proxy, only meaningful for uncompressed content streams), then the top hits were
confirmed precisely with a temporary instrumentation probe: an `eprintln!("TF_CALL {:?}", t.0)`
added to `TypedInstruction::TextFont`'s match arm plus a small scratch `hayro/examples/tf_probe.rs`
driver that rendered each page of a candidate PDF individually and counted stderr lines between
`PAGE_BOUNDARY` markers - same counting method candidates 018/019 used, not a byte-search alone
(the byte-search overcounted here: `font_truetype_7.pdf`'s 7 pages each have 218-755 raw `' Tf'`
byte matches, only page 6's precise operator count landed in the requested range). Both the probe
file and the `eprintln!` were reverted (`git checkout --` / `rm`) before any benchmarking.

**`font_truetype_7.pdf`** (`hayro-tests/pdfs/custom/`, already part of this repo's own test corpus,
7 pages), **page index 6** (the 7th page), has **218 `Tf` calls**, reusing only **7 distinct font
resource names**: `F7` (81x), `F8` (78x), `F11` (27x), `F10` (22x), `F20` (5x), `F18` (4x), `F2`
(1x) - squarely in the requested 200-300 range, small-font-set-reused shape.

## What was tried

No new source code was written for the fix itself; `patch.diff` in this folder is byte-for-byte
identical to candidates 007/012/018/019's. Only the bench's input PDF/page changed.

Bench: `bench/src/main.rs`, same pattern as 007/012/018/019 (path-dependency on `hayro`, embeds
`font_truetype_7.pdf` via `include_bytes!`, renders page index 6, 300 iterations per invocation,
10% warmup discarded, reports min/p50/mean/p95/max of `render()` wall time, FNV-1a hash of the
produced pixmap's premultiplied RGBA8 bytes checked every iteration for bit-exactness).

Measured against the real hayro source at commit `5a5f0e24` (this candidate's `base_commit`, same
as 007/012/018/019), via a full `git clone` into `./clone` (kept for this session's own re-checks,
matches this folder's `patch.diff`/`justfile`).

This session's host was noisy and non-stationary, consistent with candidate 019's own experience
in this environment (load average observed swinging between ~6.6 and ~20.6 over the course of a
few minutes, most likely other candidates' own `flock`-serialized build/bench steps queueing and
running in the same window as this one, part of the same batch). A naive twice-run comparison of
the *same* baseline binary early in the session looked deceptively stable (1.36-1.37ms both times)
before the host got noisier partway through - so, following the lesson candidate 019 wrote into
`IDEAS.md`, all comparisons below used `tools/paired.sh`'s alternating-invocation, count-the-wins
methodology instead of a naive two-runs-per-variant comparison.

### Measurement

Two fixed binaries were built once each (`baseline_bin` = unpatched, `patched_bin` = `patch.diff`
applied), then invoked alternately via an ad-hoc adaptation of `../tools/paired.sh` (adapted only
in its `run()` function to parse this bench's `p50=` field; the alternating/win-counting core is
unchanged).

**Single-core (`taskset -c 2`, 21 pairs, 300 iterations each):**

```
A (baseline): 1.4605 1.4686 1.4657 1.5125 1.4532 1.4887 1.4625 1.4538 1.4678 1.4526 1.4487 1.4610 1.4533 1.4453 1.4440 1.4425 1.4413 1.4455 1.4410 1.4493 1.4590
B (patched):  1.4493 1.4563 1.4413 1.4666 1.4721 1.4600 1.4614 1.4272 1.4324 1.4276 1.4396 1.4490 1.4216 1.4236 1.4171 1.4263 1.4256 1.4255 1.4329 1.4379 1.4251
median 1.4533ms -> 1.4329ms  (-1.4%)   B faster in 20 of 21 pairs
verdict: consistent (per paired.sh's own threshold, wins >= n-1)
```

**Multi-core (unpinned, 21 pairs, then a repeat at 25 pairs, 300 iterations each):**

```
21 pairs: median 1.4546ms -> 1.4277ms  (-1.8%)   B faster in 19 of 21 pairs  (just short of the n-1=20 "consistent" threshold)
25 pairs: median 1.4489ms -> 1.4275ms  (-1.5%)   B faster in 19 of 25 pairs
```

Every single invocation across both binaries and both pin modes (2 x (21+21+25) x 300 = 40,200
iterations total) produced the identical FNV-1a pixmap hash, confirmed via the standalone runs
used to build the paired-comparison binaries and via `paired.sh` refusing to compare if the two
binaries were byte-identical (they were not).

| Variant | single-core p50 (paired median, 21 pairs) | multi-core p50 (paired median, 21/25 pairs) | pixmap hash |
|---|---|---|---|
| baseline | 1.4533ms | 1.4546ms / 1.4489ms | identical across all runs |
| patched | 1.4329ms (-1.4%) | 1.4277ms (-1.8%) / 1.4275ms (-1.5%) | identical across all runs |

## Result

**A small but real win, consistent in direction across single- and multi-core, of roughly -1.4%
to -1.8%** at 218 `Tf` calls/page reusing 7 fonts - clearly above candidate 019's 147-call "no
change" result (5/11, 6/11 win counts, a coin flip) and clearly smaller than `img.pdf`'s 326-call
"strong win" result (-16% to -18%). Single-core hit `paired.sh`'s own strict consistency bar
(20/21 wins, `>= n-1`); multi-core landed just under it (19/21, then 19/25 on a repeat) but in the
same direction and a comparable magnitude both times, not a coin-flip split like candidate 019's
147-call result was. This narrows the bracket further: **the win becomes resolvable somewhere
between 147 calls/page (no change) and 218 calls/page (small, resolvable win)** - not yet pinned
to an exact crossover point, and the per-call saving does not appear to scale purely linearly with
call count alone (218 calls/page here yields roughly -1.5%, while 326 calls/page in `img.pdf`
yields roughly -17%, a much larger jump than the ~1.5x call-count ratio alone would predict -
likely because `img.pdf`'s own `Tf` share of total render time is itself larger, i.e. this is not
an apples-to-apples ratio, see "Why the result looks the way it does").

## Does it change behavior/output?

- **Bit-exact?** Yes. All 40,200 iterations across both pin modes, both repeat counts, and both
  binaries produced the identical pixmap hash.
- Project test suite: ran `cargo test --release -p hayro-tests --test tests -- render
  --test-threads=4` before and after the patch, in this candidate's own clone at the pinned
  commit. `--test-threads=4` was needed because the full default-thread-count run returned "0
  passed" under this session's host contention (other candidates' builds/tests running
  concurrently in the same batch) - a resource-exhaustion artifact of the *measurement
  environment*, not the patch; the reduced-thread-count run reproduced the same numbers prior
  candidates reported (274/1252) and is what's recorded here. Both runs: **274 passed, 1252
  failed** (181 filtered out), in near-identical time (0.83s vs 0.90s). Diffed the two runs'
  failing-test-name sets (extracted from cargo's own final `failures:` summary list): **identical
  set, 1252/1252 names match**. Every one of the 1252 failures is `Os { code: 2, kind: NotFound }`
  - a missing local test-asset file (this sandbox's `hayro-tests/pdfs/` has no `corpus/`
    subdirectory, only `custom/`, `load/`, `other/` - the same pre-existing environmental
  limitation every prior candidate in this workspace has noted), not a render regression.
- **Is this a breaking change?** No - same patch as 007/012/018/019, internal to
  `hayro-interpret::interpret()`, no public API change.

## Why the result looks the way it does

218 `Tf` calls/page is enough repetition for the resource-name-keyed `resolved_font_cache` to pay
for itself measurably (avoiding a `SipHash13` over the font dict's raw bytes plus a global
hashmap lookup on most of those 218 calls, since only 7 distinct resource names are ever used),
but the absolute saving here (-1.4% to -1.8% of total `render()` time) is much smaller than
`img.pdf`'s (-16% to -18%) despite only a ~1.5x difference in call count. The likely explanation,
consistent with candidate 007's own original profiling breakdown (where `Tf` alone was 6.4% of
`img.pdf`'s total render time, more than half of the "unattributed dispatch overhead" bucket): the
*fraction* of total render time spent on `Tf`-operator dispatch is not just a function of call
count, it also depends on how much other, unrelated per-page work (glyph rendering, XObject/image
handling, path drawing) shares the page with those `Tf` calls - `img.pdf`'s page apparently spends
a much larger share of its total time in `Tf` dispatch specifically than `font_truetype_7.pdf`
page 6 does, so the same absolute per-call saving translates into a much smaller relative
`render()`-time improvement here. This is consistent with, not contradicting, candidates 018/019's
existing bracket - it just adds a third real data point (147 no-change, 218 small win, 326 strong
win) that confirms the crossover sits somewhere in the 147-218 range rather than closer to 218-326.

## What's not done / open questions

- The exact `Tf`-frequency threshold where the win first becomes resolvable is now bracketed to
  "above 147 (no change) and at or below 218 (small, resolvable win)" - tighter than candidate
  019's "above 147, at or below 326" bracket, but still not pinned to a single number. A real PDF
  in the 150-217 range would narrow this further, if one exists in the local corpus (not searched
  exhaustively for this narrower sub-range - the survey here stopped once a 200-300 hit was
  found, per this candidate's own scope).
- The "why 218->1.5% but 326->17%, not a proportional scaling" question in "Why the result looks
  the way it does" above is characterized only qualitatively (a plausible explanation grounded in
  candidate 007's original profiling breakdown), not re-confirmed with fresh per-operator-bucket
  profiling on `font_truetype_7.pdf` page 6 itself - a future candidate could re-run candidate
  007's own `Instant`+`AtomicU64` bucket instrumentation (`../_reference/`) against this page to
  check what fraction of its own render time is `Tf` dispatch specifically, rather than inferring
  it from the win-size comparison alone.
- Only page index 6 of `font_truetype_7.pdf` was used; the other 6 pages of the same file (551-755
  raw `Tf` calls per page, per the probe survey) were not benchmarked - they're already well above
  `img.pdf`'s 326-call "strong win" data point, so unlikely to add new bracket information, but
  were not explicitly checked.
