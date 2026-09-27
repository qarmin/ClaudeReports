# bool_not_cast_widening_negative_one_bug

Upstream issue #691 ("Another UB found using rustlantis") reports a rustlantis-minimized program
where `!arg as usize` (logical negation of a `bool`, then widening cast to `usize`) prints `1` in
a "GCC debug" build and `-1` (i.e. `usize::MAX`) in a "GCC release" build - a value-widening
mismatch that looked like it could be the same "wrong declared GCC sign carried through a
cast/extension" bug class already confirmed elsewhere in this workspace (candidate 004's
`fetch_max`/`fetch_min`, candidate 006's `type_uint_from_ty`), but via an independent, more
minimal trigger worth checking on its own rather than assuming it's the same root cause.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Verbatim repro from issue #691, compiled directly with `./y.sh rustc` (no cargo), across both a `dev`-profile and a `release`-profile build of the `rustc_codegen_gcc` backend `.so` itself, and across `-O`/no-`-O` and `-C codegen-units=1` variations of the compiled test program | Real build, `./y.sh build` / `./y.sh build --release`, `CHANNEL=debug`/`CHANNEL=release ./y.sh rustc [-O] tmp_issue691.rs` | Every combination printed `val2:2` then `val2:1` - the correct result. No `-1`/`usize::MAX` seen in any run. |
| The same verbatim repro built as a real Cargo binary (`repro/issue691.rs` + `repro/Cargo.toml`) through `CHANNEL=debug ./y.sh cargo build` (dev profile, opt-level 0) and `CHANNEL=release ./y.sh cargo build --release` (release profile, opt-level 3), `codegen-units = 1` in both profiles (to make sure the whole program lands in a single GCC compilation unit, closest to what a real `cargo build`/`cargo build --release` would produce), `lto = "off"` in both (this environment's `download-gccjit = true` setup crashes on fat LTO for an unrelated, already-tracked reason - candidate 001) | Real build, current `master` (4cd7da5), see `repro/run.sh` and `output.txt` for the exact commands and captured output | Both the dev-profile binary and the release-profile binary printed `val2:2` / `val2:1`, byte-for-byte identical. Re-ran the built binaries 3 more times each (deterministic, no ASLR/layout dependency expected for a pure miscompilation question) - same result every time, 3/3. |
| `git log --oneline --since=2025-06-01 -- src/` (issue #691 was filed 2025-06-01) for commits touching casting/int-width logic, since a fix landing after the report date would explain a clean current-`master` negative | `git log`/`git show` on `src/builder.rs`, `src/int.rs`, `src/common.rs` | Found `035ff85 "Fix intcast to use the is_signed parameter"` (2025-08-03, i.e. after issue #691 was filed): before this commit, `Builder::intcast`'s `is_signed` parameter was accepted but never used (`_is_signed`, with a comment "NOTE: is_signed is for value, not dest_typ.") - it just called `gcc_int_cast(value, dest_typ)` directly on whatever GCC type `value` already had, with no correction for a mismatch between the caller's requested signedness and the value's actual declared GCC type. This is exactly the "wrong declared sign carried through a cast" bug class this workspace has repeatedly found elsewhere (candidates 004/006). No GitHub comment or commit message on #691 itself references this fix, so the link is circumstantial, not confirmed by re-reproducing on the pre-fix commit (see "What's not done" below). |

## Status

**Not reproducible** on current `master` (4cd7da5f9a5d9509b34ac0a766424ccdae706d33). Covered: the
issue's own verbatim repro, compiled both directly via `./y.sh rustc` and via a real `cargo`
binary; both a `dev`-profile and a `release`-profile build of the codegen backend itself; both
unoptimized and `-O3`-equivalent (`cargo build --release`) compilation of the test program;
`codegen-units = 1` to keep the whole program in one compilation unit. Every combination produced
the correct `1`, never `-1`/`usize::MAX`, deterministically across repeat runs (3/3).

**Fully reproducible**: N/A (nothing reproduced - this field only applies to `Confirmed` findings).

## Class

N/A (Not reproducible).

## Root cause

Not established as a currently-live bug: the exact reproduction from issue #691 no longer produces
a wrong value on this checkout. The most plausible explanation found this session is that commit
035ff85 ("Fix intcast to use the is_signed parameter", 2025-08-03, after the issue was filed on
2025-06-01) already addresses the general bug class the issue describes (a cast that ignores the
caller-requested signedness and just casts between whatever GCC types the value and destination
happen to have) - but this is circumstantial (found via `git log`/`git show` reading the commit,
not by checking out the pre-fix commit and re-running the exact repro against it), not a verified
before/after bisection. It's also possible the original observation depended on a specific
libgccjit version's own optimizer behavior (see the issue thread's discussion of `static` vs
`extern` linkage changing GCC's interprocedural optimization decisions) that no longer applies
after this project's own libgccjit version bumps (e.g. 3de358d, 2026-08-26) - the two explanations
aren't mutually exclusive and this session didn't isolate which one (if either) is the actual
cause.

## Introduced by

N/A - not reproducible, no live buggy commit to identify. (See "Root cause" above for the
candidate upstream fix found via `git log`, which is a *plausible fix* for the general bug class,
not an "introduced by" for a bug confirmed to still exist.)

## The fix

No fix - nothing to fix on current `master`. No `patch.diff` in this folder.

- Project test suite: N/A, no fix was applied (nothing reproduced to fix), and no source change
  was made to the main tree during this investigation (the worktree used for digging was discarded
  as-is, never merged; the main project's own working tree was never touched).

## What's not done / open questions

- This session did not check out the pre-`035ff85` commit and re-run the exact repro against it to
  confirm that commit is actually what changed the observed behavior (would require reinstalling a
  `nightly-2025-07-21`-era Rust toolchain via `rustup`, not available locally in this environment) -
  the connection to that commit is a plausible lead from reading the diff, not a bisection result.
  A future session with that toolchain available could confirm or rule this out.
- Did not attempt to reproduce the `static`-vs-`extern`/`no_mangle` linkage-dependent behavior the
  issue thread itself describes (GCC applying different interprocedural optimizations based on
  whether the mangled function is internal- or external-linkage) - if the true root cause is in
  libgccjit's own optimizer rather than in this crate's codegen, no change to this repo would ever
  fix it, and confirming that would require instrumenting/diffing GCC's own GIMPLE/IPA dumps
  (`CG_GCCJIT_DUMP_TO_FILE=1`, as suggested in the issue thread), which wasn't attempted this
  session.
- Did not sweep other `bool`-widening call sites (e.g. `bool as iN`/`bool as u128` non-native-int
  paths, or `bool` fields inside larger casts) for the same class of issue independently of this
  specific repro - this session only checked the exact reported trigger.
