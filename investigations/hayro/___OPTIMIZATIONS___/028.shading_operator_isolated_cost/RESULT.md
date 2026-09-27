# Isolating the `Shading` (`sh` operator) match arm's own `fill_path_impl` cost from the Form-XObject-recursion confound

Backlog entry from `../IDEAS.md` (added by candidate 017): candidate 017 found that
`xobject_misc` (the shared bucket `TypedInstruction::Shading` and `TypedInstruction::XObject`
both fall into) balloons to ~40-44% of total `render()` time on Form-XObject-heavy PDFs, and
attributed almost all of that to the Form-recursion double-counting confound (a Form's own `Do`
call's elapsed time re-attributing everything the Form draws to the parent bucket) rather than to
genuine `sh`-operator prep cost - but had no PDF in hand with Shadings and (near-)zero Forms to
confirm the `Shading` arm's own direct cost (it calls `fill_path_impl` directly, bypassing the
`fill_path`/`fill_stroke_path` wrappers that `draw_path`'s own instrumentation subtracts out) is
actually small on its own. This candidate measures that directly, using
`hayro-tests/pdfs/custom/shading_operator_1.pdf` (1 Form, 1 Shading - the file candidate 017 named
as the closest-to-zero-Form option in the existing corpus; `shading_operator_2.pdf` is the only
other file using the `sh` operator directly rather than through a pattern, and is smaller/simpler).

**No patch to real source was found or attempted, and none is proposed** - like candidates 011/017,
this is a measurement/characterization using additive, reverted `Instant`+`AtomicU64`
instrumentation. `patch.diff` in this folder is that instrumentation (candidates 007/010/011's
existing bucket instrumentation plus one new atomic/timer this candidate adds), included so
`just test` can reproduce the exact measurement - not a proposed source change.

## What was tried

### Step 1: reapply the existing bucket instrumentation

`interpret_dispatch_instrumentation.diff` and `pipeline_breakdown_profiling_instrumentation.diff`
(copies of `../_reference/`'s files) both still applied cleanly against current `HEAD`
(`5a5f0e24`, unchanged since candidates 007/010/011/017).

### Step 2: add a new timer isolating just the `Shading` arm's own `fill_path_impl` call

`__bucket_for` classifies both `TypedInstruction::Shading` and `TypedInstruction::XObject` into
the same `XObjectMisc` bucket (`hayro-interpret/src/interpret/mod.rs:288-289`), so the existing
`xobject_misc` timer can't tell the two apart. Added one new atomic
(`__PHASE_SHADING_FILL_NS` in `hayro-interpret/src/lib.rs`) and wrapped only the `fill_path_impl`
call inside the `Shading` match arm (`hayro-interpret/src/interpret/mod.rs`, the `TypedInstruction::Shading(s) =>` arm) with an `Instant::now()`/`fetch_add`, mirroring the existing
`__PHASE_SHOW_TEXT_NS` pattern. This isolates the `sh` operator's own direct fill cost regardless
of any parent Form's `Do`-call timer, without touching the real dispatch logic. Extended
`hayro/examples/bench_ops_reference.rs` (copy of `../_reference/bench_ops_reference.rs`) to print
`shading fill_path_impl only` and `xobject_misc bucket minus shading fill_path_impl` alongside the
existing candidate-007/011 lines.

### Step 3: pick and copy the test PDF

`shading_operator_1.pdf` (47KB, `/ShadingType 2`, i.e. an axial/linear gradient, referenced via the
`sh` operator inside a 1-Form appearance-stream-shaped content stream) - copied into this folder's
`testdata/`. Confirmed via `strings`: 1 `/Subtype /Form`, 1 `/ShadingType`.

### Step 4: measurement

`taskset -c 2`, 40 iterations, `hayro/examples/bench_ops_reference.rs`, three runs (two pinned
back-to-back for a repeat, one unpinned for the multi-core-availability check). Full raw output
saved at `breakdown_runs.txt` in this folder.

| Run | total render() | `xobject_misc` (whole bucket) | shading `sh` fill_path_impl only | `xobject_misc` minus shading fill (Form/InlineImage/marked-content, incl. Form-recursion double-count) |
|---|---:|---:|---:|---:|
| 1 (`taskset -c 2`) | 152.828ms | 202.658ms (132.6%) | 100.684ms (**65.9%**) | 101.974ms (**66.7%**) |
| 2 (repeat, `taskset -c 2`) | 152.734ms | 202.992ms (132.9%) | 100.869ms (**66.0%**) | 102.123ms (**66.9%**) |
| 3 (unpinned, multi-core) | 163.897ms | 217.482ms (132.7%) | 108.029ms (**65.9%**) | 109.454ms (**66.8%**) |
| justfile smoke-test (fresh clone, `taskset -c 2`) | 153.303ms | 204.667ms (133.5%) | 101.696ms (**66.3%**) | 102.971ms (**67.2%**) |

(`uptime` load average was 4.7-5.9 on this 24-thread machine throughout - ambient noise explains
the run-3 absolute-ms bump, not a code difference; all four runs agree on the percentage split to
within about half a point, same discipline as candidate 017.)

### Step 5: correctness check

Compared `./target/release/examples/render` PNG output for `shading_operator_1.pdf` between the
instrumented tree and a freshly reverted (`git checkout --`) clean tree, `sha256sum`:

```
rendered_0.png: 9976bc77794d886f09f67c4ef9c17e605654cda64cd81ff56234d129b8034062  (clean) == (instrumented)
rendered_1.png: 1d140028e61ee00954c20563782b6f898a4cdba35391e860a7705a874ffd5b98  (clean) == (instrumented)
```

Bit-exact on both pages, as expected - the instrumentation only wraps the existing
`fill_path_impl` call in `Instant::now()`/`AtomicU64::fetch_add` calls without touching what gets
computed (same pattern already verified by candidates 007/010/011/017).

Project test suite (`cargo test --workspace --release --no-fail-fast`), run once on the
instrumented tree and once on the freshly-reverted clean tree (both in the same worktree, so both
share the same on-disk reference-image cache - see caveat below):

- **Instrumented:** 430 passed, 1277 failed (`hayro-tests::tests`) + 2 failed (`hayro-syntax::lib`)
  + 2 hard target failures (`hayro-jbig2::asset_suite`, `hayro-jpeg2000::asset_suite`) - counts
  stable across two consecutive runs on the instrumented tree.
- **Clean (reverted):** identical - 430 passed, 1277 failed (`hayro-tests::tests`) + 2 failed
  (`hayro-syntax::lib`) + the same 2 hard target failures.
- **Caveat, unrelated to this candidate:** `hayro-tests::tests`'s `render::*` snapshot tests panic
  with "new reference image was created" on a worktree with no pre-existing snapshot cache (this
  repo doesn't ship reference PNGs in-tree, per `../projects/hayro.md`'s git-lfs gotcha) - the
  first-ever `cargo test` run in a fresh worktree showed 117 passed/1590 failed and *created* the
  missing snapshots as a side effect; every run after that (both before and after this candidate's
  instrumentation) is stable at 430/1277 against the now-populated cache. The 2 `hayro-syntax::lib`
  failures (`pdf_version_catalog`, `pdf_version_header`) and the 2 `asset_suite` hard failures
  ("No test inputs were found, run `python sync.py`") are pre-existing, environment-asset-related,
  and identical before/after - not introduced by this candidate.

## Result

**The `Shading` arm's own direct `fill_path_impl` cost is large on this file, not small - it
answers candidate 017's open question the other way from what 017's own speculation suggested.**
For `shading_operator_1.pdf`, isolating just the `sh` operator's own fill call (bypassing the
`draw_path`/`fill_stroke_path` wrappers candidate 007's instrumentation already subtracts out)
shows it accounts for **~66% of total `render()` time** on its own - not a small residual on top
of the Form-recursion confound, but the dominant single cost in this file's render. The remaining
~67% of `xobject_misc` (`xobject_misc` minus the isolated shading-fill time) is consistent with
being mostly the single Form's `Do`-call recursion double-count that candidate 017 already
characterized (the whole bucket sums to ~133% of total render time, only possible with
double-counting), not a second large chunk of genuine unaccounted prep cost.

This is specific to this file's shading, not necessarily a general "shading is always ~66% of
render time" claim - `shading_operator_1.pdf`'s `sh` operator fills `context.bbox()` (the current
clip bbox at the time `sh` is invoked), and how large that bbox is (and hence how much
`fill_path_impl` has to rasterize) depends entirely on what clip is active, which is
content-specific. See "Why the result looks the way it does" below.

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source. The `patch.diff` in this folder
is the same class of additive, already-verified-bit-exact timing instrumentation used by
candidates 007/010/011/017 (plus one new atomic/timer this candidate adds, same pattern), reverted
before finishing. Bit-exact PNG output and identical test-suite pass/fail counts confirmed in Step
5 above.

## Why the result looks the way it does

`TypedInstruction::Shading`'s match arm (`hayro-interpret/src/interpret/mod.rs`) calls
`fill_path_impl` directly on `context.bbox().to_path(0.1)` - i.e., it rasterizes a fill over
whatever the *current clip bbox* is at the point `sh` is invoked, not over some small fixed
region. In `shading_operator_1.pdf`, the `sh` operator sits inside a Form XObject that itself
appears to have little clipping restricting that bbox beyond the Form's own `BBox` entry (`0 0
362.835 35.433`, per the PDF's raw content) - `fill_path_impl` rasterizing a `ShadingType 2`
(axial/linear gradient) fill over that area is real, non-trivial rasterization work, not prep
overhead. This explains why the isolated shading-fill number is both large (~66%) and highly
stable across repeat runs (only about 1 percentage point of spread across 4 independent runs
including a fresh multi-core run) - it's dominated by actual per-pixel fill work proportional to
the shaded area, the same kind of cost `draw_path`'s own timer captures for ordinary path fills,
just routed through a different code path that candidate 007/011's original bucket instrumentation
couldn't see separately from the Form-recursion confound.

Combined with candidate 017's own finding (Form-recursion double-counting explains the bulk of the
~40-44% "unattributed" figure on Form-heavy, few/no-Shading files like
`animated-distributions.pdf`), the full picture for `xobject_misc` is: on a file with real
Shadings, a meaningful chunk of what looked like "unattributed dispatch overhead" is neither
double-counting nor prep overhead - it's genuine shading-fill rasterization cost that the original
`draw_path`/`draw_glyph`/`draw_image` buckets never had a hook to see, because the `Shading` arm
calls `fill_path_impl` directly rather than going through `fill_path()`.

## What's not done / open questions

- **Not a general result across all Shading types.** `shading_operator_1.pdf`'s `ShadingType 2`
  (axial gradient) fill is comparatively cheap per pixel; a mesh shading (`ShadingType 4/6/7`) or a
  radial shading with many stops evaluated per pixel could show a different, possibly larger or
  smaller, ratio - not tested here, and not the same as the `pattern_shading_*` files in the
  corpus (those render through the `color` bucket via `scn`/`SCN`, not directly through `sh` - see
  candidate 017's own note that `pattern_shading_type2_many.pdf` showed 0.0% `xobject_misc`).
- **Didn't try `shading_operator_2.pdf`** (the other `sh`-operator file in the corpus, also 1
  Form + 1 Shading but much smaller, 7KB) as a second data point - left as a possible follow-up if
  a second, independent confirmation is wanted.
- **No fully zero-Form `sh`-operator PDF exists in the current corpus** - both `sh`-using files
  have exactly 1 Form. This candidate's isolation technique (timing only the direct
  `fill_path_impl` call inside the `Shading` arm) sidesteps needing a zero-Form file for this
  specific measurement, since it doesn't rely on subtracting Form-recursion time at all - but a
  synthetic zero-Form `sh`-operator PDF would still be useful for cross-checking candidate 017's
  Form-recursion-only findings independently of this candidate's own technique.
- No wall-clock performance fix is proposed here, same as candidates 011/017 - this is a
  decomposition/characterization finding about where render time genuinely goes for a
  Shading-heavy file, not a claim of fixable overhead in `hayro-interpret` dispatch itself. Whether
  the `Shading` arm's own `fill_path_impl` rasterization could itself be made faster (e.g. by
  reusing whatever makes `vello_cpu::fill_path` fast for ordinary path fills, per
  `../projects/hayro.md`'s `draw_glyph`/`fill_path` lead) is a separate, unexplored question this
  candidate didn't investigate.
