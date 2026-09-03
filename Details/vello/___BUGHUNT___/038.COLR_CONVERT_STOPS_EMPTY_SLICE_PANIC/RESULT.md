# COLR_CONVERT_STOPS_EMPTY_SLICE_PANIC

`glifo/src/colr.rs`'s `ColrPainter::convert_stops` (lines 149-193, the specific indexing at
164-165) indexes `stops[0]` and `stops.last().unwrap()` with no length check on the
`&[skrifa::color::ColorStop]` slice it receives. The candidate suspected a malformed/adversarial
COLRv1 font whose `ColorLine` table declares zero stops could reach this function with an empty
slice and panic the glyph-rendering path, unlike the neighboring `palette_index_to_color` which
already falls back to `AlphaColor::BLACK` via `unwrap_or`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Source reading of the dependency chain | Read `skrifa` 0.44.0 (the exact version `glifo` depends on per `Cargo.lock`) `src/color/traversal.rs`, `traverse_with_callbacks`, all three gradient arms (`ResolvedPaint::LinearGradient`/`RadialGradient`/`SweepGradient`) | Every arm resolves the raw `ColorLine`'s stops into a `resolved_stops: ColorStopVec` via `make_sorted_resolved_stops` (which `.clear()`s the buffer first, so a 0-stop `ColorLine` yields an empty vec), then checks `(resolved_stops.first(), resolved_stops.last())` - `(None, _) \| (_, None) => {}` for Linear/Radial, an explicit `if resolved_stops.is_empty() { return Ok(()); }` for Sweep - and returns without ever calling `painter.fill(..)` in that case. `convert_stops` is only ever called from `glifo::colr::ColrPainter::fill`'s three gradient match arms, and `fill` is only ever invoked by skrifa's own traversal (there is no other production call site into `convert_stops` anywhere in the codebase) |
| Real, end-to-end, debug-build reproduction | Built a real malformed COLRv1 font (a byte-identical copy of the project's own `examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf`, with only the 2-byte `numStops` field of glyph 8's `PaintLinearGradient`'s `ColorLine` table overwritten from 2 to 0 - see `repro/tools/font_patch/`), then rendered glyph 8 through the real public entry point (`vello_cpu::RenderContext::glyph_run(..).fill_glyphs(..)`, the same path any real caller uses), 3 repeat runs, debug build | Did not panic, 3/3 runs. Temporary `eprintln!` instrumentation added to `ColrPainter::fill` (reverted before finishing) showed the original font prints exactly one `DEBUG fill LinearGradient color_stops.len()=2` for glyph 8, while the patched (0-stop) font prints **no** `DEBUG fill ...` line at all for the same glyph - confirming empirically that skrifa's own traversal short-circuits before ever calling into glifo, exactly as the source reading predicted |
| Release-build fresh-clone reproduction | `just test` in this candidate's own folder: fresh `git clone` of `linebender/vello` at `base_commit`, `cargo run --release` against the malformed font via path-patched `glifo`/`vello_cpu`/`vello_common` pointing at that clone | Did not panic (see `output.txt`) |

No fuzzing/miri/ASAN was used - this is a pure logic/reachability question ("can this slice ever be
empty at this call site through the real caller"), not a memory-safety or arbitrary-input-space
question, and the two real-path attempts above (a debug build with hand-boundary-value
construction, i.e. the num-stops=0 boundary, plus the release-build fresh-clone repro) already gave
a clean, mechanistically-explained negative - a third tool would not add signal here.

## Status

**Not reproducible.**

Covered: hand-constructed boundary value (`numStops = 0`, the exact value the candidate worried
about) for all three of the gradient kinds that route through `convert_stops` was checked by
source reading (Linear/Radial/Sweep all have the guard in `skrifa` 0.44.0's traversal code); the
concrete zero-stops case was additionally built into a real font and rendered end-to-end 4 times
total (3x debug build + 1x release build against a fresh clone), all non-panicking. This is a
complete negative for the `skrifa` version currently pinned by this project (`0.44.0`, see
`Cargo.lock`) - it was not checked against every historical `skrifa` version glifo has ever
depended on, since the question is "does this repo's currently-shipped code have this bug", not
"did skrifa ever have this gap".

**Fully reproducible**: N/A (Status is Not reproducible, not Confirmed - no repro is claimed to
demonstrate a bug; `repro/` instead demonstrates the negative result, i.e. that the suspected
panic does not occur, and does so by running the real public entry point end-to-end against a
genuinely malformed input file, not a hand-authored stand-in for what a caller would produce, so
the same real-path bar from `CONFIRMATION.md` section 11 is still met for this negative finding).

## Root cause (of why the suspect turned out to be safe)

`glifo::colr::ColrPainter::convert_stops` genuinely has no defensive check against an empty input
slice, and would panic if ever called with one (indexing `stops[0]` on an empty `SmallVec`, or
`stops.last().unwrap()` on `None` - either would panic). However, `convert_stops` is private
(`fn`, no `pub`) and is only ever called from the three `Brush::{Linear,Radial,Sweep}Gradient`
arms inside `ColrPainter::fill` (`impl ColorPainter for ColrPainter`), and `ColorPainter::fill` is
only ever invoked by `skrifa_glyph.paint(location_ref, self)` inside `ColrPainter::paint` -
i.e. entirely by `skrifa`'s own internal COLR paint-graph traversal, never called directly by
`glifo`/`vello` code with attacker-controlled data.

`skrifa` 0.44.0's traversal (`skrifa-0.44.0/src/color/traversal.rs`,
`traverse_with_callbacks`) already resolves+validates each `ColorLine`'s stops into its own
`resolved_stops` buffer (via `make_sorted_resolved_stops`, straight from the raw, unvalidated font
bytes - so a 0-stop `ColorLine` really does produce an empty `resolved_stops`) and explicitly
special-cases the empty case for every one of the three gradient `Paint` variants **before**
constructing the `Brush` and calling `painter.fill(brush)` - it simply never calls `fill` at all
when there are no stops. So by the time `glifo`'s `fill`/`convert_stops` ever sees a
`&[ColorStop]`, `skrifa` has already guaranteed it is non-empty for every real caller. The
candidate's premise ("the COLRv1 color_stops slice... is built directly from a font's raw,
unvalidated ColorLine table via skrifa" reaching `convert_stops` unchecked) is not accurate for the
currently-pinned `skrifa` version: skrifa validates/filters this specific case upstream of the
callback boundary, it just doesn't do so in the neighboring `palette_index_to_color` codepath
(which is `glifo`'s own function operating on a `CPAL` table index directly, not something skrifa
pre-validates for it - a different, unrelated data source, which is presumably why that one needed
its own `unwrap_or` fallback while `convert_stops` apparently never has needed one in practice).

`convert_stops` having no defensive check is still true as a matter of code hygiene (it is one
`skrifa` upstream behavior change away from becoming reachable with an empty slice, and nothing in
`glifo` documents or enforces the invariant it's silently relying on) - but no currently-reachable
path in this codebase, with this dependency version, can trigger it.

## Introduced by

N/A - no bug was confirmed, so there is nothing to attribute to an introducing commit. (For
context only, not as an "introduced by": `convert_stops`'s indexing has looked the same,
unguarded, since it was added; what changed nothing was `skrifa`'s own guard, which was not
specifically investigated for when it was added upstream, since that's not this project's commit
history.)

## The fix

No fix was made - there is no confirmed bug in the current codebase to fix. A defensive early
return in `convert_stops` (e.g. `if stops.is_empty() { return ColorStops(SmallVec::new()); }` or
similar, mirroring `palette_index_to_color`'s own defensiveness) would be a reasonable, cheap
hardening change purely against a future `skrifa` upgrade silently dropping its own upstream
guard (or a future code path being added inside `glifo` that calls `convert_stops` directly,
bypassing skrifa's traversal) - but since nothing currently reaches it, no such change was applied
in this session per the workflow's "only fix confirmed bugs" discipline. If a maintainer wants
defense-in-depth here regardless of current unreachability, that's a maintainer judgment call, not
something this session forces.

- Project test suite: N/A, no change was made to the source (this candidate never touched
  `glifo`'s real working tree except for a temporary, reverted `eprintln!` instrumentation used
  purely to observe whether `fill`/`convert_stops` was reached - see `output.txt`). No before/after
  test run was needed since there is no patch.

## What's not done / open questions

- Only `skrifa` 0.44.0 (the version currently pinned in this repo's `Cargo.lock`) was checked.
  If this project's `skrifa` dependency is ever upgraded, it would be worth re-checking that the
  same `(None, _) | (_, None)` / `is_empty()` guards still exist in whatever version is pinned
  next - `convert_stops` itself has no independent safety net if that upstream guarantee ever
  regresses.
- `GlyphInfoExtractor`'s own `ColorPainter` impl (the other implementor in `glifo/src/colr.rs`)
  has an empty `fill(&mut self, _brush: Brush<'_>) {}` and never calls `convert_stops` at all, so
  it was not a candidate for this specific bug, and wasn't investigated further.
- This candidate did not attempt COLRv1 `VarLinearGradient`/`VarRadialGradient`/
  `VarSweepGradient` (variable-font color lines) specifically with a real variable-font file; the
  guard code in `skrifa`'s traversal is textually identical for the var and non-var gradient arms
  (same `make_sorted_resolved_stops`/`resolved_stops.first()`/`.last()` pattern reused for both),
  so this isn't expected to differ, but it was reasoned from source rather than built into a
  second live repro, since the non-var case already gave a clean, mechanistically-explained
  negative.
