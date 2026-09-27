# Re-checking the `xobject_misc` prep-vs-device-call split against Form-XObject/Shading-bearing PDFs

Backlog entry from `../IDEAS.md` (added by candidate 011): candidate 011's `xobject_misc - draw_image`
subtraction found only ~0.3% of total `render()` time was genuine unattributed prep overhead for
`img.pdf` - but `img.pdf` has zero Form XObjects and zero Shadings, so it couldn't exercise the one
confound 011 flagged as a possible under-count: a Form XObject's nested `interpret()` call
double-counts its own operators' time into both the parent `xobject_misc` bucket (the whole
`XObject` match arm's elapsed time, including the recursive call it makes) and whichever bucket
each nested op itself belongs to. This candidate reruns 011's exact methodology against two PDFs
from the existing test corpus that actually use Form XObjects and/or Shadings.

**No patch to real source was found or attempted** - like candidate 011, this is a
measurement/characterization using the same reverted, additive instrumentation; the result itself
is the answer.

## What was tried

### Step 1: reapply the existing instrumentation and bench

`../_reference/interpret_dispatch_instrumentation.diff` and
`../_reference/pipeline_breakdown_profiling_instrumentation.diff` both still applied cleanly
against current `HEAD` (`5a5f0e24`, unchanged since candidates 007/010/011). Copied
`../_reference/bench_ops_reference.rs` (unmodified - candidate 011's `xobject_misc bucket minus
draw_image` line is already in it) to `hayro/examples/bench_ops_reference.rs`.

### Step 2: pick PDFs from the existing test corpus that actually exercise Form XObjects/Shadings

Scanned all 360 PDFs in `hayro-tests/pdfs/` (`strings *.pdf | grep -c '/Subtype */Form'` /
`'/ShadingType'`) for ones combining meaningful counts with enough content to produce a
non-trivial render time. Picked two, copied into this folder's `testdata/`:

- **`tile_clamp_bug.pdf`** (686KB): 16 `/Subtype /Form` XObjects, 2 `/ShadingType` entries, plus
  images - a mixed Form+Shading+Image file.
- **`animated-distributions.pdf`** (328KB): 112 `/Subtype /Form` XObjects, 0 `/ShadingType`, 0
  images - a Form-XObject-only file, deliberately chosen to isolate the Form-recursion confound
  from candidate 011's `draw_image` subtraction (with zero images, `xobject_misc - draw_image`
  can't hide behind any legitimate image-decode time at all).

### Step 3: measurement

`taskset -c 2`, 40 iterations, `hayro/examples/bench_ops_reference.rs` (same bench, same build, as
011), run twice back-to-back per file for a repeat, plus one unpinned run for `tile_clamp_bug.pdf`
as the multi-core-availability check. Full raw output saved at
`../_reference/tile_clamp_bug_and_animated_distributions_breakdown_017.txt`.

| File | Run | total render() | `draw_image` | `xobject_misc` (whole bucket) | `xobject_misc` minus `draw_image` |
|---|---|---:|---:|---:|---:|
| `tile_clamp_bug.pdf` | 1 (`taskset -c 2`) | 1795.1ms | 605.1ms (33.7%) | 1404.6ms (78.2%) | 799.4ms (**44.5%**) |
| `tile_clamp_bug.pdf` | 2 (repeat) | 3387.3ms | 1110.0ms (32.8%) | 2610.9ms (77.1%) | 1500.8ms (**44.3%**) |
| `tile_clamp_bug.pdf` | 3 (unpinned) | 2132.4ms | 742.1ms (34.8%) | 1683.5ms (79.0%) | 941.5ms (**44.2%**) |
| `animated-distributions.pdf` | 1 (`taskset -c 2`) | 260.0ms | 0.0ms (0.0%) | 103.7ms (39.9%) | 103.7ms (**39.9%**) |
| `animated-distributions.pdf` | 2 (repeat) | 210.9ms | 0.0ms (0.0%) | 93.0ms (44.1%) | 93.0ms (**44.1%**) |

(Run-to-run absolute-ms variance for `tile_clamp_bug.pdf` - e.g. 1795ms vs. 3387ms for the same 40
iterations - is ambient machine load noise, not a code difference; `uptime` load average was
3.1-5.95 on this 24-thread machine throughout. All runs agree on the *percentage* split, which is
what this candidate measures, to within about half a percentage point.)

### Step 4: correctness check

Compared `./target/release/examples/render` output (PNG) for both test PDFs between the
instrumented tree and a freshly reverted (`git checkout --`) clean tree, `sha256sum`:

```
tile_clamp_bug.pdf:          7b8fdc55...8357c3a  (clean)  ==  7b8fdc55...8357c3a  (instrumented)
animated-distributions.pdf:  874e1cf0...9ffde,4cd209ee...b129  (clean, 2 pages)  ==  same  (instrumented)
```

Bit-exact on both files, both pages of the 2-page file - as expected, since the instrumentation
only wraps existing logic in `Instant::now()`/`AtomicU64::fetch_add` calls without touching what
gets computed (same pattern already verified by candidates 007/010/011).

## Result

**Confirmed and quantified the confound candidate 011 flagged as unexercised by `img.pdf`.** For a
PDF with real Form XObjects and Shadings (`tile_clamp_bug.pdf`), the naive `xobject_misc -
draw_image` subtraction leaves **~44% of total `render()` time** unattributed - not ~0.3% as for
`img.pdf`. For a Form-XObject-only PDF with zero images (`animated-distributions.pdf`), it's
**~40-44%** as well, and here the entire `xobject_misc` bucket (not just the remainder after
subtracting `draw_image`) is "unattributed" by 011's method, since there's no `draw_image` time to
subtract at all.

This is **not** ~44% of real, fixable dispatch overhead, though - see "Why the result looks the
way it does" below: it is overwhelmingly the double-counting confound 011 already named, not newly
discovered prep cost. The two PDFs picked here specifically to stress-test that confound (Forms
that themselves draw paths/text/nested XObjects) show almost exactly the behavior predicted: the
"minus `draw_image`" remainder tracks nested `path_draw_whole`/`text_show_whole` time that a
Form's own recursive `interpret()` call re-attributes to the parent `xobject_misc` timer, on top of
being correctly counted in its own bucket already - the whole-bucket sum already exceeds 100% of
total render time for `tile_clamp_bug.pdf` (107.6-108.7%), which is only possible if some time is
counted more than once.

## Does it change behavior/output?

Not applicable - no source change was made or proposed; only the same temporary,
already-verified-additive timing instrumentation from candidates 007/010/011 was applied
(reverted before finishing) plus the same unmodified example file. Bit-exact PNG output confirmed
in step 4 above.

## Why the result looks the way it does

`TypedInstruction::XObject`'s match arm in `hayro-interpret/src/interpret/mod.rs` calls
`x_object.draw(resources, context, device)`, which for a Form XObject recurses into `interpret()`
again on the form's own content stream - and the *entire* elapsed time of that recursive call
(everything the form draws: paths, text, nested XObjects, image decodes) is captured by the outer
`__t_op.elapsed()` timer that gets added to the `xobject_misc` bucket for the outer `Do` operator.
Each operator *inside* the form's content stream then also gets its own `__record_bucket` call at
its own nesting depth - e.g. a `path_draw_whole`-bucketed fill inside the form is counted once
there (correctly) and a second time as part of the parent `Do`'s `xobject_misc` time (a byproduct
of how the instrumentation was written, not of anything in the real dispatch logic). Candidate 011
predicted this exact mechanism from reading the code but had no PDF in hand that exercised it -
`tile_clamp_bug.pdf`'s 16 Forms and `animated-distributions.pdf`'s 112 Forms both do, and the
measured ~40-44% unattributed figure (vs. `img.pdf`'s ~0.3%, a file with zero Forms) is consistent
with "this is mostly double-counted nested-op time, not new prep overhead" rather than "this is a
previously-hidden ~44% of real work" - a file with *only* images and no Forms (`img.pdf`) shows
none of it, and a file with *only* Forms and no images (`animated-distributions.pdf`) shows the
entire bucket as "unattributed" by this method, which only makes sense as an artifact of what gets
subtracted (`draw_image` alone) rather than a real measurement of Form-dispatch prep cost.

`Shading`'s match arm (the `sh` operator specifically, not a shading *pattern* referenced via
`scn`/`SCN`, which is a different code path bucketed under `color`) calls `fill_path_impl`
directly rather than through the same `fill_path()`/`fill_stroke_path()` helpers that `draw_path`'s
own instrumentation wraps - so genuine `sh`-operator fill time also lands in `xobject_misc` without
ever being subtracted out, on top of the Form-recursion double-counting above. `tile_clamp_bug.pdf`
has 2 `/ShadingType` entries but this candidate did not further isolate how much of its ~44%
remainder is this vs. Form recursion - see "What's not done" below.

## What's not done / open questions

- **The ~44% figure is not itself a usable "fixable overhead" estimate** - it's dominated by
  double-counted nested-operator time, per above. Turning this into an actual, fixable number
  needs the call-depth-aware accounting fix candidate 011 already named: a way to exclude a nested
  `interpret()` call's own already-counted time from the parent `Do` operator's `xobject_misc`
  bucket (e.g. a thread-local recursion-depth counter, or having the recursive `interpret()` call
  return its own elapsed nested-bucket time so the caller can subtract it before adding to its own
  bucket). Not built this session - this candidate only reran 011's existing (known-imperfect)
  methodology against PDFs that expose its limits, per the backlog entry's own framing.
- **Didn't separate the Form-recursion confound from the direct `Shading`/`fill_path_impl` gap.**
  `tile_clamp_bug.pdf` has both 16 Forms and 2 Shadings; `animated-distributions.pdf` isolates pure
  Form recursion (112 Forms, 0 Shadings, 0 images) and shows the same ~40-44%, which suggests Form
  recursion alone accounts for most of the effect and the `Shading`-direct-fill gap is secondary -
  but no PDF with Shadings and zero Forms was tried to isolate that piece on its own. Candidates
  `shading_operator_1.pdf` (1 Form, 1 Shading) or `pattern_shading_type2_many.pdf` (0 Forms, 14
  Shadings, but these render through the `color`-bucketed pattern path per above, not `sh`
  directly - checked, showed 0.0% `xobject_misc`, consistent with this) exist in the corpus for a
  future session to use.
- **Once a depth-aware fix exists**, this same `tile_clamp_bug.pdf`/`animated-distributions.pdf`
  pair would be the right regression check for it - copied into this folder's `testdata/` for that
  reason, not just as a one-off measurement input.
- No wall-clock performance fix is proposed here, same as 011 - this is purely a decomposition
  finding about the *instrumentation's* accuracy, not a claim about real, fixable render-time cost
  in `hayro-interpret` itself.
