# dash-empty-pattern-panic

The public, documented entry point `kurbo::dash(inner, dash_offset, dashes: &[f64])`
(`kurbo/src/stroke.rs:717`) delegates to `dash_iter`, which unconditionally indexes
`dashes[dash_ix]` (`dash_ix` starts at `0`) at `kurbo/src/stroke.rs:744` with no check that
`dashes` is non-empty. The sibling internal call site `stroke_with` (`kurbo/src/stroke.rs:288`)
guards this exact code path with `if style.dash_pattern.is_empty() { .. } else { dash_iter(..) }`,
but the standalone `dash()` entry point has no equivalent guard, so any caller passing an empty
dash-pattern slice panics.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug build | `cargo run` (debug), standalone repro calling `kurbo::dash(path.into_iter(), 0.0, &[])` on a real public `PathEl` sequence | Panicked immediately: `index out of bounds: the len is 0 but the index is 0` at `kurbo/src/stroke.rs:744:30` |
| Same repro, release build | `cargo run --release` | Panicked identically (this is a plain slice-index bounds check, not gated by `debug-assertions`) |
| Repeat runs (debug) | 4 consecutive `cargo run` invocations, same input | Panicked 4/4 |
| Input variation (post-fix verification) | Closed path with a positive dash offset (3.5), a negative dash offset (-2.0), and an empty path iterator, all with `dashes: &[]` | Before the fix: not separately tested (the unconditional index at line 744 fires identically regardless of these variations, since it happens before any of that state is used); after the fix: all three completed normally, confirming the fix isn't narrow to the exact original input |

No further escalation (miri/ASAN/fuzzing) was needed or attempted - this is a plain, deterministic,
unconditional out-of-bounds slice index reachable by construction, not something that needs
instrumentation to observe.

## Status

**Confirmed**. Reproduces 5/5 runs (4 debug + 1 release), bit-for-bit identical panic message and
location every time, no seed/layout dependency (it's a straight `dashes[0]` index on a
statically-known-empty slice).

**Fully reproducible**: **Yes**. The repro calls the real public API (`kurbo::dash`) directly with
a real `PathEl` sequence built the same way any caller would build one - there is no second
participant to fake here (the "input" is a plain `&[f64]` function argument the caller supplies
directly, not something produced by a peer/upstream caller over some channel), so a
single-process, single-call repro is the correct shape per `CONFIRMATION.md` section 11's own
carve-out for single-artifact input.

## Class

crash

## Root cause

`dash_iter` (`kurbo/src/stroke.rs:725`) begins by computing the dash period as
`dashes.iter().sum()` and normalizing `dash_offset` via `.rem_euclid(period)`, then immediately
does:

```rust
let mut dash_ix = 0;
let mut dash_remaining = dashes[dash_ix] - dash_offset;
```

This indexes `dashes[0]` unconditionally, before anything checks whether `dashes` is non-empty.
The only caller that goes through this safely is `stroke_with`, which gates the call behind
`if style.dash_pattern.is_empty() { stroke_undashed(..) } else { dash_iter(..) }` - `dash_iter` is
simply never invoked with an empty pattern from that path. The standalone public `dash()` function
(`kurbo/src/stroke.rs:717`), which exists specifically so callers can use the dashing iterator
"separately" from full stroke expansion (per its own doc comment), calls `dash_iter` directly with
no equivalent guard, so any caller passing `&[]` (a perfectly ordinary "no dashes" empty slice)
panics instead of getting a no-op/passthrough.

## Introduced by

e234d26 "Don't skip first dash in dash pattern (#353)" (2024-08-17).

Before this commit, `dash_impl`'s init loop was `let mut dash_remaining = dash_offset; while
dash_remaining > 0.0 { let dash_len = dashes[dash_ix]; .. }` - with the crate's own default
`dash_offset` of `0.0`, that `while` condition is false on entry, so `dashes[dash_ix]` was never
evaluated at all for the common "no offset" case (verified by reading `dash_impl` at
`e234d26~1:src/stroke.rs`). This commit rewrote the init to `let mut dash_remaining =
dashes[dash_ix] - dash_offset;`, which evaluates `dashes[0]` unconditionally regardless of
`dash_offset`, making the panic on an empty slice unconditional rather than latent. The standalone
`pub fn dash()` entry point itself was added earlier, in 431e59c "Change signature to function"
(2023-10-19), but per the above it wasn't reachable via the `dash_offset = 0.0` default until
e234d26's rewrite.

## The fix

Made the empty-`dashes` case behave the same way through `dash()` as it already does through
`stroke_with`/`stroke`: "no dashing" - the input path is passed through unchanged instead of
panicking.

Concretely: `dash()` now checks `dashes.is_empty()` and, if so, returns the original `inner`
iterator untouched instead of calling `dash_iter`. Since the `if`/`else` branches then produce two
different concrete iterator types (a plain passthrough vs. a `DashIterator`), both are wrapped in a
small local `MaybeDashed<'a, T>` enum (`Dashed(Box<DashIterator<'a, T>>)` /
`Passthrough(T)`) implementing `Iterator` by delegating to whichever variant is active, so `dash()`
keeps its existing `-> impl Iterator<Item = PathEl> + 'a` return signature (source-compatible for
all existing callers). `dash_iter`'s own signature changed from an `impl Trait` parameter to an
explicit generic `T: Iterator<Item = PathEl> + 'a` so both branches of `dash()` share the same
underlying type parameter (required for the two arms of the `if`/`else` to unify to one
`MaybeDashed<T>` type) - this is an internal (non-`pub`) function, so it has no API-compatibility
implications. `DashIterator` is boxed inside the `Dashed` variant to avoid `clippy::
large_enum_variant` (`DashIterator` is >200 bytes; CI runs with `-D warnings`).

Verified:
- Re-ran the standalone repro (`kurbo::dash(path.into_iter(), 0.0, &[])`) against the patched
  tree: no longer panics, returns the original `[MoveTo((0.0, 0.0)), LineTo((10.0, 0.0))]` path
  unchanged - re-run 4 times, consistent.
- Varied the input within the same bug class (still empty `dashes`, but: a closed path with a
  positive dash offset, a closed path with a negative dash offset, and an empty `PathEl` iterator)
  - all completed normally against the patched tree (see "What was tried" table).
- `cargo build -p kurbo` and `cargo clippy -p kurbo --all-features --all-targets` are both clean
  (no new warnings; the project's CI runs clippy with `-D warnings`).
- Project test suite (`cargo test -p kurbo --all-features`): **187 unit tests + 55 doctests
  passed, 0 failed, both before and after** the patch - identical counts, no regressions, no new
  failures.

## What's not done / open questions

- Only the standalone `dash()` entry point had this specific unguarded-index bug; `stroke`/
  `stroke_with` were already checked and are unaffected (they guard the same `dash_iter` call with
  `dash_pattern.is_empty()`). No other call site of `dash_iter` exists in the crate.
- Not checked: whether an empty `dashes` slice combined with a non-empty-but-effectively-zero
  pattern (e.g. `&[0.0, 0.0]`, giving `period == 0.0`) has a separate, unrelated issue (a `NaN`
  from `.rem_euclid(0.0)` feeding back into the same `while dash_remaining < 0.0` loop) - out of
  scope for this candidate, which is specifically about the empty-slice panic, but worth a
  follow-up look since the fix here doesn't touch that path at all.
