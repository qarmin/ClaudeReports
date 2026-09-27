# 015.upsampler_fix_no_default_features

Re-measurement of candidate 004 (`upsampler_row_buffer_reuse`, see
`../004.upsampler_row_buffer_reuse/RESULT.md` and its `patch.diff` - re-applied verbatim here, no
new patch) under `--no-default-features` (single-threaded, no rayon). 004 measured its
thread-local buffer-reuse fix for `Upsampler::upsample_and_interleave_row` only on the default
(`rayon`) build and found the wall-clock effect mostly buried in per-row rayon dispatch noise
(72.6% aggregate paired-win rate across 5 batches, only 1/5 crossing the strict consistency bar).
004's own "what's not done" section flagged the single-threaded fallback path
(`src/worker/mod.rs`) as untested and expected to show a cleaner signal, since it hits the exact
same allocation with no concurrent-access/threading complexity and no rayon dispatch overhead to
compete with. This candidate runs that measurement.

No new `patch.diff` in this folder - see `004.upsampler_row_buffer_reuse/patch.diff` for the
exact change (thread-local `RefCell<Vec<Vec<u8>>>` line-buffer cache in `src/upsampler.rs`)
re-applied for this measurement.

## What was tried

Built `___OPTIMIZATIONS___/harness/bench_decode/` twice against the main project's working tree
(once at pristine `base_commit` `eb2d7c0`, once with 004's `patch.diff` applied), both times with
the harness's `jpeg-decoder` path dependency temporarily set to `default-features = false` in
`harness/bench_decode/Cargo.toml` (confirmed via `cargo tree` that `rayon` does not appear in the
dependency graph for either build) - this is the only way to actually build the single-threaded
path, since `cargo build --no-default-features` on the harness crate itself has no effect on a
transitive dependency's own default features unless the dependency line says so. This edit was
made only to the canonical harness for the duration of this measurement and reverted to its
original (`default-features` unset, i.e. `rayon` on) state immediately afterward, since the
harness is shared by every other candidate in this workspace and most of them measure the default
`rayon` path.

| Variant | multi-core p50, `benches/large_image.jpg` (iters=30) | multi-core p50, lossless `sel1-rgb.jpg` (iters=30) |
|---|---|---|
| baseline, `--no-default-features` | 17.121 / 17.669 / 17.483 / 17.146 ms (4 batches) | 253.871 / 254.670 ms (2 batches) |
| patched, `--no-default-features` | 16.809 / 17.075 / 17.137 / 17.041 ms (4 batches) | 255.003 / 258.489 ms (2 batches) |

`tools/paired.sh` (unpinned multi-core, `RUNS=9`):

`benches/large_image.jpg` (4 independent batches - 2 more than the required minimum, run because
the first two batches gave different consistency verdicts and this candidate's whole point is
distinguishing a real small effect from noise):

- batch 1: median 17.121 -> 16.809 ms (-1.8%), patched faster 8/9 -> consistent
- batch 2: median 17.669 -> 17.075 ms (-3.4%), patched faster 6/9 -> NOT consistent
- batch 3: median 17.483 -> 17.137 ms (-2.0%), patched faster 7/9 -> NOT consistent
- batch 4: median 17.146 -> 17.041 ms (-0.6%), patched faster 9/9 -> consistent

All four medians lean the same direction (patched faster, -0.6% to -3.4%), and 2 of 4 batches
cross the strict `wins >= n-1` bar, both favoring patched, none favoring baseline. Aggregated:
30 of 36 paired comparisons (83.3%) favored the patched binary - a higher aggregate win rate,
and a more direction-consistent one (no batch favored baseline), than candidate 004's own
`rayon`-build result on the same image (72.6%, with individual batch verdicts landing on both
sides in that session's data).

`tests/reftest/images/lossless/1/jpeg_lossless_sel1-rgb.jpg` (control - lossless decoding never
calls `compute_image_parallel`/`Upsampler` per 004's own allocation-count finding, so no effect
is expected regardless of feature flags):

- batch 1: median 253.871 -> 255.003 ms (+0.4%), patched faster 3/9 -> NOT consistent
- batch 2: median 254.670 -> 258.489 ms (+1.5%), patched faster 4/9 -> NOT consistent (batch 2's
  raw data includes 3 outlier ~380ms samples in both A and B, symmetric across both binaries -
  transient system noise, not attributable to either build)

No consistent signal either direction, as expected - confirms the control still holds under
`--no-default-features` the same way it did under the default `rayon` build in 004.

Bit-exact: `bench_decode`'s per-iteration hash check (aborts on any mismatch) did not abort on any
run for either image, either binary.

## Result

A real, direction-consistent small win on `large_image.jpg` under `--no-default-features`:
every one of 4 independent `paired.sh` batches leans patched-faster (-0.6% to -3.4%), 2 of 4 cross
the strict consistency bar (both favoring patched), and the aggregate pair-win rate (83.3%, 30/36)
is both higher and more one-sided than candidate 004's own `rayon`-build measurement of the same
fix on the same image (72.6%, with some individual batches favoring baseline). This confirms the
hypothesis from 004's "what's not done" section: removing rayon's per-row dispatch overhead makes
this allocation-avoidance fix's effect easier to see, though it is still modest in absolute terms
(order 1-3%) and did not cross the strict consistency bar in a majority of batches, so it is
reported as a small, not a strong, win. The lossless control image shows no effect either
direction in either of 2 batches, exactly as expected since that code path never touches
`Upsampler`.

## Does it change behavior/output?

- **Bit-exact?** Yes - confirmed via `bench_decode`'s built-in per-iteration hash check on both
  images, both binaries, all batches.
- **Is this a breaking change?** No - see candidate 004's RESULT.md for the full analysis
  (internal-only change to `src/upsampler.rs`, no public API/format change). The same
  thread-local-buffer-persists-between-decodes nuance 004 already flagged applies identically
  here; `--no-default-features` builds use `src/worker/mod.rs`'s sequential fallback loop, which
  runs on whatever single OS thread calls `Decoder::decode()` - the thread-local buffer persists
  for that thread's lifetime the same way, just without the rayon-thread-pool angle 004's caveat
  focused on.
- Project test suite (`cargo +stable test --release --no-default-features`), main working tree:
  - Before (pristine, base_commit `eb2d7c0`): 5 passed (`src/lib.rs` unit tests) + 12 passed
    (`tests/lib.rs`) + 2 passed doctests = 19 passed, 0 failed. The 4 rayon integration tests
    (`tests/rayon*.rs`) are declared with `required-features = ["rayon"]` in `Cargo.toml` and are
    correctly skipped entirely under `--no-default-features` (not run, not failed).
  - After (patched): identical - 5 + 12 + 2 = 19 passed, 0 failed, 3 `test result` lines, 0 lines
    matching `^error` in the captured log.
  - For reference, the default-features (`rayon`-on) test suite pass count is already documented
    by candidate 004 (23 passed / 0 failed, both before and after its patch) and was not
    independently re-run here since the patch is byte-for-byte the same as 004's.

## Why the result looks the way it does

Same underlying mechanism 004 already established via its allocation-count check (`large_image.jpg`
drops from 6673 to 625 allocations per decode, ~91% reduction, all attributable to `Upsampler`'s
old per-row `Vec<Vec<u8>>` allocation) - not re-measured here since nothing about the allocation
pattern differs between the `rayon` and sequential-fallback code paths (`upsample_and_interleave_row`
itself, and its thread-local cache, are shared by both `src/worker/rayon.rs` and
`src/worker/mod.rs`'s callers). What differs is the *denominator*: under the default `rayon`
build, each row's allocation-avoidance saving competes against a per-row rayon task-dispatch cost
that 004 measured as large enough to swamp most of the signal; under `--no-default-features`,
there is no per-row dispatch step at all, so the same absolute saving is a proportionally larger
(and therefore easier to distinguish from noise) fraction of each row's total cost - exactly what
the higher, more one-sided aggregate pair-win rate (83.3% vs. 72.6%) shows.

## `just test` reproduction check

Ran `just test` in this candidate's own folder (fresh `clone/`, independent build of both
binaries, full `clone -> build-baseline -> apply -> build-patched -> bench` cycle) to confirm the
justfile actually reproduces end-to-end, not just that it reads correctly - it completed
successfully (exit 0) and printed real before/after numbers for both images, both hashes stable
and matching the ones already on record (`2f4deafc37e4ec2b` for `large_image.jpg`). This
particular run happened to land on a window of elevated host-level load (`uptime` load average
23-27 on this 24-thread machine during the run, vs. ~5-8 during the batches reported in "What was
tried" above, even though `ps` inside this environment showed no locally-visible process
consuming meaningful CPU - consistent with this being a container sharing a host with other,
externally-invisible load) - both the `large_image.jpg` paired batch (-0.7%, 5/9, NOT consistent)
and the lossless control (-2.2%, 5/9, NOT consistent, with a much wider 290-386ms spread than the
~254ms band seen earlier) came out noisier and less decisive than the batches above. This is
still useful evidence (the lossless control staying "NOT consistent" under noise is exactly what
was expected either way), but the numbers quoted in "What was tried"/"Result" above - taken under
a quieter window - are the ones this candidate's conclusion is actually based on, not this
confirmation run.

## What's not done / open questions

- Single-core (`taskset -c N`) pinning was not attempted - this workspace's own documented
  experience (`projects/jpeg-decoder.md`) is that pinning does not reliably reduce noise on this
  machine, so unpinned multi-core `paired.sh` batches were used throughout, consistent with every
  other candidate in this workspace.
- Batch 2 on the lossless control image had 3 outlier ~380ms samples (roughly 1.5x the normal
  ~254ms band) affecting both the baseline and patched columns symmetrically - not investigated
  further since it doesn't change the "no consistent signal" conclusion for that control, but
  worth knowing about if a future session revisits this exact image/machine combination and sees
  a similarly bimodal batch.
- Did not sweep `tower.jpg`/`tower_progressive.jpg`/`tower_grayscale.jpg` under
  `--no-default-features` - the task's two named images (`large_image.jpg` as the effect case,
  the lossless image as the control) were treated as sufficient to confirm/refute the specific
  hypothesis 004 left open; a future session could check whether the smaller 512x512 images show
  the same or a different pattern (they were never measured under `--no-default-features` for
  this fix either).
