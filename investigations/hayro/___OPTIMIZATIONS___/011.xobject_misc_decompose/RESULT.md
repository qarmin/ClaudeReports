# Decomposing the `xobject_misc` dispatch bucket: real device work vs. prep overhead

Backlog entry from `../IDEAS.md`: candidate 007's whole-arm bucket instrumentation measured
`xobject_misc` (`XObject`/`InlineImage`/`Shading`/marked-content-op dispatch, inside
`hayro_interpret::interpret()`'s operator loop) at 13.5% of total `render()` time for `img.pdf`,
as one whole-arm-inclusive bucket. It was unknown how much of that is genuine nested
`draw_image`/Form-XObject-recursion work (already counted/attributed elsewhere) vs. actual
unattributed prep overhead (dict/color-space resolution, `Shading`/`Pattern` construction) worth
fixing on its own - the same question 007 already answered for `text_show_whole` and
`path_draw_whole` by subtracting the already-instrumented nested device-call time from the
whole-arm bucket time.

**No patch to real source was found or attempted** - the decomposition itself is the answer, and
it comes out too small to be worth a dedicated fix (see "Result"). This candidate is a
measurement/characterization, reusing the existing instrumentation exactly like 007's own step 1,
extended with one new line.

## What was tried

### Step 1: reapply the existing whole-arm-bucket instrumentation

`../_reference/interpret_dispatch_instrumentation.diff` (`hayro-interpret`) and
`../_reference/pipeline_breakdown_profiling_instrumentation.diff` (`hayro`, needed for the
`draw_image`/`draw_path`/`draw_glyph` per-device-call timers that the subtraction needs) both
still applied cleanly against the current `HEAD` (`5a5f0e24`, unchanged since candidates 007/010).

### Step 2: extend the breakdown to subtract `draw_image` from `xobject_misc`

Copied `../_reference/bench_ops_reference.rs` to `hayro/examples/bench_ops_reference.rs` and added
one line, following the exact pattern candidate 007 used for `path_draw_whole`/`text_show_whole`:

```rust
println!("    xobject_misc bucket minus draw_image (...): {:.3} ms ({:.1}%)",
    (buckets[8].1 as f64 / 1e6) - img_ms, 100.0 * ((buckets[8].1 as f64 / 1e6) - img_ms) / total_ms);
```

`buckets[8]` is `xobject_misc`'s own accumulated time (the whole `XObject`/`InlineImage`/
`Shading`/marked-content match arms, including whatever nested device/recursive calls they make);
`img_ms` is `hayro`'s own `__PHASE_DRAW_IMAGE_NS` (time inside `Renderer::draw_image`, timed at the
`Device` trait boundary, independently of which content-stream operator triggered it).

### Step 3: what `img.pdf` actually exercises inside this bucket

Before trusting the subtraction, checked what's actually inside `xobject_misc` for this file
(`strings img.pdf | grep -c ...`): **0** `/Subtype /Form` XObjects, **0** `/ShadingType` entries,
**8** `/Subtype /Image` XObjects. So for `img.pdf` specifically, `xobject_misc`'s only
non-negligible content is the 8 `Do` (Image-XObject) dispatches plus a handful of marked-content
ops - there is no Form-XObject recursion to confound the subtraction (a recursive `interpret()`
call for a Form XObject would double-count its own nested ops' time into *both* `xobject_misc` and
whichever bucket those nested ops belong to, the same way `Shading`'s internal
`fill_path_impl` call double-counts into both `xobject_misc` and `hayro`'s `draw_path` timer - see
"What's not done" for why this matters for *other* PDFs).

### Step 4: measurement

Ran `hayro/examples/bench_ops_reference.rs` against the real `img.pdf`, `taskset -c 2`, 80
iterations (same methodology as candidate 007's step 1), twice back-to-back for a repeat, plus a
third run via `just test` from a fresh clone of the pinned commit (see this folder's `justfile`):

| Run | total render() | `draw_image` | `xobject_misc` (whole bucket) | `xobject_misc` minus `draw_image` |
|---|---:|---:|---:|---:|
| run 1 (in-place) | 314.995ms | 41.433ms (13.2%) | 42.444ms (13.5%) | 1.011ms (0.3%) |
| run 2 (repeat, in-place) | 315.099ms | 41.372ms (13.1%) | 42.350ms (13.4%) | 0.978ms (0.3%) |
| run 3 (`just test`, fresh clone) | 256.232ms | 33.900ms (13.2%) | 34.724ms (13.6%) | 0.824ms (0.3%) |

(Run 3's absolute numbers are lower than runs 1-2 purely from ambient machine load variance
between the two invocations - not from any code difference; `uptime` load average was 0.44-0.93
throughout this session. All three runs agree on the *percentage* split, which is what this
candidate is measuring.)

Full raw output saved at `../_reference/img_pdf_breakdown_011.txt`.

## Result

**~0.3% of total `render()` time** (0.82-1.19ms out of 80 iterations, roughly 10-15
microseconds/render) is `xobject_misc`'s own prep overhead not already accounted for by
`draw_image`. The other **~13.2 of the bucket's 13.5 percentage points (97-98%)** is time already
spent inside `Renderer::draw_image` (image decode/color-conversion/resize), which is real,
necessary device work already visible and attributed on its own in the breakdown - not hidden
dispatch overhead.

This is a materially different outcome from candidate 007's other two whole-arm buckets:
`text_show_whole` had 4.5-4.6pp of real prep overhead (leading to the `Tf`-cache fix, a 16-18%
win) and `path_draw_whole` had a smaller but still real 0.4pp. `xobject_misc` has neither - for
this file, essentially the entire bucket already *is* attributed, correctly-counted device work.
There is no meaningfully-sized unattributed remainder here to chase.

## Does it change behavior/output?

Not applicable - no source change was made or proposed; only temporary, additive timing
instrumentation was applied (reverted before finishing, per the workflow), and one new example
file (`hayro/examples/bench_ops_reference.rs`, also reverted) was used to print the extra line.
Ran `./target/release/examples/render img.pdf` once with the instrumentation applied as a smoke
check that it still produces output without panicking; not a correctness claim beyond that (no
source logic was touched, only wrapped in `Instant::now()`/`AtomicU64::fetch_add` calls, same
pattern already used and correctness-verified by candidates 007/010 for the identical
instrumentation).

## Why the result looks the way it does

`XObject`'s and `InlineImage`'s match arms in `hayro-interpret/src/interpret/mod.rs` do real setup
work before reaching the device call (`resources.get_x_object`/`get_shading`,
`XObject::new`/`ImageXObject::new`, color-space resolution via `context.get_color_space`), but that
setup is evidently cheap relative to the image decode/convert work it leads into - 8 `Do` calls'
worth of dict lookups and object construction costs on the order of 10-15 microseconds total per
render, dwarfed by the ~410-520 microseconds/render (41ms/80 or 34ms/80) spent actually decoding
and converting the images themselves. Unlike `Tf` (called 326 times per render, so even a small
per-call cost adds up), `Do`/XObject-dispatch operators are comparatively rare in this file (8 of
4782 total operators, well under 1%) - there's no equivalent "small cost times many calls" pattern
here for the bucket's non-device-call portion to hide in.

## What's not done / open questions

- **Only measured against `img.pdf`**, which has zero Form XObjects and zero Shadings (confirmed
  in step 3) - the ~0.3% figure is specific to this file's XObject/InlineImage-dominated,
  Form/Shading-free `xobject_misc` bucket. A PDF that makes heavy use of nested Form XObjects
  (common in PDFs generated with layered/reusable content) or tiling/shading patterns would
  exercise the *other* parts of this bucket's match arms that weren't isolated here, and the
  simple `xobject_misc - draw_image` subtraction used here would then **under-count** the real
  prep overhead: a Form XObject's own nested `interpret()` call attributes its nested ops' time to
  *both* the parent `xobject_misc` bucket (as part of the whole enclosing match-arm's elapsed time)
  and to whichever bucket each nested op itself belongs to (e.g. nested text draws add to
  `text_show_whole` too) - the same double-counting caveat 007's own instrumentation comment
  already flags generically ("may double-count time already shown above"), just not something this
  candidate could isolate further without adding call-depth-aware accounting (e.g. a thread-local
  "are we inside a nested `interpret()` call" flag to exclude recursive time from the parent arm's
  own bucket). Not built this session since `img.pdf` doesn't need it to answer the question asked.
- A PDF exercising Form XObjects/Shadings more heavily (none of the `.pdf` files referenced
  elsewhere in this workspace's `README.md` were checked for this) would be a natural next
  candidate if `xobject_misc`'s overhead is suspected to matter more for other real-world PDFs -
  added to `../IDEAS.md`.
- No perf-relevant fix is proposed here: at ~0.3% of total render time, even a hypothetical 100%
  elimination of this remainder would be within this bench's own measurement noise floor
  (candidate 007 measured this bench's noise floor at well under 2%).
