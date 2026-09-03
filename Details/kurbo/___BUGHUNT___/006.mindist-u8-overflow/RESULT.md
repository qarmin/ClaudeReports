# mindist.rs u8 overflow in `2 * n` contradicts `min_dist_param`'s own documented degree-255 support

`kurbo/src/mindist.rs`'s `min_dist_param` (`kurbo/src/mindist.rs:29-46`, doc comment at lines
16-28) claims support for "arbitrary Bezier curves up to and including degree 255", enforced by
`u8::try_from(bez1.len() - 1).expect("At most 256 control points")` (line 38-39). But `n`/`m` are
`u8`, and `2 * n` / `2 * m` (the loop bounds used throughout `S`, `C_rk`, `A_r`, `basis_function` -
lines 65-66, 86, 121-122, 176-177, 181-182, 198, 203, 206, 226, 233, 240) is itself computed in
`u8` arithmetic, which overflows `u8`'s 0-255 range for any `n > 127` - well before the documented
degree-255 boundary. This was flagged by static/manual code reading against
`HEURISTICS.md`'s "casts between integer widths, arithmetic on untrusted sizes" pattern, cross-
referenced against the doc comment's own explicit degree-255 claim (added by the commit that
introduced the `u8` storage in the first place - see "Introduced by" below).

`min_dist_param` is `pub(crate)`; its only real caller, `PathSeg::min_dist`
(`kurbo/src/bezpath.rs:1300-1319`), always passes `n, m <= 3` (`PathSeg` only has
Line/Quad/Cubic variants, via `as_vec2_vec`), so this discrepancy is not reachable through
kurbo's shipped public API today - see "Fully reproducible" below for what that means for how
this was reproduced.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug-assertions build (matches `HEURISTICS.md`'s "arithmetic on untrusted sizes/indices, casts between integer widths" -> no sanitizer needed) | `cargo test`/`cargo run`, default dev profile (debug-assertions=true, overflow-checks=true, this repo's Cargo default), a hand-built 129-point (`n=128`) Bezier "curve" passed directly to `min_dist_param` via a temporary in-crate `#[test]` first, then via `repro/`'s vendored-source approach | **Reproduced** on the first attempt - deterministic panic ("attempt to multiply with overflow") inside `S()` at `mindist.rs:176:18`, 3/3 runs. No further tools needed (soft cap of 3 not exhausted; stopped after a clean, deterministic positive on approach 1). |
| Same input, `--release` build | `cargo run --release` | Confirms the release-mode counterpart: **no panic**, `2 * n` silently wraps (u8 arithmetic, no overflow checks in release), `min_dist_param` returns a meaningless result `(1.0, 0.5, 0.5)` instead of erroring - a silent-wrong-output logic bug, not just a debug-build crash. |

## Status

**Confirmed**

Reproduces 3/3 runs, default dev-profile (debug-assertions on) build, deterministic (pure
computation, no randomness/threading/layout-dependence) - see `output.txt`. Release build does not
panic; instead it silently returns a bogus, non-error result (also captured in `output.txt`) - both
build-profile outcomes were checked (per `CONFIRMATION.md` section 2).

**Fully reproducible**: **No** - justified. `min_dist_param` is `pub(crate)`: it cannot be invoked
from any external crate's code at all, by Rust's own visibility rules, regardless of any wrapper
kurbo itself ships (`PathSeg::min_dist` is the only caller, and it structurally can never pass
`n/m > 3`, since `PathSeg` only has Line/Quad/Cubic variants). There is no real "second
participant" being faked here (this isn't a peer/caller-input-shape finding in the sense
`CONFIRMATION.md` section 11 is about) - the obstacle is that the *only* entry point for this
finding is not part of kurbo's public API surface at all. `repro/` calls the exact, real,
unmodified `min_dist_param` (compiled verbatim via `#[path]` from whatever is currently checked
out in `./clone`, so the same source runs for both "before" and "after" against `patch.diff`) with
a genuine, valid 129-control-point (degree-128) Bezier curve - exactly the kind of input the
function's own doc comment says it accepts - rather than reimplementing or approximating the
function's logic. What it cannot do is reach that function through kurbo's actual public API,
because no such path exists; that inaccessibility is itself the fact this candidate documents.

## Class

logic / crash (build-profile-dependent: panics under debug-assertions, silently returns a wrong
result in release)

## Root cause

`kurbo/src/mindist.rs:38-39` (introduced by commit 9ab9ef7, see below) stores `n = bez1.len() - 1`
and `m = bez2.len() - 1` as `u8`, validated only against `u8::try_from`'s own success range
(0-255), matching the doc comment's claim of "up to and including degree 255". But every loop bound
and intermediate computation built on top of `n`/`m` throughout this module uses `2 * n` / `2 * m`
computed in that same `u8` type (e.g. `for r in 0..(2 * n)` at line 65, `for r in 0..=2 * n` at
line 176, `basis_function(2 * n, r, u)` at line 179, `choose(2 * n, r)` at line 198) - and `u8`
arithmetic can only represent values up to 255, so `2 * n` overflows as soon as `n > 127`
(`2 * 128 = 256`). The doc comment's "degree 255" claim and the `u8`-typed doubled loop bounds are
mutually inconsistent: the implementation can only actually support `n, m <= 127` without
overflowing, half of what the doc comment (and the `expect`/`try_from` check enforcing it) claims.

## Introduced by

9ab9ef7 "Fix wrapping arithmetic in `min_dist_param` by limiting Bezier degree (#542)" (2026-01-22)

This commit narrowed `n`/`m` (and `r`/`k`/loop indices throughout the module) from `usize` to `u8`
"to easily prevent wrapping" (per its own commit message) and added the doc comment claiming
degree-255 support plus the `u8::try_from(...).expect("At most 256 control points")` check - but
did not account for `2 * n`/`2 * m` themselves needing to fit in that same `u8` type, so it
introduced a narrower, u8-specific overflow (`n > 127`) while writing a doc comment/guard claiming
a wider bound (`n > 255`) that the new `u8` types could never actually deliver on. Confirmed via
`git log -S` on the `u8::try_from` guard text plus reading the commit's own diff
(`git show 9ab9ef7 -- kurbo/src/mindist.rs`).

## The fix

Tightened `min_dist_param`'s documented and enforced degree limit from 255 down to 127 - the actual
limit its `u8`-typed `2 * n`/`2 * m` arithmetic can represent without overflowing in any build
profile - and updated the doc comment, the `# Panics` section, and the three
`#[expect(clippy::cast_possible_truncation, ...)]` comments in `S`/`C_rk`/`A_r` to match. Concretely:
a new `assert!(bez1.len() <= 128 && bez2.len() <= 128, "At most 128 control points")` runs *before*
any `2 * n`/`2 * m` arithmetic is attempted, so degree-128-or-greater input now fails a plain,
documented assertion (present in every build profile, not just when overflow-checks happen to be
enabled) instead of hitting undefined/build-profile-dependent behavior (a debug-only panic with a
generic "attempt to multiply with overflow" message, or a silently wrong result in release).

Checked against more than the one triggering input (`CONFIRMATION.md` section 7): re-ran with
`n = 127` (bez1.len() == 128, exactly at the new limit) - this no longer hits the `2 * n`
overflow (`2 * 127 = 254` fits `u8` cleanly) and no longer panics from the specific bug this
candidate targets; it does still panic, but from a different, pre-existing, unrelated bug (see
"What's not done" below) - which itself demonstrates the *targeted* overflow is gone at that
input, since the failure moved to a different location for a different reason. Re-ran `n = 128`
(bez1.len() == 129, the original repro) against the patched tree in both debug and release: both
now fail identically and deterministically at the new `assert!` (`mindist.rs:39`, "At most 128
control points"), not at the old overflow site - see `output.txt`.

- Project test suite (`cargo test -p kurbo`, run directly in a worktree of the main project, not
  via `repro/`'s own `./clone`): **before**: 187 lib tests passed, 0 failed; 55 doctests passed, 0
  failed. **after** (patch applied): 187 lib tests passed, 0 failed; 55 doctests passed, 0 failed.
  No new failures introduced; the existing suite never exercised `n/m > 3` in the first place
  (matches "Fully reproducible" above), so it could not have caught this either way.

## What's not done / open questions

- **A genuinely deeper, separate bug was noticed while verifying the fix at the new boundary
  (`n = 127`)**: `choose()` (`kurbo/src/mindist.rs`, the binomial-coefficient helper) accumulates
  into a plain `u32`, and central binomial coefficients like `choose(254, 127)` (≈6.6e75) vastly
  exceed `u32::MAX` (≈4.3e9) - this overflows (panics in debug, wraps in release) at a *much*
  lower degree than 127 (`choose(2n, n)` first exceeds `u32::MAX` somewhere around `n ≈ 17-18`,
  not checked exactly). This means even the newly-documented "degree 127" limit this candidate's
  fix establishes is not actually safe/correct end-to-end for most of that range - only that the
  *specific* `2 * n` u8-overflow this candidate targets is fixed. Not investigated or fixed this
  session (separate root cause, separate location, would need its own candidate - one bug per
  candidate per `SKILL.md`). Recorded as a new suspect below.
- Whether a real widening (`u16` for `n`/`m`/`r`/`k` throughout the module, plus a correspondingly
  wider `choose()` accumulator, likely `u64` or `f64`) to actually deliver on a real degree-255
  (or any degree meaningfully above ~17) guarantee is worth doing at all is a design question for
  maintainers - `min_dist_param` is `pub(crate)` and, as this candidate documents, its only real
  caller never needs more than `n, m = 3` today, so the doc comment's degree-255 claim may never
  have been meant to be load-bearing outside of "won't panic on already-small inputs by
  construction of `u8`" in the first place.
- Not checked whether any other module in this crate stores a similar small-int size/degree and
  doubles/multiplies it without a matching bound - this was a single, targeted candidate, not a
  codebase-wide sweep for the same pattern.
