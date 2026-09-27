# Correction

**Date noticed:** 2026-08-27 (after `git pull`, base commit `83c4801` -> current `b50e020`)

**What changed:** fixed upstream, independently of this bug-hunt session, in commit `b26f840ae7586ac54ae2dbdae68f98eb72850e03`.

`src/math/bartlett.rs`'s `bartlett_hann()` now divides by the kernel's actual radius (`fac =
x/(l+l)` with `l=2.0`, i.e. divide by `4.0`) instead of `l-1=1.0`, and uses the standard `0.48`
constant instead of `0.4832`. Upstream also added a `bartlett_hann_peaks_at_center_and_decays_to_zero`
unit test, which passes on current `HEAD`.

**Does the original finding still hold?** No - re-ran this candidate's own `repro/` (adapted to
build against current `HEAD` instead of `base_commit`) and it no longer reproduces. The original
`RESULT.md` is left untouched below as a record of what was true at `base_commit`.
