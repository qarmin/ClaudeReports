# collect_arb_over_bag_bool_type_mismatch

`ParRunnerInfallible::collect_arb_over_bag` in `src/infallible/par_runner.rs` (gated behind the
`experimental` Cargo feature) calls `self.nt_state(params, &iter, iter.size_hint(), None)` at line
216, passing `&iter` (a `&I`) where `ParRunner::nt_state`'s second parameter is `is_source_serialized:
bool`. Every sibling caller in the same file (`next`, `next_any`, `reduce`, `collect`, `collect_arb`)
correctly passes `I::is_source_serialized()`. Flagged by an existing `code-audit-structured` pass
(`___FINDINGS___/findings.toml`, logic_error/easy) claiming `cargo check --features experimental
--lib` fails to compile.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo check --features experimental --lib` (debug build, direct read of the audit claim) | run twice in a row against pristine `HEAD` (b9214fbe) | Both runs failed identically with `error[E0308]: mismatched types` at `src/infallible/par_runner.rs:216:48` ("expected `bool`, found `&I`") |
| Standalone `repro/` (fresh `git clone` + real `cargo run --release`) | 2 full fresh-clone runs before the fix, 2 after (via `just test` / manual reruns) | Before: dependency fails to build with the same E0308 both times. After (patch applied): dependency builds and the binary runs, printing the correct sum |

No miri/ASAN/fuzzing was used - this is a plain type-checker rejection (not UB, not adversarial
input), so a debug-assertions build (`cargo check`) is the correct and sufficient tool; escalating
to a sanitizer or fuzz harness would add nothing here.

## Status

**Confirmed**. Reproduces 2/2 times under a plain `cargo check --features experimental --lib`
(debug build) directly in the working tree, and 2/2 times via a fresh `git clone` + `cargo build`
of the dependency from a genuinely separate crate (see `repro/`). This is a deterministic
compile-time type error, not a flaky/layout-dependent one - every run gives byte-identical rustc
output.

**Fully reproducible**: **Yes**. There is no second participant to fake here (unlike
`CONFIRMATION.md` section 11's client/server case) - the bug is a plain `rustc` type-checker
rejection triggered purely by enabling the crate's own `experimental` Cargo feature and building
it, with no runtime input at all. `repro/` is a genuine standalone downstream crate (its own
`Cargo.toml`, depending on a real, separately-cloned checkout of `orx-parallel` with
`features = ["experimental"]`) that runs the real, unmodified build of the library through
`cargo run`, exactly as any real user turning on that feature would.

## Class

logic (a plain argument-type mismatch caught by the compiler at build time - it never reaches
runtime, so `collect_arb_over_bag` and anything that would call it under `experimental` is
currently completely unusable).

## Root cause

`ParRunner::nt_state` (`src/runner/par_runner.rs:66`) takes `is_source_serialized: bool` as its
second parameter and uses it at line 84 (`if is_source_serialized { Self::configure_for_serialized_input(...) }`).
Every other caller of `nt_state` in `src/infallible/par_runner.rs` passes `I::is_source_serialized()`
(a `bool` from the `ConcurrentIter` trait) in that position. `collect_arb_over_bag`'s call instead
passes `&iter` (a `&I`) - almost certainly a copy/paste slip while wiring up this newer method (it
still has `iter` in scope as a local right above, unlike the other callers which use `&iter` a few
lines later for the actual scoped-computation closure, so `&iter` was likely typed by muscle memory
from that later line instead of `I::is_source_serialized()`). Since `&I` and `bool` are unrelated
types, this simply fails to typecheck - it cannot compile under any input, so there is no
runtime-observable failure mode beyond "this function, and the crate as a whole under
`--features experimental`, does not build."

## Introduced by

b9214fbeb714e4bb8f8858676a818cef2f21b097 "iter of iter uses fixed chunk heuristic (#198)" - this
is the same commit that was checked out as `HEAD` for this candidate; `git log -p --follow -- src/infallible/par_runner.rs`
shows the `nt_state(params, &iter, iter.size_hint(), None)` line was introduced by this commit's
own diff (previously `nt_state` took only `(params, size_hint, computation_max_nt)`, before the
`is_source_serialized` parameter existed at all in any caller). The bug has therefore never existed
in a state where `cargo check --features experimental --lib` succeeded on any commit prior to this
one on this branch.

## The fix

Changed the one call site to match every sibling caller:

```
-        let (_, state) = self.nt_state(params, &iter, iter.size_hint(), None);
+        let (_, state) = self.nt_state(params, I::is_source_serialized(), iter.size_hint(), None);
```

This is the same one-line pattern every other method in the same `impl` block already uses, so it
is not narrow to one specific input in the sense of `CONFIRMATION.md` section 7 - there is no
"input" here at all (it's a compile error independent of any runtime value), and the fix makes this
call site textually identical in shape to five already-correct sibling call sites in the same file.

- Project test suite (before the fix, pristine `HEAD`): `cargo test --lib` (default features, no
  `experimental`) - 99 passed; 0 failed (the buggy line is `#[cfg(feature = "experimental")]`-gated
  and is simply not compiled into the default-feature build, so the default test suite was never
  affected by this bug). `cargo test --features experimental --lib` could not even be attempted
  before the fix - `cargo check --features experimental --lib` itself fails to compile (E0308,
  shown above), so there is no "before" test-pass count under that feature to report beyond
  "0 tests ran, compile error."
- Project test suite (after the fix): `cargo test --lib` (default features) - 99 passed; 0 failed
  (unchanged, as expected). `cargo test --features experimental --lib` - 113 passed; 0 failed (the
  additional 14 tests are ones gated behind `experimental` elsewhere in the crate that simply could
  not run at all before, since the crate didn't compile under that feature).

## What's not done / open questions

- `collect_arb_over_bag` itself is not currently called from anywhere else in the crate (`grep -rn
  "collect_arb_over_bag"` outside its own definition and its `thread_execution` implementation
  finds no call site) - it looks like scaffolding for a not-yet-wired-up public API, marked
  `#[allow(unused)]`. This candidate only demonstrates that the function fails to compile at all
  under `--features experimental`; it does not (and cannot yet, since there is no public entry
  point reaching it) demonstrate a runtime consequence beyond "the crate does not build with this
  feature enabled."
- No other `nt_state` call site in the crate has this mistake - `grep -rn "nt_state(params, &iter"
  src/` after the fix returns nothing, and all other callers already matched the correct pattern
  before this candidate touched anything.
