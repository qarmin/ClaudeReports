# `mindist.rs`'s `choose()` overflows/wraps in `u32` far below the module's documented degree-255 range

`kurbo/src/mindist.rs`'s `choose(n: u8, k: u8) -> u32` (lines 244-257) accumulates the binomial
coefficient C(n, k) in a plain `u32`. `min_dist_param`'s own doc comment (lines 16-28) claims
support for "arbitrary Bezier curves up to and including degree 255". `choose` is called (via
`basis_function`/`C_rk`/`A_r`) with arguments up to `2 * n`, and C(2n, n) already exceeds
`u32::MAX` (4,294,967,295) starting around n = 16 (measured exactly below) - vastly below the
documented degree-255 boundary, and also below the degree-127 boundary candidate 006's own patch
would newly document for the separate `2 * n` u8-doubling overflow it targets. This candidate was
raised directly by candidate 006's own "What's not done" section (a new suspect it noticed but
did not investigate), which already estimated the threshold at "n ~= 17-18" - confirmed here.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (matches `HEURISTICS.md`'s "arithmetic on untrusted sizes/indices" - no sanitizer needed) + debug-assertions build, binary search over `n` from 8 to 18 with an in-tree `#[test]` calling `min_dist_param` directly | `cargo test`, default dev profile (debug-assertions=true, overflow-checks=true, this repo's Cargo default) | **Reproduced** on the first attempt. Binary search pinned the exact threshold: `n <= 15` completes normally; `n = 16, 17, 18` all panic ("attempt to multiply with overflow") inside `choose()` at `mindist.rs:252:18` (the `p *= n;` line) - confirming candidate 006's own estimate. Chose `n = 18` (C(36,18) = 9,075,135,300, over 2x past `u32::MAX`) as the repro's fixed input for a comfortable margin. No further tools needed (soft cap of 3 not exhausted; stopped after a clean, deterministic positive on approach 1). |
| Same input (`n = 18`), `--release` build | `cargo run --release` (via `repro/`, see below) | Confirms the release-mode counterpart: **no panic**, `choose()`'s internal `u32` multiply/divide silently wraps, and `min_dist_param` returns a meaningless `(NaN, 0.5, 0.5)` instead of erroring - a silent-wrong-output logic bug, not just a debug-build crash (same shape candidate 006 found for its own, higher-degree-threshold overflow). |

## Status

**Confirmed**

Reproduces 3/3 runs, default dev-profile (debug-assertions on) build, deterministic (pure
computation, no randomness/threading/layout-dependence) - see `output.txt`. Release build does not
panic; instead it silently returns a bogus, non-error `NaN` result (also captured in
`output.txt`) - both build-profile outcomes were checked (per `CONFIRMATION.md` section 2).

**Fully reproducible**: **No** - justified, identically to candidate 006. `min_dist_param` and
`choose` are `pub(crate)`/private inside kurbo: neither can be invoked from any external crate's
code at all, by Rust's own visibility rules, regardless of any wrapper kurbo itself ships
(`PathSeg::min_dist` is the only real caller, and it structurally can never pass `n/m > 3`, since
`PathSeg` only has Line/Quad/Cubic variants). There is no real "second participant" being faked
here (this isn't a peer/caller-input-shape finding in the sense `CONFIRMATION.md` section 11 is
about) - the obstacle is that the *only* entry point for this finding is not part of kurbo's
public API surface at all. `repro/` calls the exact, real, unmodified `min_dist_param`/`choose`
(compiled verbatim via `#[path]` from whatever is currently checked out in `./clone`, so the same
source runs for both "before" and "after" against `patch.diff`) with a genuine, valid
19-control-point (degree-18) Bezier curve - exactly the kind of input the function's own doc
comment says it accepts - rather than reimplementing or approximating the function's logic. What
it cannot do is reach that function through kurbo's actual public API, because no such path
exists; that inaccessibility is itself the fact this candidate documents (same as candidate 006).

## Class

logic / crash (build-profile-dependent: panics under debug-assertions, silently returns a wrong
`NaN` result in release)

## Root cause

`kurbo/src/mindist.rs:244-257`'s `choose(n, k)` computes the binomial coefficient using the
classic "multiply then divide" running-product technique, entirely in `u32`:

```rust
fn choose(n: u8, k: u8) -> u32 {
    let mut n = n as u32;
    let k = k as u32;
    if k > n { return 0; }
    let mut p = 1;
    for i in 1..=(n - k) {
        p *= n;   // <- overflows here, even when the FINAL C(n,k) would still fit u32
        p /= i;
        n -= 1;
    }
    p
}
```

Every call site (`basis_function`, `C_rk`, `A_r`) immediately casts the result `as f64` and uses
it in a ratio/weighted-sum of `f64` curve points - the function has no reason to be integer-typed
at all except historical accident (see "Introduced by" below). Two compounding problems, both
already present in a plain-`u32` `choose()`:

1. The *final* value C(n, k) itself exceeds `u32::MAX` well before the module's documented
   degree-255 boundary: C(2n, n) first exceeds `u32::MAX` at n = 18 (C(36,18) =
   9,075,135,300), confirmed by direct computation.
2. Even more severely, the multiply-before-divide loop body (`p *= n; p /= i;`) can overflow an
   *intermediate* product even when the *final* C(n,k) would still fit in `u32` - confirmed by
   binary search: `choose(32, 16)` (n=16, C=601,080,390, comfortably under `u32::MAX`) already
   panics, because the running product temporarily climbs past `u32::MAX` partway through the
   loop before the corresponding division brings it back down. This makes the real, effective
   safe threshold for this implementation's technique lower than a naive "does the final
   binomial coefficient fit in u32" calculation would suggest.

`min_dist_param` is called recursively via its own subdivision (see lines 125-158) with the same
`bez1`/`bez2` (hence the same `n`/`m`) every time, so once an input's `n` crosses this threshold,
every recursive call hits the identical overflow - it is not a rare/edge-of-subdivision-only
occurrence for a given input.

## Introduced by

81fbd0691382837b49c4cf26b48bd6dea78b45e4 "Compute the minimum distance between two pathsegs."
(2021-08-25)

This is the original commit that added `mindist.rs` (and `choose`) to the codebase at all -
`choose` returned `u32` (via `p as u32` truncation at the end, with the running accumulator `p`
itself typed as `usize` internally) from its very first version, i.e., the "cast the final result
down to u32" design flaw is as old as the module itself, confirmed via `git log -S`/`git show` on
`fn choose(n: usize, k: usize) -> u32` (the pre-refactor signature). A later commit, 9ab9ef7 "Fix
wrapping arithmetic in `min_dist_param` by limiting Bezier degree (#542)" (2026-01-22, the commit
candidate 006 root-causes), changed `n`/`k`'s parameter types from `usize` to `u8` and made the
internal accumulator `p` itself `u32`-typed (previously it was `usize`, with only the final
`as u32` cast at the end) - this changed the *exact failure mode* this candidate observes (a
debug-mode panic on the internal `u32` multiply, rather than a full-range `usize` computation
silently truncated to `u32` only at the very end) without being the commit that introduced the
underlying "return type/accumulator too narrow for the claimed degree range" design flaw itself.
Both commits are noted here since they're both relevant to "what changed and when": 81fbd06
introduced the flaw, 9ab9ef7 (while fixing a different, adjacent issue) changed how visibly it
manifests.

## The fix

Changed `choose(n: u8, k: u8)` to accumulate and return `f64` instead of `u32`, and updated the
three call sites (`basis_function`, `C_rk`, `A_r`) to drop their now-redundant `as f64` casts
(`choose`'s result is already `f64`). Rationale: every call site immediately used the integer
result only to cast it to `f64` and combine it with other `f64` values (curve points, ratios) -
there was no exact-integer computation anywhere downstream that this throws away. `f64` doesn't
overflow/wrap/panic the way a fixed-width integer does; instead it gracefully loses precision for
very large binomial coefficients (silent but bounded/well-understood floating-point behavior, not
UB or wraparound), which matches how the rest of this approximate-minimum-distance algorithm
already operates throughout (`S`, `C_rk`, `A_r`, `D_rk` are all `f64` end-to-end).

Verified against more than the one triggering input (`CONFIRMATION.md` section 7):
- `n = 18` (the original repro, C(36,18) ~= 9.08e9, past `u32::MAX`): re-ran in a `#[test]` in the
  patched worktree - no panic, no `NaN`, in both debug and release builds; result `(1.0, 0.5,
  0.5)` in both.
- `n = 100` (a much larger degree, well within even candidate 006's proposed degree-127 limit,
  C(200,100) ~= 9.05e58, astronomically past any fixed-width integer type, exact only to `f64`'s
  ~15-17 significant digits): re-ran the same way - no panic, no `NaN`, in both debug and release;
  result `(0.9999999999999994, 0.5, 0.5)` in both (the tiny deviation from `1.0` is the expected,
  bounded floating-point precision loss the fix's own doc comment calls out, not a new bug).
- Added a permanent regression test, `test_choose_no_overflow_past_u32_range`, asserting
  `choose(36, 18)` matches the expected value within `1e-9` relative error (see `patch.diff`).
- `cargo clippy -p kurbo --all-targets` on the patched tree: no new warnings for `mindist.rs`.

Also re-ran candidate 006's own repro input (`n = 128`, i.e. `bez1.len() == 129`) against this
fix alone (without candidate 006's separate `2 * n` u8-widening patch): confirms this fix does
**not** address candidate 006's own `u8`-doubling overflow (a different root cause, different
location, `n`/`m`/loop indices themselves still `u8`-typed) - the two are independent bugs in the
same module, exactly as candidate 006's "What's not done" section anticipated, and each needs its
own fix; this candidate's fix only addresses `choose`'s own accumulator/return type.

- Project test suite (`cargo test -p kurbo`, run both directly in the worktree used to develop
  the fix and inside `repro/`'s own `./clone`): **before**: 187 lib tests passed, 0 failed; 55
  doctests passed, 0 failed. **after** (patch applied, including the 1 new regression test add):
  188 lib tests passed, 0 failed; 55 doctests passed, 0 failed. No new failures introduced.

## What's not done / open questions

- **Does not fix candidate 006's separate `2 * n` u8-doubling overflow** (see above) - that
  candidate's own fix (tightening the documented/enforced degree limit) is still needed
  independently; the two patches are not mutually exclusive and could both be applied (though
  they were developed and verified independently against a pristine tree each, per this skill's
  one-candidate-one-fix discipline, and were never combined into one diff).
- Whether a real, exact-integer solution (e.g. `u128`, or Pascal's-triangle-style additive
  recurrence instead of multiplicative, which stays exact far longer) would be preferable to the
  `f64`-based fix here for precision-sensitive callers is a design question for maintainers -
  `min_dist_param`'s only real caller today never needs `n > 3` (see "Fully reproducible" above),
  so the practical precision impact of this fix is currently unobservable through any shipped
  code path.
- Not checked whether `basis_function`'s own `.powi()` calls on `(1.0 - u)`/`u` introduce any
  additional numerical-stability concerns at very large `n` - out of scope for this candidate,
  which is specifically about `choose()`'s integer overflow.
- Not checked whether any other module in this crate has a similar "binomial coefficient /
  combinatorial accumulator in a narrow fixed-width integer type" pattern - this was a single,
  targeted candidate, not a codebase-wide sweep for the same pattern (see `SKILL.md`'s "Critical
  pitfalls" #7).
