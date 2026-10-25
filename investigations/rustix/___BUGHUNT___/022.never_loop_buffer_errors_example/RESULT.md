# never_loop in examples/buffer_errors.rs::allocate_loop

`examples/buffer_errors.rs` demonstrates how to fix a set of closure/borrow-checker errors that
arise when using the `Buffer` trait. One of its helper functions, `allocate_loop` (inside
`error_retry_indirect_closure`), is commented as "reduced from the xattr crate, src/sys/
linux_macos.rs line 119" and is meant to illustrate the real-world "probe required size, then
fill, retrying if the size changed between calls" pattern. This looked suspicious because the
`loop { ... }` body always returns unconditionally on its first pass (both arms of the inner
`match`, plus the leading `?`, return out of the function), so the retry it's meant to model can
never actually happen - exactly the shape `clippy::correctness`'s deny-by-default `never_loop`
lint exists to catch.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo clippy --example buffer_errors --features all-apis` | stable 1.98.1, this container's existing toolchain, run 3x in a row | Fails every time with `error: this loop never actually loops` (clippy::never_loop, deny-by-default); exit code 101. 3/3 runs identical. |
| `cargo build --example buffer_errors --features all-apis` (no clippy) | same toolchain | Succeeds - plain `rustc` has no equivalent lint, so only `cargo clippy` is affected, not a normal build/test. |
| `cargo test --workspace` (before and after the fix) | same toolchain | 22 passed / 0 failed / 1 ignored, identical before and after - the example file isn't exercised by the test suite at all, so the fix can't regress it. |
| `grep -n clippy .github/workflows/main.yml` | - | No match - confirms the stated root cause that CI has no clippy job, so this has shipped unnoticed. |
| `git log --follow` / `git show <rev>:examples/buffer_errors.rs` on the file | - | The `loop { ... }` shape is present verbatim in the commit that introduced the file; never changed since. |

No further tools were needed - this is a deterministic, single-process static-analysis result
(clippy analyzing fixed source text), not a runtime/input-dependent bug, so a debug build, miri,
ASAN, or a fuzz harness have nothing applicable to add here (see "Fully reproducible" below for
why `CONFIRMATION.md` section 11 doesn't apply to this candidate either).

## Status

**Confirmed**. Reproduces 3/3 runs against the unmodified tree in this environment, and
independently re-confirmed in a fresh clone of the exact `base_commit` via this candidate's own
`just test` (see `output.txt`, "BEFORE patch" section) - fully deterministic, no flakiness, no
seed/layout dependency (it's a static lint over fixed source text, not a runtime condition).

**Fully reproducible**: **Yes**. The repro runs the real, unmodified `cargo clippy` command
against the real, unmodified example file in a real clone of the project at `base_commit` - there
is no second participant to fake here (`CONFIRMATION.md` section 11 is about one participant's
reaction to another's runtime-produced input; this finding is a single deterministic static-
analysis tool reading fixed source text, with no runtime input or peer/caller involved at all).

## Class

logic (secondarily: tooling/CI-gap - the lint failure itself is 100% deterministic and would be
caught instantly by any clippy-including CI job, but none exists, per `.github/workflows/main.yml`
having no clippy step)

## Root cause

`allocate_loop`'s body is:

```rust
loop {
    let ret = f(&mut [])?;       // propagates on Err, falls through to the next line on Ok
    vec.resize(ret, 0);

    match f(&mut vec) {
        Ok(size) => { ...; return Ok(vec); }  // returns
        Err(err) => return Err(err),          // returns
    }
}
```

Every path out of the loop body returns from the function on the very first iteration: the `?`
after the first `f(&mut [])` call exits on `Err`, and *both* arms of the `match` on the second
`f(&mut vec)` call return unconditionally. There is no `continue`, and no code path that falls
through to the top of the loop again - so the `loop` can only ever run once, which is precisely
what `clippy::correctness`'s `never_loop` lint (deny-by-default since it's a correctness lint, not
merely a style one) flags as a bug: a `loop` that structurally cannot loop almost always indicates
the author intended retry semantics that the code doesn't actually implement. That is exactly the
case here - the comment explicitly frames this as modeling "retrying if size changed between
calls" from the real `xattr` crate, but the mock functions `f`/`g` in this example always return
`Ok(0)` and there was never a code path that would actually retry even if they didn't (e.g. no
check for an `ERANGE`-style error from the second call that would legitimately warrant looping
back). The retry semantics were aspirational/illustrative text, not implemented control flow.

This is a `cargo clippy --examples` (or `--workspace --examples`) compile failure, not a runtime
bug: a plain `cargo build`/`cargo test` never invokes this lint, so the example always ran
correctly - the failure is specifically "this source cannot be compiled by clippy at all", visible
only to a clippy-enabled toolchain/CI job.

## Introduced by

a7685238e0b7e6e9cc3a20a53b9ebeb3c3467a2d "Add more documentation about error messages. (#1356)"
(2025-02-27) - this is the commit that added `examples/buffer_errors.rs` (confirmed via `git log
--follow --diff-filter=A`); `allocate_loop`'s `loop { ... }` body already had this exact
never-actually-loops shape in that very first version of the file (confirmed via `git show
a7685238:examples/buffer_errors.rs`), and was carried through two later doc-only touch-ups
(7592cd60, f26edabb) unchanged.

## The fix

Per the candidate's stated fix directions, replaced the structurally-broken `loop` with a single
pass that reflects the code's actual (never-retrying) behavior, rather than inventing new retry
logic the mock `f`/`g` functions have no way to exercise or verify:

```rust
fn allocate_loop<F: FnMut(&mut [u8]) -> io::Result<usize>>(mut f: F) -> io::Result<Vec<u8>> {
    let mut vec: Vec<u8> = Vec::new();
    let ret = f(&mut [])?;
    vec.resize(ret, 0);

    let size = f(&mut vec)?;
    vec.truncate(size);
    vec.shrink_to_fit();
    Ok(vec)
}
```

This keeps the function's actual observable behavior completely unchanged (it never retried in
practice anyway, since every path returned on the first pass) while making the control flow
honestly match what the code does - no more bare `loop` implying a retry that can't happen. It
also happens to match clippy's own suggested rewrite for the `question_mark` warning that was
already being emitted on the same block (visible in the "BEFORE" clippy output in `output.txt`),
so the fix isn't inventing a new shape, just applying the rewrite clippy itself already proposed,
one level up (removing the enclosing `loop` as well as flattening the `match`).

The alternative direction (make the loop actually retry) was not taken: there is no real condition
in this example to retry on - `f`/`g` are fixed mock functions that always return `Ok(0)`, so any
invented retry condition (e.g. "retry if an `ERANGE`-shaped error comes back") would be untested,
unreachable code in the example, which would be actively misleading in a file whose whole purpose
is to show working, exercised patterns.

Verified:
- `cargo clippy --example buffer_errors --features all-apis` after the patch: exit code 0, no
  `never_loop` diagnostic (3/3 runs; also independently re-confirmed via `just test` in a fresh
  clone, see `output.txt`'s "AFTER patch" section).
- Project test suite: `cargo test --workspace` - **22 passed / 0 failed / 1 ignored before, 22
  passed / 0 failed / 1 ignored after** (identical; the example file isn't part of the test
  suite, so no regression was possible, but the suite was still run both ways per the candidate
  loop's mandatory step 7).

## What's not done / open questions

- No CI job runs `cargo clippy` anywhere in this repository (`.github/workflows/main.yml` has no
  clippy step) - this is why the bug shipped unnoticed since 2025-02-27. Adding a clippy CI job
  is a separate, broader change than this one bounded source fix and was left to the project
  maintainers to decide (it would likely surface the file's other, non-deny-by-default warnings
  too - `unnecessary_literal_unwrap` at line 84 and the pre-existing `wrong_self_convention`
  warning in `src/timespec.rs` - which are out of scope for this candidate).
  deny-by-default `never_loop` is correctness-level clippy, enough on its own to justify at least
  `cargo clippy --examples` in CI, without necessarily taking on every pedantic/style lint too.
- Swept the rest of the crate with `cargo clippy --all-targets --features all-apis`: the only
  `error:`-level diagnostic anywhere in the crate is this one `never_loop` hit - no sibling
  instance of the same control-flow shape exists elsewhere in `src/` or `examples/`. (This sweep
  does surface other, non-deny-by-default `warning:`-level lints across the crate that were not
  investigated or fixed here, out of scope for this one candidate.)
