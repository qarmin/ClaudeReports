# Correction

**Date noticed:** 2026-08-27 (after `git pull`, base commit `83c4801` -> current `b50e020`)

**What changed:** fixed upstream, independently of this bug-hunt session, in commit `b26f840ae7586ac54ae2dbdae68f98eb72850e03`.

`src/compute_weights.rs`'s general (non-area) branch of `generate_weights` now has an
`else if size > 0` fallback that zeroes the row and sets the tap nearest the window centre to `1`,
instead of leaving an all-`0.0` weights row when `weights_sum == 0.0`. Upstream also added an
`every_window_has_weights_summing_to_one` unit test (checked across `Bilinear`/`Lanczos3`/
`Gaussian`/`Bartlett`/`BartlettHann`/`Lanczos3Jinc`), which passes on current `HEAD`.

**Does the original finding still hold?** No - re-ran this candidate's own `repro/` (adapted to
build against current `HEAD` instead of `base_commit`) and it no longer reproduces. The original
`RESULT.md` is left untouched below as a record of what was true at `base_commit`.
