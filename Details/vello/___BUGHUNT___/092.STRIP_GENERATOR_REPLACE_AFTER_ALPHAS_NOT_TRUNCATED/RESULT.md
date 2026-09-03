# STRIP_GENERATOR_REPLACE_AFTER_ALPHAS_NOT_TRUNCATED

`sparse_strips/vello_common/src/strip_generator.rs`'s `render_with_clip` handles
`StripStorage`'s `GenerationMode::ReplaceAfter(n)` by truncating `strip_storage.strips` to
`[0..n]`, but never truncates `strip_storage.alphas` to match, even though each `Strip` records
an `alpha_idx` offset into `alphas`. This looked suspicious as a heuristic "growth-without-bound
buffer paired with a truncation that only clips one of two related collections" pattern, and
because `ReplaceAfter` is unused by any in-tree caller (only `Replace`/`Append` are used), so it's
a latent, unexercised public-API path.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction via a standalone repro against the real public API (`StripGenerator::generate_filled_rect_fast`/`generate_filled_path`, `StripStorage::set_generation_mode(GenerationMode::ReplaceAfter(n))`), release build | 500 simulated incremental-redraw frames (a static 2000x2000 background + a small repeatedly-redrawn dirty rect, `n = static_strip_count`) | `alphas` grew from 16000 to 336000 bytes (~640 bytes/frame) while `strips` stayed at 1022 (bounded); repeated 3 additional times, bit-for-bit identical result each run |
| Variant input in the same bug class: `n = 0` boundary, `generate_filled_path` entry point instead of `generate_filled_rect_fast`, 300 frames | release build | `alphas` grew to 124800 bytes (from an expected few-hundred-byte single-frame size) - confirms the bug isn't narrow to the exact original repro shape |
| Reading `strip.rs`'s `render_impl`/`rect.rs`'s `render` to confirm the mechanism | static code reading | Confirmed each `Strip::new(..., alpha_buf.len() as u32, ...)` records the alpha write cursor at strip-creation time, and alpha bytes are appended strictly in strip-index order, so `strips[n].alpha_idx()` is exactly the correct alpha-truncation boundary for `ReplaceAfter(n)` |

No sanitizer/miri/fuzz needed - this is a pure logic bug (unbounded memory growth, not a
crash/UB), fully explainable and reproducible via direct construction of the documented API usage
pattern (`ReplaceAfter`'s own doc comment: "preserving strips in `[0..n]`" implies the rest should
also be discarded, which it isn't for `alphas`).

## Status

**Confirmed**. Reproduces 4/4 runs (1 initial + 3 repeat runs), bit-for-bit identical output each
time, release build, no sanitizer/fixed seed involved (pure deterministic logic bug). Also
reproduces with a second, differently-shaped input in the same bug class (see table above).

**Fully reproducible**: **Yes**. This is a single-participant finding: a caller (the crate's own
public API user, e.g. a retained-mode scene renderer redrawing one dirty rect per frame) calls
`StripGenerator`'s real public entry points (`generate_filled_rect_fast`/`generate_filled_path`)
directly, in a loop, exactly the pattern `GenerationMode::ReplaceAfter`'s own doc comment describes
as its purpose. There is no second participant/peer/wire input being faked here - CONFIRMATION.md
section 11 doesn't apply (no "how does X react to what Y sends" boundary; it's "does this API's own
state grow unboundedly across a self-contained sequence of its own public method calls").

## Class

logic (unbounded memory growth / resource leak within a long-lived buffer)

## Root cause

`render_with_clip` (`sparse_strips/vello_common/src/strip_generator.rs`, function starting
~line 256) matches on `strip_storage.generation_mode`:

```rust
match strip_storage.generation_mode {
    GenerationMode::Replace => strip_storage.strips.clear(),
    GenerationMode::Append => {}
    GenerationMode::ReplaceAfter(n) => strip_storage.strips.truncate(n),
}
```

For `ReplaceAfter(n)`, only `strips` is truncated to `[0..n]`; `alphas` (the coverage-byte buffer
every `Strip` indexes into via its own `alpha_idx`/`alpha_idx()` field) is left completely
untouched. Every subsequent call to `generate_filled_path`/`generate_filled_rect_fast` in
`ReplaceAfter` mode appends its new strips' alpha data starting from wherever `alphas.len()`
already was - it never reclaims the bytes that belonged to the just-discarded `strips[n..]`. Since
the whole point of `ReplaceAfter` (per its own doc comment: "preserving strips in `[0..n]`") is a
repeated, per-frame partial redraw, `alphas.len()`/`.capacity()` grows by roughly one discarded
frame's worth of coverage bytes every single call, unboundedly, even though the strip metadata
that referenced the discarded alpha region is gone forever and never read again.

## Introduced by

8ffd7d5a "vello_hybrid: Better interleaving of raw strips and layers (#1479)" (2026-03-02) - this
commit introduced `GenerationMode::ReplaceAfter` itself (previously only `Replace`/`Append`
existed) and its handling in `render_with_clip`, already missing the corresponding `alphas`
truncation from the very first version of the match arm (confirmed via `git log -S ReplaceAfter`
and reading the commit's diff to `strip_generator.rs` directly).

## The fix

Changed the `ReplaceAfter(n)` arm to also truncate `alphas`, using the retained strip at index `n`
(if it still exists after the check) to find the exact byte offset to truncate to - `Strip::new`
always records the current `alphas.len()` as its own `alpha_idx` at creation time, and alpha bytes
are appended strictly in increasing strip-index order (confirmed by reading `strip.rs`'s
`render_impl` and `rect.rs`'s `render`), so `strips[n].alpha_idx()` is exactly the number of alpha
bytes still referenced by the retained `strips[0..n]`:

```rust
GenerationMode::ReplaceAfter(n) => {
    if let Some(strip) = strip_storage.strips.get(n) {
        let alpha_idx = strip.alpha_idx() as usize;
        strip_storage.strips.truncate(n);
        strip_storage.alphas.truncate(alpha_idx);
    }
}
```

If `n >= strips.len()` (nothing to discard - the same case where `Vec::truncate(n)` was already a
no-op before this fix), `strips.get(n)` is `None` and neither buffer is touched, preserving the
existing no-op behavior for that case.

Verified against two inputs in the same bug class (per CONFIRMATION.md section 7):
- Original repro (`repro/src/main.rs`, 500 frames, `n = static_strip_count`,
  `generate_filled_rect_fast`): `alphas` before fix = 336000 bytes after 500 frames; after fix =
  16640 bytes (bounded, ~1.3 bytes/frame residual growth from one extra sentinel strip's tiny
  alpha footprint per frame, not unbounded growth).
- Variant repro (`repro/src/bin/variant.rs`, 300 frames, `n = 0`, `generate_filled_path`): `alphas`
  before fix = 124800 bytes after 300 frames; after fix = 416 bytes (bounded).

- Project test suite (`cargo test -p vello_common -p vello_cpu -p vello_hybrid --release --lib
  --tests`, in an isolated git worktree at the same base commit): **before fix**: 160+81+122 = 363
  passed, 0 failed, 1 ignored (`cargo test ... --release` including doctests also shows one
  pre-existing, unrelated `vello_cpu` doctest compile failure - `glifo::atlas::GlyphCacheStats` not
  found under a release-build cfg gate - present identically before and after this patch, unrelated
  to `strip_generator.rs`). **After fix**: identical 363 passed, 0 failed, 1 ignored - no
  regressions introduced by this change.

## What's not done / open questions

- `GenerationMode::Replace` has a closely related but distinct issue: its match arm only calls
  `strip_storage.strips.clear()`, never `alphas.clear()`, in `render_with_clip` itself - so a caller
  that uses `Replace` mode across multiple frames *without* separately calling
  `StripStorage::clear()` (which does clear both) would see the same unbounded `alphas` growth this
  candidate fixes for `ReplaceAfter`. This wasn't fixed here (out of scope for this candidate, and
  every in-tree `Replace`-mode caller currently relies on an external `reset()`/`clear()` call each
  frame, so it isn't currently reachable in practice) - flagged as a new suspect below rather than
  bundled into this patch, per "one bug per candidate folder."
- `ReplaceAfter` remains unused by any in-tree caller as of this commit (confirmed via
  `grep -rn "ReplaceAfter"`), so this fix has no behavioral effect on any current production code
  path - it only affects any future caller that adopts `ReplaceAfter` for retained-mode partial
  redraws, which is the scenario its own doc comment describes as the intended use.
