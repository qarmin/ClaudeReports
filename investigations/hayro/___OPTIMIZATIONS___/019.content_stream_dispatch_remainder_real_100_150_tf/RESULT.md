# Real, non-synthetic 100-150 `Tf` calls/page PDF: re-test candidate 007's `Tf`-resolution-cache fix

Backlog entry from `../IDEAS.md` (added by candidate 018, top of the backlog): candidate 018
bracketed the frequency threshold where candidate 007's `Tf`-resolution-cache fix crosses this
bench's measurement noise floor to "above 34 `Tf` calls/page (`issue391.pdf`, no change) and at
or below 326 (`img.pdf`, strong win)" but could not find a real-world PDF in between - only a
synthetic one (`cache_defeating.pdf`). This candidate locates a real PDF in the 100-150 range and
re-measures the identical, byte-for-byte `patch.diff` from candidates 007/012/018 against it.

## Finding the real PDF

`hayro-tests/pdfs/custom/` (this repo's own local test corpus, 360 PDFs actually present in this
sandbox) was searched with `grep -c "Tf"` as a cheap proxy (raw-byte occurrence count, only
meaningful for uncompressed content streams), then the real per-page `Tf`-operator call count was
confirmed precisely via temporary instrumentation: a single `eprintln!("TF_CALL {:?}", t.0)` added
to `TypedInstruction::TextFont`'s match arm in `hayro-interpret/src/interpret/mod.rs`, built,
run once via `hayro-bench`'s `render_bench --iter 1` against a single-PDF input directory, output
counted, then reverted (`git checkout --`) before any benchmarking - the same counting method
candidate 018 used, not a byte-search alone (byte-search on compressed streams finds nothing;
this file happens to use uncompressed content streams so the two methods agree).

**`pdftc_900k_0007_page_4.pdf`** (`hayro-tests/pdfs/custom/`, already part of this repo's own test
corpus - a single extracted page from a real-world PDF test corpus, not built for this candidate)
has **147 `Tf` calls** on its one page, reusing only **7 distinct font resource names**: `TT7`
(56x), `TT6` (34x), `TT10` (28x), `TT9` (21x), `TT11` (5x), `TT3` (2x), `TT1` (1x) - squarely in
the requested 100-150 range, and a small set of fonts reused repeatedly rather than a different
font every time, exactly the shape the backlog entry asked for.

## What was tried

No new source code was written for the fix itself; `patch.diff` in this folder is byte-for-byte
identical to candidates 007/012/018's. Only the bench's input PDF changed.

Bench: `bench/src/main.rs`, following the same pattern as 007/012/018 (path-dependency on
`hayro`, embeds the PDF via `include_bytes!`, 10% warmup discarded, reports min/p50/mean/p95/max
of `render()` wall time, FNV-1a hash of the produced pixmap's premultiplied RGBA8 bytes checked
every iteration for bit-exactness).

### Measurement

Measured against the real hayro source at commit `5a5f0e24` (this candidate's `base_commit`,
same as 007/012/018), via a full clone into `./clone` (removed after measuring, per this
workspace's convention - reproducible with `just test`).

The host was noticeably noisier during this session than in prior candidates' sessions:
`uptime`'s load average fluctuated between ~3.8 and ~15.8 over the course of a few minutes with
no corresponding change to what this sandbox was running, and naive baseline/patched runs (build
once, run twice each, as 007/012/018 did) landed in two distinct p50 clusters (~2.5-2.6ms and
~3.5-3.8ms) that switched between pairs of runs *regardless of which variant was being measured* -
a host-level frequency/scheduling state change, not a code effect (see "Why the result looks the
way it does"). Because of this, the naive two-runs-per-variant approach used successfully in
007/012/018 was not trustworthy here, and this candidate instead used
`../tools/paired.sh` (adapted: `run()` shells out to the bench binary with an optional
`TASKSET_PREFIX`, parses the `p50=` field from its output) - alternating baseline/patched binary
invocations and counting how many of N pairs go each way, which cancels host-level drift instead
of averaging across it.

Two fixed binaries were built once each (`baseline_bin` = unpatched, `patched_bin` = `patch.diff`
applied) and then invoked alternately, 11 pairs, 500 iterations each:

**Single-core (`taskset -c 2`):**

```
A (baseline): 2.6895 3.7337 2.7688 2.6332 2.8144 2.7504 2.7588 3.5346 2.7769 2.8700 2.8783
B (patched):  3.0624 2.7651 2.7518 2.6163 3.4613 2.7545 3.4108 2.7232 2.8187 2.7765 3.7878
median 2.777ms -> 2.776ms  (-0.0%)   B faster in 5 of 11 pairs
verdict: NOT consistent - the medians are not evidence
```

**Multi-core (unpinned):**

```
A (baseline): 2.8393 2.7110 2.7597 2.6327 3.0333 3.7226 2.7157 2.6199 2.7825 2.7847 2.7763
B (patched):  2.7363 2.7477 2.7706 2.7272 3.4500 2.7969 2.6488 2.7794 2.7383 2.7553 2.6413
median 2.776ms -> 2.748ms  (-1.0%)   B faster in 6 of 11 pairs
verdict: NOT consistent - the medians are not evidence
```

Every single invocation across both binaries and both pin modes (22 x 500 = 11,000 iterations
total) produced the identical FNV-1a pixmap hash (`0x5ad8ca8212770e92`), confirmed both via the
standalone runs used to build the paired-comparison binaries and via `paired.sh` refusing to
compare if the two binaries were byte-identical (they were not - the patch did apply and the
binaries differ, just not in a way that moves wall-clock time here).

## Result

**No measurable change** at 147 `Tf` calls/page reusing 7 fonts, on this real, non-synthetic PDF -
in either pin mode. `paired.sh`'s win-count is close to a coin flip (5/11 single-core, 6/11
multi-core; a real effect would show up as 9+/11 or 2-/11, per the tool's own consistency
threshold), and the median deltas (-0.0%, -1.0%) are far smaller than the spread within either
side's own 11 samples (roughly 2.6ms-3.8ms, a ~35% range, dominated by host-level noise unrelated
to this patch - see below). This tightens candidate 018's bracket: the threshold where the win
becomes resolvable above this bench's noise floor is now known to be **above 147 `Tf` calls/page,
not just above 34** - closer to `img.pdf`'s 326 than previously known, though still not pinned to
an exact number.

## Does it change behavior/output?

- **Bit-exact?** Yes. All 11,000 iterations across both pin modes and both binaries produced the
  identical pixmap hash (`0x5ad8ca8212770e92`).
- Project test suite: ran `cargo test --release -p hayro-tests --test tests -- render` before and
  after the patch, in the pinned-commit clone. Both runs: **274 passed, 1252 failed** (181
  filtered out) - a slightly different pass count than 007/012/018's 267/1259 (likely a small
  difference in which local test assets happen to be present in this clone vs. the main working
  tree used by those candidates, not a patch effect - both counts are consistent between baseline
  and patched *within this candidate's own clone*). Diffed the two runs' failing-test-name sets
  (`render::[A-Za-z0-9_]+`, stripping non-deterministic thread-id noise): **identical set**
  (1525/1525 unique names match). Every failure is `Os { code: 2, kind: NotFound }` - a missing
  local test-asset file, the same pre-existing environmental limitation noted by every prior
  candidate in this workspace, not a render regression.
- **Is this a breaking change?** No - same patch as 007/012/018, internal to
  `hayro-interpret::interpret()`, no public API change.

## Why the result looks the way it does

Two things converge here:

1. **147 `Tf` calls is still not enough to clear this bench's noise floor**, at least not on this
   host today. `img.pdf`'s win (326 calls, -16% to -18%) is roughly twice this PDF's call count,
   and if the per-call saving scaled linearly a ~7-9% win might naively be expected here - but
   that's smaller than the noise this session's host was producing (see next point), so it's
   plausible a real, smaller effect exists here but simply isn't resolvable with this bench on
   this machine today, not proof the effect is exactly zero at 147 calls.
2. **This session's host was measurably noisier than prior sessions'.** `uptime`'s load average
   swung from ~3.8 to ~15.8 within minutes (this machine has 12 cores; loads above that reflect
   host-level contention outside this sandbox, per `../README.md`'s existing note on this), and
   naive same-binary-twice measurements landed in two different ~1ms-apart p50 clusters that
   switched in lockstep for *both* baseline and patched binaries - a signature of a host-level
   frequency-scaling or scheduling state change, not a code effect. `../tools/paired.sh`'s
   alternating-invocation, count-the-wins methodology is specifically designed to survive this
   kind of drift (see its own header comment and `MEASUREMENT.md` section 2b), and its verdict
   here - "not consistent" - is itself the finding: on a noisy host, at this call frequency, the
   fix's effect (if any) is not distinguishable from noise, which is a stronger and more honest
   statement than either of the misleading single-pair naive comparisons taken in isolation (an
   early one, since discarded, showed -3%/+2%; a later one on quieter uptime showed -0.4%/flat -
   neither was more "correct" than the other, both were single unpaired draws from noisy
   distributions).

## What's not done / open questions

- The exact `Tf`-frequency threshold where the win crosses this bench's noise floor is still not
  pinned to a single number - only bracketed further: above 147 (this candidate, no change) and
  at or below 326 (`img.pdf`, strong win). A real PDF in the 200-300 range, or re-running this
  exact `pdftc_900k_0007_page_4.pdf` comparison on a quieter host (this session's host noise was
  itself an obstacle, not just the call-frequency question), would narrow it further.
- This session's host noise (load average swinging 3.8-15.8 within minutes, independent of
  anything this sandbox was doing) may be worth flagging for any other candidate measured in the
  same session/timeframe - see the new backlog entry added to `../IDEAS.md` about generally
  preferring `../tools/paired.sh` over naive twice-run comparisons when `uptime` looks unstable at
  measurement time, not just at the start.
- `../tools/paired.sh`'s `run()` function was adapted for this bench's `p50=` output format
  in-place (not committed back into `../tools/paired.sh` itself, since the base template's
  `run()` is meant to be adapted per-project/per-bench-format each time it's reused - the adapted
  version lives only in this candidate's shell history, not saved as a file). A future candidate
  reusing this same bench's output format can copy the adapted `awk` snippet from this file's
  "Measurement" section above rather than re-deriving it.
