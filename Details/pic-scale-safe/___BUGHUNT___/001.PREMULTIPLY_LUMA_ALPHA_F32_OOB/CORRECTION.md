# Correction

**Date noticed:** 2026-08-27 (after `git pull`, base commit `83c4801` -> current `b50e020`)

**What changed:** fixed upstream, independently of this bug-hunt session, in commit `aa0ae81c9625cddd3ff730771ff855c7850fa996`.

`src/alpha.rs`'s `premultiply_luma_alpha_f32`/`premultiplied_luma_alpha_f32` now write/read
`chunk[1]`/`src[1]` (the actual alpha element of the 2-element window) instead of the
out-of-bounds `chunk[2]`/`src[2]`. Commit message: "Fix off-by-one indexing in premultiply" - "As
spotted by Claude in issue #18, straightforward to correct." (i.e. this exact class of finding was
already reported upstream and fixed there, independently of this workspace).

**Does the original finding still hold?** No - re-ran this candidate's own `repro/` (adapted to
build against current `HEAD` instead of `base_commit`) and it no longer reproduces. The original
`RESULT.md` is left untouched below as a record of what was true at `base_commit`.
