# Correction

**Date noticed:** 2026-08-27 (after `git pull`, base commit `83c4801` -> current `b50e020`)

**What changed:** fixed upstream, independently of this bug-hunt session, in commit `b26f840ae7586ac54ae2dbdae68f98eb72850e03`.

`src/math/bartlett.rs`'s `bartlett()` was rewritten to fold `x` to `\|x\|`, return `0` beyond the
`2.0` support radius, and use `1 - 0.5*x` on `[0,2]` - now a correct triangular window peaking at
`1` at the centre. Upstream also added a `bartlett_is_triangle_over_support` unit test, which
passes on current `HEAD`.

**Does the original finding still hold?** No - re-ran this candidate's own `repro/` (adapted to
build against current `HEAD` instead of `base_commit`) and it no longer reproduces. The original
`RESULT.md` is left untouched below as a record of what was true at `base_commit`.
