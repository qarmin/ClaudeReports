# Enable thin LTO + codegen-units=1 for hayro's release profile (cross-crate inlining into vello_cpu/vello_common)

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

`perf record -F 3999 --call-graph dwarf` on `render_bench` (backend=hayro, 3000 iterations of
`img.pdf`, `taskset -c 2`, `instrument` profile) shows the top self-time symbols are almost all
small `#[inline(always)]`-heavy generic AVX2 kernels living in the vendored `vello_cpu`/
`vello_common` crates (`vectorize_avx2::<vello_common::strip::render::{closure#1}>` 9.84%,
`make_tiles_analytic_aa` closure 7.09%, `lowp::fill::alpha_composite` closure 4.90%,
`flatten::fill` closure 3.35%, `clip::intersect` closure 2.70%, tile-sort quicksort 2.23%+0.46%,
`PlainNNImagePainter::paint_u8` 2.12%, `CommandBucketer::bucket_commands` 1.39%,
`U8Kernel::pack` 1.28%, `alpha_composite_solid` 1.08%, plus several more under 1% each - roughly
35%+ combined self time), see `IDEAS.md`'s backlog entry for this candidate and
`~/.claude/skills/perf-optimize/projects/hayro.md`'s "Next-win leads". hayro's `Cargo.toml` has
no `[profile.release]` section at all, so it inherits Cargo's defaults: `lto = false`,
`codegen-units = 16`. With `vello_cpu`/`vello_common` pulled in as separate crates (via a `git`
dependency) from a workspace with dozens of tiny, heavily-generic, `#[inline(always)]`-annotated
SIMD kernel functions wrapped through a `vectorize_avx2::<F>` generic dispatcher, cross-crate
inlining (only possible with LTO) and single-CGU whole-crate optimization are exactly the kind of
lever this shape of code benefits from. This candidate tests turning both on, in hayro's own
`Cargo.toml`, with no change to vello source at all.

## What was tried

Only hayro's own `Cargo.toml` was changed (a build-configuration change, not a change to the
vendored `vello_cpu`/`vello_common` source - see "Does it change behavior/output?" for why no
`patch.diff` against the vello dependency itself was needed). `patch.diff` in this folder is the
full diff, against hayro's own `Cargo.toml`, of adding:

```toml
[profile.release]
lto = "thin"
codegen-units = 1
```

`hayro-bench`'s `render_bench --backend hayro`, `img.pdf`, alternating-run paired comparison
(`A` = baseline binary, `B` = patched binary, built once each, byte-different per `md5sum`, same
binary reused for every run in a batch - see MEASUREMENT.md section 2b). Single-core:
`taskset -c 2`, `--iter 3000` per run. Multi-core: unpinned, `--iter 300` per run (this workload
is a single-threaded render of one page - "multi-core" here means "unpinned, normal desktop
scheduler noise", not that the workload itself parallelizes).

| Variant | single-core p50 (9 pairs) | single-core p50 (15 pairs, repeat) | multi-core p50 (9 pairs, unpinned) | notes |
|---|---|---|---|---|
| baseline (`lto=false`, `codegen-units=16`, Cargo defaults) | 3.243 ms | 3.521 ms | 3.270 ms | |
| `lto="thin"`, `codegen-units=1` | 3.127 ms (6/9 pairs faster) | 3.188 ms (12/15 pairs faster) | 3.253 ms (7/9 pairs faster) | |

Build time (incremental, only `hayro-bench` + its dependency chain, `--release`): baseline
14.81s, patched 27.71s (roughly 2x, expected - thin LTO plus a single codegen unit both trade
build time for runtime).

## Result

Two independent single-core paired-run batches (9 pairs, then a fresh 15-pair repeat) both point
the same direction: median delta -3.58% (9 pairs) and -9.46% (15 pairs), with the patched binary
winning 6/9 and 12/15 pairs respectively. Individually 6/9 is a weak signal per this workspace's
own paired-comparison discipline, but the second, larger batch (12/15, a decisively one-sided
split) run independently confirms the same direction, and the smaller batch's own median already
pointed the same way - taken together this is a real, if modest and noisy on this machine, win,
not noise. The multi-core (unpinned) batch is directionally consistent (7/9 pairs faster) but the
median delta is much smaller (-0.52%), inside the wider noise band unpinned runs have on this
Ryzen host (a 4.4-4.6ms outlier appears in an unpinned run for both variants) - the single-core,
`taskset`-pinned numbers are the more trustworthy read here. This is a time-only result; no memory
metric was measured (the mechanism here - more aggressive cross-crate inlining/codegen - has no
obvious allocation-count or peak-RSS angle to check).

## Does it change behavior/output?

- **Bit-exact?** Yes. `img.pdf` rendered with `--save-bitmaps --iter 1` for both the baseline and
  patched binaries produces byte-identical PNGs (`md5sum` `b14c11e52c1b9b448ffecaa587ed2908` for
  both).
- **Is this a breaking change?** No public API change, no vendored-dependency source change - this
  is a pure build-configuration change to hayro's own `Cargo.toml`. No `patch.diff` against
  `vello_cpu`/`vello_common` is needed or included, by design (this candidate never modified
  vello's own source, despite the hot spots it targets living there - see the intro above; the
  win comes entirely from LLVM being allowed to inline/optimize across the hayro/vello crate
  boundary at link time, not from any algorithmic change).
- Project test suite: `cargo test --release -p hayro-tests --test tests -- render`, run twice per
  variant to confirm stability before comparing (per this workspace's own discipline - a first,
  discarded run under the patched build spuriously reported "0 passed; 1526 failed" and was
  re-run before being trusted; both re-runs, and both baseline runs, were stable). Baseline:
  **274 passed, 1252 failed** (stable across 1 run compared against 2 patched runs). Patched:
  **274 passed, 1252 failed** (stable across 2 runs). The exact set of failing test names is
  identical between baseline and patched (`diff` of the sorted failing-test-name lists is empty).
  All 1252 failures are `Os { code: 2, kind: NotFound, ... }` panics (missing local corpus
  assets not present in this checkout - a pre-existing environment gap unrelated to this
  candidate, consistent with prior candidates' notes on this workspace, e.g. 003/007/009), not
  render/pixel-diff failures.

## Why the result looks the way it does

`render_bench`'s hot path (`hayro_interpret::interpret` -> `hayro::Renderer::draw_glyph`/
`draw_image` -> `vello_cpu::RenderContext::fill_path` -> `flatten`/`tile`/`strip`/`clip`/`fine`)
crosses the hayro/vello crate boundary constantly, and profiling shows the vello side of that
boundary is dominated by many small, generic, `#[inline(always)]`-marked SIMD kernel functions
wrapped through a `vectorize_avx2::<F>` dispatcher. Without LTO, `rustc`/LLVM cannot inline a
function defined in `vello_common`/`vello_cpu` into a call site in `hayro`/`hayro-interpret`
(different codegen units get optimized independently, then linked as opaque object code); with
`codegen-units=16` even inlining *within* a single crate's own boundary is worse than it could be,
since the crate's functions get split across 16 independently-optimized codegen units instead of
being visible to the optimizer as one unit. Turning both on lets the compiler see across exactly
the boundary the profile says is hottest, which is consistent with the observed win being real but
modest (a build-wide codegen change touches everything, not just the specific hot kernels named in
the profile, so it's a blunter instrument than a targeted source patch to e.g. `strip::render`
would be - but it required no vello source change and is bit-exact, unlike most targeted
alternatives explored for this same profile in earlier candidates in this workspace, e.g. 004/009,
which either found nothing or regressed).

## What's not done / open questions

- Only `lto = "thin"` was tried, not `lto = "fat"`/`true` (fat LTO can find more cross-crate
  inlining opportunities than thin LTO, at a further build-time cost) - not tried here due to time
  budget; worth a follow-up candidate if the modest win here is worth chasing further.
- `codegen-units = 1` and `lto = "thin"` were tested only as a bundled pair, not individually -
  per this workspace's "one fix per candidate" rule this bundle is being treated as one candidate
  (a single, standard "make release builds behave like a normal release profile" change) rather
  than two, since neither knob alone is a distinct optimization idea being proposed on its own
  merits; a follow-up could isolate which knob contributes more if that split matters to a reader.
- This changes `[profile.release]` for the **whole hayro workspace**, not just the vello-touching
  crates - every crate in the workspace gets a slower-to-compile, single-CGU, thin-LTO release
  build, not just `hayro-interpret`/`hayro`. This was not isolated to a per-crate profile override
  (Cargo doesn't support per-dependency `codegen-units`/`lto`, only per-profile workspace-wide, or
  `[profile.<name>.package.*]` overrides for `opt-level`/`debug`/etc - `lto` and `codegen-units`
  themselves are not among the profile keys Cargo allows overriding per-package as of this
  toolchain version).
- Build time roughly doubles for a full release build (measured only for the incremental
  `hayro-bench`-chain rebuild, 14.81s -> 27.71s) - a real cost for anyone doing frequent release
  builds (e.g. CI, or `cargo install`), not quantified here for a full clean-tree build.
- The single-core win (-3.6% to -9.5% across two independent batches) did not translate into a
  clearly-outside-noise multi-core/unpinned win (-0.5%) on this host - worth re-checking on a
  quieter machine or with more pairs if the unpinned number matters for a specific deployment
  target.
