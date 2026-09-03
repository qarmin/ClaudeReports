# OUTLINE_DRAW_UNWRAP_DRAWERROR_PANIC

`glifo/src/glyph.rs`'s `OutlineCacheSession::get_or_insert` (the main glyph-outline caching path,
reached from every glyph-fill/stroke/underline call) does
`outline_glyph.draw(draw_settings, &mut drawing_buf).unwrap()` on a `skrifa::outline::OutlineGlyph`
obtained from font data. `skrifa::outline::DrawError` (`RecursionLimitExceeded`, `TooManyPoints`,
`InvalidAnchorPoint`, `PostScript(CffError)`, `Read(ReadError)`, `InsufficientMemory`, ...) is
returned by skrifa itself when it detects a glyph it cannot safely rasterize - this candidate
checked whether a real, well-formed-but-pathological font file can drive `.draw()` into returning
`Err` for a glyph that `outlines.get(glyph_id)` (the only upstream guard in this file) already
accepted, turning a bad-but-parseable font into a hard process panic.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: a hand-built TrueType font (via the `write-fonts` crate) whose glyph id 1 is a composite glyph that references itself as its only component (an infinite composite cycle) | Debug + release build, driven through `skrifa` 0.44.0's own public `FontRef`/`OutlineGlyphCollection` API directly (no glifo) to characterize the failure mode before building the full glifo repro | Did not reproduce a `.draw()` panic: `OutlineGlyphCollection::get(gid)` itself already calls skrifa's internal `Outlines::outline()` (the composite-tree counting/traversal pass) and swallows the same `RecursionLimitExceeded` the counting pass hits, returning `None`. Since glifo's own code only ever calls `.draw()` on an outline it got back from a successful `outlines.get()`, a self-referencing/cyclic composite never reaches `create_outline_glyph`/`get_or_insert` at all - this particular `DrawError` variant is already effectively guarded, in this exact code, before `.draw()` is called. |
| Manual boundary-value construction: a second hand-built TrueType font mirroring skrifa's own regression test for oss-fuzz issue 391753684 (`composite_with_too_many_points`, `skrifa-0.44.0/src/outline/glyf/mod.rs`) - a composite glyph combining two references to a 40,000-point simple glyph, for 80,000 total points, over skrifa's internal 64k (`u16::MAX`) per-outline point limit | Same direct-skrifa probe first (debug build) | `outlines.get(gid)` succeeds (point-count isn't checked during the counting/traversal phase that backs `get`); `outline.draw(...)` returns `Err(DrawError::TooManyPoints(gid))`. This is exactly the "outline obtained, then `.draw()` fails" shape glifo's `.unwrap()` cannot survive. |
| Same font driven through the real, public, unmodified `vello_cpu` glyph-rendering entry point (`RenderContext::glyph_run(&mut resources, &font).font_size(16.0).fill_glyphs(std::iter::once(glyph))`, glyph id 2 = the too-many-points composite), release build, `repro/` | Real end-to-end path, no internal-API shortcuts | **Reproduced**: hard panic at `glifo/src/glyph.rs:1985:69`, `called \`Result::unwrap()\` on an \`Err\` value: TooManyPoints(GlyphId(2))`. |
| Same repro re-run 3x | Same release build/font/glyph id | Reproduces deterministically 3/3 (no randomness anywhere in the path: fixed font bytes, fixed glyph id, fixed font size) - see `output.txt`. |

Never needed a 3rd escalation tool (miri/ASAN/fuzz) - once the `TooManyPoints` variant was
identified as reachable past `outlines.get()` (by mirroring skrifa's own already-published
regression test for that exact gap), a plain hand-built font plus a real end-to-end run through the
public API was sufficient to both trigger it and observe the panic.

## Status

**Confirmed.** Reproduces deterministically (3/3 runs, release build, no fixed seed needed - fully
deterministic font bytes/glyph id/font size) via a real, unmodified call through `vello_cpu`'s
actual public glyph-rendering API.

**Fully reproducible**: **Yes.** `repro/` drives the real, unmodified public entry point a genuine
application would use - `vello_cpu::RenderContext::glyph_run(&mut resources, &font).font_size(16.0)
.fill_glyphs(...)` - with no internal-API shortcuts and no cache-config tampering. The only
"unusual" input is the font file itself: a complete, well-formed TrueType font (valid `head`/`hhea`/
`maxp`/`hmtx`/`loca`/`glyf` tables, correctly cross-referenced) whose only pathology is that one
glyph's total point count (80,000, via a composite combining two references to a 40,000-point
simple glyph) exceeds skrifa's internal 64k limit - the same shape of font a real subsetter bug, a
deliberately malicious font, or (per skrifa's own changelog, this is a real oss-fuzz-discovered
construction, issue 391753684) a fuzzer-derived font could produce. There is no second participant
to fake here (see `CONFIRMATION.md` section 11's "malformed file handed to a parser's public entry
point" exception) - this is a single-process, single-artifact reproduction of a genuine bug in how
the renderer reacts to a legitimately-parseable font.

## Class

crash (unconditional panic / process abort on adversarial-but-well-formed font input, in the main
glyph-outline caching path used by every glyph fill/stroke/decoration call).

## Root cause

`OutlineCacheSession::get_or_insert` (`glifo/src/glyph.rs`) calls
`outline_glyph.draw(draw_settings, &mut drawing_buf).unwrap()` on every cache miss. `draw` returns
`skrifa::outline::DrawError` for several classes of glyph data that skrifa's own `outline()`/`get()`
pass does not reject up front - `TooManyPoints` is the concrete one demonstrated here: the
counting/traversal pass that backs `OutlineGlyphCollection::get(gid)` accumulates a composite
glyph's total point count without bounding it, so `get()` returns `Some(outline)` for an outline
that is later rejected, only once actual scaling/rasterization is attempted inside `.draw()`, when
skrifa's `FreeTypeScaler` construction checks the accumulated point count against its internal u16
buffer-sizing limit. glifo's only upstream guard against a bad outline
(`let Some(outline) = outlines.get(glyph_id) else { continue; };`, both in the main
`draw_glyphs` loop and in `decoration_spans`) is therefore insufficient on its own - it filters
outlines skrifa rejects at `get()` time, but not outlines skrifa accepts at `get()` time and only
rejects at `.draw()` time. The `.unwrap()` assumed the latter case couldn't happen.

(A different `DrawError` variant, `RecursionLimitExceeded`, was also investigated as a candidate
trigger for the same `.unwrap()`, but turned out to already be filtered by the existing `get()`
guard for this exact code path - see "What was tried" above. `TooManyPoints` is the variant that
survives past that guard.)

## Introduced by

5796226b "Import glifo into the vello workspace (#1539)" - this is the commit that brought the
`glifo` crate (and this exact `.unwrap()`) into this repository's history as a 1:1 copy-paste of a
pre-existing, externally-developed crate (per that commit's own message: "First commit is a 1:1
copy-paste of the crate, all subsequent commits finetune it to work with the workspace structure").
`git log -S`/`git blame` on `glifo/src/glyph.rs` both dead-end at this import commit for the
`outline_glyph.draw(...).unwrap()` line - the code's actual original authorship predates this
repository's own tracked history (the pre-import `glifo` crate's own history is not part of this
checkout).

## The fix

Changed `OutlineCacheSession::get_or_insert` to return `Option<CachedOutline<'_>>` instead of
`CachedOutline<'_>`: on a `draw()` failure, it now returns the pooled drawing buffer to the free
list and returns `None` instead of unwrapping, mirroring the exact "skip this glyph" behavior this
file already uses for a `None` from `outlines.get(glyph_id)`. This return-type change propagates
through every caller:

- `create_outline_glyph` (`glifo/src/glyph.rs`) now returns `Option<GlyphType<'a>>`.
- The main `draw_glyphs` outline-rendering loop and `decoration_spans` (both in
  `glifo/src/glyph.rs`) now use `let Some(...) = ... else { continue; };` at their respective call
  sites, skipping only the one unrenderable glyph instead of aborting the whole render.
- Both COLR call sites (`glifo/src/colr.rs`, `ColrPainter::get_outline` and
  `GlyphInfoExtractor::get_outline`) already returned `Option<CachedOutline<'_>>` themselves (via
  `?` on the `outline_glyphs.get()` call) and previously wrapped the result in `Some(...)`; they now
  return `get_or_insert`'s own `Option` directly.

This is sufficient for more than just the one triggering point count: the check is a plain
`is_err()` on skrifa's own `Result`, so it covers every `DrawError` variant that can reach this
call, not just `TooManyPoints` (verified `RecursionLimitExceeded` behavior separately - see "What
was tried" - which turns out not to reach this line at all for this exact code path, but would also
be handled gracefully if it ever did, e.g. via a future skrifa change to when that check happens).

Re-running `repro/` (release build) against the patched tree: no longer panics, "did not reproduce:
rendering completed without panicking" on 3/3 runs (previously panicked 3/3 runs against the
unpatched tree) - see `output.txt`.

- Project test suite:
  - `cargo test -p glifo --all-features`: 24 passed / 0 failed before -> 24 passed / 0 failed after
    (unit tests), 0/0 doctests both before and after.
  - `cargo test -p vello_cpu --all-features`: 87 passed / 0 failed before -> 87 passed / 0 failed
    after (unit tests), 1/1 doctests both before and after.
  - `cargo test -p vello_hybrid --all-features`: 122 passed / 0 failed before -> 122 passed / 0
    failed after (unit tests), 0 passed / 1 ignored doctests both before and after.
  - `cargo test -p vello_sparse_tests --all-features`: 89 passed / 3538 failed, both before *and*
    after the patch (identical failure set in both cases, confirmed by running the exact same
    command against the unpatched tree) - this is a pre-existing environment issue in this sandbox
    unrelated to the patch (this crate's snapshot/reference-image tests appear to need setup this
    checkout doesn't have), not a regression introduced by the fix.
  - `cargo check --workspace --all-features`: clean (no errors) both before and after.

## What's not done / open questions

- Only `TooManyPoints` was actually driven to a reproduction; the other `DrawError` variants
  (`InvalidAnchorPoint`, `PostScript(CffError)`, `Read(ReadError)`, `InsufficientMemory`,
  `HintingFailed`) were not individually constructed and tried against this exact `.unwrap()` this
  session. The fix does not depend on which variant occurs (it treats every `Err(_)` uniformly), so
  it should cover all of them, but that wasn't independently verified per-variant.
- `RecursionLimitExceeded` specifically was shown *not* to reach this `.unwrap()` via the normal
  `outlines.get()` -> `.draw()` path in this glifo version/skrifa version pairing (skrifa 0.44.0) -
  worth re-checking if either dependency is upgraded, since that guarantee lives in skrifa's
  internal implementation, not in glifo's own code.
- Did not check whether the same `TooManyPoints`-shaped gap (an outline accepted by `outlines.get()`
  but rejected once actual rasterization/scaling is attempted) exists anywhere else that calls
  `skrifa::outline::OutlineGlyph::draw` - a repo-wide grep during this session
  (`grep -rn "outline_glyph.draw"` across `glifo/src` and the `sparse_strips/*/src` crates) found
  only the one call site fixed here, so this is believed to be the only occurrence, but this wasn't
  exhaustively re-verified against every crate in the workspace.
- A longer fuzzing campaign specifically targeting hand-crafted `glyf`/`CFF` tables driven through
  `glifo`'s public glyph-rendering API (via `fuzz-bug-repro`, run separately) could plausibly turn
  up other `DrawError`-shaped or entirely different panics in the same area; not attempted this
  session beyond the two manually-constructed boundary fonts above.
