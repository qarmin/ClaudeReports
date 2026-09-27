# UseSlice unsound blanket `unsafe impl Sync`

`src/use_var/use_slice.rs:8-24` has `unsafe impl<'a, T: 'a> Sync for UseSlice<'a, T> {}` with
no `T: Send` bound. `UseSlice` is reachable through the public, safe API `Par::use_slice` /
`ParOption::use_slice` / `ParResult::use_slice`, all of which only require `U: Sync` (not
`Send`). `Par::use_slice`'s executor (`ParRunnerInfallibleUse` in
`src/infallible_use/par_runner.rs`) hands each worker thread a `&mut T` for its own slice index
via `UseSlice::init_get`/`get`, using the crate's real thread pool (`BasicPool`,
`src/pool/pool_impl/basic.rs`), which spawns genuine, persistent `std::thread::spawn` OS worker
threads. Handing `&mut T` (exclusive access) to a thread other than the one that owns/created
`T` is exactly the operation `Send` exists to gate - so any `Sync + !Send` element type
(constructible by any downstream caller with one `unsafe impl Sync` of their own) can be pushed
across real OS thread boundaries without ever needing to be `Send`. Source: flagged in
`___FINDINGS___/findings.toml` (class: use_after_free, easy). Candidate slug in `IDEAS.md`:
"UseSlice unsound blanket unsafe impl Sync".

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: witness `Sync + !Send` type (`RcSync(Rc<Cell<i64>>)`, manual `unsafe impl Sync` only) driven through the crate's real public `Par::use_slice` API with `num_threads(4)` and a tight per-thread clone/drop loop on the shared, non-atomically-refcounted `Rc` | `cargo run --release`, `debug = true` in the release profile for readable backtraces, 4 worker threads x 300,000 iterations each | Reproduced: `Rc::strong_count` corrupted to nonsensical huge values (~1e14) in most runs, or the process crashes outright with `Illegal instruction (core dumped)` in others |
| `gdb -batch -ex run -ex bt` on the crashing case | Same binary, default release build | Confirmed the crash is real heap/control-flow corruption, not a benign panic: `SIGILL` inside the thread pool's own type-erased task runner (`Task::run` / `run_impl<W>` in `src/pool/pool_impl/basic.rs`), i.e. a `Box<W>` allocated on one thread was corrupted (by the concurrent, unsynchronized `Rc` refcount race) badly enough that another thread later executing through it hit a corrupted/garbage code path |
| Repeat-run check per `CONFIRMATION.md` section 1 | 10 consecutive runs of the same release binary, no fixed seed | 10/10 reproduced (mix of corrupted-count outputs and outright `SIGILL` crashes - see `output.txt`) |
| Fresh-clone reproducibility check via the candidate's own `justfile` (`just clone && just reproduce-before`) | 3 separate fresh clones of the exact `base_commit`, each rebuilt from scratch | 3/3 reproduced on the very first run each time (2 corrupted-count, 1 hard crash) - see `output.txt` |

No `miri`/ASAN escalation was needed - the observed effects (a numerically corrupted, physically
impossible `Rc::strong_count`, and a real `SIGILL` crash inside unrelated pool machinery with a
plausible corruption mechanism confirmed via `gdb`) already constitute a directly observable,
repeatable memory-safety violation without further instrumentation.

## Status

**Confirmed**. Reproduces 10/10 runs (release build, no fixed seed, no sanitizer needed) in the
original dev iteration, and 3/3 runs from a genuinely fresh clone of `base_commit` via the
candidate's own `justfile`. The specific numeric corruption values differ every run (as expected
for a real data race, no fixed seed), but every run shows either a corrupted, non-physical
`Rc::strong_count` or an outright crash - never the correct, race-free result.

**Fully reproducible**: **Yes**. `repro/` drives the crate's real, public `Par::use_slice` entry
point (not a hand-built stand-in for it), executed by the crate's own real default thread pool
spawning genuine OS worker threads (`BasicPool`, `std::thread::spawn`). This is a single-participant
finding (how the crate's own generic API reacts to a caller-supplied type parameter), not an
interaction between two independent programs/peers, so CONFIRMATION.md section 11's "real second
participant" requirement does not apply here (see that section's own carve-out: "a malformed
[input] handed to [a real API]... is the genuine external input; there's no 'other side' being
impersonated"). The witness type `RcSync` is not a fake peer - it is exactly the kind of caller-
supplied `T` the public `use_slice::<U>` API is documented to accept.

## Class

memory-safety (data race / use-after-free-adjacent heap corruption via an unsynchronized,
non-atomic `Rc` refcount shared across real OS threads by a soundness hole in a generic bound)

## Root cause

`Par::use_slice<'a, U>` (`src/infallible/par.rs:579`, and its `ParOption`/`ParResult` siblings)
only requires `U: Sync + 'a`. It builds a `UseSlice::new(slice)` and hands it to the executor,
which (in the parallel, non-sequential path of every operation in
`src/infallible_use/par_runner.rs`, e.g. `collect`/`reduce`/`next`/`next_any`/`collect_arb`) calls
`self.pool_mut().scoped_computation(...)` and spawns one real `ParThreadPool::run_in_scope` task
per worker thread index. Each task calls `u.init_get(th_idx)` on the shared `&UseSlice`, which
(`src/use_var/use_slice.rs:29-36`) does `unsafe { &mut *self.ptr.add(th_idx) }` - handing that
worker thread exclusive mutable access to the `T` value that was originally constructed
(allocated/initialized) on whatever thread built the `&mut [T]` slice passed into `use_slice`.

That is precisely the operation Rust's `Send` marker trait exists to gate: moving/using a value
from a thread other than the one that produced it. `UseSlice<'a, T>`'s own `unsafe impl Sync`
(`src/use_var/use_slice.rs:23`) has no `T: Send` bound, so it does not actually need `T` to be
safe to hand across threads this way - it only requires that `&UseSlice` itself be shareable
(which is true regardless of `T`, since `UseSlice` itself only stores a raw pointer + length).
This lets any caller plug in a `T` that is `Sync` (satisfying the public API's stated bound) but
`!Send` and whose soundness genuinely depends on single-thread-only mutation/ownership transfer
(the textbook example being `Rc<_>`'s non-atomic reference count) - and the crate will still
happily hand `&mut T` to a different real OS thread than the one that created it, racing on any
internal state `T` was relying on being thread-confined.

`UseVec` (`src/use_var/use_vec.rs`), the sibling "owned worker-local state" type, already gets
this right: its `impl<T: Send, ...> Use for UseVec<T, F>` requires `T: Send`. `UseSlice` never
had the equivalent bound.

## Introduced by

e36facd2 "define UseSlice" (2026-05-30). `git log --follow` on the (later renamed/reorganized)
file shows this commit introduced the type at its original path
(`src/infallible_use/use_var/use_slice.rs`) with the exact same unbounded
`unsafe impl<'a, T: 'a> Sync for UseSlice<'a, T> {}` already present - the bug has existed since
the type's first introduction and was carried through every later rename/reorganize/refactor
untouched.

## The fix

Add a `T: Send` bound everywhere `UseSlice<'a, T>`'s soundness actually depends on it, mirroring
`UseVec`'s existing `T: Send` bound:

- `src/use_var/use_slice.rs`: `unsafe impl<'a, T: 'a> Sync for UseSlice<'a, T> {}` -> `unsafe impl<'a, T: Send + 'a> Sync for UseSlice<'a, T> {}`, and `impl<'a, T: 'a> Use for UseSlice<'a, T>` -> `impl<'a, T: Send + 'a> Use for UseSlice<'a, T>` (required transitively, since `Use: Sync`).
- `src/infallible/par.rs`, `src/option/par.rs`, `src/result/par.rs`: each `use_slice`'s `where U: Sync + 'a` -> `where U: Send + Sync + 'a`, so the public API's own bound reflects the real requirement instead of relying on a caller happening to notice `UseSlice` wouldn't compile otherwise (which is the state it's in without this change - the internal `Use` impl requiring `Send` while the public entry point still only demanded `Sync` would be a confusing compile error pointing at the wrong bound).

Total diff: +5/-5 lines across 4 files (`patch.diff` in this folder).

Verified: after applying `patch.diff` to `src/`, the exact same `repro/` (the `RcSync` witness
type) no longer compiles at all against the patched crate - `error[E0277]: Rc<Cell<i64>> cannot
be sent between threads safely`, pointing directly at the new `U: Send + Sync + 'a` bound. This
is the correct "fixed" outcome for this class of bug: the unsound plug-in is now rejected at
compile time rather than needing to be caught at runtime.

Also checked a second input in the same bug class per CONFIRMATION.md section 7: the same witness
type driven through `.reduce(...)` and `.collect(...)` instead of `.for_each(...)` (both also
route through `UseSlice` in `src/infallible_use/par_runner.rs`) is rejected by the patched bound
in exactly the same way (same `E0277`), since the fix is at the type level (`UseSlice`/`Use`
impls + the public bound), not tied to any specific downstream operation.

- Project test suite: `cargo test --release` in the worktree - **260 passed; 0 failed; 16
  ignored** before the patch, and **260 passed; 0 failed; 16 ignored** after the patch (identical
  counts; no existing test constructs a non-`Send` `use_slice` element, so nothing broke).

## What's not done / open questions

- `UseVec` (`src/use_var/use_vec.rs`) was checked and already requires `T: Send` correctly - it
  is not affected by this bug class.
- Not checked: whether any other `Use`-trait implementor added in the future could reintroduce
  this same class of bound gap; no automated check (e.g. a `Sync`-but-not-`Send` auto-trait
  lint) was added to prevent a recurrence - a maintainer might want a compile-fail test
  (`trybuild`) asserting `use_slice` rejects a `!Send` element type, to guard against a future
  regression of this exact bound.
- The observed crash (`SIGILL` inside the thread pool's generic task runner) was root-caused via
  `gdb` to the general vicinity of heap corruption reaching unrelated pool internals, but the
  exact byte-level corruption mechanism (which specific freed/reused allocation got clobbered)
  was not traced further - not needed for confirmation, since the corrupted-`strong_count`
  evidence alone (also reproduced independently in the same runs) is unambiguous.
- A longer, dedicated fuzzing/loom campaign was not run (out of scope for this candidate's soft
  3-approach cap) and was not necessary here - the race is essentially guaranteed to manifest
  given `Rc`'s well-documented non-atomic refcount and real concurrent access from `n_threads`
  real OS threads with no synchronization at all.
