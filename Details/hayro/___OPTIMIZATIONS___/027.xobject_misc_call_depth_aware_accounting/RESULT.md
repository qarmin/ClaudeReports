# Call-depth-aware accounting for the `xobject_misc` whole-match-arm bucket

Backlog entry from `../IDEAS.md` (added by candidates 011/017): candidate 017 confirmed candidate
011's predicted confound - for PDFs with real Form XObjects, the naive `xobject_misc - draw_image`
subtraction is dominated by double-counted nested-operator time (the whole-bucket sum exceeds
100% of total `render()` time for `tile_clamp_bug.pdf`), not real prep overhead. Both 011 and 017
named the same fix without building it: a call-depth-aware way to exclude a nested `interpret()`
call's own already-bucketed time from the parent `Do`/`XObject` operator's `xobject_misc` bucket.
This candidate builds and measures that fix.

**No patch to real hayro source is being proposed** - like candidates 011/017, this only extends
the existing temporary, additive `Instant`/`AtomicU64` profiling instrumentation (always reverted,
never merged); the depth-aware fix lives entirely inside that instrumentation, not in the real
dispatch logic it measures.

## What was tried

### Mechanism

Added one new global counter, `__TOTAL_BUCKET_GROSS_NS` (`hayro-interpret/src/lib.rs`),
incremented by every existing `__record_bucket` call (the "gross", whole-match-arm-elapsed
recording candidates 007/010/011/017 already had), at every recursion depth. In the dispatch
loop (`hayro-interpret/src/interpret/mod.rs`), the instrumentation now snapshots this counter
immediately before and immediately after running an op's match arm; the delta is exactly the
bucket time already recorded by any nested `interpret()` call made during that op (a Form
XObject's own recursive content-stream interpretation, at any further depth, transitively - no
explicit recursion-depth counter needed, since the running total already reflects however many
levels deep the nested calls went). Subtracting that delta from the op's own gross elapsed time
gives a "self" time, recorded into a parallel set of `_SELF_NS` counters
(`__debug_op_buckets_self_ns()`), alongside the original unmodified gross ones
(`__debug_op_buckets_ns()`, kept as-is so both can be compared in the same run).

`hayro/examples/bench_ops_reference.rs` (candidate 011/017's bench, copied and extended) prints
both: the original gross buckets (candidate 007/011's table, unchanged) and the new SELF-time
buckets, plus a `xobject_misc SELF minus draw_image` line (candidate 011's own subtraction
pattern, applied to the depth-aware SELF number instead of the gross one).

### Correctness of the mechanism, checked by construction

The SELF-bucket sum can no longer exceed 100% of total render time (unlike the gross sum, which
was 105-110% for both test PDFs - the sub-100% cross-check that first flagged the double-counting
in 017 in the first place). This held on every run below (73.2-77.1% depending on file), which is
itself evidence the subtraction is behaving as designed - the mechanism generically catches ANY
nested `interpret()` call recorded via `__record_bucket`, not just the `XObject` match arm
specifically. Concretely, `path_draw_whole`'s own gross-vs-self numbers also differ
(`tile_clamp_bug.pdf`: 368.9ms gross vs. 294.5ms self) even though `PathDraw` ops don't themselves
recurse into `interpret()` - this is `fill_path`/`fill_stroke_path` triggering a tiling pattern's
own nested `interpret()` call during a fill, confirming the fix isn't special-cased to Form
XObjects and catches the general double-counting mechanism wherever it occurs.

### Measurement

Same two PDFs candidate 017 used (`tile_clamp_bug.pdf`: 16 Forms, 2 Shadings, images;
`animated-distributions.pdf`: 112 Forms, 0 Shadings, 0 images), same methodology (`taskset -c 2`,
`hayro/examples/bench_ops_reference.rs`, 40 iterations, two runs each for a repeat, plus one
unpinned run and one fresh-clone run via this folder's own `just test`):

| File | Run | total render() | `xobject_misc` GROSS (candidate 011/017, buggy) | `xobject_misc` SELF (candidate 027, depth-aware) | GROSS sum of all buckets | SELF sum of all buckets |
|---|---|---:|---:|---:|---:|---:|
| `tile_clamp_bug.pdf` | 1 (`taskset -c 2`) | 1698.1ms | 46.8% | **14.2%** | 110.3% | 73.3% |
| `tile_clamp_bug.pdf` | 2 (repeat) | 1723.4ms | 46.7% | **14.1%** | 110.2% | 73.2% |
| `tile_clamp_bug.pdf` | 3 (unpinned) | (varies, see raw log) | 46.7% | **14.2%** | n/a | 73.2% |
| `tile_clamp_bug.pdf` | 4 (`just test`, fresh clone, 20 iter) | (varies) | n/a | **13.7%** | n/a | 73.9% |
| `animated-distributions.pdf` | 1 (`taskset -c 2`) | 73.4ms | 44.5% | **13.9%** | 107.7% | 77.1% |
| `animated-distributions.pdf` | 2 (repeat) | 73.9ms | 44.4% | **13.9%** | 107.4% | 76.9% |
| `animated-distributions.pdf` | 3 (`just test`, fresh clone, 20 iter) | 41.8ms | n/a | **13.5%** | n/a | 75.7% |

("`xobject_misc` GROSS/SELF" columns are the "bucket minus `draw_image`" figure in both cases -
the same subtraction candidate 011 introduced, applied once to the old gross bucket and once to
the new depth-aware self bucket, so the two numbers are a direct apples-to-apples comparison of
the same quantity under the old vs. new accounting. `tile_clamp_bug.pdf`'s absolute `total
render()` ms varies run to run from ambient machine load, same as candidate 017 observed - `uptime`
load average was 2.4-4.7 during this session on this 24-thread machine; the *percentage* figures
above are stable to within about half a point across repeats, which is what this candidate
measures.)

Full raw output saved at `../_reference/xobject_misc_depth_aware_027.txt`.

## Result

**The depth-aware fix drops the previously-unusable ~44-47% "unattributed" figure down to a
stable ~13.5-14.2%** for both PDFs - consistent with candidate 011's original ~0.3% figure being
the true no-Forms baseline and confirming that essentially all of 017's ~44% remainder really was
the double-counting artifact both candidates predicted, not previously-hidden real prep cost.
`animated-distributions.pdf` (112 Forms, 0 images, 0 Shadings) and `tile_clamp_bug.pdf` (16 Forms,
2 Shadings, images) land within half a point of each other (13.5-14.2%) despite very different
Form/Shading/image mixes, which is itself evidence the fix generalizes rather than having been
tuned to one file.

As a sanity check independent of the specific percentages: the SELF-bucket sum across all 9
buckets never exceeds 100% of total render time on any run (73.2-77.1%), whereas the GROSS sum
consistently did (105-110%) - exactly the property that should hold once double-counting is
removed and didn't before.

This ~13.5-14.2% is now a genuinely usable "how much prep overhead is there in `xobject_misc`"
estimate for Form/Shading-heavy PDFs - unlike 017's ~44%, which this candidate's own measurement
shows was never a real, fixable-overhead number. Whether ~13.5-14.2% of total render time is
itself worth chasing as a future fix candidate is a separate question this candidate doesn't
answer (see "What's not done" below) - this candidate's contribution is making the *instrument*
trustworthy for that question, not answering it.

## Does it change behavior/output?

Not applicable to real hayro source - no change was made or proposed to `hayro`/`hayro-interpret`'s
actual dispatch/rendering logic; only the same category of temporary, additive
`Instant::now()`/`AtomicU64::fetch_add` instrumentation already used and correctness-verified by
candidates 007/010/011/017 was extended, plus one new example file
(`hayro/examples/bench_ops_reference.rs`, matching 011/017's own extension pattern). Verified
bit-exact PNG output (`sha256sum`) between a freshly reverted clean tree and the instrumented tree
for both test PDFs:

```
tile_clamp_bug.pdf:          7b8fdc55...8357c3a  (clean, from candidate 017's own record)  ==  7b8fdc55...8357c3a  (this candidate, instrumented)
animated-distributions.pdf:  874e1cf0...9ffde,4cd209ee...b129  (clean, 2 pages)  ==  same  (this candidate, instrumented)
```
identical to the hashes candidate 017 already recorded for the same two files - confirms the new
counters/subtraction logic added here doesn't touch anything that affects rendered output, same as
the base instrumentation it extends.

**Project test suite, before (baseline, reverted tree) vs. after (this candidate's instrumentation
applied), both `cargo test --release --workspace --no-fail-fast` in the same worktree/environment:**
identical in both: `hayro-jbig2`/`hayro-jpeg2000` `asset_suite` targets fail to build/run
(pre-existing environment issue - this repo's test corpus requires `hayro-tests/sync.py` to
download PDFs/fonts from Cloudflare R2, not run in this session, per `hayro-tests/README.md`);
`hayro-syntax --lib`: 199 passed, 2 failed in both baseline and patched (same 2 tests failing in
both, pre-existing, unrelated to any bucket-instrumentation code); `hayro-tests --test tests`: 430
passed, 1277 failed in both baseline and patched, same pre-existing cause (missing downloaded test
corpus). All other workspace crates: passing in both, no change (`37 passed`, `48 passed`, `2
passed`, `75 passed`, `1 passed`, `1 passed`, `2 passed` - identical counts before/after). One
transient run showed different `hayro-tests` counts (117 passed/1590 failed) when run as part of a
single very first `--workspace` invocation that also freshly linked the new example binary
concurrently; re-running twice more (baseline and patched) reproduced the stable 430/1277 figure
both times - this is pre-existing flakiness in `hayro-tests`' shared test state under load (its
own `README.md`/candidate-017 already document the missing-corpus failure mode; nothing in this
candidate's own patch touches `hayro-tests`), not something introduced by this candidate.

Not a breaking change - no public API, file format, or documented behavior is touched; the two
new `#[doc(hidden)]` debug functions (`__debug_op_buckets_self_ns`, and the internal
`__record_bucket_self`/`__TOTAL_BUCKET_GROSS_NS`) are the same "TEMPORARY profiling
instrumentation, remove before merging" category already present and labeled as such throughout
`../_reference/`'s diffs.

## Why the result looks the way it does

The mechanism is exactly what candidates 011/017 predicted: `TypedInstruction::XObject`'s match
arm calls `x_object.draw(...)`, which for a Form XObject recurses into `interpret()` on the form's
own content stream. Under the old (gross-only) instrumentation, the *entire* elapsed time of that
recursive call was captured by the outer `Do` operator's own `__t_op.elapsed()` and added to
`xobject_misc`, on top of each nested operator already being correctly bucketed once at its own
depth - a straightforward double-count. The fix works because `__TOTAL_BUCKET_GROSS_NS` is a
single flat counter incremented by *every* `__record_bucket` call regardless of nesting depth;
snapshotting it around an op's own match-arm execution captures exactly "how much bucket time got
recorded by anything that ran nested inside this op", which is precisely the quantity that needs
subtracting - no explicit thread-local depth counter was needed, because the running-total delta
already encodes arbitrarily deep recursion transitively (a Form containing another Form containing
a path fill all show up in the same delta, at whatever depth they occurred).

The residual ~13.5-14.2% (down from ~44-47%) is consistent with 011's own explanation for `img.pdf`
(zero Forms, only ~0.3% residual): dict/resource lookups (`get_x_object`/`get_shading`),
`XObject::new`/`ImageXObject::new` construction, color-space resolution, and (per 017's own
observation) `Shading`'s direct `fill_path_impl` call, which isn't routed through the same
`draw_path`-instrumented helper and so was never subtracted out by the `- draw_image` term either
before or after this fix - all real, uninstrumented work inside the `xobject_misc` match arm that
isn't a *recursive* `interpret()` call and so was never part of what this candidate's subtraction
targets.

## What's not done / open questions

- **Whether ~13.5-14.2% of total render time is itself worth a real, non-instrumentation fix is
  not answered here** - this candidate only makes the measurement trustworthy, per its own scope
  (see "Result" above). A future candidate could use this same depth-aware instrumentation
  (already built, reusable via `../_reference/` once this diff is folded in - see below) to decide
  whether that residual is dominated by `Shading`'s un-subtracted `fill_path_impl` time, XObject
  dict-resolution cost, or something else, the same way 011 broke down `img.pdf`'s own much
  smaller residual.
- **This candidate's own diff was not copied into `../_reference/` as a new canonical version** -
  by convention (see `../README.md`) `_reference/` holds the *current* best version of each piece
  of instrumentation for future candidates to reuse; a future session extending this bucket
  breakdown further should decide whether to promote this candidate's depth-aware version there
  (superseding the older gross-only `interpret_dispatch_instrumentation.diff`) or keep both side by
  side for comparison - left as an open call rather than done unilaterally here, since `_reference/`
  is shared state other in-flight candidates might also be touching.
- **Not measured against `img.pdf`** (011's own zero-Form file) - not needed to answer this
  candidate's question (the fix's effect is only visible where nesting occurs), but would be a
  quick sanity check that the depth-aware SELF number for `img.pdf` still matches 011's original
  ~0.3% gross figure almost exactly (since with zero Forms/Shadings, gross and self should be
  identical there) - left for a future session if the `_reference/` promotion above happens.
- **Multi-core (unpinned) run only done once for `tile_clamp_bug.pdf`**, not
  `animated-distributions.pdf` - the bench is single-threaded internally (no thread-parallel
  rendering), so pinning vs. not pinning only affects ambient noise, not the mechanism being
  measured; the unpinned run's percentages matched the pinned ones to within 0.1pp, consistent
  with this being a low-parallelism-sensitivity measurement, so the second file's unpinned run was
  skipped as redundant.
