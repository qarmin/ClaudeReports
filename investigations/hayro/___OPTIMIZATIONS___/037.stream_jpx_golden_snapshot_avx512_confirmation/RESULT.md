# `stream_jpx_3/5/6` golden-snapshot failures: confirming (and refuting) the AVX-512 hypothesis

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

Follow-up to candidates `002`/`008`, which noted in passing that `hayro-tests`' `stream_jpx_3/5/6`
fail against their committed-on-disk (but git-ignored, per `hayro-tests/.gitignore`) golden PNG
snapshots on a clean checkout in this environment, and hypothesized this was the same
FMA-vs-non-FMA bulk/remainder precision split already confirmed elsewhere in the IDWT filter
(candidate `002`'s own `RESULT.md`: SIMD bulk path uses real hardware FMA regardless of compile
flags, scalar remainder does not unless compiled with `target-feature=fma`). This candidate's job
was to actually test that hypothesis by regenerating output with/without AVX-512 dispatch and
checking whether the failing-pixel count moves. **It does not move (or barely does) - the
hypothesis is refuted.** The real cause, found by isolating variables one at a time, is a stale
golden snapshot that predates a real upstream decode-accuracy fix (PR #1340, merged 2026-08-16),
unrelated to SIMD/FMA/AVX-512 at all.

## What was tried

`fearless_simd` (the SIMD dispatch crate used by `hayro-jpeg2000`/`hayro-jbig2`/`vello_cpu`)
exposes exactly the knob this candidate needed without touching any source:
`RUSTFLAGS="--cfg disable_dispatch_avx512"` (and `disable_dispatch_avx2`) turn off automatic
multiversioning to that level, per `fearless_simd`'s own `src/lib.rs` doc comment - the CPU still
supports these levels ([`is_x86_feature_detected`] returns true; this host has full AVX-512F/BW/CD
/DQ/VL/IFMA/VBMI), only the crate's own dispatcher is told not to pick them, so a fallback to the
next-lower level is exercised on otherwise-identical hardware.

Golden references used: this worktree is a fresh `git worktree add`, and `hayro-tests/snapshots/`
is git-ignored (generated locally per `hayro-tests/README.md`'s "Generating the baseline
snapshots" - never checked into git), so a fresh worktree starts with **no** reference images at
all (confirmed: the first run produced "new reference image was created" panics for all 4 tests,
with fresh 09:22 timestamps). The 4 `stream_jpx_*.png` files actually used as the golden reference
below were copied in from the main project's own working tree (dated **2026-07-21**, i.e. from
whenever a previous session there last ran the full suite once with `REPLACE`) - copied into
`testdata/` in this folder for reproducibility. `md5sum` confirms these are byte-identical to what
the main tree still has.

All 4 runs: `cargo test -p hayro-tests --release --test tests -- stream_jpx --nocapture
--test-threads=1`, same golden references, same machine, same binary rebuilt per row (`hayro`
crate's `simd` feature - and therefore `hayro-jpeg2000`'s own default `simd` feature - left at its
default **on** throughout, per workspace `Cargo.toml`):

| Variant | `stream_jpx_1` | `stream_jpx_3` | `stream_jpx_5` | `stream_jpx_6` |
|---|---|---|---|---|
| Default build (AVX-512-eligible dispatch, this host's actual runtime path) | pass (0 diff) | **65974** px diff | **91173** px diff | **29580** px diff |
| `RUSTFLAGS="--cfg disable_dispatch_avx512"` (falls back to AVX2+FMA) | pass (0 diff) | **65974** px diff (unchanged) | **91173** px diff (unchanged) | **29580** px diff (unchanged) |
| `RUSTFLAGS="--cfg disable_dispatch_avx512 --cfg disable_dispatch_avx2"` (falls back to SSE4.2) | pass (0 diff) | **65974** px diff (unchanged) | **91173** px diff (unchanged) | **29588** px diff (+8, i.e. +0.027%) |
| Default build, **with upstream commit `49037586` (#1340) reverted** in this worktree's own `hayro-jpeg2000` source (see `testdata/pr1340.diff`) | pass | **pass (0 diff)** | **pass (0 diff)** | **pass (0 diff)** |

Image sizes (for the percentage context the candidate description cited): `stream_jpx_3`
595x842 = 500,990 px (65974/500990 = 13.2%), `stream_jpx_5` 512x512 = 262,144 px (91173/262144 =
34.8%), `stream_jpx_6` 540x720 = 388,800 px (29580/388800 = 7.6%) - matches the "13-35%" figure in
the candidate description exactly, confirming this is the same pre-existing failure candidate 008
saw, not a new one introduced by this session.

## Result

Not a performance result - this candidate found no code to bench, per "pure
measurement/investigation" in `SKILL.md`. The measured evidence above answers the confirmation
question directly:

- Disabling AVX-512 dispatch entirely: **0 pixel-count change** on 2 of 3 failing tests, unchanged
  on the third.
- Disabling AVX-512 *and* AVX2 dispatch (forcing SSE4.2, the lowest SIMD level `fearless_simd`
  still auto-vectorizes with on this target): **+8 pixels out of 29580 (+0.027%)** on one test,
  still 0 change on the other two. This +8 is plausibly the genuine, much smaller,
  already-documented FMA/vector-width remainder-boundary effect from candidates `002`/`008`/`034`
  (which found single-digit-to-tens of 1-LSB pixel differences from the same mechanism) - not
  the dominant cause of the 13-35% failure rate.
- Reverting PR #1340's actual algorithmic change (no SIMD/dispatch change at all, default AVX-512
  build): **all 3 previously-failing tests pass bit-exact.** This isolates the entire 65974/91173/
  29580-pixel discrepancy to that one commit.

## Does it change behavior/output?

**N/A - measurement/investigation only, no patch applied to real source.** The one source edit
made during this session (`git apply -R testdata/pr1340.diff` against `hayro-jpeg2000/src/j2c/
{bitplane,decode}.rs`, to isolate the confound) was reverted (`git checkout --`) immediately after
its one confirming test run, before any further work - not proposed as a fix, since reverting a
real upstream accuracy fix is obviously not a valid response to a stale-snapshot problem. No
`patch.diff` in this folder, by design (this candidate's finding requires no code change - see
"What's not done" below for the actual actionable follow-up).

Project test suite: not run as a before/after pair for this candidate specifically (no patch was
applied for real) - the 4 `stream_jpx_*` tests were run repeatedly across the table above as the
measurement itself, and a broader `-- render::` run was attempted but is not a meaningful
data point here: this worktree only has golden references for the 4 `stream_jpx_*` files (copied
in from the main tree, see above), so every *other* render test in the 1525-test suite fails with
"new reference image was created" in this worktree, an artifact of the fresh-worktree/git-ignored-
snapshots setup (see "What was tried" above), not a real finding - not worth copying over 1500
more PNGs from the main tree just to get a clean baseline count for a candidate that isn't
benching or patching anything.

## Why the result looks the way it does

`hayro-tests/snapshots/` is deliberately git-ignored (per its own README: generated locally by
running the suite once, then diffed against on every subsequent run) - it is not a versioned
fixture that gets updated in the same commit as the code that changes rendering output. PR #1340
(merged 2026-08-16, "Apply reconstruction midpoint to fully decoded irreversible coefficients")
fixed a real bug in `Coefficient::reconstructed` (`hayro-jpeg2000/src/j2c/bitplane.rs`): a
reconstruction-midpoint offset (JPEG2000 spec Formula E-6) was being skipped for *fully decoded*
irreversible (lossy, 9-7 wavelet) coefficients, when it should only be skipped for fully decoded
*reversible* ones - so every fully-decoded lossy coefficient landed on the edge of its
quantization interval instead of the middle. The PR's own commit message quantifies this against
`opj_decompress` ground truth on 30 real codestreams: "roughly 3.4 million differing samples
become 5,900, and no remaining difference exceeds one level" - i.e. this fix moves a large
fraction of decoded samples by up to one quantization level, which is exactly the "many pixels,
small max deviation" shape this candidate's description described (13-35% of pixels, 2-4/255 max
channel deviation) and exactly why the AVX-512/FMA hypothesis looked initially plausible (both
mechanisms produce "widespread but small" diffs) despite being unrelated. The golden snapshots on
disk (2026-07-21) predate this fix by about a month; nobody regenerated them after #1340 landed,
so `stream_jpx_3/5/6` (all irreversible/lossy JPX streams, per the commit message's own "all of
them reversible" exception for the codestreams that stayed byte-identical) have been failing on
every clean checkout since, regardless of who runs the suite or on what hardware - `git stash`
(candidate 008's own check) can't distinguish "stale fixture" from "hardware-dependent" because
neither one is affected by stashing a working-tree patch.

## What's not done / open questions

- **The actual, actionable fix for the test suite** is unrelated to any source patch: regenerate
  `hayro-tests/snapshots/render/custom/stream_jpx_{3,5,6}.png` (`REPLACE=1 cargo test -p
  hayro-tests --release --test tests -- stream_jpx`, per `hayro-tests/README.md`) against current
  `main`. Not done here because `hayro-tests/snapshots/` is git-ignored - regenerating it produces
  no diff `git` can see or record in this workspace, and it's a test-fixture action, not a
  `___OPTIMIZATIONS___`/`___BUGHUNT___`-shaped deliverable. Flagged in this session's final report
  instead so it can be folded into a maintainer-facing note.
  It's plausible other, non-`stream_jpx` snapshot tests touching irreversible JPX content are
  similarly stale for the exact same reason (same PR, same mechanism) - not checked here, since
  this worktree never had the full ~1500-file golden set to check against (see "Does it change
  behavior/output?" above).
- The +8-pixel (+0.027%) wobble between AVX2 and SSE4.2 dispatch on `stream_jpx_6` alone (not on
  `stream_jpx_3`/`stream_jpx_5`) is real and reproducible, but tiny and already covered by
  candidates `002`/`008`/`034`'s own documentation of the FMA bulk/remainder split - not
  re-investigated further here since it's two orders of magnitude smaller than the effect that
  actually explains this candidate's numbers.
- Not checked: whether AVX-512 (or AVX2) dispatch produces any *other* observable precision
  difference elsewhere in the render pipeline (`vello_cpu` also uses `fearless_simd`, and both
  `--cfg disable_dispatch_*` flags apply to it too, not just `hayro-jpeg2000`) - out of scope,
  since this candidate's only job was the specific `stream_jpx_3/5/6` hypothesis from candidate
  `008`.
