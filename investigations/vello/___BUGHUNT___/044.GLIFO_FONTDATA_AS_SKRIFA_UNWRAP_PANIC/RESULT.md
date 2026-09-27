# GLIFO_FONTDATA_AS_SKRIFA_UNWRAP_PANIC

`glifo/src/glyph.rs`'s private `FontDataExt::as_skrifa()` helper (used at every point `glifo`
actually needs to look inside the font: `draw_glyphs`, `decoration_spans`, and twice in
`prepare_glyph_run`) does:

```rust
FontRef::from_index(self.data.data(), self.index).unwrap()
```

`skrifa`'s `FontRef::from_index` returns `Result<FontRef, ReadError>` and fails whenever `index`
is out of range for the given font data (`ReadError::InvalidCollectionIndex`), or the sfnt/ttc
header itself can't be parsed at all (`ReadError::InvalidSfnt`, among other `ReadError` variants).
`peniko::FontData::new(data: Blob<u8>, index: u32)` (re-exported from
`linebender_resource_handle`) is a fully public, unchecked constructor - it never validates that
`index` is actually a valid collection index for `data`, nor that `data` parses as a font at all.
This `unwrap()` is reached unconditionally the moment any caller of `vello_cpu`'s or
`vello_hybrid`'s public `RenderContext::glyph_run(...)` API calls `.build()`/`.fill_glyphs()`/
`.stroke_glyphs()` on a `FontData` built this way. This looked suspicious both as a
caller-controlled-input `unwrap()` sitting in a public rendering entry point, and because
`vello_encoding/src/glyph_cache.rs` does the textually-identical `FontRef::from_index(...)` call
one layer over in the "classic" `vello` crate but uses `.ok()?` there instead of `.unwrap()` - an
established, already-used-elsewhere safer pattern for the exact same failure mode this call site
doesn't follow.

Note this is a distinct panic site from candidate 040 (`HEAD_TABLE_UNWRAP_UNITS_PER_EM_PANIC`),
even though both ultimately sit in `prepare_glyph_run`/`glyph.rs`: `as_skrifa()`'s own `unwrap()`
(line 1419) fires *before* candidate 040's `.head().unwrap()` (line ~1629) is even reached, for any
`mode` (`Direct`/`AbsorbScaleUnhinted`/`AbsorbScaleHinted` alike), since the unconditional
`upem` computation calls `as_skrifa()` first. Candidate 040's fix (a `head()` fallback) does not
touch this bug at all - a font whose sfnt/ttc structure itself doesn't parse, or whose collection
index is out of range, still panics here regardless.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: take a real, valid, non-collection font (`examples/assets/roboto/Roboto-Regular.ttf`, unmodified byte-for-byte), and call `peniko::FontData::new(bytes, 1)` - a caller-supplied index of 1 on a font that isn't a collection (only index 0 is valid) | debug-assertions build (`cargo run`) and release build (`cargo run --release`), single-process, real `vello_cpu::RenderContext::glyph_run(...).fill_glyphs(...)` public API call | Panics identically in both builds: `called \`Result::unwrap()\` on an \`Err\` value: InvalidCollectionIndex(1)`, at the exact line named in the candidate |
| Repeat-run check | 4 more runs of the release binary + 1 more `cargo run` (debug) against the same `FontData` | 5/5 panicked, deterministic (no seed/layout dependence - collection-index lookup either succeeds or doesn't) |
| Variant input in the same bug class (CONFIRMATION.md section 7): 32 zero bytes (not a font at all) passed as `FontData::new(garbage, 0)`, driven through the same real `glyph_run(...).fill_glyphs(...)` path | debug + release, single process | Also panics: `InvalidSfnt(0)`, same line - confirms the bug class isn't narrowly specific to the out-of-range-index shape |

A fuzz harness or miri were not needed: this is a plain, deterministically-triggerable
"caller-controlled `Result` unwrapped" defect (same class as candidate 040), not a memory-safety
or flaky-timing issue - manual boundary-value construction (the simplest possible boundary value,
`index = 1` on a non-collection font) found it on the first attempt.

## Status

**Confirmed**. Reproduces 5/5 runs (4 release + 1 debug) for the out-of-range-index input, plus a
second, independently-panicking input in the same class (garbage bytes, `InvalidSfnt`) - both
deterministic, same panic location, no flakiness.

**Fully reproducible**: **Yes**. The repro loads a real font file (and, for the second variant,
uses plain non-font bytes) and hands it to `vello_cpu`'s real, public
`RenderContext::glyph_run(...)` entry point - the exact same code path and API a real consumer
embedding `vello_cpu` to render text with an externally-supplied `(font bytes, collection index)`
pair would use. This is a single external artifact (a byte blob plus an index, both fully
caller-controlled via `peniko::FontData::new`'s public, unchecked constructor) - there is no second
participant to fake; a font-collection index recorded against the wrong file, a subsetting/merging
tool that drops fonts from a `.ttc` without updating callers' stored indices, or simply a caller
passing an arbitrary/untrusted byte blob as "a font" are all realistic real-world sources of
exactly this shape.

## Class

crash

## Root cause

`FontDataExt::as_skrifa()` is the single choke point every other piece of `glifo`'s glyph-run code
goes through to get a `skrifa::FontRef` from the caller-supplied `peniko::FontData`. It calls
`skrifa::FontRef::from_index(data, index)`, which validates the sfnt/ttc structure and the
collection index and returns `Result<FontRef, ReadError>` precisely because both of those things
can legitimately fail for caller-controlled input - `peniko::FontData::new` performs no validation
at construction time. The code unconditionally `.unwrap()`s this `Result` instead of propagating
the failure, so any `(data, index)` pair that isn't already a valid font at that exact index
panics the entire render call, at every one of the four internal call sites that reach it
(`draw_glyphs`, `decoration_spans`, and twice inside `prepare_glyph_run`).

Confirming this really is "caller can trigger it, not just adversarial byte-mangling": no font
bytes were altered at all for the primary repro - the only "malformed" part is the caller-supplied
`index` field, which is a plain public `u32` on a struct (`peniko::FontData`) with a fully public,
unchecked `new(data, index)` constructor. `vello_encoding/src/glyph_cache.rs`'s
`GlyphCache::session` calls the textually identical `skrifa::FontRef::from_index(font.data.as_ref(), font.index)`
but ends the expression in `.ok()?` (returning `None` from a function that already returns
`Option<GlyphCacheSession>`) instead of `.unwrap()` - confirming a missing/invalid font-collection
entry is meant to be handled gracefully elsewhere in this same codebase, not treated as
impossible.

## Introduced by

5796226b "Import glifo into the vello workspace (#1539)" (2026-03-27) - `git blame` on the exact
`FontRef::from_index(self.data.data(), self.index).unwrap()` line shows this is the commit that
first added it to this repository's tracked history (glifo was imported wholesale from an external
repository in this commit; `git log -S` on this exact expression against `glifo/src/glyph.rs`
returns only this one commit, i.e. the expression has never been touched since). Whether the
`unwrap()` itself predates the import in glifo's original (external, untracked-here) history was
not investigated - "not determined" for anything before this repo's own history, same caveat
`candidate 040`'s writeup states for the same imported file.

## The fix

Changed `FontDataExt::as_skrifa()`'s signature from `fn as_skrifa(&self) -> FontRef<'_>` to
`fn as_skrifa(&self) -> Option<FontRef<'_>>` (`.unwrap()` -> `.ok()`), and updated all four
non-test call sites to handle `None` gracefully instead of assuming success:

- `draw_glyphs`: returns immediately (no-op, draws nothing) if the font doesn't parse - a run that
  can't be read has nothing valid to draw.
- `decoration_spans`: returns an empty iterator (changed its return type from
  `impl Iterator<Item = Rect> + 'c` to `Box<dyn Iterator<Item = Rect> + 'c>` to allow this - it's a
  private, single-call-site function, so this is a purely internal signature change) - no font
  means no decoration geometry to compute.
- `prepare_glyph_run`'s `AbsorbScaleHinted` branch: falls back to `hinting_instance: None`
  (unhinted) rather than panicking while building the hint-cache key - the transform/font-size
  math in that branch doesn't actually depend on the font parsing successfully, only the hinting
  instance does.
- `prepare_glyph_run`'s unconditional `upem` computation: falls back to a new
  `FALLBACK_UNITS_PER_EM: f32 = 1000.0` constant when the font doesn't parse, mirroring candidate
  040's fallback approach for the same field, and the same reasoning: this value is only exercised
  further downstream by `draw_glyphs`/`decoration_spans`, both of which now no-op for an invalid
  font anyway, so its exact value has no observable effect once the font fails to parse. (The
  pre-existing, separate `.head().unwrap()` panic on this same line - candidate 040's bug, for a
  font that parses fine but has a broken `head` table - is deliberately left untouched here: fixing
  it is out of scope for this candidate, per "one bug per candidate folder.")

Verified against a second input in the same bug class (CONFIRMATION.md section 7): both the
original out-of-range-index input and the independently-constructed garbage-bytes variant
(`InvalidSfnt`) no longer panic after the patch - re-run 1/1 each in this exact sequence via
`repro/`, plus 3+ additional confirmations of the primary repro during investigation (debug and
release).

- Project test suite: `cargo test -p glifo` - 22 passed, 0 failed both before and after the patch
  (no new failures, no fixed failures - the existing unit tests don't exercise a malformed
  `FontData`/out-of-range index at all, consistent with this being a genuine gap rather than
  something already covered). `cargo test -p vello_cpu` was also run after the patch for broader
  coverage of the public `glyph_run` API surface: 81 passed, 0 failed, plus 1 passing doctest.
  (The workspace-wide `vello_sparse_tests` snapshot-image test crate was not re-run for this
  candidate - candidate 040's `RESULT.md` already documents that its ~445 failures in this
  environment are a pre-existing `git-lfs`-not-installed environment issue unrelated to any code
  change, not something this patch could affect either way.)

Before/after re-running `repro/`: before the patch, both `repro` and `variant_garbage` panic
(`InvalidCollectionIndex(1)` / `InvalidSfnt(0)`, exit 1 after being caught by `catch_unwind` and
rethrown as a nonzero exit); after the patch, both complete normally and print "did not reproduce".

## What's not done / open questions

- The same `FontRef::from_index(...).unwrap()` panic class does not appear to exist anywhere else
  in `glifo` - `as_skrifa()` was the only such call site outside of `#[cfg(test)]` code (a test
  helper at `glifo/src/glyph.rs:2328` also calls `.as_skrifa()` but only ever with the crate's own
  known-good bundled test fonts, so it was left as `.as_skrifa().unwrap()` rather than handled
  gracefully - not reachable with caller-controlled input).
- Not checked: whether `vello`'s (the "classic" GPU-encoding crate, not `vello_cpu`/`glifo`) own
  font-handling code in `vello/src/scene.rs` has an equivalent unchecked `FontRef::from_index`/
  `FontRef::new` call independent of the `.head().unwrap()` one candidate 040 already flagged
  there - a different crate, a different code path, left as a separate suspect.
- No fuzzing was run against `glifo`'s font-loading path as a whole (same note as candidate 040) -
  a short `fuzz-bug-repro` campaign (run separately, per that skill's own scope) targeting
  `GlyphRunBuilder::build` with mutated real fonts and mutated/out-of-range indices as seeds would
  likely be a fast way to find further `unwrap()`-on-`ReadError`/`Option::None` panics beyond the
  two (this one and candidate 040's) already found by hand.
