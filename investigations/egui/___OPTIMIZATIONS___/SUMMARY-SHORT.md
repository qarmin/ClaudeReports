
# Optimization wins (Strong win / Small win only)

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results and
regressions) and the full methodology behind each number.

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 001 | Small win | Text layout runs the full Unicode grapheme iterator even for plain ASCII | +15/-2 | -9% uncached text layout |
| 002 | Small win | Per-row text mesh bounds computed with NaN-aware f32::min/max per vertex | +9/-3 | -2..3% uncached text layout |
| 003 | Strong win | Text vertex transform loop re-tests per-row invariants for every vertex | +23/-1 | -46% tessellate_text, -18..22% demo tessellation |
| 006 | Small win | Color32 hashes as length-prefixed byte slice instead of one u32 | +9/-1 | -2..3% cached text layout lookup (microbench) |
| 007 | Small win | Per-character fallback-face resolution does two hash-map lookups even inside a run | +23/-7 | -2.4..3% uncached text layout |
| 009 | Small win | Per-paragraph glyph vec regrown by doubling during text layout | +3/-0 | -26.6% allocated bytes, -1.3% time per uncached layout |
| 012 | Strong win | Label layout cache key hashed with ~35 tiny hasher writes per label | +39/-17 | -11% cached layout lookup, -3.5% demo UI build |
| 014 | Small win | Per-glyph atlas cache key hashed from six fields | +3/-4 | -1.4..2% uncached text layout |
| 015 | Strong win | Shape plan recompiled for every text run | +58/-5 | -11.7% relayout of short changing labels |
| 016 | Small win | Path normals call libm hypotf per point via Vec2::length | +3/-1 | -2..3% tessellation / widget scenes |
| 017 | Strong win | Meshes regrown by doubling each frame; reserve last frame size instead | +55/-8 | -7% demo tessellation, -15% painter-heavy scene, -21% allocated bytes |
| 018 | Small win | Frame shape list regrown by doubling every pass | +7/-1 | -10% allocated bytes, -23% peak live per UI pass |
| 020 | Small win | Widget registration takes the context lock 5-7 times | +200/-165 | -2..4% widget-heavy UIs, 21 -> 15 locks per button |
| 021 | Strong win | Rounded-rect outline normals recomputed for every rect | +45/-1 | -38% filled rounded rect tessellation, -16.5% button column |
