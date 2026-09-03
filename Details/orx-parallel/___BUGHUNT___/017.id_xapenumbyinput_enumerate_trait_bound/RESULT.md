# Id<I>'s XapEnumByInput::enumerate() trait-bound restriction

The project risk map (`projects/orx-parallel.md`) flags `id.rs`'s `enumerate()` trait-bound
restriction as a compile-only check worth pinning down, framed as distinct from candidate 007
(`IdUse`'s `XapUseEnumByInput` impl hard-coding `Size = One` in
`src/infallible_use/xap_variants/id.rs`). The candidate as seeded pointed primarily at `Id<I>`'s
own `XapEnumByInput` impl in `src/infallible/xap_variants/id.rs:32-38`
(`impl<I> XapEnumByInput for Id<I>`), asking whether *that* impl itself carries a restrictive
bound blocking `.enumerate()` somewhere.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Static comparison of `Id<I>`'s `XapEnumByInput` impl (`src/infallible/xap_variants/id.rs:32`) against the trait definition (`src/infallible/xap_enum.rs:4`) and every other impl of the trait crate-wide (`grep -rn "XapEnumByInput for" src/`) | Static reading only, no build | `Id<I>`'s impl is `impl<I> XapEnumByInput for Id<I>` - **no bound on `I` at all**, the least restrictive form possible. The only other `infallible`-side impls (`OneM`, `BinM`, `ManyM`) each require `X: XapEnumByInput<Size = One/Bin/Many>` on their wrapped xap, matching their own fixed cardinality - not a restriction unique to `Id`. The only impl anywhere in the crate that actually hard-codes an unnecessary `Size = One` bound is the *sibling* `infallible_use` file's `IdUse` impl (`src/infallible_use/xap_variants/id.rs:45`), already the exact subject of candidate 007 |
| 2. Real end-to-end attempt through the crate's public API: `(0..10).into_par().filter(\|x\| *x % 2 == 0).enumerate()` (`repro/src/bin/filter_then_enumerate.rs`) - `.filter()` produces `OneF<Id<i32>, ...>`, i.e. `Id<I>` is still present in the chain, just wrapped | `cargo build --bin filter_then_enumerate`, stable rustc 1.98.0, against a pristine clone of `base_commit` | Fails to compile: `E0599`, "trait bounds were not satisfied", pointing at `OneF<Id<i32>, ...>: XapEnumByInput` being unsatisfied - **not** at any bound on `Id<i32>` itself (`Id<i32>: XapEnumByInput` holds unconditionally). Repeated 2/2 with identical output |
| 3. Control experiment through the public API: `(0..10).into_par().map(\|x\| x * 2).enumerate()` (`repro/src/bin/map_then_enumerate_control.rs`) - `.map()` produces `OneM<Id<usize>, G>`, whose own impl requires `X: XapEnumByInput<Size = One>`, which `Id<usize>` satisfies unconditionally (`Id`'s `Size` is always `One`) | `cargo run --bin map_then_enumerate_control`, same build/clone | Compiles and runs correctly, producing the expected `[(0, 0), (1, 2), ..., (9, 18)]` - direct, positive proof that `Id<I>`'s own `XapEnumByInput` impl is not restrictive anywhere it is actually reachable through the public API. Repeated 2/2 with identical output |
| 4. Re-verification of the sibling file's own restriction (candidate 007's subject) at this exact `base_commit`, independently: `(0..10).into_par().filter_map(...).use_new(\|_\| ()).enumerate()` (`repro/src/bin/filter_map_use_new_then_enumerate.rs`) | Same build/clone | Fails to compile with the same `E0599` shape candidate 007 already documented and root-caused - `IdUse<X, U>`'s `Size = One` bound is unreachable today because no `Bin`/`Many`-sized `XapEnumByInput` value can currently be constructed through the crate's public API at all (see candidate 007's `RESULT.md` for the full exhaustive argument, unchanged since it was written - same `base_commit` `b9214fbe`). Repeated 2/2 with identical output |

No `miri`/ASAN/fuzzing escalation was applicable - this is a purely compile-time trait-bound
question, resolved by reading every relevant `impl` in the crate (approach 1) plus direct
compiler diagnostics on real end-to-end scenarios (approaches 2-4), not something a runtime tool
would add information to.

## Status

**Not reproducible.** `Id<I>`'s own `XapEnumByInput` impl in `src/infallible/xap_variants/id.rs`
carries no restrictive bound whatsoever - it is already the maximally permissive form
(`impl<I> XapEnumByInput for Id<I>`) and was verified (approach 3) to work correctly everywhere it
is actually reachable through the public API. The compile failure that *is* observable when
chaining `.filter()`/`.filter_map()` before `.enumerate()` (approach 2) is real, but its cause is a
different, already-catalogued limitation - `OneF`/`BinF`/`ManyF`/`OneX`/`BinX`/`ManyX` (the
filter/filter_map/flat_map-producing xap variants) never implement `XapEnumByInput`/
`XapUseEnumByInput` at all, on either the `infallible` or `infallible_use` side. This is
`___FINDINGS___/findings.toml`'s own `architecture`-typed finding (locations
`src/infallible/xap_variants/{bin_f,bin_x,one_f,one_x,many_f,many_x}.rs` and their
`infallible_use` counterparts) - explicitly excluded from this bug-hunt workspace's `IDEAS.md`
backlog by design (`architecture`-typed findings were left out of the seed on 2026-08-30, see
`projects/orx-parallel.md`'s risk map), not a crash/UB/logic-correctness defect this skill
re-litigates. The `infallible_use` sibling file's own `Size = One` bound on `IdUse` (approach 4) is
the exact subject already fully investigated and marked Not reproducible in candidate 007 - this
candidate's independent re-check (approach 4) confirms the identical conclusion still holds at
this exact `base_commit`, so this is a duplicate of 007's finding rather than a new, distinct one.

**Fully reproducible**: N/A (Status is Not reproducible, not Confirmed).

## Class

N/A (Not reproducible).

## Root cause

Not applicable in the "confirmed bug" sense.

- `Id<I>`'s own `XapEnumByInput` impl (`src/infallible/xap_variants/id.rs:32-38`) has never carried
  a restrictive bound since it was first written (`763e93fb` "implement XapEnumerable for Id",
  2026-03-31 - the commented-out placeholder it replaced was likewise unbounded). There is nothing
  to fix here; the risk map's framing of this as a suspect distinct from candidate 007 turned out,
  on investigation, to point at code that is already correct.
- The real, separate restriction that blocks `.filter(...).enumerate()`/`.filter_map(...).enumerate()`
  end-to-end (approach 2) lives entirely in the `*F`/`*X` xap variants never implementing
  `XapEnumByInput` at all - `Id<I>` (wrapped inside `OneF`/etc.) is not the blocker; `OneF` itself
  is. This is the pre-existing `architecture`-typed finding in `findings.toml`, out of this
  workspace's declared scope (see `IDEAS.md`'s own note on why `architecture`-typed findings were
  excluded from the backlog).
- The sibling file's real inconsistency (`IdUse`'s `Size = One` bound, approach 4) is candidate
  007's finding verbatim, re-confirmed unchanged at this same `base_commit` - see that candidate's
  `RESULT.md` for the full root-cause argument and its own best-effort `Introduced by` (`a1a84149`
  "define Id to be generic over sizes").

## Introduced by

Not applicable - `Id<I>`'s own impl (this candidate's primary subject) was never restrictive to
begin with, so there is no introducing commit for a bug that doesn't exist there. For the sibling
file's bound (approach 4, a duplicate of candidate 007), see that candidate's `RESULT.md`:
a1a84149 "define Id to be generic over sizes" (2026-04-09).

## The fix

Not attempted - per `SKILL.md` step 6, a bounded fix is only attempted for a `Confirmed`
candidate, and this one is `Not reproducible`. There is nothing to fix on `Id<I>`'s own impl (it
is already unbounded/correct). The two things a maintainer might still choose to act on, neither of
which this candidate ships a patch for since neither is this candidate's own confirmed bug:
- The sibling `IdUse` bound's harmless cleanup - already covered by candidate 007's own writeup.
- The broader `*F`/`*X` variants-never-implement-`XapEnumByInput` architecture gap - a considerably
  larger change (implementing the trait for six more variants on each side, threading `(usize, ...)`
  through zero-or-many cardinality), explicitly out of this skill's bug-hunt scope per the project's
  own `IDEAS.md` seeding notes.

- Project test suite: N/A - no fix was applied/verified against a reproduction, since nothing
  reproduced as a new, distinct bug. For reference, `cargo test --lib` on pristine `base_commit`
  (before any local edits) passes cleanly: 99 passed, 0 failed, 0 ignored (same result candidate
  007 recorded, this candidate made no source edits so no before/after comparison applies).

## What's not done / open questions

- This candidate does not itself investigate or re-litigate the broader `architecture`-typed
  finding (missing `XapEnumByInput`/`XapUseEnumByInput` impls on `*F`/`*X` variants) beyond
  confirming it's the real cause of the compile failure observed in approach 2 - that finding
  remains deliberately out of this bug-hunt workspace's scope (see `IDEAS.md`'s seeding notes), not
  silently dropped.
- Whether the risk map's original phrasing ("id.rs's enumerate() trait-bound restriction") intended
  to point at the `infallible` file, the `infallible_use` file, or was simply an imprecise
  restatement of candidate 007's own finding was not resolvable with certainty - this candidate
  investigated the most literal reading (a restriction on `Id<I>`'s own impl) and, finding none,
  also re-checked the one adjacent real restriction that does exist (the sibling file, already
  candidate 007) to make sure nothing new was being missed.
