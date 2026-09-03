# MINDIST_C_RK_U8_OVERFLOW_PLUS_ONE

`kurbo/src/mindist.rs`'s `C_rk` (lines 187-209) computes `upsilon = r.min(n)` / `varsigma = k.min(m)`
as `u8`, then loops `for i in theta..upsilon + 1` (line 196) / `for j in sigma..varsigma + 1` (line
204). The candidate hypothesis was that `upsilon + 1` (or `varsigma + 1`) overflows `u8` when
`upsilon`/`varsigma == 255`, exactly at the crate's documented degree-255 boundary (`bez1.len() ==
256`, i.e. `n == 255`) - distinct from already-confirmed candidate 006 (`2 * n`/`2 * m` u8
multiplication overflow, which triggers earlier, at `n > 127`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end call through the actual entry point (`min_dist_param`, `pub(crate)`), degree-255 curves (256 control points each) on both curves, debug-assertions build | `cargo test -p kurbo --lib`, dev profile (debug-assertions=true, overflow-checks=true) | Panics, but at `mindist.rs:176:18` (`for r in 0..=2 * n` inside `S()`) - the already-known candidate 006 `2 * n` overflow. Never reaches `C_rk`'s `upsilon + 1` line for this input. |
| Same real end-to-end call, `--release` build (overflow-checks=false) | `cargo test -p kurbo --release --lib`, ~182s (heavy but bounded computation, no hang) | No panic anywhere. Returns `(NaN, 0.5, 0.5)` - a wrong result from `choose()`'s independent u32 overflow (candidate 019), not from an addition overflow at `upsilon + 1`/`varsigma + 1`. |
| Manual boundary-value construction: exhaustive closed-form sweep of every possible `n` in `0..=255`, computing the maximum `r` any real call site (`S()`'s inclusive `0..=2*n` range, `min_dist_param`'s exclusive `0..2*n` range) could ever pass into `C_rk`/`A_r` given `u8` wrapping-mul semantics for `2*n` | Pure Python arithmetic model of the exact `u8` semantics used in the actual code (no Rust build needed - this is a closed-form property of the loop bounds, not something that needs running) | Maximum `upsilon` ever reachable through any real call path is **254** (at `n=255`, wrapped `2*n=254`), never 255 - so `upsilon + 1` maxes out at 255, which fits `u8` cleanly. Same argument applies symmetrically to `varsigma`/`m`/`k`. |
| Direct, out-of-band call to the private `C_rk` helper (bypassing the r/k-generating loops entirely), forcing `r=255` on a 256-control-point curve (`n=255`) | `cargo test -p kurbo --lib`, dev profile | **Does** panic, at `mindist.rs:196:21`, "attempt to add with overflow" - confirms the line itself is mechanically capable of overflowing in isolation. But `C_rk`'s only real (non-test) caller is `D_rk`, itself only called from `S()`/`min_dist_param`'s own loops - both of which are bounded by `2*n`/`2*m` per the sweep above, so this exact call shape (`r=255` together with `n=255`) is never actually produced by any legitimate call path. |
| Sweep of `n` from 1 to 255 (`bez1.len()` = 2 to 256) through the real `min_dist_param` entry point, debug build, `catch_unwind` per iteration | `cargo test -p kurbo --lib`, dev profile, `n_plus_1 in [2,4,8,16,32,64,128,129,200,255,256]` | Every panic across the full degree range fires either at `choose()`'s u32 overflow (candidate 019, first, around n~15-31) or at `2 * n`'s u8 overflow (candidate 006, from n=128 up) - never once at `upsilon + 1`/`varsigma + 1`. |

Full captured terminal output for every run above is in `output.txt`.

## Status

**Not reproducible** through any real call path in the current code.

Covered: the actual entry point (`min_dist_param`) at the exact degree-255 boundary the candidate
names, in both debug and release builds; a full closed-form sweep of every possible degree
`0..=255`; and a direct isolated call to the named line to confirm the line itself is not immune to
the arithmetic overflow class in general (it is - see Approach 4 above) but that no real caller can
ever put it in that state. No further tool escalation was needed - the closed-form sweep (Approach
3) is exhaustive over the entire relevant input space (all 256 possible `n` values), not a sampled
or time-boxed search, so there is no "ran out of budget" caveat to state here.

## Class

N/A (not reproducible).

## Root cause (why the suspect turns out to be safe)

`upsilon = r.min(n)` can only equal 255 if **both** `r == 255` and `n == 255` simultaneously. But
every real caller of `C_rk`/`A_r` (`D_rk`, called only from `S()` and the two loops inside
`min_dist_param`) generates its `r`/`k` values from a range whose upper bound is `2 * n` (or
`2 * m`) - computed in that *same* `n`'s `u8` arithmetic. Because `2 * n` is always even, its `u8`
value (with or without wraparound) can only ever be one of `0, 2, 4, ..., 254` - never 255 or 256 -
so the maximum `r` any real call site can ever produce is 254 (`S()`'s inclusive range) or 253
(`min_dist_param`'s exclusive range). Since `n <= 255` always, and the actual max reachable `r` is
254, `upsilon = r.min(n) <= 254` in every case, and `upsilon + 1 <= 255` always fits `u8`. The
"distinct expression, distinct overflow" framing in the candidate description doesn't hold: the
same `2 * n` u8 arithmetic that candidate 006 already flags is exactly what prevents `r` (and hence
`upsilon`) from ever reaching the value this candidate's overflow would require. In a debug build
that surfaces as candidate 006's own overflow (or candidate 019's earlier `choose()` u32 overflow)
firing first, well before `n` gets anywhere near 255; in a release build (no overflow checks), it
surfaces as the wraparound capping `r` low enough that `upsilon + 1` never has anywhere near 255 to
overflow past.

This is a real property of the line in isolation (Approach 4 in "What was tried" shows the line
does overflow if called directly with `r=255, n=255`) but that call shape is unreachable dead code
given the current source's call graph - not a second, independently-triggerable bug alongside
candidate 006.

## Introduced by

N/A (not reproducible; no bug to attribute - see candidate 006's own RESULT.md, `9ab9ef7`, for the
commit that introduced the related-but-distinct `2 * n` overflow this candidate's unreachability
argument depends on).

## The fix

Not applicable - no reproducible defect found this session, so no patch was written.

- Project test suite: not run against a patch (none was written). N/A.

## What's not done / open questions

- This candidate's unreachability argument depends entirely on the current source's call graph
  (only `S()`/`min_dist_param`'s two loops ever generate `r`/`k` values passed into `C_rk`/`A_r`).
  If candidate 006's `2 * n` u8 overflow were ever fixed by widening `n`/`m`/`r`/`k` to a larger
  integer type (one option candidate 006's own RESULT.md leaves open) *without* also revisiting
  `upsilon + 1`/`varsigma + 1`, this exact overflow could become newly reachable at the true
  degree-255 boundary - worth a fresh check if/when candidate 006 is ever actually fixed upstream,
  not just re-verified against the current unfixed source as done here.
- `choose()`'s independent u32 overflow (candidate 019, already confirmed) fires even earlier than
  candidate 006's `2 * n` overflow across most of the degree range (Approach 4 above), which
  further narrows how much of the documented degree-255 range is genuinely exercisable at all
  today - not investigated further here since both are already separately confirmed candidates.
