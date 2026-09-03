# Correction

**Date noticed:** 2026-08-27 (after `git pull`, base commit `83c4801` -> current `b50e020`)

**What changed:** fixed upstream, independently of this bug-hunt session, in commit `b26f840ae7586ac54ae2dbdae68f98eb72850e03`.

`src/math/bessel_order_one.rs`'s `jinc_f64`/`jinc_f32` now special-case `x==0` to `0.5` (the
correct removable-singularity limit) instead of `0.0`, and `src/math/lanczos.rs`'s `lanczos_jinc`
no longer has its own separate `x==0` special case at all (falls through to the corrected `jinc`).
Upstream also added a `jinc_zero_limit_is_half` unit test, which passes on current `HEAD`.

**Does the original finding still hold?** No - re-ran this candidate's own `repro/` (adapted to
build against current `HEAD` instead of `base_commit`) and it no longer reproduces. The original
`RESULT.md` is left untouched below as a record of what was true at `base_commit`.
