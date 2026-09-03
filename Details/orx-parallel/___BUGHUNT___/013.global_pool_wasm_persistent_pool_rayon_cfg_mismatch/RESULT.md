# global_pool_wasm_persistent_pool_rayon_cfg_mismatch

`src/pool/global_pool.rs` selects the `persistent-pool-rayon`-backed static (case 2, rayon-core
pool) whenever `feature = "persistent-pool-rayon"` is on and NOT `all(feature = "wasm",
target_arch = "wasm32")` - i.e. it also picks this case on a native (non-wasm32) build with both
`wasm` and `persistent-pool-rayon` features enabled. But `src/pool/pool_impl/mod.rs` (and
`src/pool/pool_impl/rayon_core.rs`) only export/define `build_default_rayon_thread_pool` under the
narrower `not(feature = "wasm")`, with no `target_arch` qualifier - so that function is configured
out of scope in exactly that same combination, and `global_pool.rs` references a name that doesn't
exist there.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo check` on native target with the exact feature combo named in the candidate | `cargo check --no-default-features --features std,wasm,persistent-pool-rayon` (x86_64-unknown-linux-gnu, rustc 1.98.0) | error[E0425]: cannot find value `build_default_rayon_thread_pool` in this scope, at src/pool/global_pool.rs:22:19 |
| `cargo check --all-features` on the same native target | same target/rustc | same E0425 (plus one unrelated, pre-existing error[E0308] in src/infallible/par_runner.rs:216 - see "New suspects" below, not part of this candidate) |
| repeat-run for determinism | 3x `cargo check --no-default-features --features std,wasm,persistent-pool-rayon` after the fix | all 3 succeeded identically - this is a deterministic compile-time cfg mismatch, not a flaky runtime condition |
| standalone public-API repro (`repro/`) depending on the crate with those features, calling `orx_parallel::pool::get_global_pool()` | `cargo run --release` against the real checkout, then against a fresh clone at base_commit | fails to compile before the fix (same E0425), compiles and runs after applying `patch.diff` |

No miri/ASAN/fuzzing was applicable here - this is a `cfg`/feature-gating mismatch between two
files causing a plain, deterministic compiler error, not a runtime crash, UB, or adversarial-input
class of bug. Manual reading of the three relevant `#[cfg(...)]` blocks (`global_pool.rs`,
`pool_impl/mod.rs`, `pool_impl/rayon_core.rs`) plus a debug/release `cargo check`/`cargo run` was
the appropriate and sufficient tool here.

## Status

**Confirmed**. Deterministic: 3/3 `cargo check` runs reproduce (and, before the fix, 100% of the
`repro/` build attempts fail identically) - this is a build-time `cfg` predicate mismatch, not a
flaky condition, so it reproduces every single time the given feature combination is compiled on a
non-wasm32 target.

**Fully reproducible**: **Yes**. `repro/` is a standalone crate that depends on `orx-parallel`
through its own public API (`orx_parallel::pool::get_global_pool()`, `orx_parallel::ParThreadPool`)
with `default-features = false, features = ["std", "wasm", "persistent-pool-rayon"]` - the same way
a real downstream crate enabling these Cargo features would. There is no second participant to fake
here (this is a compile-time cfg-gating defect internal to the one crate, not one component
reacting to another's runtime-produced input), so CONFIRMATION.md section 11's "real second
participant" bar doesn't apply to this candidate; the repro drives the real end-to-end trigger path
(the library's own `Cargo.toml` feature flags -> its own `#[cfg(...)]` gates) exactly as it happens
in the wild.

## Class

logic (a build-breaking `cfg`-gating inconsistency; not a runtime crash/UB/data-race/memory-safety
issue - the bug is that the crate fails to *build* at all for a valid, allowed feature
combination).

## Root cause

Three `#[cfg(...)]` sites are all supposed to express the same logical condition ("case 2:
persistent-pool-rayon selected, and NOT (wasm feature enabled on an actual wasm32 target)"), but
two of them were written with a narrower, stale predicate:

- `src/pool/global_pool.rs:16-22` (the `PERSISTENT_POOL` static) and its `DefaultPool`/
  `get_global_pool` counterparts further down the same file: `not(all(feature = "wasm",
  target_arch = "wasm32"))` - correct, target-arch-qualified.
- `src/pool/pool_impl/mod.rs:18-23` (the `pub use rayon_core::build_default_rayon_thread_pool;`
  re-export) and `src/pool/pool_impl/rayon_core.rs:64-69` (the function definition itself):
  `not(feature = "wasm")` - stale, no `target_arch` qualifier.

So with `wasm` and `persistent-pool-rayon` both enabled and the target NOT `wasm32` (a native build
with `--features wasm,persistent-pool-rayon`, or `--all-features` on x86_64/aarch64):
`global_pool.rs`'s `not(all(feature = "wasm", target_arch = "wasm32"))` evaluates `true` (target
isn't wasm32), so it emits code that references `build_default_rayon_thread_pool` - but
`pool_impl/mod.rs`'s `not(feature = "wasm")` evaluates `false` (the `wasm` feature *is* on), so the
`pub use` re-export (and the function itself in `rayon_core.rs`) is compiled out. The name is
referenced in one file and configured out of existence in the other two, giving `error[E0425]:
cannot find value 'build_default_rayon_thread_pool' in this scope`.

## Introduced by

cd729c60 "twist features for persistent and transient pools" (2026-08-27). Before this commit,
`global_pool.rs` used the same simple `not(feature = "wasm")` predicate as `pool_impl/mod.rs` (both
introduced together, consistently, in e9a5ae88 "clippy fixes", 2026-08-06) and additionally had a
dedicated "case 0: all-features on native targets" branch that used `OncePool` instead of the rayon
pool for exactly the wasm+non-wasm32 combination, sidestepping this mismatch entirely. cd729c60
rewrote `global_pool.rs`'s case-2 predicate to the target-arch-qualified `not(all(feature = "wasm",
target_arch = "wasm32"))` form (and removed the old "case 0" special-casing) but did not make the
matching change in `pool_impl/mod.rs` or `pool_impl/rayon_core.rs`, leaving the three sites
inconsistent from that commit onward.

## The fix

Change the `#[cfg(...)]` predicate in both `src/pool/pool_impl/mod.rs` (the `pub use
rayon_core::build_default_rayon_thread_pool;` re-export) and `src/pool/pool_impl/rayon_core.rs`
(the `build_default_rayon_thread_pool` function definition) from `not(feature = "wasm")` to
`not(all(feature = "wasm", target_arch = "wasm32"))`, matching `global_pool.rs`'s existing
predicate exactly. This is a 2-line change (1 line each in 2 files); see `patch.diff`.

Verified:
- `cargo check --no-default-features --features std,wasm,persistent-pool-rayon` (the exact
  reproduction command from the candidate) - fails before, succeeds after (3/3 repeat runs after).
- `cargo check --all-features` - the E0425 from this candidate is gone after the fix; the unrelated
  pre-existing E0308 in `src/infallible/par_runner.rs:216` still remains (out of scope for this
  candidate, see "New suspects" below).
- `repro/` (standalone crate, public API) - fails to build before, builds and runs after, both
  against the live checkout and against a fresh clone of `base_commit` with `patch.diff` applied.
- Project test suite:
  - `cargo test --no-default-features --features std` (default-features build, unaffected by this
    bug either way): 260 passed; 0 failed; 16 ignored - identical before and after.
  - `cargo test --no-default-features --features std,wasm,persistent-pool-rayon` (the specific
    combination this bug breaks): fails to even compile before (lib and lib-test both fail with
    the same E0425) -> after: 260 passed; 0 failed; 17 ignored; 0 measured; 0 filtered out.

## What's not done / open questions

- `cargo check --all-features` still fails after this fix, due to an unrelated, pre-existing
  `error[E0308]` (mismatched types, `bool` vs `&I`) in `src/infallible/par_runner.rs:216`
  (`self.nt_state(params, &iter, iter.size_hint(), None)` vs `nt_state`'s own signature in
  `src/runner/par_runner.rs:66`). That is a separate defect, not investigated as part of this
  candidate - noted here only so it isn't mistaken for a sign that this candidate's fix is
  incomplete. Worth a candidate of its own.
- Did not check whether any other `cfg` sites in the crate (outside the three touched here)
  encode this same "wasm feature vs. wasm32 target" distinction inconsistently; the three found via
  `grep` for `feature = "wasm"` alongside `persistent-pool-rayon`/`build_default_rayon_thread_pool`
  were the only ones referencing this specific function, so no broader sweep was done.
- wasm32 builds themselves (`target_arch = "wasm32"`) were not tested here (no wasm32 target/tooling
  set up in this environment) - this candidate is specifically about the native-target
  wasm-feature-enabled-but-not-wasm32-target combination, which is unaffected by wasm32 toolchain
  availability since `cargo check`/`cargo test` above all ran on the native
  x86_64-unknown-linux-gnu target.
