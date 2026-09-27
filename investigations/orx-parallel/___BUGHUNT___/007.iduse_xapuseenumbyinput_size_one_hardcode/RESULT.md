# IdUse's XapUseEnumByInput impl hard-codes Size = One

`src/infallible_use/xap_variants/id.rs:45-52` (`IdUse<X, U>`'s `XapUseEnumByInput` impl) is bounded
as `impl<X: XapEnumByInput<Size = One>, U> XapUseEnumByInput for IdUse<X, U>`, hard-coding
`Size = One` even though `IdUse<X, U>`'s own `XapUse::Size` (a few lines above) is simply
`X::Size` - unconstrained, any size. `findings.toml` claims this unnecessarily blocks
`.enumerate()` on a `ParUse` pipeline entered via `use_new`/`use_vec`/`use_slice` whenever the
underlying xap has `Size = Bin`/`Size = Many`, "even in cases where that xap is built purely from
`BinM`/`ManyM` (map-only) steps that themselves do implement `XapUseEnumByInput`". Source:
`___FINDINGS___/findings.toml` (logic_error, easy).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Real end-to-end attempt through the crate's public API: `(0..10).into_par().filter_map(...).use_new(\|_\| ()).enumerate()` (`repro/src/bin/use_new_scenario.rs`) - a `Size = Bin` xap (via `.filter_map`) wrapped in `IdUse` via `use_new`, then `.enumerate()` called on the resulting `ParUse` | `cargo build --bin use_new_scenario`, stable rustc 1.98.0, against a pristine clone of `base_commit` | Fails to compile: `E0599`, "the method `enumerate` exists ... but its trait bounds were not satisfied", pointing at `IdUse`'s missing `XapUseEnumByInput` impl - repeated 2/2 with identical output |
| 2. Control experiment, same shape with `use_new`/`IdUse` removed entirely: `(0..10).into_par().filter_map(...).enumerate()` (`repro/src/bin/plain_scenario.rs`) - `Self::Xap` here is `OneF<Id<i32>, ...>` (what `.filter_map` produces), never touching `IdUse` at all | Same build, same clone | Fails to compile with the **same** "trait bounds were not satisfied" shape (`E0599`) - repeated 2/2 with identical output |
| 3. Systematic code read of every `Xap`/`XapUse` variant that changes cardinality (`Size`) in both `src/infallible/xap_variants/` and `src/infallible_use/xap_variants/`: `bin_f.rs`, `bin_x.rs`, `many_f.rs`, `many_x.rs`, `one_f.rs`, `one_x.rs` (`OneF`/`OneX`/`BinF`/`BinX`/`ManyF`/`ManyX` and their `U`-prefixed `_use` counterparts), cross-checked against an exhaustive `grep -rn "XapEnumByInput for\|XapUseEnumByInput for" src/` across the whole crate | Static reading only, no build | None of the 12 cardinality-changing (`filter`/`filter_map`/`flat_map`) variants implement `XapEnumByInput`/`XapUseEnumByInput` at all, on either the `infallible` or `infallible_use` side. The grep confirms only 5 impls of the trait exist crate-wide: `Id`, `OneM`, `BinM`, `ManyM` (infallible) and `IdUse` (infallible_use) - see "Root cause" below for why this makes the finding's premise unreachable |
| 4. Applied the finding's own suggested fix directly to the clone (relax `impl<X: XapEnumByInput<Size = One>, U>` to `impl<X: XapEnumByInput, U>`, drop the now-unused `use crate::sizes::One;` import) and rebuilt both scenario binaries again | Same build/clone, patched `id.rs` only | Identical `E0599` failure, byte-for-byte the same error shape, on **both** binaries - confirms the `Size = One` bound is not actually gating anything reachable today |

No `miri`/ASAN/fuzzing escalation was applicable - this is a purely compile-time trait-bound
question, fully resolved by `rustc`'s own trait-solver diagnostics plus a static, exhaustive
cross-check of every relevant `impl` in the crate (approach 3), not something a runtime tool would
add information to.

## Status

**Not reproducible.** The code shape `findings.toml` describes is real and was verified to exist
exactly as described (`git show`/direct file read of `id.rs:45`), but the specific consequence
claimed - `.enumerate()` being blocked on a `ParUse` pipeline "built purely from `BinM`/`ManyM`
(map-only) steps that themselves do implement" `XapUseEnumByInput` at `Size = Bin`/`Many` - cannot
currently be triggered through the crate's public API, because no such xap can exist in the first
place (see "Root cause"). Applying the finding's own suggested relaxation to a real clone and
rebuilding the exact scenario it describes (approach 4) produces the identical compile failure,
which is the concrete evidence for this conclusion, not just an argument from reading the code.

**Fully reproducible**: N/A (Status is Not reproducible, not Confirmed).

## Class

N/A (Not reproducible).

## Root cause

Not applicable in the "confirmed bug" sense - the code inconsistency the finding points at is real
(`IdUse`'s `XapUseEnumByInput` impl really is bounded to `Size = One` while its `XapUse` impl is
unconstrained), but it has no observable effect on any currently-constructible pipeline. Reasoning,
confirmed by exhaustively reading every `Size`-changing xap variant in the crate:

- `Size` (`One`/`Bin`/`Many`) only ever changes away from `One` through a `filter`/`filter_map`/
  `flat_map` step (`OneF`/`OneX` on the `infallible` side, `UOneF`/`UOneX` on the `infallible_use`
  side, and their `Bin`-/`Many`-input siblings `BinF`/`BinX`/`ManyF`/`ManyX`/`UBinF`/`UBinX`/
  `UManyF`/`UManyX`). Every entry point into the crate (`into_par`, `use_new`, `use_vec`,
  `use_slice`, ...) starts from `Id::new()`/`IdUse::new(Id::new())` at `Size = One`
  (`git grep -n "Id::new()"` across `src/into_parallel/` and `src/infallible/par.rs` confirms
  this for every source).
- None of those 12 cardinality-changing variants implement `XapEnumByInput`/`XapUseEnumByInput` at
  all (confirmed by reading each file and by a crate-wide `grep` finding only 5 total impls of the
  trait, none of them one of these 12 types).
- The only types that *do* implement the trait at `Size = Bin`/`Size = Many` (`BinM`/`ManyM`/
  `UBinM`/`UManyM`, the map-only "M" steps `findings.toml` refers to) only *propagate* an existing
  implementation from their own input (`impl<X: XapEnumByInput<Size = Bin>, ...> ... for
  BinM<X, G>`) - they never *originate* one. Since the one and only way to reach `Size = Bin`/
  `Many` in the first place (a filter/filter_map/flat_map step) never implements the trait, this
  chain has no base case to propagate from: it is currently impossible, via any public entry point
  in this crate, to construct a value of any type implementing `XapEnumByInput`/
  `XapUseEnumByInput` whose `Size` is `Bin` or `Many`.
- Consequently, `IdUse<X, U>`'s `Size = One` bound on its `XapUseEnumByInput` impl is never
  actually more restrictive than the truth on the ground today: relaxing it (as `findings.toml`'s
  own suggested fix does) does not let any additional, currently-rejected valid program compile,
  because there is no `X` satisfying the relaxed bound with `Size != One` that doesn't already fail
  to compile for the unrelated, broader reason above (approach 4's control experiment is the direct
  evidence for this).
- Git archaeology on why the bound exists at all: `a1a84149` ("define Id to be generic over
  sizes", 2026-04-09) is the commit that generalized this struct's `XapUse::Size` from a
  `One`-only predecessor (then still named `IdOne`, see `a69f3e96`/`50493a02`) to `X::Size` for any
  size - but it left the adjacent `XapUseEnumByInput` impl's bound untouched at `Size = One`, an
  apparent oversight from that generalization rather than a deliberate restriction. This matches
  `findings.toml`'s own framing ("serves no purpose since `IdUse` is a transparent identity
  wrapper") - it is a real, harmless-to-fix inconsistency, just not one with any currently
  observable behavioral impact given the rest of the crate's present state.

## Introduced by

a1a84149 "define Id to be generic over sizes" (2026-04-09) - the commit that generalized this
struct's cardinality handling to `X::Size` (any size) while leaving the adjacent
`XapUseEnumByInput` impl bound at the pre-generalization `Size = One`. Not marked required/mandatory
here since the candidate never reached `Confirmed`, but it was identified this session via
`git log -S`/`git show` on the exact impl line and is stated for completeness.

## The fix

Not attempted as a verified "fix" in the `patch.diff` sense - per `SKILL.md` step 6, a bounded fix
is only attempted/shipped for a `Confirmed` candidate, and this one is `Not reproducible`. For the
record, the finding's own suggested fix (`impl<X: XapEnumByInput, U> XapUseEnumByInput for
IdUse<X, U>`, dropping `<Size = One>` and the then-unused `use crate::sizes::One;` import) was
tried directly against the clone as part of approach 4 above and rebuilds cleanly for the crate
itself (no downstream compile errors introduced), but does not change the outcome of either
scenario binary - it is a real, harmless code-hygiene cleanup (removes a now-meaningless bound),
not a behavior-changing fix, since nothing currently reachable exercises the relaxed bound. It is
not shipped as `patch.diff` here since `SKILL.md` reserves that file for a fix verified to resolve
an actually-reproduced behavior difference, which this is not.

- Project test suite: N/A - no fix was applied/verified against a reproduction, since nothing
  reproduced. For reference, `cargo test --lib` on pristine `base_commit` (before any local edits)
  passes cleanly: 99 passed, 0 failed, 0 ignored.

## What's not done / open questions

- This candidate does not rule out that a *future* change to the crate (e.g. adding
  `XapEnumByInput`/`XapUseEnumByInput` impls for `OneF`/`OneX`/`BinF`/`BinX`/`ManyF`/`ManyX` /
  their `U`-prefixed counterparts, to support enumerating through a filter/flat_map step) would
  make `IdUse`'s `Size = One` bound suddenly start rejecting valid programs. If that ever happens,
  the fix `findings.toml` suggests (drop the `Size = One` constraint) becomes directly relevant and
  should be revisited then - this candidate's "not reproducible" verdict is a statement about the
  crate's *current* state (`base_commit` b9214fbe), not a permanent one.
- Whether the crate's maintainer *intends* to eventually support `.enumerate()` after a
  filter/filter_map/flat_map step at all (i.e. whether the missing `XapEnumByInput` impls on
  `OneF`/`OneX`/etc. are themselves an intentional gap or an oversight) was not investigated - that
  would be a separate, considerably larger feature-shaped question, out of scope for this
  candidate's narrow claim about `IdUse`.
- The harmless code-hygiene cleanup itself (relaxing the bound, removing the dead `use
  crate::sizes::One;` import) was verified to build cleanly but is not included as `patch.diff`
  per the reasoning above; a maintainer may still want to apply it purely for code clarity/future-
  proofing, independent of this candidate's reproduction verdict.
