# Ideas

Untested ideas only - see `SUMMARY.md` for what's already been tried.

## Backlog (not yet tried)

Consumed top-to-bottom by default when a session isn't given a specific target.

| Added | Idea | Why it looks promising | Source |
|---|---|---|---|
| 2025-07-21 (ported) | `vello_common/src/pixmap.rs`'s `multiply_alpha` | Same scalar-division shape as `take_unpremultiplied` - worth the same LUT treatment | candidate 001 investigation |
| 2025-07-21 (ported) | `from_png`'s premultiply loop | Same scalar-division shape as `take_unpremultiplied` - worth the same LUT treatment | candidate 001 investigation |
| 2025-07-21 (ported) | Scalar LUT-gather in `vello_cpu/src/fine/lowp/gradient.rs`'s gradient painter | Flagged but judged low-confidence - may already be the best available option without hardware gather support in `fearless_simd` | candidate 001 investigation |
| 2026-08-24 | Expose PNG compression level as a caller-configurable knob on `into_png` (rather than a fixed hardcoded default) | Candidates 002/003 both found real, one-line wins by changing the hardcoded level (Balanced->Fast->Fastest), each trading more file size for speed - a knob would let callers pick the tradeoff instead of this crate deciding it once for everyone. Not attempted; would be a small API-surface change (new parameter or a `with_compression` builder), not just a source-internal swap like 002/003. | candidate 003 (open question, not itself investigated) |
