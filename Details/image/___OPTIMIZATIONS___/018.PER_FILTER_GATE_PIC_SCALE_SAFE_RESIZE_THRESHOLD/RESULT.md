# imageops::resize Rgba<u8> fast path - per-filter gate instead of candidate 017's uniform threshold

Follow-up to candidate 017 (`try_pic_scale_safe_rgba8_resize` in `src/imageops/sample.rs`, the
pic-scale-safe delegation fast path for `Rgba<u8>` resizes). Candidate 017 added a single
output-pixel-count threshold (`RGBA_PREMULTIPLY_FAST_PATH_MIN_OUTPUT_PIXELS = 1_000_000`) gating
the non-constant-alpha (premultiply-needing) branch for *every* filter, to guard against a
`Triangle`-at-400x300 regression - but 017's own measurements already showed that threshold gives
up large wins (28-50% faster) for `CatmullRom`/`Gaussian`/`Lanczos3` at 400x300 while `Triangle`
itself was only ever a wash that session, not a regression. This candidate was tasked with
replacing the uniform threshold with a per-filter gate (Triangle only, and only if it still
regresses on re-measurement).

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (unchanged since candidates 013-017).
`patch.diff` re-includes 013+014+015's changes plus this candidate's own change, so it applies
standalone against the pinned base commit (013/014/015/017's own patches were all reverted at the
end of their sessions, per this skill's "the one rule that matters most").

Benchmark: the crate's own pre-existing `benches/imageops.rs::bench_resize`, already extended by
candidate 014/017 with `src_rgba` (non-constant alpha, 1920x1080) and `src_rgba_opaque` (constant
alpha) sources at 400x300 and (non-opaque only) 2000x2000. Reused as-is.

**Step 1 - re-measure whether `Triangle` at 400x300 still regresses with no gate at all**, before
deciding what the per-filter gate should actually do. Built a `nogate` variant: 013+014+015 with
017's `if` block (threshold check) deleted, so the fast path always runs unconditionally for every
filter. Compared against `A_original` (pure upstream `resize`, no 013/014/015/017 patch at all),
alternating runs:

**`resize rgba 400x300 Triangle`, non-constant alpha, single-core (`taskset -c 3`), 4 paired runs:**

| Variant | run 1 | run 2 | run 3 | run 4 |
|---|---|---|---|---|
| A_original | 8.29 ms | 8.45 ms | 8.15 ms | 8.57 ms |
| nogate (013+014+015, unconditional) | 8.01 ms | 7.85 ms | 7.76 ms | 7.60 ms |

`nogate` is consistently *faster* than `A_original` in all 4 runs (~4-11%), not slower - the
regression candidate 014 originally documented, and that candidate 017 already found not to
reproduce for the fused (013+014+015) code, still does not reproduce today with no gate at all.

**Same comparison, multi-core (unpinned), 3 paired runs:**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| A_original | 4.11 ms | 4.24 ms | 4.26 ms |
| nogate | 4.13 ms | 4.27 ms | 4.02 ms |

Within noise (a wash), consistent with the single-core result and with candidate 017's own
finding for the fused variant.

**Conclusion from step 1:** since `Triangle` does not regress at 400x300 even with zero gating,
the per-filter-gate design ("gate Triangle, never gate the other four") degenerates to "don't gate
anything" - there is currently no filter for which any gate is needed. The patch below therefore
removes candidate 017's threshold mechanism entirely (constant, doc comment, and the `if` check)
rather than re-adding a narrower version of it, and updates the stale comment above the removed
check to explain why (see `patch.diff`).

**Step 2 - full before/after measurement**, comparing `uniform` (013+014+015+017 exactly as
candidate 017 left it - this candidate's baseline) against `patched` (this candidate's actual
change: 013+014+015, threshold removed):

**`resize rgba 400x300 <filter>`, non-constant alpha, single-core (`taskset -c 3`):**

| Filter | uniform (baseline) | patched | delta |
|---|---|---|---|
| Triangle (3 runs) | 8.46 / 8.06 / 8.77 ms | 8.68 / 7.93 / 8.16 ms | wash, within noise either direction |
| CatmullRom (2 runs) | 14.72 / 14.09 ms | 9.99 / 10.39 ms | ~30% faster |
| Gaussian (3 runs) | 20.84 / 20.01 / 19.96 ms | 10.22 / 10.20 / 10.50 ms | ~49% faster |
| Lanczos3 (2 runs) | 20.84 / 20.70 ms | 12.67 / 13.14 ms | ~38% faster |

**Same, multi-core (unpinned), 2 paired runs each:**

| Filter | uniform (baseline) | patched | delta |
|---|---|---|---|
| Triangle | 4.26 / 4.20 ms | 4.25 / 4.08 ms | wash (patched marginally faster) |
| CatmullRom | 7.28 / 9.18 ms | 5.46 / 5.39 ms | ~25-41% faster |
| Gaussian | 10.27 / 12.12 ms | 5.35 / 5.32 ms | ~48-56% faster |
| Lanczos3 | 10.72 / 10.56 ms | 6.88 / 6.76 ms | ~36% faster |

**Sanity check - above the old threshold (2000x2000), where both variants already took the fast
path, single-core (`taskset -c 3`):**

| Filter | uniform | patched |
|---|---|---|
| Triangle (2 confirmatory runs) | 13.98 / 13.98 ms | 13.80 / 13.96 ms |
| Lanczos3 (1 run) | ~40.8 ms | ~39.9 ms |

Within noise - confirms the change has no effect above the size where 017's threshold already let
the fast path through, as expected (an initial single Triangle run at this size showed a larger,
uncorroborated spread on both sides, 22-27 ms, not reproduced on immediate re-run - treated as a
cold-start/noise artifact per this workspace's documented noise patterns, not a real effect).

**Sanity check - constant alpha (opaque), never takes the premultiply branch so was never gated by
017's threshold either, single-core, 3 paired runs (after discarding one uncorroborated cold-start
outlier on the first `uniform` invocation, ~4.5ms, immediately not reproduced on re-run):**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| uniform | 2.198 ms | 2.200 ms | 2.199 ms |
| patched | 2.170 ms | 2.178 ms | 2.211 ms |

Within noise, as expected - this path is untouched by either variant.

## Result

Removing candidate 017's uniform output-size threshold entirely (this candidate's actual change,
since re-measurement found no filter currently needs gating) restores the large 400x300 wins
candidate 017 had traded away: ~30% faster for `CatmullRom`, ~49-56% faster for `Gaussian`, ~36-38%
faster for `Lanczos3`, both single-core and multi-core. `Triangle` is a wash either way, matching
017's own finding and today's fresh re-measurement in "What was tried" above. Above the old
threshold (2000x2000) there is no change, as expected, since both variants already took the fast
path there.

## Does it change behavior/output?

- **Bit-exact given a fixed variant?** Partially - see below, this is the interesting case.
  Verified via a temporary standalone example (`examples/_resize_rgba_gate_diff_tmp.rs`, written
  and run this session, removed before capturing `patch.diff`) resizing a deterministic 1920x1080
  non-constant-alpha `RgbaImage` to both 400x300 and 2000x2000 across all five `FilterType`
  variants, hashing the raw output bytes (`DefaultHasher`) for `A_original`, `uniform`, and
  `patched`:
  - At 2000x2000: `uniform` and `patched` produce **identical** hashes for every filter (e.g.
    Triangle: `5b4b258eac869be4` both) - confirms the removed threshold made no difference above
    the old cutoff, as designed.
  - At 400x300: `Nearest` is identical between `uniform`/`patched` (never takes the premultiply
    branch, unaffected either way). For `Triangle`/`CatmullRom`/`Gaussian`/`Lanczos3`, `uniform`'s
    hash matches `A_original`'s exactly (e.g. Triangle: `b1cc3629a246f863` both - confirms
    `uniform`'s below-threshold fallback really is bit-identical to the pure original algorithm),
    while `patched`'s hash for these same four filters **differs** from both `A_original` and
    `uniform` (e.g. Triangle: `4e7b3cfa8e0e7448`). This is not a *new* characterized difference:
    it is the same pic-scale-safe-vs-original-algorithm difference candidates 013/014 already
    documented (dominant `+1`-per-channel delta from the pre-existing `___OTHER___/001`
    accumulator bug, plus a smaller genuine algorithm-difference spread), which candidate 017's
    threshold had suppressed at this specific size by forcing a fallback to the original
    algorithm below 1,000,000 output pixels. Removing the threshold means every non-constant-alpha
    `Rgba<u8>` resize now takes the fast path (and its already-known output difference)
    regardless of output size, not just above the old threshold.
- **Is this a breaking change?** No new breakage beyond what candidates 013/014 already documented
  for the `Rgba<u8>` delegation itself - this candidate only changes *which output sizes* take
  that already-characterized code path (now: all of them), not what the code path itself does.
- **Project test suite:** 331 passed, 0 failed, both before (`uniform`, 013+014+015+017) and after
  (`patched`, 013+014+015 with the threshold removed) - `cargo test --release --lib`. Same count
  both times, no new failures. The existing suite does not pin `imageops::resize`'s exact numeric
  output for `RgbaImage` inputs (same as candidates 013/014/015/017 found).

## Why the result looks the way it does

The task's proposed design ("gate only `Triangle`, and only if it still regresses on
re-measurement") was followed literally: `Triangle` was re-measured first, specifically to decide
whether the per-filter gate should exist at all for that one filter. It did not regress (a
consistent 4-11% single-core win and a multi-core wash across 4+3 paired runs today), matching
candidate 017's own same-session finding that the regression candidate 014 originally documented
only ever reproduced in the un-fused 013+014 variant, not the fused 013+014+015 code currently at
HEAD of this backlog chain. Since no filter currently regresses, a "per-filter gate" has nothing
left to gate - the honest implementation of the proposed fix, given today's data, is to remove the
gating mechanism entirely rather than keep dead/never-triggered threshold logic around for a
regression that both this session and the immediately preceding one failed to reproduce on this
machine.

## What's not done / open questions

- **Whether the original 400x300 `Triangle` regression is fully retired for good, or just
  quiescent on this specific machine across two sessions now**, remains unresolved - candidate
  017 already flagged this and it is not settled by this candidate either, since the same
  fused-alloc code (013+014+015) was re-measured on the same machine. A future session on a
  different machine, or one that deliberately reproduces candidate 014's original test
  conditions more closely, would be needed to fully close this out. If a future session does
  observe a real regression for `Triangle` (or any other filter) at small output sizes again,
  the natural next step is to reintroduce a per-filter threshold *for that filter specifically*
  using this candidate's removed code as a starting point (see `patch.diff`'s predecessor,
  candidate 017's own patch, for the exact shape).
- Only `taskset -c 3` was used for single-core pinning, consistent with candidates 014/015/017's
  own methodology, not swept across other cores.
- Fewer paired runs were used here than in 017's own session for some cells (2-4 per cell rather
  than 3, given step 1 already needed its own separate round of measurements) - the effect sizes
  observed (25-56%) are well above this machine's documented noise floor (a few percent, per
  `README.md`), so this is not expected to change the conclusion, but a future session wanting
  tighter confidence intervals could add more rounds.
