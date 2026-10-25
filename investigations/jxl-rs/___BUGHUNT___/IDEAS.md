<!--
Copy into ___BUGHUNT___/IDEAS.md. This file holds UNCHECKED suspects only - the backlog of places
to look next. Once something is actually investigated (a candidate folder exists with a
Status/RESULT.md), it belongs in SUMMARY.md, not here - SUMMARY.md is the single source of truth
for what's been checked, this file is only what hasn't. When a backlog item gets investigated,
remove its row from here rather than moving it to an "already checked" list in this file.

Append to the backlog continuously while working a candidate, not just during a dedicated
discovery round - anything noticed in passing goes here immediately so it isn't lost.

The watermark line below persists across sessions - see SKILL.md's "GitHub issue/PR inspiration"
section. Update it to `#<MAX_NUMBER_SEEN> (<REPO>)` every time a raw sync runs (both values printed
by the sync command), even if nothing relevant was found; older unreviewed numbers below it are
intentionally never revisited. If the sync's printed `REPO` doesn't match the repo already recorded
here, treat the old number as stale (reset to 0 before applying `--since`) instead of silently
applying it to a different repo's own issue/PR numbering - issue/PR numbers are only unique within
one repo.
-->

# Ideas

Unchecked suspects only - see `SUMMARY.md` for what's already been investigated.

Last GitHub issue/PR checked for inspiration: **#1027** (repo: libjxl/jxl-rs)

## Backlog (not yet investigated)

Consumed top-to-bottom by default when a session isn't given a specific target - see `SKILL.md`.

| Added | Suspect | Why it looks suspicious | Source |
|---|---|---|---|
| 2026-10-12 | Memory use for a valid header declaring up to 2^28 pixels followed by a real frame header | Crafted headers stopped at the frame header (007), so allocation size vs. declared size was never observed; default `sample_limit` is None | 007 follow-up |
| 2026-10-12 | 32-bit run of large test images (i686 or wasm32 with enough memory) | usize overflow in pixel*channel arithmetic; i686 could not be linked here (no 32-bit libgcc), wasm replay only covered tiny corpus files | 007 |
| 2026-10-12 | aarch64 / wasm32 simd128 backends vs scalar | only x86_64 levels were diffed (006) | 006 |
| 2026-10-12 | Open PRs 1023/1024 (f16 storage / f16 math) once merged | new numeric paths, `U8 vs U16` +-1 level already differs on main | __issues_raw/pulls |
| 2026-10-12 | patches.rs `read_internal`: `(positions.last().x as i32 + delta_x) as usize` and `-delta_x as usize` | i32 overflow / wrap into a huge usize when delta_x is near i32::MAX or i32::MIN; the bound check `pos.x + ref_pos_xsize > xsize` could then wrap | 016 |
| 2026-10-12 | Crash input `015.../crash-b049f904...`: frame placed entirely outside the canvas | simple pipeline asserts; check what the low-memory pipeline outputs | 015 |
