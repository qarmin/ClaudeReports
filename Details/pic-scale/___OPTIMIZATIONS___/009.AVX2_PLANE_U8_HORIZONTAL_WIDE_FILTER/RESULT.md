# Re-measuring candidate 003's AVX2 horizontal kernel for planar (1-channel) u8 under a genuinely wide Lanczos3 filter

Candidate 003 added `src/avx2/plane_u8.rs` (an AVX2 tier for `src/factory/plane_u8.rs`'s x86
horizontal-convolution dispatch, widening the SSE4.1 kernel's "zero-extend to i16, `madd_epi16`"
trick from 8 taps/instruction to 16) and measured a real win (-14% to -20%) on the existing
`app/benches/resize_plane/main.rs` `"Pic scale Plane8(Quality): Lanczos 3"` case, which only
resizes to 1/4 size - a filter width (`kernel_size`) that, per the same math candidates 006/007
already probed for `plane_u16_lb`, tops out around 24 taps for a 4x downscale (Lanczos3's
`min_kernel_size=3` support radius scaled by the downscale ratio), so the new 16-tap main loop
fires at most once per output pixel there. `IDEAS.md` flagged this as untested for plane/CbCr u8
specifically, after candidates 007/008 found that whether an AVX2 widening's win survives a wide
(`kernel_size~96`) filter depends on kernel shape (interleave-shuffle-removal wins hold; pure
width-doubling wins vanish). This candidate closes that gap for the plane (CN=1) u8 case: it adds
a new criterion case (`"Pic scale Plane8 wide filter: Lanczos 3"`, same as the existing
`"Pic scale Plane8(Quality): Lanczos 3"` case but downscaling to 1/16 instead of 1/4) and
re-measures candidate 003's unmodified patch against it.

## What was tried

Only one variant: candidate 003's existing AVX2 kernel + factory wiring, unchanged (`patch.diff`,
byte-identical to `003.AVX2_PLANE_U8_HORIZONTAL/patch.diff`), measured against a new bench case
(`bench_addition.diff`, adds `"Pic scale Plane8 wide filter: Lanczos 3"` to
`app/benches/resize_plane/main.rs` - `u8`, `WorkloadStrategy::PreferQuality` (same strategy as the
existing case this mirrors), `ThreadingPolicy::Single`, nasa asset resized to 1/16 in both
dimensions instead of 1/4). `app/Cargo.toml`'s `pic-scale` feature list was given `"avx", "sse"`
for both binaries (not part of either diff, reverted after measurement). Two prebuilt
`resize_plane` bench binaries (baseline = pristine `plane_u8.rs`/`factory/plane_u8.rs`/
`avx2/mod.rs` + this candidate's new bench case; patched = baseline + candidate 003's patch) were
compared with `tools/paired.sh` (`RUNS=9`, alternating A/B, point estimate = criterion's reported
mean), run from `app/` (the bench opens `../assets/...` relative to cwd).

| Variant | median (`taskset -c 3`) | median (unpinned) | notes |
|---|---|---|---|
| baseline vs. AVX2 (candidate 003's patch), pinned repeat 1 (load ~11 at start) | 4.008 -> 3.892 ms (**-2.9%**), 5/9 | - | "NOT consistent" |
| same, pinned repeat 2 (load dropped to ~3-4) | 1.953 -> 3.515 ms (**+79.9%**), 5/9 | - | "NOT consistent" - contaminated by a bimodal ~1.9ms/~3.9ms clustering visible in the raw pairs, not attributable to the patch (both A and B hit both modes) |
| same, pinned repeat 3 | 1.923 -> 1.892 ms (**-1.6%**), 5/9 | - | "NOT consistent" |
| same, pinned repeat 4 | 1.986 -> 2.139 ms (**+7.7%**), 3/9 | - | "NOT consistent" |
| same, unpinned repeat 1 | - | 2.273 -> 1.926 ms (**-15.3%**), 9/9 | "consistent" |
| same, unpinned repeat 2 | - | 2.271 -> 1.899 ms (**-16.4%**), 9/9 | "consistent" |

`uptime` load ranged from ~11 (start of session) down to ~3-4 (later pinned repeats) across these
6 runs. All 4 pinned (`taskset -c 3`) repeats were "NOT consistent" per `paired.sh`'s own
8/9-or-better rule, with small and inconsistently-signed deltas (-2.9%, +79.9% contaminated,
-1.6%, +7.7%) - one repeat's raw samples clustered bimodally between a ~1.9ms and a ~3.9ms mode
for *both* A and B within the same 9-run sequence, the same host-level noise pattern already
documented in this workspace's `README.md`/`projects/pic-scale.md` for `resize_cbcr`, now also
seen on core 3 for `resize_plane`'s wide-filter case specifically. Both unpinned repeats, by
contrast, were clean and decisively "consistent" at essentially the same magnitude as candidate
003's original narrow-filter measurement (-14% to -20%).

Raw pairs (ms), pinned repeat 2 (the contaminated one):
`A: 1.9411 3.943 1.9603 3.2598 3.9184 1.9033 1.9041 1.9532 1.8981` /
`B: 4.1893 3.8203 3.8738 3.5147 3.8261 1.859 1.8766 1.914 1.9374`

Raw pairs (ms), unpinned repeat 1:
`A: 2.2735 2.2612 2.2725 2.2792 2.2759 2.2248 2.2656 2.2742 2.2848` /
`B: 1.9268 2.0633 1.9104 1.9469 1.9179 1.9336 1.9258 1.926 1.8968`

Raw pairs (ms), unpinned repeat 2:
`A: 2.2193 2.2436 2.2151 2.354 2.2713 2.6933 2.269 2.2968 2.3212` /
`B: 1.8948 1.8985 1.8965 1.8917 2.2234 1.9475 1.9812 1.8988 1.9509`

## Result

Mixed, split by measurement mode: both **unpinned** repeats show a decisive, consistent win at
essentially the same magnitude as candidate 003's original narrow-filter measurement (**-15.3%
and -16.4%**, both 9/9 "consistent") - the win holds under the wide filter. All 4 **pinned**
(`taskset -c 3`) repeats were "NOT consistent", with small and inconsistently-signed point
estimates (-2.9% to +7.7%, one contaminated by a bimodal noise mode unrelated to the patch) - the
pinned-core measurement for this specific bench case did not produce a usable signal on this box
during this session, regardless of direction.

## Does it change behavior/output?

- **Bit-exact?** Yes. Candidate 003's own `bit_exact_check/` crate (copied unchanged into this
  candidate's folder, `Cargo.toml`'s `pic-scale` path-dependency repointed at this checkout - 11
  size/scale/strategy combinations sweeping tap counts through 1/2/3/4/5/7/8/9/16/17+ per output
  pixel), built once against pristine `plane_u8.rs`/`factory/plane_u8.rs`/`avx2/mod.rs` and once
  with candidate 003's patch applied (same checkout, patch applied/reverted via `git apply`/
  `git apply -R` in between, `--release`): both runs printed the identical
  `FINAL_HASH=11635962358689872026` - the same hash candidate 003 recorded - and identical
  per-case `first8=[...]` samples.
- **Is this a breaking change?** No - internal dispatch only, no public API change. The new
  criterion case is bench-only, not part of the library's public surface.
- Project test suite (`cargo test -p pic-scale --features avx,sse,colorspaces,rdm --lib`): **66
  passed, 0 failed** before (pristine `plane_u8.rs`) and **66 passed, 0 failed** after (candidate
  003's patch applied) - identical counts, no new failures.

## Why the result looks the way it does

The win itself is expected to hold at the same magnitude regardless of filter width, per candidate
007's finding for the *structurally different* `plane_u16_lb` case: 007 found that a pure
width-doubling widening (no interleave-shuffle to collapse) showed a genuine null result at both
narrow and wide filter widths on this box, while candidate 003/004's u8 kernels are a different
shape again - `plane_u8`'s SSE kernel already does a plain zero-extend (no shuffle), same shape
candidate 007 called out as the "doesn't help" pattern for `u16`, yet this candidate's *unpinned*
measurement shows a clear, consistent win at both filter widths. This candidate's own contribution
is the negative finding about pinned-core measurement, not a new claim about the kernel: `taskset
-c 3` repeatedly failed to produce a decisive signal for this specific bench case+box combination
this session (regardless of load, which varied ~3x across the 4 pinned repeats), while unpinned
wall-clock (this workspace's stand-in for "realistic multi-core" per `README.md`'s ground rules)
was clean and decisive both times. This is consistent with, but a stronger instance of, the
bimodal per-sample clustering this workspace's `README.md`/`projects/pic-scale.md` already
documented for `resize_cbcr`'s pinned runs - here it showed up for `resize_plane`'s *wide-filter*
case specifically (not the original narrow-filter case candidate 003 measured, which had clean
pinned results).

## What's not done / open questions

- This closes the specific gap flagged in `IDEAS.md` for `resize_plane`/CN=1 u8, with a caveat:
  the win is confirmed to hold under a wide filter via the unpinned measurement, but the pinned
  single-core measurement for this exact case was inconclusive on this box this session - someone
  with a quieter/dedicated box (or `perf stat` instruction counts instead of wall-clock) could
  settle whether that's just this session's noise or something specific to pinning core 3 for
  this particular bench case.
- The analogous gap for CbCr (CN=2) u8 is addressed separately in
  `010.AVX2_CBCR8_HORIZONTAL_WIDE_FILTER/`.
- Not measured on hardware without AVX2 (dispatch already falls back to SSE4.1/scalar correctly
  there; no behavior change expected, not specifically re-verified this session).
- The new `"Pic scale Plane8 wide filter: Lanczos 3"` bench case itself remains uncommitted
  (`bench_addition.diff` in this folder, reapplied by this candidate's own `justfile`) - per this
  workspace's rules, adding it to the real `app/benches/resize_plane/main.rs` permanently is a
  separate, later decision for whoever reads this report.
