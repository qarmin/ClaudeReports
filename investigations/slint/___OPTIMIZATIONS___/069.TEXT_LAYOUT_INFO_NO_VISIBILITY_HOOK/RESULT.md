# 69 - `Text::layout_info` has no bounded, low-risk fix for the off-screen-reshape gap

IDEAS.md's top backlog entry (added 2026-08-23, narrowed after candidate 065): the dirty-region-walk
half of "subtree bounds" was tried as candidate 065 (mixed, not a net win). This candidate is the
other half - `Text::layout_info` itself, per candidate 013's finding that it "needs a different fix
shape entirely" since a `Text`'s width is not derivable without shaping it. This is a code-reading
investigation to find a *bounded* fix inside `text_layout_info`
(`internal/core/items/text.rs:643-702`) - it did not turn one up, and this file documents why in
enough detail that the next session doesn't have to re-derive it.

## What was investigated

Three candidate fix shapes, each checked against the actual code and, where possible, against
numbers already measured by earlier candidates in this workspace:

### 1. Skip shaping when the item's width is already fixed (not layout-derived)

If a `Text`'s `width` has an explicit pixel binding (as in `perf_test`'s `FixedLayoutRecordRow`/
`--row-style layout-fixed`), does the enclosing `HorizontalLayout` still ask it for
`layout_info(Horizontal)`? **Already answered, and answered no: candidate 044 measured this exact
row style** (`044.CLIP_PREVENTS_STARTUP_SHAPING/RESULT.md`, "obvious suspects" section): fixed
widths, no stretch, no elide still produced 60 081 paragraph shapes at 20 000 rows - identical to
the stretch+elide row style. Candidate 044's own conclusion: "A `HorizontalLayout` computes its own
layout-info from its children whether or not it has anything left to solve, and a `Text`'s
layout-info is only known after shaping. Removing the stretch does not remove the question, it only
removes what the answer is used for." This closes off "detect an unused width at the layout level"
as a viable angle - the layout genuinely always asks, regardless of whether the answer changes
anything.

### 2. Compute `Orientation::Vertical`'s height from font metrics instead of shaping (`TextWrap::NoWrap` only)

`text_layout_info`'s vertical branch for `NoWrap` calls
`implicit_size(None, TextWrap::NoWrap).height` - a full shape - even though, for a single style run,
height is in principle `line_count * line_height` and `line_count` is just a count of hard newlines
in the text; both derivable from `RendererSealed::font_metrics` (`internal/core/renderer.rs:154-164`)
without shaping. This looked like the one concretely-actionable idea in candidate 013's writeup
("the height is derivable from font metrics alone").

It does not survive contact with how `layout_info` is actually called, for two independent reasons:

- **The horizontal branch already forces the shape, unconditionally, for every `Text` in a layout -
  candidate 044 already measured this.** `044.CLIP_PREVENTS_STARTUP_SHAPING` found that even a row
  with every child width fixed, no stretch, no elide (`layout-fixed`) still produced the same
  60 081 paragraph shapes as the stretch+elide row style - "a `HorizontalLayout` computes its own
  layout-info from its children whether or not it has anything left to solve." So the horizontal
  branch's `implicit_size(None, TextWrap::NoWrap)` call already runs for every `Text` inside any
  `HorizontalLayout`/`GridLayout`, regardless of whether the vertical branch is ever reached for that
  same item in the same pass.
- **That call uses the *exact same* cache key as the vertical branch's `NoWrap` call** - both are
  `implicit_size(None, TextWrap::NoWrap)`, i.e. `TextLayoutCache`'s `(item, wrap)` key with `wrap =
  NoWrap` (`internal/core/items/text.rs:671` and `:688`; the cache key itself is documented in
  `CachedParagraphs`'s doc comment in `internal/core/textlayout/sharedparley/cache.rs` - width is
  applied later by line breaking, not part of the shape key). So for any item where the horizontal
  branch already ran this frame (per the point above: every `Text` inside any layout, always), a
  vertical-branch fast path would only ever hit an already-warm cache entry - there is no shape left
  to avoid. And for an item where the horizontal branch does *not* run (a `Text` outside any layout
  that reads its size at all), `layout_info` isn't being called in the first place, so there is
  nothing for a vertical-only fast path to save there either. Candidate 013's own measurement is
  consistent with this: 3 shapes per changed row (one per `Text`), not 6 (one per `Text` per
  orientation) - there was never a redundant second shape to remove.

Also a correctness concern on its own, independent of the above: replicating parley's line-height
bit-for-bit (leading, `line_height_factor` rounding, per-run ascent/descent for text that falls back
across fonts mid-line) outside the shaper is a second, separately fragile implementation of the same
metric that the draw path also has to agree with - exactly the invariant `projects/slint.md` section 4
warns about ("measuring and drawing share one cache entry... a lazy path may skip the *work*, never
the *reads*"). Not attempted given (a) and (b) above already rule out any measurable benefit.

### 3. A "width is unused" flag from the compiler (candidate 012's option 2)

Already tracked as its own, separate, larger backlog entry (candidate 012's "compiler-provable
children stay inside root" idea, listed in IDEAS.md as enabling both this and candidate 049's render
pruning) - a static/interval analysis of `x`/`y`/`width`/`height` expressions across the whole
component graph, not something this candidate's scope (a fix confined to
`Text::layout_info`/`text_layout_info`) can absorb. Left as-is in the backlog rather than duplicated
here.

## Result

No patch. All three fix shapes bounded enough to fit a single candidate either (1) are already
proven not to help by a prior candidate's own measurement (044), or (2) would provide zero
measurable benefit on any layout construct the compiler actually generates, confirmed by code
reading against `text_layout_info`'s two orientation branches and `TextLayoutCache`'s cache-key
sharing (no new patch to attribute a number to). No build/bench run was needed beyond re-reading the
numbers already on record in candidates 013 and 044, since the question was "does a bounded fix
exist", not "how much does an implemented one help".

## Does it change behavior/output?

N/A - no patch applied to real source.

## Why the result looks the way it does

The width of an unwrapped `Text` is genuinely load-bearing information for every layout combinator
that reads it (main-axis distribution needs it even when the final width is fixed, cross-axis
alignment needs the *other* axis too) - there is no "the answer doesn't matter" case reachable
through the compiler's own generated code, only through hypothetical layouts nobody's generator
produces. The dependency this backlog item is chasing is structural (the layout pass has no
`visible`/`on-screen` bit at all, per `projects/slint.md`'s "same gap on three passes" note) rather
than a local inefficiency inside `Text::layout_info` itself - which is exactly what candidate 013
concluded and this candidate reconfirms with more code-level detail, without finding a way around it
that fits in one candidate's scope.

## What's not done / open questions

- The two larger ideas this backlog item explicitly named as the real fix shapes - a
  compiler-provable "width is unused" flag (candidate 012 option 2) and a visibility notion
  available one pass before layout runs - remain untried and are already tracked as their own
  backlog entries, not duplicated here.
- Not measured: by the same logic as candidate 044 (a `HorizontalLayout` unconditionally queries
  `Horizontal` from every child), a `VerticalLayout` of `Text`s with a fixed `width` (so nothing ever
  needs the horizontal branch) should symmetrically query only `Vertical` - a case fix shape 2 could
  actually fire for. Not confirmed against real code or measured; `perf_test` has no such scene
  (every row style is a `HorizontalLayout`). Flagged in IDEAS.md as a narrower follow-up rather than
  attempted here, since it would need a new scene to even observe, and the correctness risk of a
  parley-independent height computation (noted above) applies regardless of how it's reached.
