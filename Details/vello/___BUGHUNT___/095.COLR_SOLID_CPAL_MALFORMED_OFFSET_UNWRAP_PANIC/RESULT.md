# COLR_SOLID_CPAL_MALFORMED_OFFSET_UNWRAP_PANIC

Follow-up to candidates 081/086, which diffed classic vello's `vello/src/scene.rs::conv_brush`
(`skrifa::color::Brush` -> `peniko::Brush`) against `glifo/src/colr.rs`'s
`ColorPainter::fill`/`ColrPainter` for the `SweepGradient`/`RadialGradient` variants and found
real, one-sided divergences in each. This candidate does the same line-by-line diff for the
remaining two variants, `Brush::LinearGradient` and `Brush::Solid`, and finds one real, reachable
bug - not a cross-crate divergence this time, but a plain unconditional double-`unwrap()` inside
`conv_brush`'s own `color_index` helper (the `Solid` arm) that panics on a real, directly
constructible malformed-but-still-COLR-and-CPAL-bearing font.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `vello/src/scene.rs`'s `conv_brush` alongside `glifo/src/colr.rs`'s `ColorPainter::fill` for the `LinearGradient` arm, cross-checked against `skrifa` 0.44.0's own `traversal.rs` (`ResolvedPaint::LinearGradient` handling) | manual code reading | No divergence found. `glifo`'s `LinearGradient` arm has an `if stops.len() == 1 { fill_solid(..) }` special case that `conv_brush` lacks - but tracing `skrifa::color::traversal`'s own `LinearGradient`/`RadialGradient`/`SweepGradient` handling shows this is **dead code on all three gradient variants, not just `LinearGradient`**: whenever the original `ColorLine` has exactly one color stop, `make_sorted_resolved_stops` produces a single resolved stop, and the `(first_stop, last_stop)` match computes `color_stop_range = 0.0`; for `extend != Pad` this returns *before* ever calling `painter.fill(Brush::XxxGradient{..})` at all (paints nothing), and for `extend == Pad` it pushes a **second**, duplicate stop (`offset += 1.0`) before calling `fill`, so `color_stops.len()` is already >= 2 by the time `ColorPainter::fill` ever sees it. `stops.len() == 1` inside any of `glifo`'s three gradient arms is therefore unreachable through the real skrifa paint-graph traversal - consistent with, and generalizing, what candidate 086 already found for `RadialGradient`'s `r1 <= 0.0 \|\| stops.len() == 1` condition (086 only chased the `r1 <= 0.0` half; this confirms the `stops.len() == 1` half is dead in all three gradient arms, not just here). No p0==p1 guard gap either: skrifa's own traversal already intercepts `p1 == p0 \|\| p2 == p0 \|\| cross_product(...) == 0.0` before ever calling `ColorPainter::fill`/`conv_brush`, substituting a first-stop-only `Brush::Solid` itself - matching what 086 already found for this exact case. Extend/stop-offset normalization (padding to `[0.0, 1.0]`, in `glifo::convert_stops`) was also compared against `vello_encoding`'s own ramp-building code and found to handle un-normalized/out-of-range stops equivalently on both sides |
| Read `conv_brush`'s `Solid` arm (`color_index`) alongside `glifo/src/colr.rs`'s `palette_index_to_color`, both called with a raw `palette_index: u16` sourced directly from `PaintSolid.paletteIndex`/`PaintVarSolid.paletteIndex` font bytes via `skrifa::color::traversal`'s `Brush::Solid` construction (no upstream validation, confirmed by reading `traversal.rs` - same finding candidate 042 already made for `glifo`'s side) | manual code reading | Found a real divergence, in the *opposite* direction from 086 (this time `conv_brush` is the more fragile side): `color_index` does `cpal.color_records_array().unwrap().unwrap()` - an **unconditional** double `unwrap()` on `Option<Result<&[ColorRecord], ReadError>>`, independent of `debug_assertions`. `glifo`'s equivalent (`palette_index_to_color`) uses `.cpal().ok()?.color_records_array()?.ok()?[idx]` - propagating `None`/`Err` gracefully via `?` (though its raw `[idx]` slice index at the end is itself a *different*, already-tracked bug, candidate 042). `font.cpal().is_ok()` (checked once, in `try_draw_colr`, before ever calling `conv_brush`) only proves the CPAL table's fixed 12-byte header parses - it says nothing about whether `colorRecordsArrayOffset` itself resolves to valid, in-bounds data, so `color_records_array()` returning `None` (null offset) or `Some(Err(ReadError::OutOfBounds))` (an offset pointing past the table/file) is real, reachable input, not merely theoretical |
| Built a real end-to-end repro (see below): a byte-patched copy of the project's own COLRv1 test font (`examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf`) with only the CPAL table's 4-byte `colorRecordsArrayOffset` field changed to an out-of-bounds value, driven through `vello::Scene::draw_glyphs(..).draw(..)` (the real public API, no GPU device needed - the panic happens during CPU-side scene encoding, before anything is submitted to a renderer) | debug persisted in release build (`cargo run --release`), 3 repeat runs before the fix, 3 after | Confirmed - panics deterministically 3/3 runs against pristine `base_commit`; no longer panics 3/3 runs after the one-line fix |

No `miri`/ASAN/fuzzing needed - this is a plain, unconditional `Result::unwrap()` panic on
adversarial-but-realistic font bytes (not memory-unsafety, no `unsafe` involved), so a real
boundary-value font construction plus a real end-to-end run was the right tool and gave a clean,
deterministic signal on the first attempt.

## Status

**Confirmed** - reproduces deterministically 3/3 runs (release build, no randomness/seed
dependency anywhere in this path - a plain safe-Rust `Option`/`Result` unwrap on data read
straight from the font file).

**Fully reproducible**: **Yes**. Per `CONFIRMATION.md` section 11, this is the single-participant
case exempted from needing a second, independently-driven participant: the "adversarial input" is
a single external artifact (a malformed font file) handed to the library's real public entry point
(`vello::Scene::draw_glyphs(&font).draw(..)`), the same way a real embedding application would load
an untrusted/attacker-supplied font - there is no "peer" whose behavior is being faked. `repro/`
drives this real entry point directly, with no GPU/renderer needed (the panic happens purely in
scene encoding).

## Class

crash

## Root cause

`vello/src/scene.rs`'s `conv_brush` (`Brush::Solid` arm) calls `color_index(cpal, palette_index)`,
which - for any `palette_index != 0xFFFF` (i.e. not the "use the foreground/context color"
sentinel) - does:

```rust
let actual_colors = cpal.color_records_array().unwrap().unwrap();
```

`Cpal::color_records_array()` (a generated `read-fonts` accessor) returns
`Option<Result<&[ColorRecord], ReadError>>`: `None` if the CPAL table's `colorRecordsArrayOffset`
field is a null offset, `Some(Err(ReadError::OutOfBounds))` (or similar) if that offset points
outside the table/file's actual byte extent, and `Some(Ok(slice))` only in the well-formed case.
The caller only ever checks `font.cpal().is_ok()` (in `DrawGlyphs::try_draw_colr`, before
constructing `DrawColorGlyphs`) - and `Cpal::read_with_args` only validates that the table's fixed
12-byte header (`MIN_SIZE`) is present, never that `colorRecordsArrayOffset` itself resolves to
real data. A font can therefore have a CPAL table that reads as "present and structurally valid"
(`font.cpal().is_ok()` is `true`) while its `colorRecordsArrayOffset` is null or dangling - fully
legal for a truncated/corrupted-but-recognizable CPAL table, exactly the class of input a
malicious or fuzzer-generated font would produce.

`palette_index` itself is forwarded completely unvalidated from raw `PaintSolid.paletteIndex` /
`PaintVarSolid.paletteIndex` font bytes all the way through `skrifa::color::traversal` to
`ColorPainter::fill(Brush::Solid { palette_index, .. })` (confirmed by reading
`skrifa-0.44.0/src/color/traversal.rs` - no bounds/validity check anywhere in that path, the same
finding candidate 042 already made independently for `glifo`'s identical situation). So a COLRv1
glyph whose top-level (or any) paint is a `PaintSolid` referencing an ordinary, otherwise-valid,
in-range palette index (e.g. the test font's own glyph 84, which legitimately uses index 4) still
reaches this `unwrap().unwrap()` and panics the instant the *CPAL table itself* (not the index) is
malformed in this specific way - independent of whether the palette index used happens to be valid
or not.

This is the mirror image of `glifo/src/colr.rs`'s already-tracked bug (candidate 042: an unchecked
`[palette_index as usize]` slice index once `color_records_array()` *does* resolve successfully)
in the same class ("classic vello and `glifo` are two independent, incompletely-defensive Rust
re-implementations of the same skrifa COLR conversion, and each one is missing a *different* half
of the input validation the other one has") - `glifo`'s `palette_index_to_color` already handles
this exact `None`/`Err` case gracefully via `?`, while `conv_brush`'s `color_index` does not.

## Introduced by

886604b2 "Render COLR Glyphs (Emoji) (#615)" (2024-06-21) - the commit that introduced
`conv_brush`/`color_index` (then in the same form, still using
`cpal.color_records_array().unwrap().unwrap()`) as part of adding COLR/CPAL glyph rendering
support to classic vello. Confirmed via `git log -S "color_records_array().unwrap().unwrap()" --
vello/src/scene.rs`, which finds exactly this one commit.

## The fix

`vello/src/scene.rs`'s `color_index`: replace the double `unwrap()` with `?`-propagation, mirroring
`glifo`'s already-correct handling of the same `Option<Result<..>>`:

```rust
// TODO: Error reporting in the `None`/`Err` cases
let actual_colors = cpal.color_records_array()?.ok()?;
```

`color_index` already returns `Option<AlphaColor<Srgb>>` and its caller (`conv_brush`'s `Solid`
arm) already has a well-defined fallback for `None` - `unwrap_or(foreground_brush.to_owned()
.multiply_alpha(alpha))`, the same fallback already used for the explicit `palette_index ==
0xFFFF` sentinel and for an out-of-range-but-parseable index. This makes a CPAL table with a
null/dangling `colorRecordsArrayOffset` behave the same way as those other two already-handled
"can't resolve a real color" cases (fall back to the foreground/context color) instead of
panicking - consistent with the rest of the function, not a new special case bolted on.

Verified: `repro/` no longer panics after the patch (3/3 runs; prints "did NOT panic: encoded
glyph 84 ... successfully" instead of the original `unwrap()` panic). See `output.txt` for the
real captured before/after output including the full panic backtrace.

- Project test suite: `cargo test --workspace --lib --release` - **441 passed / 0 failed / 1
  ignored** before the fix, byte-for-byte identical (**441 passed / 0 failed / 1 ignored**) after.
  No unit test anywhere in the workspace exercises a CPAL table with an unresolvable
  `colorRecordsArrayOffset`, which is why this went undetected; the fix introduces no regression
  in the cases they do cover.

## What's not done / open questions

- `LinearGradient` and the `stops.len() == 1` dead-code question were checked for all three
  gradient variants (see "What was tried" above) and found to have no real, reachable divergence -
  this candidate's only real finding is the `Solid`-arm CPAL panic described above.
- Symmetric to 042: this candidate's fix only addresses `conv_brush`'s handling of a malformed
  `colorRecordsArrayOffset`; `glifo`'s own, separately-tracked unchecked-index bug (a valid
  `colorRecordsArrayOffset` but an out-of-range `palette_index`) is 042's concern, not this one's -
  the two bugs are independent and were found/fixed independently, in different crates, from
  different malformed fields of the same CPAL table.
- Not investigated this session: whether the same "`.cpal().is_ok()` only proves the header
  parses" gap has any other consumer in classic vello (`conv_brush`'s `Solid` arm, reached via
  `Brush::Solid`, was the only call site of `color_index`/`color_records_array` found by grep in
  `vello/src/scene.rs`).
- Only classic vello's `conv_brush` (used by both classic vello's GPU and CPU-shader-fallback
  rendering paths, since both consume the same `Scene` encoding) was exercised; `glifo`'s
  equivalent panic path for the *same* malformed-CPAL-table input (its `.cpal().ok()?` call inside
  `palette_index_to_color` already handles it gracefully, confirmed by reading, not separately
  re-verified by running `vello_cpu` against this exact patched font this session).
