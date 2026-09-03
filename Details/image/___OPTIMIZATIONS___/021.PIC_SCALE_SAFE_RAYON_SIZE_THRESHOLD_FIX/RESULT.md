# pic-scale-safe rayon size-threshold fix - call-site-only attempt

Candidate 019 confirmed a real, small regression when pic-scale-safe's `rayon` feature is
forwarded (candidate 002's Cargo.toml change): at `400x300->32x24`, `Triangle` and (single-core
only) `Lanczos3` were slower with pic-scale-safe's internal `par_chunks_exact` dispatch enabled
than without it. `IDEAS.md` row 36 asked whether pic-scale-safe's public API exposes any
non-rayon-forced entry point `src/imageops/resize.rs` could call below a size threshold, so the
fix could land as a same-repo call-site change instead of a full vendor-and-patch of
pic-scale-safe. This candidate answers that question by reading pic-scale-safe 0.1.11's source
(`resizer.rs`, `fixed_point_dispatch.rs`, `floating_point_dispatch.rs`) and then tries the one
call-site-only workaround that doesn't require touching pic-scale-safe's own code: routing small
resizes through a cached single-thread `rayon::ThreadPool` via `.install()`, which forces any
`par_chunks_exact`/`par_chunks_exact_mut` nested inside pic-scale-safe onto one worker without
changing pic-scale-safe's source at all.

**Finding on the API question itself: no.** pic-scale-safe has no runtime parameter for
choosing serial vs. parallel, or a thread count, on any of its public `resize_*` functions
(checked all 12 signatures in `resizer.rs`). The `rayon`/non-`rayon` split is entirely a
compile-time `#[cfg(feature = "rayon")]` branch baked into `fixed_point_dispatch.rs` /
`floating_point_dispatch.rs` / `resize_nearest.rs` / `trc_handler.rs` - once the feature is
compiled in, every call always takes the `par_chunks_exact` branch, unconditionally, with no
per-call opt-out and no internal size gate of its own.

## What was tried

This candidate's patch bundles three changes (see `justfile` header for why): candidate 002's
`pic-scale-safe/rayon` Cargo.toml forward (needed for there to be any regression to fix at all),
candidate 019's `bench_resize_dynamic` criterion group (the harness), and this candidate's own
change - a `SMALL_RESIZE_SOURCE_PIXEL_THRESHOLD = 500_000` (source `width * height`) gate in
`src/imageops/resize.rs`: below it, the whole match/`pic_scale_safe::resize_*` call is run inside
a lazily-built (`OnceLock`), cached single-thread `rayon::ThreadPool::install()`; at or above it,
unchanged. 400x300 source = 120,000 px (below); 4032x3024 source = 12,193,536 px (above).

"A (unfixed_parallel)" = candidate 002+019's state only (Cargo.toml forward + bench, no
threshold). "B (fixed)" = A plus this candidate's threshold gate. 10 paired runs for the small
scenarios, 4-6 for the large control (matches this workspace's usual reduced-repeat-count
convention for control cases). `taskset -c 3` for single-core; unpinned for multi-core (24
logical cores, `uptime` load average 0.78-17.65 during this session - elevated at times but this
workspace's `projects/image.md` already documents that `/proc/loadavg` here is host-wide, not
container-scoped, and not by itself a sign of contention affecting these pinned/alternating
runs).

| Variant / scenario | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | B-faster pairs |
|---|---|---|---|
| A, 400x300->32x24 Triangle | 75.372 us | 69.506 us | - |
| B, 400x300->32x24 Triangle | 75.535 us (+0.2%) | 71.378 us (+2.7%) | single: 4/10; multi: 1/10 |
| A, 400x300->32x24 Lanczos3 | 209.240 us | 141.320 us | - |
| B, 400x300->32x24 Lanczos3 | 208.990 us (-0.1%) | 208.260 us (**+47.4%**) | single: 6/10; multi: 0/10 |
| A, 4032x3024->400x300 Lanczos3 (large control) | (not measured single-core this candidate) | 4888.100 us | multi: 3/6 |
| B, 4032x3024->400x300 Lanczos3 (large control) | | 4905.700 us (+0.4%) | |
| A, 4032x3024->64x48 Triangle (large-source control) | | 3851.900 us | multi: 2/4 |
| B, 4032x3024->64x48 Triangle (large-source control) | | 3930.000 us (+2.0%) | |

Raw paired data saved under `raw/` (`small_triangle_singlecore.txt`,
`small_triangle_multicore.txt`, `small_lanczos_singlecore.txt`, `small_lanczos_multicore.txt`,
`large_lanczos_multicore.txt`, `large_to_small_triangle_multicore.txt`).

## Result

**The fix does not help, and for `Lanczos3` it is a large, consistent, reproducible
regression relative to already-parallel code.** On single-core (`taskset -c 3`), both filters are
a wash (4/10 and 6/10 pairs, deltas within ±0.2%) - expected, since pinning the whole process to
one core already forces all of A's worker threads onto that one core regardless, so A and B end
up doing similar work on the same core either way.

On multi-core (unpinned, the realistic configuration): `Triangle` is a small, consistent loss for
the fix (+2.7%, only 1/10 pairs favoring B) - not a fix for candidate 019's small `Triangle`
regression, just a different, smaller regression on top of the already-regressed state. `Lanczos3`
is a **severe, unanimous regression** (+47.4%, 0/10 pairs favoring B, from ~141 us to ~208 us) -
worse than what candidate 019 characterized as the "regressed" state to begin with, because
candidate 019 found `Lanczos3` at this exact size was *not* regressed on multi-core (parallel was
faster than serial there, 8/10 pairs). This patch's blanket, filter-blind size gate throws that
existing multi-core win away along with the (filter-dependent) problem it was meant to fix.

The large-source control cases (`4032x3024->400x300` and `4032x3024->64x48`, both far above the
500,000-pixel threshold) are unaffected either way (+0.4%/+2.0%, both within this workspace's
established noise floor, 3/6 and 2/4 pairs) - confirming the threshold correctly leaves normal-size
resizes on the untouched code path, so the regression above is specific to the small-size branch's
own new code, not a side effect of the gate itself misfiring on large inputs.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_021.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed
  (`DefaultHasher`, over pixel bytes and dimensions) `DynamicImage::resize_exact` output for all 5
  `FilterType` variants across four scenarios (`RgbImage` and `RgbaImage` sources at
  `400x300->32x24`, `4032x3024->64x48`, `4032x3024->400x300`), built once with only the
  Cargo.toml+bench changes (A) and once with this candidate's threshold gate added (B). All 20
  hashes matched exactly between the two builds (see `raw/hashes_fixed.txt` /
  `raw/hashes_unfixed.txt` - identical apart from the unrelated compile-timing line) - expected,
  since routing through a different rayon thread pool changes nothing about which chunks compute
  which output bytes.
- **Is this a breaking change?** No public API changes; behavior only differs in wall-clock time,
  not output, and only when the `rayon` feature is enabled (not currently forwarded into
  pic-scale-safe on `main` - this candidate's Cargo.toml change re-applies candidate 002's
  proposed forward, still not itself applied to `main`).
- Project test suite: 550/551 passed, both before (Cargo.toml forward + bench only, no threshold
  gate) and after (full patch), with the same single pre-existing unrelated failure both times
  (`resizing_with_catmul` in `tests/regression.rs` - same failure noted in candidates
  007/008/011/016/019/020's `RESULT.md`s, not touched by this candidate).

## Why the result looks the way it does

Routing the *entire* `pic_scale_safe::resize_*` call through `ThreadPool::install()` on a
dedicated single-thread pool does not reproduce "serial, no rayon at all" performance - it still
pays rayon's task-dispatch/park-wake machinery to hand the whole call to the pool's one worker
thread and block the caller until it's done, on top of losing whatever genuine multi-core
parallelism the call would have gotten by staying on the default (larger) global pool. For
`Triangle` (cheap kernel, ~70-75 us of actual work at this size) that dispatch/hand-off overhead
is a small, roughly fixed cost relative to the work, showing up as a small consistent loss
(+2.7%). For `Lanczos3` (wider kernel, ~140-210 us of work at this size), the *default* multi-core
path was already meaningfully parallelizable even at 120,000 source pixels - candidate 019 found
multi-core `Lanczos3` here was *not* regressed by enabling rayon (8/10 pairs favored the default
parallel path over serial). Forcing this candidate's single-thread pool throws that real
parallel speedup away, which is why `Lanczos3`'s regression (+47%) is far larger in both relative
and absolute terms than `Triangle`'s: the size threshold used here (source pixel count only) is
filter-blind, but candidate 019 already established the underlying regression is
filter-dependent (`Triangle` regresses, `Lanczos3` mostly doesn't) - gating on size alone
conflates the two and actively hurts the filter that didn't have a problem to begin with.

## What's not done / open questions

- A **filter-aware** gate (only routing `Triangle`-like cheap-kernel filters through the
  single-thread pool, leaving `Lanczos3`/`CatmullRom`/`Gaussian` on the default path
  unconditionally) was not tried - candidate 019's own finding, and this candidate's `Lanczos3`
  result, both point at this being necessary for any call-site fix to not actively regress the
  costlier filters. This is the most promising next step if a call-site-only fix is still wanted.
- A capped-but->1 thread count (e.g. `num_threads(2)` or `num_threads(4)` instead of 1) for the
  small-size branch was not tried - might recover some of `Lanczos3`'s lost parallelism while
  still cutting down on however many threads the default global pool would otherwise recruit for
  a tiny job; not measured.
- Since the public API offers no way to avoid pic-scale-safe's own dispatch overhead from outside
  its source (confirmed above), a fix that actually reduces dispatch overhead - rather than just
  changing how many threads absorb it - would still need vendoring pic-scale-safe per SKILL.md's
  "vendoring a suspect dependency" guidance (e.g. adding a real minimum-chunk-count gate inside
  `fixed_point_dispatch.rs`/`floating_point_dispatch.rs` itself, falling back to the existing
  serial loop for small inputs instead of dispatching to any thread pool at all). Not attempted
  this session.
- The 500,000-source-pixel threshold itself was not bisected (same open item as candidate 019 /
  `IDEAS.md` row 35) - moot for this candidate's own conclusion (the mechanism regresses
  `Lanczos3` regardless of where exactly the cutoff sits), but still open for any future
  filter-aware attempt.
- Single-core numbers for the two large control scenarios were not collected (multi-core is the
  configuration that matters for this mechanism, per the "why" section above, and single-core was
  already shown to be a wash for both small scenarios).
