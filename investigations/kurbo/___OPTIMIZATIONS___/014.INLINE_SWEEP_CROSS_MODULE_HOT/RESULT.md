# 014. Crate-wide sweep for candidate 006's pattern - 9 more `#[inline]` hints

Candidate 006 got -15%/-27% on `flatten` from four plain `#[inline]` attributes on small
`quadbez.rs` functions whose only caller lives in another module, and candidate 007 showed the hint
*regresses* when the body is large (`CubicBez::subsegment`, +5.0% wall clock). 006's own
"what's not done" list asked for a crate-wide sweep for the same pattern; this is that sweep, with
007's size filter applied.

## What was tried

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`,
`valgrind-3.25.1`. Base commit `ca273499e3e48bd2de6f02aa8e99a148984e45f3`.

### How the candidate set was enumerated

Three independent passes, all against the release harness binary:

1. **`perf record`** on 15 scenes (`offset`, `stroke`, `dash`, `simplify`, `fit`, `segments`,
   `intersect`, `bbox`, `area`, `arclen`, `flatten`, `mixed-flatten`, `subdivide`, `deriv`,
   `svg-parse`) - the "small function appearing as its own symbol" tell 006 describes.
2. **`nm -C -S`** on the binary, sorted by symbol size - a kurbo function that still has a symbol
   was not fully inlined anywhere, and the size column is exactly 007's filter.
3. **`objdump -d` + call-site counting** - the decisive one. For every kurbo symbol, how many
   real `call` sites remain in the binary. This distinguishes "has a symbol because one cold
   caller needs it" from "is called for real from a hot path".

Pass 3's result for the small functions (baseline binary, un-inlined call sites):

| function | file | code size | real call sites | callers in another module? |
|---|---|---|---|---|
| `BezPath::push` | `bezpath.rs` | 87 B | 18 | yes (`stroke`, `offset`, `svg`, `fit`, `simplify`, `arc`) |
| `common::solve_quadratic` | `common.rs` | 415 B | 10 | yes (`cubicbez`, `quadbez`, `bezpath`, `offset`, `stroke`, `fit`) |
| `SvgLexer::get_maybe_relative` | `svg.rs` | 94 B | 10 | no (same module, but outlined anyway) |
| `PathSeg::tangents` | `bezpath.rs` | 319 B | 6 | yes (`offset`, `simplify`) |
| `BezPath::close_path` | `bezpath.rs` | 56 B | 4 | yes |
| `BezPath::truncate` | `bezpath.rs` | 11 B | 3 | yes |
| `SvgLexer::get_number_pair` | `svg.rs` | 311 B | 2 | no (same module, outlined anyway) |
| `CubicBez::inflections` | `cubicbez.rs` | 454 B | 1 | yes (`offset`) |
| `Arc::to_cubic_beziers` | `arc.rs` | 3-line body | (see below) | yes (`stroke`, `svg`, `bezpath`) |

**Excluded by 007's size filter**, even though all of them are outlined on a hot path:
`common::solve_cubic` (1124 B, 9 call sites, 17.0% of the `intersect` scene and 10.5% of `fit`),
`simplify::moment_integrals` (1159 B), `CurveDist::eval_ray` / `eval_arc` (1050/738 B, 12.3% of
`fit`), `SimplifyState::flush` (5119 B), `CubicBez::extrema` (512 B, contains a `sort_by` call),
`SvgLexer::get_number` (506 B, dominated by an inner `f64::from_str` call),
`CubicBez::subsegment` (449 B - already refuted by 007), `stroke::extend_reversed` (573 B),
`StrokeCtx::do_line` (256 B, same module, 4 call sites - small enough but not on the measured hot
path).

### The change

**Nine plain `#[inline]` attributes, +17/-0 lines** (9 attributes + 8 lines of comment), no logic
touched, all on private / `pub(crate)` / small-body public functions:
`BezPath::push`, `BezPath::close_path`, `BezPath::truncate`, `PathSeg::tangents`,
`common::solve_quadratic`, `CubicBez::inflections`, `SvgLexer::get_number_pair`,
`SvgLexer::get_maybe_relative`, `Arc::to_cubic_beziers`.

All nine symbols disappear from the patched binary (`nm -C | grep -c` = 1 -> 0 for each), i.e.
every one of them is now inlined at every call site.

### Wall clock (`tools/paired.sh`, `RUNS=15`)

Host load average was 8-15 throughout (5 sibling candidates running); absolute medians drifted
(`stroke` read 11.9 ms in the baseline noise-floor pass and 14.0 ms during the single-core paired
run), which is why only the paired win counts are quoted.

| scene | single-core p50 (`taskset -c 2`) | multi-core p50 (unpinned) |
|---|---|---|
| `simplify` (500 iters) | 0.273 -> 0.216 ms (**-20.8%**), **15/15** `consistent` | 0.274 -> 0.214 ms (**-21.8%**), **15/15** `consistent` |
| `bbox` (2000 iters) | 0.057 -> 0.051 ms (**-9.4%**), **15/15** `consistent` | 0.057 -> 0.051 ms (**-9.4%**), **15/15** `consistent` |
| `svg-parse` (500 iters) | 0.316 -> 0.300 ms (**-5.1%**), **15/15** `consistent` | 0.315 -> 0.299 ms (**-5.1%**), **15/15** `consistent` |
| `dash` (20) | 8.763 -> 8.758 ms (-0.1%), 8/15 `NOT consistent` | 8.880 -> 8.910 ms (+0.3%), 9/15 `NOT consistent` |
| `stroke` (20) | 13.994 -> 13.967 ms (-0.2%), 9/15 | 11.866 -> 11.825 ms (-0.3%), 10/15 |
| `offset` (20) | 14.354 -> 14.399 ms (+0.3%), 7/15 | 12.393 -> 12.420 ms (+0.2%), 5/15 |
| `fit` (20) | 17.962 -> 17.947 ms (-0.1%), 9/15 | 15.667 -> 15.667 ms (-0.0%), 9/15 |
| `svg-write` (200) | 0.835 -> 0.830 ms (-0.6%), 10/15 | 0.675 -> 0.677 ms (+0.3%), 6/15 |
| `intersect` (500) | 0.564 -> 0.559 ms (-0.9%), 12/15 | 0.419 -> 0.418 ms (-0.1%), 9/15 |
| `flatten` (200) | 0.753 -> 0.752 ms (-0.1%), 8/15 | 0.664 -> 0.669 ms (+0.8%), 5/15 |

Baseline noise floor for the three winning scenes, three unpatched passes each
(`taskset -c 2`, 20 iters): `simplify` 0.2218 / 0.2193 / 0.2201 ms (+-0.6%), `bbox` 0.0447 /
0.0446 / 0.0448 ms (+-0.2%), `svg-parse` 0.4116 / 0.3889 / 0.2463 ms - `svg-parse` at 20 iters is
badly unstable, which is why the paired run uses 500 iters. All three wins are far outside their
own noise floor and land 15/15 in both modes.

### Independent reproduction from a fresh clone (this folder's own `just test`)

Cloned at the pinned commit, baseline and patched built from scratch, `RUNS=15`:

| scene | single-core | multi-core |
|---|---|---|
| `simplify 500` | 0.219 -> 0.165 ms (**-24.6%**), 14/15 `consistent` | 0.219 -> 0.166 ms (**-24.3%**), 15/15 `consistent` |
| `bbox 2000` | 0.044 -> 0.042 ms (**-5.2%**), 13/15 `NOT consistent` | 0.045 -> 0.042 ms (**-5.1%**), 15/15 `consistent` |
| `svg-parse 500` | 0.243 -> 0.230 ms (**-5.3%**), 13/15 `NOT consistent` | 0.245 -> 0.231 ms (**-5.4%**), 15/15 `consistent` |
| `dash 20` (control) | 7.545 -> 7.505 ms (-0.5%), 12/15 | - |
| `stroke 20` (control) | 11.982 -> 11.941 ms (-0.3%), 9/15 | - |
| `offset 20` (control) | 12.519 -> 12.514 ms (-0.0%), 8/15 | - |
| `flatten 200` (control) | 0.669 -> 0.675 ms (**+0.9%**), B faster in **1 of 15**, `consistent` | - |

`CHECK` identical on all 25 scenes, all nine symbols gone from the patched binary, binary
8,129,672 -> 8,171,504 bytes (+0.51%). The two directional differences from the in-tree run are
worth stating rather than smoothing over:

- `bbox` reproduces at **-5.2%** here versus **-9.4%** in-tree. The `Ir` win (-3.38%) is the same
  in both, so the in-tree -9.4% is the less reliable of the two figures.
- **`flatten` reads as a small but `consistent` regression (+0.9%, 1 of 15) from the fresh clone,
  and as nothing in-tree** (7/15 and 8/15 on a repeat run of the in-tree binaries). Callgrind on
  the fresh-clone binaries themselves says `Ir` 34,685,484 -> 34,658,869, i.e. **-0.077%**, so the
  extra 6 microseconds are not extra instructions - it is per-build code layout, of the same kind
  as the `dash` finding below, and it is not a reproducible property of the patch. Anyone taking
  this patch and caring about `flatten` should re-measure it on their own build.

### Deterministic instruction counts (`valgrind --tool=callgrind`)

| scene | baseline `Ir` | patched `Ir` | delta |
|---|---|---|---|
| `simplify` (20) | 69,309,731 | 65,174,614 | **-5.97%** |
| `bbox` (100) | 76,679,210 | 74,090,825 | **-3.38%** |
| `svg-parse` (20) | 143,452,294 | 139,584,536 | **-2.70%** |
| `fit` (1) | 488,879,245 | 487,530,825 | -0.28% |
| `fit-opt` (1) | 3,033,204,936 | 3,026,164,876 | -0.23% |
| `area` (2) | 11,575,681 | 11,549,059 | -0.23% |
| `segments` (20) | 7,039,565 | 7,012,945 | -0.38% |
| `mixed-flatten` (5) | 30,036,896 | 29,993,510 | -0.14% |
| `dash` (2) | 370,094,190 | 369,713,833 | -0.10% |
| `flatten` (5) | 34,685,192 | 34,658,697 | -0.08% |
| `stroke` (2) | 594,157,955 | 594,018,857 | -0.02% |
| `stroke-miter` (2) | 593,615,610 | 593,476,150 | -0.02% |
| `offset` (2) | 594,685,581 | 594,631,423 | -0.01% |
| `svg-write` (20) | 256,363,029 | 256,336,544 | -0.01% |
| `arclen` (2) | 24,536,587 | 24,538,312 | +0.01% |
| `intersect` (20) | 112,920,976 | 113,280,336 | **+0.32%** |

### Per-function attribution (deterministic `Ir`, one subset at a time)

Five subsets were each built and measured alone, so the win is attributed rather than assumed:
S1 = the three `BezPath` mutators, S2 = `PathSeg::tangents`, S3 = `solve_quadratic`,
S4 = `inflections`, S5 = the two `SvgLexer` helpers. (This table predates adding
`Arc::to_cubic_beziers`, so its `dash` column is the pre-fix one - see below.)

| scene | S1 | S2 | S3 | S4 | S5 | all five |
|---|---|---|---|---|---|---|
| `simplify` | -0.43% | **-5.30%** | -0.70% | -0.00% | -0.65% | -5.63% |
| `bbox` | -0.04% | -0.00% | **-3.34%** | -0.00% | -0.00% | -3.38% |
| `svg-parse` | -0.02% | -0.00% | -0.11% | -0.00% | **-2.79%** | -2.70% |
| `fit` | -0.58% | -0.00% | -0.22% | -0.00% | +0.00% | -0.28% |
| `intersect` | -0.00% | +0.00% | **+0.32%** | +0.00% | +0.00% | +0.32% |
| `dash` | +2.12% | +1.94% | +2.11% | +2.12% | +2.12% | +2.02% |

So: `simplify`'s win is `PathSeg::tangents`, `bbox`'s is `solve_quadratic`, `svg-parse`'s is the
two SVG lexer helpers, and `inflections` (S4) contributes **nothing measurable anywhere** - it is
in the patch because it structurally qualifies (454 B, `ArrayVec<f64, 2>` return, sole caller in
another module), not because it earns its place.

### The `dash` regression, and why the patch has a ninth attribute

Every one of the five subsets regressed `dash` by ~+2.1% `Ir`, *including S4*, whose only
annotated function (`CubicBez::inflections`) is never called at all in the `dash` scene. That
ruled out a semantic cause. `callgrind_annotate` located it exactly:

- baseline: `Arc::to_cubic_beziers` is inlined into `stroke::round_join` (5,991,520 `Ir` attributed
  to `arc.rs:kurbo::stroke::round_join`);
- patched: `<Arc>::to_cubic_beziers::<round_join::{closure#0}>` reappears as its own 8,312,268 `Ir`
  symbol and `__sincos_fma` grows 9,177,016 -> 10,074,472 `Ir`.

i.e. marking *any* of these functions `#[inline]` shifts codegen-unit partitioning enough to flip
an unrelated inlining decision in `round_join`, and `round_join` is 9.8% of the `dash` scene. A
control build (`#[inline]` on `Triangle::is_zero_area`, a function no scene touches) left `dash`
`Ir` at 370,094,587 vs the baseline's 370,094,474, so it is not "any recompile" - it is specific
to perturbing these modules' CGU contents.

Adding `#[inline]` to `Arc::to_cubic_beziers` (a genuine member of the candidate set on its own
merits: a three-line body, callers in `stroke`/`svg`/`bezpath`, no hint) pins that decision and
removes the regression outright: `dash` +2.018% -> **-0.103%**, and `simplify` improves further
(-5.63% -> -5.97%). Also tried and rejected as redundant: additionally annotating
`ArcAppendIter::next` produced byte-for-byte identical `Ir` on all nine measured scenes, so it was
left out.

### Binary size and allocations

Binary size **grew**: 8,113,432 -> 8,166,632 bytes (**+53,200, +0.66%**). This is the opposite of
006, whose binary shrank - nine inlined functions with 55 total call sites duplicate more code
than they delete.

Allocation counts and byte volumes are identical before and after on every scene checked
(`stroke` 25.1 allocs / 1,361,928 B, `simplify` 39.1 / 769,312 B, `fit` 1173.1 / 318,536 B,
`svg-parse` 10.1 / 114,696 B).

## Result

Three scenes win, measured in both modes with unanimous paired runs and confirmed by deterministic
instruction counts:

- `simplify`: **-20.8% single-core / -21.8% multi-core**, 15/15 in both, `Ir` -5.97%
- `bbox`: **-9.4% / -9.4%**, 15/15 in both, `Ir` -3.38%
- `svg-parse`: **-5.1% / -5.1%**, 15/15 in both, `Ir` -2.70%

Both wins reproduce from a fresh clone, `simplify` at the same magnitude (-24%) and `bbox`/
`svg-parse` at about half (-5%). Seven other scenes (`stroke`, `stroke-miter`, `offset`, `dash`,
`fit`, `svg-write`, `flatten`) are unchanged in wall clock (all between -0.9% and +0.9%, all
`NOT consistent` at 5-12 of 15 pairs in-tree, all within +-0.3% `Ir`) - with the one exception that
`flatten` read +0.9% at 1 of 15 from the fresh clone while being flat in-tree and -0.077% in `Ir`;
see the reproduction section. `intersect` is the one scene that retires *more* instructions
(+0.32% `Ir`) without a matching wall-clock effect (-0.9% / -0.1%, 12/15 and 9/15).

Dimensions measured: time (primary, `paired.sh` RUNS=15, both pinned and unpinned), instruction
count (secondary, deterministic), binary size (+0.66%), allocation count and bytes (unchanged).

**The wall-clock win is again 2-3.5x the instruction-count win** (simplify 3.5x, bbox 2.8x,
svg-parse 1.9x), the same asymmetry 001 and 006 recorded. It is what the mechanism predicts:
`PathSeg::tangents` returns a 32-byte `(Vec2, Vec2)`, `solve_quadratic` returns a 24-byte
`ArrayVec<f64, 2>`, and both SVG helpers return `Result<Point, SvgParseError>` - all four were
round-tripping their return value through memory on every call, which costs cycles that retired
instructions do not see.

I am leaning on the **wall clock** here, not `Ir`: 15/15 in both pinned and unpinned modes on all
three winning scenes is stronger evidence than the `Ir` numbers, and the `Ir` numbers under-report
the effect by 2-3.5x for the reason above. `Ir` is what carried the `dash` diagnosis, where wall
clock could not resolve a 2% effect at host load 15.

## Does it change behavior/output?

- **Bit-exact?** **Yes.** `CHECK` compared across **all 25 harness scenes**: 25 identical, 0
  differing (checked twice - for the eight-attribute version and again for the shipped
  nine-attribute one). `#[inline]` cannot change floating-point results in Rust: there is no
  fast-math, and inlining licenses neither reassociation nor contraction.
- **Is this a breaking change?** No. No signature, no logic, no visibility changed. Six of the nine
  functions are private or `pub(crate)`; the three public ones (`BezPath::push`, `close_path`,
  `truncate`) only gain an attribute. `#[inline]` on a non-generic function puts more into crate
  metadata, which is a small compile-time cost, not a semantic one.
- **Does this folder need a `patch.diff`?** Yes, and it has one - the change is to the tested
  project's own source (`kurbo/src/{arc,bezpath,common,cubicbez,svg}.rs`), +17/-0 lines.
- Project test suite: `cargo test --workspace --all-features --no-fail-fast` -
  **252 passed / 0 failed before, 252 passed / 0 failed after**; 4 `test result` lines both times
  (187 + 10 + 55 + 0, one ignored), `grep -cE '^error'` = 0 both times. No new failures, no
  changed failures.
- `cargo build -p kurbo --no-default-features --features libm`: succeeds, 0 errors.
- `cargo clippy --workspace --all-features`: 1 warning, the pre-existing `unneeded late
  initialization` in `kurbo/src/common.rs`. No new warnings.

**Incidental, unrelated to this patch:** the very first baseline `cargo test` run failed with
`polycool poly::tests::planted_root_deg5` - an `arbtest` randomized test with a time-derived seed
(`0xce849f7300010000`), which found a planted root recovered to 7.1e-10 when the test allows
7.06e-12. Re-running passed. This is a pre-existing flaky test on the unpatched tree, present in
both the before and after runs' code, and is being reported to `___BUGHUNT___`.

## Why the result looks the way it does

Release builds use 16 codegen units, so a function whose callers sit in another module can land in
a different CGU and never be inlined - 006's finding. What 014 adds is *which* functions in kurbo
still had that shape, and how unevenly it pays:

- **`PathSeg::tangents` -> `simplify` -20.8%.** `tangents` is 319 B and returns `(Vec2, Vec2)`,
  four `f64`s through memory. `simplify.rs:345-346` calls it **twice per adjacent segment pair**
  and immediately uses only one component of each result (`last.tangents().1`,
  `seg.tangents().0`), so inlining lets LLVM delete half the work at each site outright, not just
  the call. That is why this one scene moves 20% on a 3-line diff.
- **`solve_quadratic` -> `bbox` -9.4%.** `CubicBez::extrema` calls it twice per cubic (once per
  coordinate) and iterates the returned `ArrayVec<f64, 2>` immediately; inlining SROAs the
  `ArrayVec` away. `solve_quadratic` was the single most-called small outlined function in the
  crate (10 real call sites, all cross-module).
- **The two `SvgLexer` helpers -> `svg-parse` -5.1%.** `get_maybe_relative` is a 94-byte,
  five-line wrapper returning `Result<Point, SvgParseError>` and was called from 10 real call
  sites inside `BezPath::from_svg` - in the *same module*, which is the surprise. Same-module is
  not sufficient for inlining; LLVM's cost model simply declined, and `#[inline]` (an
  `inlinehint`) changes that verdict. So "callers in another module" is a good way to *find*
  candidates but is not the actual precondition.
- **`stroke`/`offset`/`dash`/`fit` do not move**, because their time is in `eval_err`,
  `offset_rec` and `arclen_rec`, and those already sit in the same module as their callers -
  nothing in this patch is on their critical path. `BezPath::push` had the most outlined call
  sites of anything (18), and inlining all of them bought -0.43% `Ir` on `simplify` and -0.58% on
  `fit` and nothing anywhere else: the cost of a `Vec::push` is the store and the capacity check,
  not the call.
- **`intersect` retires 0.32% more instructions** because `PathSeg::intersect_line` calls both
  `solve_quadratic` and `solve_cubic`; inlining only the first grew the function and pushed
  `solve_cubic`'s (excluded, 1124 B) call further from its operands. It does not show up in wall
  clock.

007's size rule held up: every function under ~500 B either helped or was neutral, and the one
place a *large* body got pulled into a hot caller by side effect (`intersect`) is the only
instruction-count regression left in the table.

## What's not done / open questions

- **`common::solve_cubic` (1124 B, 9 outlined call sites, 17.0% of `intersect` and 10.5% of `fit`)
  is the biggest remaining outlined symbol and was deliberately excluded** by 007's size filter.
  It sits right between 006's beneficiaries and 007's regression in size, and it returns an
  `ArrayVec<f64, 3>` through memory, which is 006's mechanism. It deserves its own candidate
  rather than being folded in here on a guess. Same for `CurveDist::eval_ray` (1050 B, 12.3% of
  `fit`) and `simplify::moment_integrals` (1159 B).
- `CubicBez::inflections` is in the patch and does nothing measurable. A maintainer wanting the
  minimal diff can drop it (and its one line); the measurement quoted here includes it.
- The `Arc::to_cubic_beziers` attribute is load-bearing for the `dash` number but for a
  layout/CGU-partitioning reason, not a semantic one. That means the exact set of nine is tuned
  against `rustc 1.98.0` with 16 CGUs; a different toolchain or `codegen-units` setting could
  re-flip `round_join`'s inlining. The fresh-clone `flatten` reading (+0.9% at 1 of 15, against
  -0.077% `Ir`) is the same class of effect showing up somewhere else. The three scene wins are
  mechanism-driven and reproduce across builds; the sub-1% wobble on scenes this patch does not
  touch is build-specific and would want re-measuring per toolchain.
- A cheap follow-up nobody has run: build the whole crate with `codegen-units = 1` and re-measure
  the same scenes. If `simplify`/`bbox`/`svg-parse` win anyway, that is a much simpler upstream
  answer than 13 hand-placed attributes across 006 + 014, and it would also settle how much of
  this class of win is CGU-boundary artifact versus real inlining benefit.
- Not measured stacked with 006 or 009 - like every candidate in this workspace, this was measured
  against the pristine base commit. 006 (`quadbez.rs`) and 009 (`PathSeg::bounding_box`) touch
  disjoint functions, but 009 and this candidate's `solve_quadratic` hint both target the `bbox`
  scene, so their -6..-10% and -9.4% will not simply add.
- `#[inline(always)]` was not tried, on 007's evidence.
- Untried small outlined functions left on the table: `StrokeCtx::do_line` (256 B, 4 call sites,
  same module), `StrokeCtx::reset` (68 B), `PathSeg as ParamCurve>::eval` (211 B),
  `<BezPath as Shape>::{area,perimeter,bounding_box}` (29-34 B pure forwarders to the slice impl,
  called once per scene iteration so unmeasurable here but free for downstream users),
  `BezPath::apply_affine` (297 B).
