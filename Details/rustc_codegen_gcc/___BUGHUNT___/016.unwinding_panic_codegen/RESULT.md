# unwinding_panic_codegen

Unwinding/panic codegen (`Builder::invoke`, the cleanup-landing-pad/pending-cleanup-region
machinery in `src/builder.rs`/`src/context.rs`, and the `__rust_try` shim in
`src/intrinsic/mod.rs`) was flagged in this workspace's own `projects/rustc_codegen_gcc.md` as
"not yet explored at all" after two prior sessions that found several confirmed bugs elsewhere in
this codebase - worth a dedicated look since a codegen backend miscompiling unwind edges or
double-panic/abort semantics would be a real, easy-to-miss correctness class distinct from the
"wrong declared sign" family this workspace has mostly found so far.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of the unwind-specific codegen paths: `Builder::invoke` (both the `feature = "master"` try/catch-region path and the `not(feature = "master")` always-take-`then` fallback), `cleanup_landing_pad`/`resume`/`filter_landing_pad`, `CodegenCx::populate_cleanup_regions` (BFS-clones every block reachable from a landing pad into its own cleanup region), and `codegen_gnu_try`/`get_rust_try_fn` (the `__rust_try` shim `catch_unwind` compiles down to) | Reading only, current master (4cd7da5) | Identified the two live suspects this session actually dug into with real builds: (1) whether `populate_cleanup_regions`'s per-landing-pad BFS clone is still correct once several `invoke()` call sites in the same function share downstream cleanup blocks (a chain of several potentially-panicking calls with interleaved `Drop` locals - common, real MIR shape), and (2) whether `panic in a destructor while already unwinding` (a documented special case in libstd's own panic runtime, requires the backend's unwind machinery to actually let the second panic re-enter cleanly) matches real LLVM-backend behavior. `cleanup_pad`/`cleanup_ret`/`catch_pad`/`catch_switch`/`get_funclet_cleanuppad` are `unimplemented!()` but are the MSVC-SEH-only Landing-Pad API surface (`wants_msvc_seh` is checked and unconditionally not taken on this Linux/libgccjit target) - not exercised on this target, ruled out as not reachable here. |
| Differential testing (real LLVM-backend `rustc` vs. the project's own GCC-backend build, `dev` cargo profile, `lto = "off"`): `unwind_drop.rs` (3-level call chain, 2 locals per frame, nested `catch_unwind`, checks exact `Drop` order on unwind) | `rustc -O` vs. `CHANNEL=release ./y.sh cargo build --manifest-path repro/Cargo.toml --bins` (backend built once via `./y.sh build --release`), stdout diffed byte-for-byte | MATCH, byte-for-byte identical stdout, 2/2 runs |
| Same technique, `double_panic.rs` (a `Drop::drop` that itself panics while already unwinding from a first panic - libstd's own "panic in a destructor during cleanup" abort path) | Same build; checked exit code (134 = SIGABRT) and the final stderr lines ("panic in a destructor during cleanup" / "thread caused non-unwinding panic. aborting.") | MATCH, both LLVM and GCC backends abort with exit 134 and the identical final diagnostic, 2/2 runs |
| Same technique, `extern_c_unwind.rs` (a `panic!()` inside a `extern "C" fn`, which the compiler's own `AbortUnwindingCalls` MIR pass, not backend-specific codegen, must turn into an abort rather than letting the unwind cross the non-Rust-ABI boundary) | Same build; checked exit code (134) and final diagnostic ("panic in a function that cannot unwind" / "thread caused non-unwinding panic. aborting.") | MATCH, 2/2 runs |
| Same technique, `multi_unwind.rs` (a function with 4 sequential `#[inline(never)]` calls, each capable of panicking, each preceded by a fresh `Drop` local - directly exercises `populate_cleanup_regions`'s BFS with several `invoke()` landing pads whose downstream cleanup chains overlap/chain into each other), run once per possible panic trigger point (after call 1/2/3/4, and once with no panic at all) inside a `catch_unwind`, comparing full stdout across all 5 trigger values | Same build | MATCH, byte-for-byte identical stdout across all 5 trigger values, 2/2 runs |
| Same technique, `thread_unwind.rs` (a panic inside a `thread::spawn`-ed closure on a real second pthread stack, `JoinHandle::join` retrieving and downcasting the `Box<dyn Any + Send>` payload, plus a nested `catch_unwind`/`panic::resume_unwind` re-throwing an already-caught payload with a `Drop` guard live across the resume) | Same build | MATCH, byte-for-byte identical stdout, 2/2 runs |
| Repeated the full differential battery above with the backend-compiled test programs' own cargo profile changed from `opt-level = 0` (default `dev`) to `opt-level = 3` (`[profile.dev] opt-level = 3` in repro's own `Cargo.toml`, `lto` still `"off"`) - this project's own history (candidate 010) shows GCC's optimizer can reveal a bug an unoptimized build hides | Same backend build (backend `.so` itself unaffected - only the test programs' own optimization level changed), same 5 programs | MATCH on every program, byte-for-byte identical stdout / same exit codes as the `opt-level = 0` run |

## Status

**Not reproducible.** Covered: `Builder::invoke`'s try/catch-region path, the cleanup-landing-pad /
`resume` / pending-cleanup-region BFS machinery, and the `__rust_try` shim, via six differential
programs exercising nested `catch_unwind`, multi-frame `Drop` ordering during unwind, a chain of
several call sites sharing cleanup blocks, a double panic during destructor cleanup (must abort),
a panic crossing a non-Rust-ABI (`extern "C"`) boundary (must abort), and a panic on a real spawned
thread combined with `panic::resume_unwind` - each checked at both the test programs' `opt-level =
0` and `opt-level = 3`. Every program's stdout and exit code matched the real LLVM-backed rustc's
output byte-for-byte on every run (2/2 repeat runs per program per opt-level - deterministic, no
flakiness observed). Not covered (see "What's not done" below): MSVC/SEH-style unwinding
(`cleanup_pad`/`catch_pad`/`catch_switch`, all `unimplemented!()`) - not reachable on this
Linux/libgccjit target, so not a live gap here; the project's own already-tracked known-failing
tests that mention unwinding (`tests/ui/sepcomp/sepcomp-unwind.rs`,
`tests/run-make/foreign-double-unwind/`, `tests/ui/unwind-no-uwtable.rs`,
`tests/ui/extern/issue-64655-*.rs`, all listed in `tests/failing-*.txt`) were not independently
re-diagnosed this session - see "What's not done".

**Fully reproducible**: N/A (nothing reproduced - this field only applies to `Confirmed` findings).

## Class

N/A (Not reproducible).

## Root cause

Not established as a currently-live bug - no discrepancy found. The specific mechanism this
session worried about most (`CodegenCx::populate_cleanup_regions`'s BFS-based cloning of every
block reachable from a landing pad into that landing pad's own private cleanup region, done once
per `invoke()` call site with `self.cx.landing_pads.borrow().contains(&catch)`) turns out to
behave correctly even when several `invoke()` sites in the same function share downstream cleanup
blocks (the `multi_unwind.rs` case): each call site's own cleanup region gets its own independent
clone of the reachable subgraph, so overlapping/chained cleanup chains don't corrupt each other's
region or produce a wrong `Drop` order. Likewise, the special-cased "abort instead of unwind"
paths (`extern "C"` boundary, double panic during a destructor) are actually decided by libstd's
own panic runtime and rustc's `AbortUnwindingCalls` MIR pass rather than by backend-specific
codegen logic here, and this backend does not interfere with that decision reaching the runtime
correctly.

## Introduced by

N/A - not reproducible, no live bug to attribute to a commit.

## The fix

No fix - nothing to fix on current `master`. No `patch.diff` in this folder.

- Project test suite: N/A, no fix was applied (nothing reproduced), and no source change was made
  to the main tree during this investigation (all building/testing happened in an isolated git
  worktree at `___BUGHUNT___/_reference/wt-016`, discarded after this session, never merged; the
  main project's own working tree was never touched).

## What's not done / open questions

- The project's own `tests/failing-*.txt` lists several already-known-failing tests whose names
  mention unwinding (`tests/ui/sepcomp/sepcomp-unwind.rs`, `tests/run-make/foreign-double-unwind/`,
  `tests/ui/unwind-no-uwtable.rs`, `tests/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs`,
  `tests/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs`). These are pulled from
  rustc's own upstream `tests/ui`/`tests/run-make` trees by `./y.sh test`'s test-suite machinery,
  not present as files in this repository itself, so independently re-diagnosing *why* each one
  currently fails would require setting up that full upstream test-suite fetch (not done this
  session - out of scope for a single differential-testing pass) rather than being a quick
  follow-up. Since they're already tracked as known-failing rather than a surprise, this session
  treated them as out of scope rather than re-deriving already-known information, but a future
  session with that infrastructure set up could turn "known failing" into an actual root-caused
  `RESULT.md` for one of them.
- Did not check panic unwinding through SIMD-vectorized code, through `#[naked]`/inline-asm frames
  (`asm.rs`'s own `MAY_UNWIND` diagnostic already rejects inline asm with `may_unwind` outright, so
  that specific combination is intentionally rejected at compile time, not a runtime gap), or
  through generics/trait-object call chains specifically (this session's `multi_unwind.rs` used
  concrete `#[inline(never)]` functions, not a `dyn Trait`/generic call boundary crossing an
  unwind edge).
- Did not check unwinding interaction with `panic = "abort"` profiles (this session only exercised
  the default `panic = "unwind"` strategy) - `try_intrinsic`'s `!bx.sess().panic_strategy().unwinds()`
  branch (unconditionally returning `false`/never invoking `catch_func`) was read but not exercised
  with a real `panic = "abort"` build.
- Did not attempt `miri`/ASAN on this code (see this workspace's own `README.md`: this crate's
  `unsafe` is FFI-into-libgccjit, not something either tool fits well) - differential testing was
  judged the correct primary tool per `HEURISTICS.md`'s "logic bugs" row, and was used exhaustively
  (six distinct programs, two optimization levels, repeat-run each) rather than switched away from
  after a single negative.
