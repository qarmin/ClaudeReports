# imageops::resize Rgba<u8> fast path - output-pixel-count threshold on the premultiply branch

Follow-up to candidates 013/014/015 (`try_pic_scale_safe_rgba8_resize` in `src/imageops/sample.rs`,
the pic-scale-safe delegation fast path for `Rgba<u8>` resizes). Candidate 014 originally measured
the fast path as a ~21% regression vs. the original per-pixel algorithm for a non-constant-alpha
400x300 `Triangle` downscale (8.1974ms -> 9.8883ms); candidate 015's fused-alloc
`premultiplied_rgba8` swap narrowed that to roughly a wash (~8.0-8.2ms). This candidate adds an
output-pixel-count threshold (`RGBA_PREMULTIPLY_FAST_PATH_MIN_OUTPUT_PIXELS = 1_000_000`) so that
the fast path's non-constant-alpha (premultiply-needing) branch falls back to the original
algorithm below the threshold, on the theory that the fixed premultiply cost (scales with *source*
pixel count) dominates at small output sizes but is a small fraction of the total cost at large
output sizes.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (unchanged since candidates 013/014/015).
`patch.diff` re-includes 013+014+015's changes plus this session's threshold addition, so it
applies standalone against the pinned base commit (013/014/015's own patches were reverted at the
end of their sessions, per this skill's "the one rule that matters most").

Benchmark: the crate's own pre-existing `benches/imageops.rs::bench_resize`, already extended by
candidate 014 with `src_rgba` (non-constant alpha, 1920x1080) and `src_rgba_opaque` (constant
alpha) sources at 400x300 and (non-opaque only) 2000x2000. Reused as-is.

Four prebuilt `--release` criterion binaries were compared (alternating A/B runs, seconds apart,
same machine, `uptime` load 1-8 throughout, well under the 24-core count):

- **A_original**: pure upstream `resize` (no 013/014/015/017 patch at all) - the true original
  per-pixel algorithm.
- **B_013_014**: 013+014 only (the un-fused delegation candidate 014 originally measured as
  regressed).
- **baseline** (013+014+015): the fused-alloc fast path, unconditional (no size threshold) - this
  candidate's actual baseline.
- **patched** (013+014+015+017): baseline plus this candidate's threshold.

**Important finding, stated up front:** on this machine, *today*, the small-size regression
candidate 014 originally documented for `Triangle` does not reproduce for `baseline` (013+014+015)
- it is a small win or a wash, not a regression, for every filter tried at 400x300 except (as
expected) it's dramatically faster for `CatmullRom`/`Gaussian`/`Lanczos3`. See "Why the result
looks the way it does" for the implication this has for this candidate's own effect.

**`resize rgba 400x300 Triangle`, non-constant alpha, single-core (`taskset -c 3`), 3 paired runs:**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| A_original | 4.1271 ms | 4.0696 ms | 4.0704 ms |
| B_013_014 | 5.2354 ms | 4.8155 ms | 4.7901 ms |
| baseline (013+014+015) | 4.0489 ms | 3.9337 ms | 3.9177 ms |
| patched (+ this threshold) | 4.1422 ms | 4.0455 ms | 4.0438 ms |

`B_013_014` reproduces the originally-documented regression vs. `A_original` today (~17-25%
slower). `baseline` is at or slightly faster than `A_original` (not regressed). `patched` falls
back to `A_original`'s code path below the threshold and tracks it closely, as expected - but that
means `patched` gives up `baseline`'s small win here instead of preserving it.

**`resize rgba 400x300 Lanczos3`, non-constant alpha, single-core, 3 paired runs:**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| A_original | 10.161 ms | 10.156 ms | 10.502-11.011 ms |
| B_013_014 | 7.3782 ms | 7.3886 ms | 7.4079 ms |
| baseline | 6.4979 ms | 6.6373 ms | 6.5390 ms |
| patched | 10.142 ms | 10.419 ms | 10.139 ms |

`baseline` is ~35-40% faster than `A_original` here (a clear win, not a wash). `patched` falls back
below the threshold and loses that entire win, landing back at `A_original`'s ~10.1-11.0ms.

**`resize rgba 400x300 CatmullRom` / `Gaussian`, non-constant alpha, single-core (one run each):**

| Variant | CatmullRom | Gaussian |
|---|---|---|
| A_original | 6.9983-7.0345 ms | 10.059-10.076 ms |
| baseline | 5.0641-5.0849 ms | 5.0536-5.0613 ms |
| patched | 6.9758-7.0010 ms | 10.043-10.071 ms |

Same pattern: `baseline` is a large win (~28% and ~50% faster respectively) that `patched` entirely
gives up by falling back below the threshold.

**`resize rgba 400x300 Triangle`/`Lanczos3`/`CatmullRom`, multi-core (unpinned), one run each:**

| Variant | Triangle | Lanczos3 | CatmullRom |
|---|---|---|---|
| A_original | 4.2422-4.3033 ms | 10.284-10.325 ms | 7.0400-7.0616 ms |
| baseline | 4.2619-4.4715 ms | 6.5140-6.5325 ms | 5.1321-5.1819 ms |
| patched | 4.0774-4.0966 ms | 10.123-10.177 ms | 7.2165-7.2914 ms |

Same pattern as single-core: Triangle is a wash between baseline/patched (noise-level, patched
even marginally faster in this one run); Lanczos3/CatmullRom show patched losing baseline's large
win by falling back.

**`resize rgba opaque 400x300 Triangle`, constant alpha (never takes the premultiply branch, so
the threshold never triggers - sanity check), single-core:**

| Variant | run |
|---|---|
| A_original | 4.0687-4.1022 ms |
| baseline | 2.1647-2.1752 ms |
| patched | 2.2270-2.2469 ms |

`baseline` and `patched` both ~2x faster than `A_original` and close to each other, as expected -
the threshold only guards the non-constant-alpha branch, so the constant-alpha path is unaffected
either way.

**`large/resize rgba 2000x2000 Triangle`/`Lanczos3`, non-constant alpha, single-core
(`--measurement-time 3`) and multi-core:**

| Variant | Triangle single-core | Lanczos3 single-core | Triangle multi-core |
|---|---|---|---|
| A_original | 45.030-45.130 ms | 63.537-80.488 ms | 47.440-47.668 ms |
| baseline | 12.710-13.280 ms | 20.146-22.394 ms | 13.279-13.437 ms |
| patched | 13.078-13.209 ms | 20.011-20.150 ms | 12.531-12.767 ms |

`baseline` and `patched` are within noise of each other at 2000x2000 (both ~3.4x faster than
`A_original`), confirming the threshold has no effect at this size, as designed - the large-size
win from candidates 013/014/015 is fully preserved.

## Result

At 2000x2000 (above the 1,000,000-output-pixel threshold), `patched` is statistically
indistinguishable from `baseline` for both filters tried - the large-size win (~3.4x faster than
the original algorithm) is preserved exactly, as designed, since the threshold gate is never
triggered at this size.

At 400x300 (below the threshold), `patched` correctly falls back to the original algorithm's
performance (confirmed both by timing tracking `A_original` closely and by bit-exact hash equality
- see below). However, on this session's machine, `baseline` (013+014+015, unconditional fast
path) was **not** regressed at 400x300 for any filter tried today - it ranged from a wash (`Triangle`,
~0-5%) to a large win (`Lanczos3` ~35-40%, `CatmullRom` ~28%, `Gaussian` ~50% faster than
`A_original`). Falling back below the threshold therefore does not recover a regression (none was
observed today) and instead **gives up** `baseline`'s existing win at 400x300 for
`CatmullRom`/`Gaussian`/`Lanczos3`, landing back at the slower original-algorithm numbers for those
filters. For `Triangle` specifically the two are within noise of each other either way.

`B_013_014` (the un-fused, pre-015 candidate) does still reproduce a real ~17-25% regression vs.
`A_original` for `Triangle` at 400x300 today, confirming candidate 014's original finding was real
for that specific variant - it is candidate 015's fused-alloc fix, not a threshold, that already
resolved it in the code currently at HEAD of this backlog chain.

## Does it change behavior/output?

- **Bit-exact given a fixed variant?** Yes. Verified via a temporary standalone example
  (`examples/_resize_rgba_threshold_diff_tmp.rs`, written and run this session, removed before
  capturing `patch.diff`) resizing a deterministic 1920x1080 non-constant-alpha `RgbaImage` to both
  400x300 and 2000x2000 across all five `FilterType` variants, hashing the raw output bytes
  (`DefaultHasher`) for `A_original`, `baseline`, and `patched`:
  - At 400x300, `patched`'s hash for `Triangle`/`CatmullRom`/`Gaussian`/`Lanczos3` is **identical**
    to `A_original`'s (e.g. Triangle: `b1cc3629a246f863` both), confirming the fallback really
    executes the same original algorithm bit-for-bit. `Nearest` never takes the premultiply branch
    (unaffected by the threshold either way) and its hash differs from `A_original` (already a
    characterized difference from candidate 013/014 - the fast path uses a different, valid
    Nearest implementation - not new to this candidate).
  - At 2000x2000, `patched`'s hash for every filter is **identical** to `baseline`'s (e.g.
    Triangle: `5b4b258eac869be4` both), confirming the threshold makes literally no difference
    above the cutoff.
- **Is this a breaking change?** No new breakage beyond what candidates 013/014 already documented
  for the `Rgba<u8>` delegation itself (constant-vs-non-constant alpha and `Nearest`-vs-other-filter
  differences from the original algorithm) - this candidate only changes *which* of two
  already-existing, already-characterized code paths a given call takes, based on output size.
- **Project test suite:** 331 passed, 0 failed, both before (013+014+015 baseline) and after
  (013+014+015+017 patched) - `cargo test --release --lib`. Same count both times, no new
  failures. The existing suite does not pin `imageops::resize`'s exact numeric output for
  `RgbaImage` inputs (same as candidates 013/014/015 found).

## Why the result looks the way it does

The threshold's design assumption - that premultiplying the whole source is a fixed cost that
dominates at small output sizes - was based on candidate 014's original 400x300 `Triangle`
measurement (8.1974ms -> 9.8883ms, ~21% slower). That measurement does not reproduce today: this
session's fresh paired run shows `baseline` (013+014+015, which already includes candidate 015's
fused-alloc fix) at or faster than the original algorithm for every filter at 400x300, most
dramatically for `CatmullRom`/`Gaussian`/`Lanczos3` (28-50% faster). It's unclear from this session
alone whether the original regression was itself partly measurement noise/machine-state-dependent
(this machine's absolute numbers today are noticeably faster across the board than the historical
numbers recorded in candidates 014/015's own `RESULT.md`s, suggesting some machine-level difference
between sessions - e.g. thermal/frequency-scaling state - rather than a code difference, since
`B_013_014` here does still reproduce a real `Triangle`-specific regression matching 014's original
report), or whether candidate 015's fused-alloc fix by itself was already sufficient to close the
regression for good and today's numbers reflect that. Either way, an output-size threshold that
falls back unconditionally for every filter below a fixed output-pixel cutoff cannot distinguish
between "this filter is currently regressed at this size" (apparently only ever true for `Triangle`,
and only in the un-fused 013+014 code, not 013+014+015) and "this filter has a large win at this
size that a threshold would needlessly discard" (true for `CatmullRom`/`Gaussian`/`Lanczos3` today) -
a single output-pixel-count number can't encode a per-filter distinction, so applying it uniformly
trades away real, currently-measured wins to guard against a regression that, on this machine
today, only the already-superseded 013+014 (pre-015) variant actually has.

## What's not done / open questions

- **Whether the original 400x300 `Triangle` regression is fully retired or just currently
  quiescent** was not conclusively settled - `B_013_014` still shows it, `baseline` (013+014+015)
  does not, on this session's machine. A future session on a different/more consistent machine
  should re-check whether `baseline` alone (no threshold) ever regresses `Triangle` at small sizes
  again before concluding candidate 015 alone is sufficient.
- **A per-filter (rather than purely size-based) heuristic** is the natural next idea this
  candidate's result points to - e.g. gate the threshold only for `Triangle` (the one filter ever
  observed regressed) rather than for all five, so `CatmullRom`/`Gaussian`/`Lanczos3` keep their
  large small-size win unconditionally. Not attempted this session; added to `IDEAS.md`.
- Only `taskset -c 3` was used for single-core pinning (not swept across other cores) - consistent
  with candidates 014/015's own methodology, not re-verified against pinning noise here.
- The threshold constant (`1_000_000` output pixels) was chosen as a round number between the two
  measured data points (120,000 / 4,000,000), not empirically narrowed with intermediate sizes -
  moot given the "Result" above (the threshold's exact value doesn't matter if it's discarding a
  currently-real win rather than avoiding a currently-real regression), but would need
  re-examination if a future per-filter version of this idea is tried.
