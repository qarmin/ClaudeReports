# Correction

**Date noticed:** 2026-08-27 (after `git pull`, base commit `83c4801` -> current `b50e020`)

**What changed:** fixed upstream, independently of this bug-hunt session, in commit `b26f840ae7586ac54ae2dbdae68f98eb72850e03`.

`src/math/gaussian.rs`'s `gaussian()` now computes `den = 2*sigma*sigma` (no longer squared a
second time) and uses `exp(-(x*x)/den)` (no longer `exp(-x/den)`), matching the textbook normal
density. Upstream also added a `gaussian_matches_textbook_formula` unit test, which passes on
current `HEAD`.

**Does the original finding still hold?** No - re-ran this candidate's own `repro/` (adapted to
build against current `HEAD` instead of `base_commit`) and it no longer reproduces. The original
`RESULT.md` is left untouched below as a record of what was true at `base_commit`.
