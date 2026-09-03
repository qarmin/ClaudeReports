# Correction

**Date noticed:** 2026-08-27 (after `git pull`, base commit `83c4801` -> current `b50e020`)

**What changed:** fixed upstream, independently of this bug-hunt session, in commit `b26f840ae7586ac54ae2dbdae68f98eb72850e03`.

`src/resize_fixed_point.rs` and `src/resize_floating_point.rs` now compute the input-length check
by reusing the already-overflow-checked `src_stride`/size computation (`src_slice_len`) instead of
recomputing `source_size.width * CHANNELS * source_size.height` with plain multiplication - exactly
the fix this candidate's own `RESULT.md` proposed.

**Does the original finding still hold?** No - re-ran this candidate's own `repro/` (adapted to
build against current `HEAD` instead of `base_commit`) and it no longer reproduces. The original
`RESULT.md` is left untouched below as a record of what was true at `base_commit`.
