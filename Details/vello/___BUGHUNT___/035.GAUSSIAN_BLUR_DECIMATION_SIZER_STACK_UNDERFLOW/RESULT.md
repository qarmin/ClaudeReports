# GAUSSIAN_BLUR_DECIMATION_SIZER_STACK_UNDERFLOW

`vello_common::filter::gaussian_blur::DecimationSizer::upscale` (`sparse_strips/vello_common/src/filter/gaussian_blur.rs:211`)
does `self.dim_stack.pop().unwrap()`, assuming every `upscale()` call is preceded by a matching
`downscale()` push. `DecimationSizer` and both methods are fully `pub` (the struct is declared in
`pub mod gaussian_blur` under `pub mod filter` in `vello_common`'s crate root, and `vello_common`
has no `publish = false`), so this is reachable from any downstream crate calling the real public
API in an order the type system doesn't prevent, not just from the two internal call sites
(`vello_cpu`'s `apply_blur` and `vello_hybrid`'s `FilterPassBuilder::emit_blur_sequence`) - both of
which happen to keep `downscale`/`upscale` balanced via matched `0..n_decimations` loops and so
never hit this themselves.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: call the public `vello_common::filter::gaussian_blur::DecimationSizer::new` + `.upscale()` directly, with no preceding `.downscale()` call, via a standalone crate using only the public API | debug build (`cargo test`, in-tree) and release build (`cargo run --release`, standalone `repro/` against a fresh clone) | Panics deterministically, 3/3 runs (debug, in-tree) plus 1/1 release run against a fresh clone: `called \`Option::unwrap()\` on a \`None\` value` at gaussian_blur.rs:211 |
| Same, but with one balanced `downscale()`/`upscale()` pair first, then one extra `upscale()` beyond it (a different call sequence in the same bug class, per `CONFIRMATION.md` section 7) | debug build (in-tree `#[should_panic]` test) and release build (standalone `repro/`) | Panics identically, same code path, same message shape |

Both approaches use the real, public entry point (`DecimationSizer::new` + `.downscale()`/`.upscale()`)
with no fabricated internal state - see "Fully reproducible" below.

## Status

**Confirmed**, reproduces 3/3 runs in a debug build (in-tree `cargo test`) and 1/1 in a release
build against a freshly cloned checkout (standalone `repro/`), for both call-sequence variants
above. No layout/seed dependency - fully deterministic (a plain state-machine/arity bug, not
memory-layout-dependent UB).

**Fully reproducible**: **Yes**. `repro/src/main.rs` calls only `vello_common`'s real public API
(`DecimationSizer::new`, `.downscale()`, `.upscale()`) exactly the way a downstream crate would -
there is no second participant or wire format involved here: the "external input" this bug is
about is simply a caller invoking a public method in an order the type system doesn't prevent, and
this repro does exactly that directly, with nothing to fake.

## Class

crash

## Root cause

`DecimationSizer` tracks a stack of "target dimensions to clamp back to" pushed by every
`downscale()` call, popped by the matching `upscale()`. `upscale()` unconditionally does
`self.dim_stack.pop().unwrap()`, assuming the stack is never empty at that point. Nothing in the
type - a plain struct with two independent `pub fn`s and no phantom-type/builder pattern encoding
"downscale must precede upscale" - prevents a caller from invoking `.upscale()` immediately after
`.new()`, or from calling it one time more than the number of prior `.downscale()` calls; both
sequences pop an empty `Vec` and panic with a bare, uninformative `Option::unwrap()` message and no
`# Panics` documentation on the public method.

The two current internal call sites (`vello_cpu::filter::gaussian_blur::apply_blur` and
`vello_hybrid::filter::FilterPassBuilder::emit_blur_sequence`) both happen to drive `downscale`/
`upscale` through matched `0..n_decimations` loops derived from the same `n_decimations` value, so
neither internal caller currently triggers this - the bug is exclusively about the type's exposed
public contract, not about any currently-reachable internal misuse.

## Introduced by

3a0908d6 "vello_hybrid: Initial support for filter effects (#1494)" (2026-03-18) - this is the
commit that introduces `DecimationSizer` and the unchecked `dim_stack.pop().unwrap()` in
`upscale()` (found via `git log -S "dim_stack.pop().unwrap()"` on this file, single matching
commit).

## The fix

Replaced the bare `.unwrap()` with `.expect("upscale() called without a matching downscale()")`
and added a `# Panics` doc section to `DecimationSizer::upscale`, matching the same "document and
clarify the panic-on-misuse contract, don't silently paper over caller error" approach used for the
analogous `pop_layer` stack-underflow bug in `031.RECORD_POP_LAYER_STACK_UNDERFLOW`. Turning this
into a `Result`-returning API, or otherwise making `upscale()` a no-op / tolerant of the mismatch,
was judged out of scope for a bounded fix: `DecimationSizer` is an internal bookkeeping helper for a
specific two-phase (`downscale*` then `upscale*`) usage pattern, and silently tolerating a mismatch
would hide a real caller bug (returning made-up dimensions) rather than surfacing it - the same
reasoning `031`'s `RESULT.md` used for `pop_layer`.

Before/after (see `output.txt` for the full captured output of both variants against a fresh clone,
via the `repro/`+`patch.diff` recipe in this folder's `justfile`):

- Before: `thread 'main' panicked at .../gaussian_blur.rs:211:57: called \`Option::unwrap()\` on a \`None\` value`
- After: `thread 'main' panicked at .../gaussian_blur.rs:219:14: upscale() called without a matching downscale()`

Checked one more input in the same class (per `CONFIRMATION.md` section 7): a balanced
`downscale()`/`upscale()` pair followed by one extra `upscale()` (rather than no `downscale()` at
all) hits the identical code path and panics with the identical new message - the fix is not narrow
to the specific "zero downscales" input that was first tried.

- Project test suite (run once against the pristine tree, once with `patch.diff` applied, both in
  the main project's own checkout, `git checkout --` between the two - not the standalone `repro/`
  clone):
  - `cargo test -p vello_common --lib`: 159 passed / 0 failed, both before and after.
  - `cargo test -p vello_cpu --lib`: 81 passed / 0 failed, both before and after.
  - `cargo test -p vello_hybrid --lib`: 122 passed / 0 failed, both before and after.

## What's not done / open questions

- Whether `DecimationSizer`'s public API should instead be redesigned (e.g. a builder/phantom-type
  encoding that makes an unmatched `upscale()` a compile error, or a `Result`-returning method so a
  downstream caller can recover) is a public-API design decision for the maintainers, not attempted
  here - the bounded fix keeps the existing panic-on-misuse contract, just documents and clarifies
  it, exactly as `031`'s fix did for the analogous `pop_layer` case.
- Only the two currently-existing internal call sites (`vello_cpu::apply_blur`,
  `vello_hybrid::FilterPassBuilder::emit_blur_sequence`) were checked for whether they can reach
  this themselves; both were confirmed by code reading to keep `downscale`/`upscale` counts
  balanced via the same `n_decimations` value driving both loops, so this candidate's finding is
  specifically about the type's public-API misuse surface for any downstream crate, not about a
  reachable-today internal caller bug.
