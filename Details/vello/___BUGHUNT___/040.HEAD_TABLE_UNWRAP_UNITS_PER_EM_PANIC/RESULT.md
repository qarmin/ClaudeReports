# HEAD_TABLE_UNWRAP_UNITS_PER_EM_PANIC

`glifo/src/glyph.rs`'s `prepare_glyph_run` (the per-glyph-run setup function, run unconditionally
every time `GlyphRunBuilder::build`/`.fill_glyphs`/`.stroke_glyphs` is called through `vello_cpu`'s
or `vello_hybrid`'s public `RenderContext::glyph_run` API) does:

```rust
let upem = run.font.as_skrifa().head().map(|h| h.units_per_em()).unwrap().into();
```

`skrifa`'s `head()` accessor returns `Result<Head, ReadError>` and fails whenever the font's
`head` table is missing or malformed, which is a normal thing for a corrupted/truncated/adversarially-crafted
font file to have even while the rest of the sfnt table directory is well-formed. This looked
suspicious because it's a caller-controlled-input `unwrap()` sitting directly in a rendering entry
point that any consumer of `vello_cpu`/`vello_hybrid` reaches by simply calling
`RenderContext::glyph_run(...)` with a `peniko::FontData` built from arbitrary bytes (`FontData::new`
is fully public and takes any `Blob<u8>`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: take a real, valid font (`examples/assets/roboto/Roboto-Regular.ttf`), flip one byte in the `head` table's 4-byte tag in its sfnt table directory (leaving every other byte, including all other tables' offsets/checksums, untouched) | debug-assertions build (`cargo run`) and release build (`cargo run --release`), single-process, real `vello_cpu::RenderContext::glyph_run(...).fill_glyphs(...)` public API call | Panics identically in both builds: `called \`Result::unwrap()\` on an \`Err\` value: TableIsMissing(Tag(head))`, at the exact line named in the candidate |
| Repeat-run check | 4 more runs of the release binary + 1 more `cargo run` (debug) against the same corrupted font bytes | 5/5 panicked, deterministic (no seed/layout dependence - table lookup either finds the tag or it doesn't) |

A fuzz harness or miri were not needed: this is a plain, deterministically-triggerable
"caller-controlled Result unwrapped" defect, not a memory-safety or flaky-timing issue - manual
boundary-value construction (heuristic in `HEURISTICS.md` for "arithmetic/parsing on untrusted
input, deterministic failure mode") found it on the first attempt and 3 escalating tools were not
needed.

## Status

**Confirmed**. Reproduces 5/5 runs (4 release + 1 debug), deterministic - same panic message and
location every time, no flakiness.

**Fully reproducible**: **Yes**. The repro loads a real font file and hands it to `vello_cpu`'s
real, public `RenderContext::glyph_run(...)` entry point - the exact same code path and API a real
consumer embedding `vello_cpu` to render text with an externally-supplied font would use. A
malformed font file *is* the single external artifact here (comparable to "a malformed file handed
to a parser's real public `open_file` entry point" in `CONFIRMATION.md` section 11) - there is no
second participant to fake; a corrupted/truncated font download, a deliberately crafted font, or
simply a buggy font subsetter/optimizer run by whoever produced the font file are all realistic
real-world sources of exactly this "valid sfnt directory, broken `head` table" shape.

## Class

crash

## Root cause

`prepare_glyph_run` needs a font's units-per-em value to decide the glyph-outline cache size and
scale factor. It gets this via `skrifa`'s `FontRef::head()`, which returns `Result<Head, ReadError>`
because the `head` table can legitimately be missing (font subsetting tools sometimes drop it) or
fail to parse (truncated/corrupted file, or its record's declared length being too short for the
table's fixed 54-byte layout). The code unconditionally `.unwrap()`s this `Result` instead of
falling back to a default or otherwise handling the error, so any font whose `head` table doesn't
parse cleanly panics the entire render call. Note this is not specific to the `AbsorbScaleHinted`
mode named in the candidate description: the `upem` computation sits after the `match mode { ... }`
block and executes for every mode (`Direct`, `AbsorbScaleUnhinted`, `AbsorbScaleHinted`) alike -
any call to `.build()`/`.fill_glyphs()`/`.stroke_glyphs()` on a font with this defect panics,
regardless of what transform is in effect.

Confirming the table itself really can't be found (not "garbage in, panic out"): `FontRef::from_index`
only parses the sfnt header and table-directory records (12+16*N bytes) - it does not validate
individual tables' contents - so a font can pass `from_index` fine while `head()`'s
`self.expect_table()` (in `read-fonts`) still fails to find/parse the table. `skrifa`'s own outline
construction (`glyf::Outlines::new`, `outline/glyf/mod.rs:133`) treats exactly this situation as an
ordinary `Option::None` (`font.head().ok()?.units_per_em()`) rather than panicking, confirming a
missing/malformed `head` table is meant to be handled gracefully, not treated as impossible.

## Introduced by

92d98d44 "glifo: Always draw non-hinted glyphs at upem size (#1629)" (2026-05-22) moved this exact
`.head().map(|h| h.units_per_em()).unwrap()` expression from inside `draw_glyphs` (previously
computed once per fill/stroke draw call, at `glifo/src/glyph.rs` old line 75 in that diff) into the
new `prepare_glyph_run` (now computed once per `.build()` call, i.e. even earlier, before any glyph
is even iterated) - `git log -S "units_per_em() ).unwrap()" -- glifo/src/glyph.rs` shows this is the
only commit that touched this exact expression's surrounding structure after glifo's initial import.
The unwrap-on-Result antipattern itself predates that commit (it already existed, unconditionally
called from `draw_glyphs`, at the point glifo was first imported into this workspace: 5796226b
"Import glifo into the vello workspace (#1539)") - 92d98d44 changed *where* and *how early* it runs,
not whether it could panic at all, so a maintainer investigating "was this always broken" should
know the underlying `.unwrap()` predates 92d98d44 even though this candidate's specific panic
location (line 1629, inside `prepare_glyph_run`) was introduced by it.

## The fix

Replaced `.unwrap()` with `.unwrap_or(FALLBACK_UNITS_PER_EM)` where `FALLBACK_UNITS_PER_EM: u16 =
1000` is a new documented constant, matching skrifa's own graceful-degradation convention for this
exact situation (see Root cause above) instead of introducing a new one. This keeps `upem` a plain
`f32` (no API/type changes propagate anywhere else), so the fix is fully local to the one call site.
1000 has no functional significance beyond being a common, plausible units-per-em value (typical of
PostScript-flavored fonts) - the point is only to keep rendering going instead of panicking; a font
with a broken `head` table has bigger problems than this fallback value being slightly off from
whatever the font's original intended metric was.

- Project test suite: `cargo test -p glifo` - 22 passed, 0 failed both before and after the patch
  (no new failures, no fixed failures - the existing unit tests don't exercise a corrupted `head`
  table at all, consistent with this being a genuine gap rather than something already covered).
  `cargo test -p vello_sparse_tests --test tests -- glyph` was also run for broader coverage: 445
  failed / 0 passed / 99 ignored **both before and after** the patch, identically - these failures
  are a pre-existing environment issue unrelated to this change (`git-lfs` is not installed in this
  environment, so the test crate's snapshot PNGs are LFS pointer files, and every image-comparison
  test panics at `sparse_strips/vello_sparse_tests/tests/util.rs:403` on
  `Unsupported(UnsupportedError { format: Unknown, kind: Format(Unknown) })` trying to decode a
  pointer file as a PNG - confirmed identical failure count/reason with the patch reverted). Treated
  as N/A for judging this fix; `-p glifo`'s own unit tests are the meaningful signal here and they
  are clean before and after.

Before/after re-running `repro/`: before the patch, `cargo run --release` panics
(`TableIsMissing(Tag(head))`, exit 1 after being caught by `catch_unwind` and rethrown as a nonzero
exit); after the patch, the exact same input completes normally and prints "did not reproduce".

## What's not done / open questions

- `FontRef::from_index(self.data.data(), self.index).unwrap()` in the same file's
  `FontDataExt::as_skrifa()` (glifo/src/glyph.rs) has the identical defect class one layer up: a
  font whose *sfnt table directory itself* (not just one table) is malformed panics there instead,
  before `prepare_glyph_run` is even reached. Not investigated this round - a different call site,
  a different `ReadError` variant, and arguably a bigger attack surface (any structurally-broken
  font, not just a broken `head` table) - left as a separate suspect.
- `vello/src/scene.rs:672` (`vello`, the "classic" GPU-encoding crate, not `vello_cpu`/`glifo`) has
  the exact same `font.head().map(|h| h.units_per_em()).unwrap()` pattern in `try_draw_colr`, for
  COLR (color) glyph runs specifically. Not investigated this round since it's a different crate
  and a different code path, but it is the same bug class and likely panics the same way.
- `sparse_strips/vello_example_scenes/src/emoji_grid.rs` also calls `.head()` on a font; not
  checked whether it `.unwrap()`s or handles the error (it's example/demo code exercising bundled
  known-good fonts, so lower priority, but noted for completeness).
- No fuzzing was run against `glifo`'s font-loading path as a whole; given how directly this
  first attempt succeeded, a short `fuzz-bug-repro` campaign (run separately, per that skill's own
  scope) targeting `GlyphRunBuilder::build`/`prepare_glyph_run` with mutated real fonts as seeds
  would likely turn up further `unwrap()`-on-`ReadError` panics beyond just `head()` (e.g. other
  skrifa table accessors called elsewhere in `glyph.rs`).
