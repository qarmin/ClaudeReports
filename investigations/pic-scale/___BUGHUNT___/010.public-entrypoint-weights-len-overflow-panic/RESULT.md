# Public resize entry points: unchecked `source_size` reaches an unchecked `usize` multiply in filter-weight generation

Backlog candidate: do the public resize entry points (`src/lib.rs`, `Scaler`/`ImageStore`
construction) validate documented-safe input ranges (0, 1, huge, overflow-adjacent width/height)
before doing unchecked arithmetic feeding into unsafe SIMD kernels? Investigated end-to-end via
manual boundary-value construction against `Scaler::plan_rgba_resampling` (and, by code reading,
its siblings for the other pixel formats) with `width`/`height` values at 0, 1, and several
overflow-adjacent points.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `ImageStore::new`/`borrow`/`ImageStoreMut::new`/`borrow` (`src/image_store.rs`): both compute `width * height * N` with a plain, unchecked `usize` multiply to size-check the caller's buffer | reading only | Confirmed these constructors can accept a mismatched buffer if `width*height*N` wraps past `usize::MAX` - but every actual resize entry point (`resample()`/`scale()`) subsequently calls `.validate()` on the resulting store, which re-derives the identical product via `checked_mul` on `isize` (`src/support.rs::check_image_size_overflow`) and rejects it (`isize::MAX` is far below the `usize` wraparound point, so this second check always catches it first) - no reachable memory-safety bug from this specific angle |
| Code reading of `DefaultPlanner::plan_generic_resize`/`AlphaPlanner::plan_generic_resize_with_alpha` (`src/plan/*_planner.rs`): `destination_size` is passed through `check_image_size_overflow` before planning, `source_size` is not validated at all before being handed to `T::make_weights(function, source_size.{width,height}, destination_size.{width,height})` | reading only | Found the asymmetry: only `destination_size` is bounds-checked pre-plan; `source_size` (equally caller-supplied, equally unvalidated at this point) is not |
| Manual boundary-value construction (`examples/overflow_probe.rs`, temporary, reverted): called `Scaler::plan_rgba_resampling` directly with `source_size`/`destination_size` at 0, 1, and several overflow-adjacent points (`usize::MAX/8`, `isize::MAX-1`, `1<<40`, `1<<62`, `usize::MAX`), debug (`overflow-checks = true`, Cargo's default `dev` profile) and release builds | `cargo run --example` / `cargo run --release --example`, this host | Most extreme values correctly returned `Err(OutOfMemory(..))` (the `try_reserve_exact` calls in `try_vec!` fail gracefully for allocations no real machine can satisfy) - **except** `source_size=(1<<62, 1)`, `destination_size=(2, 1)` (and several nearby combinations), which **panicked** in the debug build: `attempt to multiply with overflow` at `src/math/weights.rs:179`, reliably, 3/3 runs. Release build (default, no `overflow-checks`) did not panic for the exact values tried (the wrapped product happened to still be huge, so it hit the same graceful `OutOfMemory` path) - see "Root cause" for why this is build-dependent, not absent |

## Status

**Confirmed.** Deterministic panic, reproduces 3/3 runs in-tree plus 2/2 fresh-clone runs via
`repro/`'s own `just test` (5/5 total), same debug build, same input, no randomness involved (per
`CONFIRMATION.md` section 1/2 - this is a debug-assertions-only manifestation, explicitly noted
below, not release-build UB).

**Fully reproducible: Yes.** `repro/` calls the real, unmodified public API
(`Scaler::plan_rgba_resampling`) with plain `usize` values a real caller could pass directly - no
internal-only shim, no hand-authored bytes standing in for another participant's output. This is a
single-participant finding (a caller passing raw integers to a public constructor/entry point) -
`CONFIRMATION.md` section 11's "two participants" bar does not apply.

## Class

crash

## Root cause

`DefaultPlanner::plan_generic_resize` (`src/plan/default_planner.rs`) validates `destination_size`
for isize/overflow safety via `check_image_size_overflow` before building a plan, but never
validates `source_size` the same way (`AlphaPlanner::plan_generic_resize_with_alpha` has the
identical asymmetry). Both `source_size.width` and `source_size.height` are plain caller-supplied
`usize` values (via the fully public `ImageSize::new`, itself unvalidated) with no upper bound
enforced anywhere before they reach `T::make_weights` -> `generate_weights` in
`src/math/weights.rs`.

Inside `generate_weights`, `scale = in_size / out_size` (`in_size` = `source_size`'s corresponding
dimension) feeds `kernel_size = round(filter_base_size * scale.max(1))`, cast from float to
`usize` via a *saturating* cast (so `kernel_size` can reach values close to `usize::MAX` for a
sufficiently large `in_size`/`out_size` ratio, without itself overflowing anything). That
`kernel_size` is then combined with `out_size` via a **plain, unchecked `*` operator**
(`kernel_size * out_size`, line 179; the `is_area` branch at line ~310 has the identical shape
with a fixed `kernel_size = 2`) to size the weights `Vec` passed to this crate's own `try_vec!`
macro. `try_vec!` itself is written defensively (`Vec::try_reserve_exact` -> graceful
`PicScaleError::OutOfMemory` on failure, no panic) - but that safety net only ever sees the value
*after* the unchecked multiply has already happened, so an input that makes the multiply itself
overflow never reaches it: in a debug/`overflow-checks=true` build this panics before `try_vec!`
runs at all; in a default `release` build (`overflow-checks` off) it silently wraps, and the
crate's own `try_vec!` net catches the *wrapped* value for the specific inputs tried this session
(all of which happened to still wrap to something implausibly large) - but nothing about the
computation guarantees that in general, since wraparound can land on an arbitrarily small number
for a different `in_size`/`out_size` pair (not found within this session's time budget - see
"What's not done"). Every other place in this codebase that does equivalent size arithmetic on
caller-controlled dimensions (`check_image_size_overflow`/`check_image_size_overflow_with_stride`
in `src/support.rs`, used throughout `ImageStoreMut::validate`/`ImageStore::validate`) already uses
`checked_mul` specifically to avoid this - this one call site in `src/math/weights.rs` is the
outlier, reached before any store-level `.validate()` ever runs (plan-building happens from bare
`ImageSize` integers, no buffer involved yet).

This is a robustness gap in the plan-building phase, not the SIMD kernels the original backlog
suspicion named directly - by the time any unsafe SIMD code runs, a real `ImageStore`'s own
`.validate()` (checked `isize` arithmetic) has already run and would reject a store whose
`width`/`height` don't match a validated `source_size`, so this specific bug's blast radius is "the
crate panics where its own public API contract (`Result<_, PicScaleError>`) says it should return
an error" rather than memory unsafety.

## The fix

`src/math/weights.rs`, both call sites (`!is_area` branch, line 179; `is_area` branch, line ~311):
replace the unchecked `kernel_size * out_size` with `kernel_size.checked_mul(out_size)`, returning
`PicScaleError::OutOfMemory(usize::MAX)` on `None` instead of proceeding into the unchecked
multiply. This routes the exact same class of oversized input into the crate's existing,
already-graceful `OutOfMemory` error path instead of a panic - matching the `checked_mul` pattern
already used by `check_image_size_overflow` elsewhere in the codebase.

Verified:
- Re-ran `repro/`'s `just test` against the patched clone (see `output.txt`): the same
  `source_size=(1<<62, 1)`, `destination_size=(2, 1)` input that panicked before the fix now
  returns `Err(OutOfMemory(18446744073709551615))` - no panic, exit code 0.
- Re-ran the same fix against 3 more inputs in the same bug class (not just the one that first
  triggered it, per `CONFIRMATION.md` section 7): `destination_size=(3, 1)`,
  `source_size=usize::MAX` (`destination_size=(2, 1)`), and the same pair at
  `destination_size=(2, 1)` again for repeat-count confirmation - all now return `Err(OutOfMemory)`
  instead of panicking, in both debug and release builds, 3/3 each.
- Project test suite (`cargo test --lib`, default features): **66 passed, 0 failed** both before
  and after the fix (isolated via `git stash push -- src/math/weights.rs` / `git stash pop` around
  the two runs, all other working-tree state held constant). No regression; no pre-existing test's
  behavior changed.

## What's not done / open questions

- **Only `Scaler::plan_rgba_resampling` (Lanczos3) was used as the concrete trigger.** By code
  reading, every other `plan_*_resampling*` entry point in `src/scaler.rs` shares the exact same
  `source_size`-unchecked path through `DefaultPlanner::plan_generic_resize`/
  `AlphaPlanner::plan_generic_resize_with_alpha`, so the same bug class should be reachable through
  any of them (any pixel format, any bit depth) - not independently re-verified for each one this
  session.
- **A release-build (no `overflow-checks`) input that wraps the wrong-way (to a small value
  instead of a still-huge one) was not found this session.** The values tried all happened to wrap
  to something `try_reserve_exact` still correctly rejects as too large - but nothing in the code
  guarantees that in general, since the wrapped result is exactly whatever `(kernel_size * out_size)
  mod 2^64` happens to be for the chosen inputs. Whether such a value exists that's both reachable
  (given `kernel_size`'s own float-precision-limited granularity for huge `scale`) and small enough
  to make `try_vec!` allocate a plausible-looking but wrong-sized `Vec` - which could then feed a
  genuinely undersized buffer into the (safe-Rust, bounds-checked) code that fills it, most likely
  surfacing as a slice-index panic rather than memory unsafety, since nothing downstream of this
  specific function does raw unsafe indexing against it directly - was not determined; the
  `checked_mul` fix closes this class of input regardless of whether that particular sub-case is
  reachable.
- The initial `ImageStore::new`/`borrow`/`ImageStoreMut::new`/`borrow` unchecked-multiply
  observation (see "What was tried", first row) was traced through to `.validate()` and found not
  independently reachable as a bug - not fixed here since there is nothing to fix (the second,
  checked validation already covers it), but noted in case a future refactor removes or reorders
  that validation call.
