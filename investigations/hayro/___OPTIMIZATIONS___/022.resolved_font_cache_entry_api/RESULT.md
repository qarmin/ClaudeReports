# `resolved_font_cache`'s miss path via the `Entry` API instead of `get()`+`insert()`

Backlog entry from `../IDEAS.md`: candidate 021 measured the worst case's entire +0.452%
regression (vs. no cache at all) as wrapper overhead around the `resolved_font_cache` miss path -
a failed `get(name)` immediately followed by a separate `insert(name.clone(), ...)` is a classic
double hashmap-traversal pattern. This candidate rewrites that miss path with
`resolved_font_cache.entry(name.clone()).or_insert_with(...)` to collapse it to a single
traversal, and re-profiles with candidate 021's exact callgrind methodology
(`worst_case_cache.pdf`, 326 `Tf` calls, every one a distinct never-reused resource name, so
`resolved_font_cache` misses on all of them) to see whether the regression shrinks.

`rustc 1.98.0 (88d9e12ae 2026-08-18)` / `cargo 1.98.0` (recorded per candidate 016's backlog
note).

## What was tried

Same `resolved_font_cache` feature as candidates 007/012/018/019/020/021 (a
`FxHashMap<Name, Option<TextStateFont>>` scoped per `interpret()` call), with only the miss-path
body of the `TypedInstruction::TextFont` arm rewritten:

```rust
let font = resolved_font_cache
    .entry(name.clone())
    .or_insert_with(|| {
        if let Some(font_dict) = font_dict_cache.get(name).cloned() {
            context.resolve_font(&font_dict)
        } else if let Some(font_dict) = resources.get_font(name) {
            font_dict_cache.insert(name.clone(), font_dict.clone());
            context.resolve_font(&font_dict)
        } else {
            Font::new_standard(StandardFont::Helvetica, &context.settings.font_resolver)
                .map(TextStateFont::Fallback)
        }
    })
    .clone();
```

Reused candidate 021's exact bench (`bench/src/main.rs`, unchanged) and testdata
(`worst_case_cache.pdf`, unchanged) against the same working-tree commit (`5a5f0e24`, no upstream
changes to this file since 021). Both variants' `Ir` counts were confirmed exactly reproducible
across two independent runs each before being compared.

| Variant | total `Ir` (5 renders, run 1) | total `Ir` (5 renders, run 2) | `Ir`/render (avg) | pixmap hash |
|---|---|---|---|---|
| baseline (no `resolved_font_cache` at all) | 255,536,020 | 255,536,020 | 51,107,204 | `0x31cced99a6bfe199` |
| candidate 021 (`get()`+`insert()` miss path) - for reference, from 021's own run | 256,691,166 | 256,691,166 | 51,338,233 | `0x31cced99a6bfe199` |
| this candidate (`Entry` API miss path) | 256,475,348 | 256,475,348 | 51,295,069.6 | `0x31cced99a6bfe199` |

| Comparison | delta `Ir` (5 renders) | delta `Ir`/render | delta % vs. no-cache baseline |
|---|---|---|---|
| 021 vs. baseline | +1,155,276 | +231,055 | +0.452% |
| this candidate vs. baseline | +939,328 | +187,866 | +0.368% |
| **this candidate vs. 021 (the actual thing being tested)** | **-215,948** | **-43,189/render** | **-18.7% of 021's own regression** |

This candidate's own baseline (255,536,020) differs from candidate 021's recorded baseline
(255,535,890) by 130 `Ir` over 5 renders - not a real behavioral difference; the candidate
folder's path length differs from 021's (`022.resolved_font_cache_entry_api` vs.
`021.resolved_font_cache_worst_case_callgrind`) and gets embedded as absolute-path string
constants (panic locations, `include_bytes!` debug paths) in the binary, which callgrind counts
as instructions to build/compare. Immaterial here since both this candidate's baseline and
patched numbers come from binaries built in the same folder, so the comparison that matters
(this candidate vs. itself) is unaffected.

Also checked the second, not-yet-separated backlog item: whether the miss path clones the
resolved `Option<TextStateFont>` twice. Reading both the original (`021`) and this candidate's
source shows exactly **one** clone in each: `021`'s version clones once for
`resolved_font_cache.insert(name.clone(), resolved.clone())` and then moves (not clones) the
original `resolved` out as the trailing expression; this candidate's version has `or_insert_with`
move its return value into the map directly (no clone) and then clones once, from the map, to
produce the caller's value. Confirmed via `callgrind_annotate`: the `interpret::interpret`
function's own attributed self-cost line is bit-for-bit identical between baseline and patched
(184,970 `Ir` both), and no separate `TextStateFont::clone` symbol shows any difference between
021's variant and this one - the backlog's "clone twice" concern does not hold up on inspection;
it was likely a misreading of the extra `drop_glue::<Option<TextStateFont>>` call visible in
021's own annotated output (a drop of the moved-from-and-reinstated temporary, not a second
clone).

## Result

The `Entry` API rewrite is a **partial**, not complete, reduction of candidate 021's worst-case
regression: **-18.7%** of the original regression's size (+187,866/render vs. +231,055/render,
both measured against the same true baseline with no `resolved_font_cache` at all), exactly
reproducible across two independent callgrind runs of each variant (021's own recorded numbers,
and this candidate's numbers, are each internally consistent run-to-run). The `Entry` API still
leaves a small net regression on this guaranteed-cache-miss worst case (+0.368% vs. +0.452%), not
a win - it shrinks, but does not eliminate, the wrapper overhead, exactly as anticipated in the
backlog item that queued this candidate.

`callgrind_annotate` shows the residual extra cost is not clone-related (see above) but comes
from `hashbrown`'s own `Entry`/`rustc_entry` machinery: `<HashMap<...>>::rustc_entry` (40,880
`Ir`), `RawTable::reserve_rehash` (51,860 + 27,050 `Ir` across two call sites), and
`std::collections::hash::map` (52,190 `Ir`) all appear as new named symbols in the patched
binary's annotated output with no equivalent in the baseline (which has no `resolved_font_cache`
at all) - `entry()` still has to probe the table for a slot before `or_insert_with`'s closure
even runs, and that + insertion bookkeeping still costs more than the identical no-cache path,
just less than a fully separate `get()` call followed by a fully separate `insert()` call.

Wall-clock was not separately re-measured for this candidate (single-core or multi-core): the
effect being tested here (+0.368%) is smaller than candidate 021's own +0.452%, which 021 already
established was below what this sandbox's wall-clock noise floor can resolve (candidates
018/019's `paired.sh` comparisons at similar or larger magnitudes came back as coin flips) -
re-attempting wall-clock at an even smaller magnitude would not add information beyond what
018/019/021 already established.

## Does it change behavior/output?

- **Bit-exact?** Yes. Baseline, candidate 021's variant, and this candidate's variant all produce
  the identical pixmap hash (`0x31cced99a6bfe199`) for `worst_case_cache.pdf`.
- Not re-run against the project test suite in this candidate - font resolution *semantics* are
  unchanged from 007/012/018/019/020/021 (same cache, same key, same scoping), only the
  hashmap-access pattern differs, which cannot affect what font gets resolved. No reason to expect
  the previously-established failing-test-name set (pre-existing missing-test-asset environment
  issues, unrelated to any of this family of patches) to differ here.
- **Is this a breaking change?** No - internal to `hayro-interpret::interpret()`, no public API
  change, same as the rest of this candidate family.

## Why the result looks the way it does

`resolved_font_cache`'s miss-path overhead comes from two hashmap touches under the original
`get()`+`insert()` pattern: one failed lookup, one insert. `Entry` collapses this to conceptually
one traversal, but `HashMap::entry()` (via `hashbrown`'s `rustc_entry`) still has to locate (or
determine the absence of) a slot before handing control to `or_insert_with`'s closure, and then
still does the reserve-check/insert bookkeeping - so the "one traversal instead of two" framing
is directionally correct (this candidate is cheaper than 021's) but the two operations were never
as fully separate/redundant as "double traversal" suggests once inlining and hashbrown's internal
probing are accounted for; some of what `get()`+`insert()` paid twice, `entry()` still pays once,
plus its own entry-specific bookkeeping. The net effect: real, reproducible, but smaller than
"shrink to (or near) zero" would have suggested - about a fifth of the original regression is
recovered, four-fifths of it isn't.

## What's not done / open questions

- Only `Ir` was profiled (no `--cache-sim=yes`/`--branch-sim=yes`), consistent with every prior
  candidate in this family.
- No attempt was made to reduce the residual `Entry`-API overhead further (e.g. computing the
  hash once and using `raw_entry_mut` / a lower-level `hashbrown` API directly) - flagged as a new
  backlog idea rather than tried here, since it would move further from safe, idiomatic
  `std`/`FxHashMap` usage for a sub-0.4% effect that itself isn't resolvable at wall-clock
  granularity (per candidates 018/019/021).
- As with 021, this is the theoretical worst case (every `Tf` call a fresh, never-reused resource
  name), not a demonstrated real-world regression - no PDF in `hayro-tests`' 360-file corpus is
  known to have this shape.
