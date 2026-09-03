# pixarray Color2D::at/at_mut: GitHub issue #488's "hang" claim

Checks a third-party security researcher's report ([GitHub issue #488](https://github.com/dnglab/dnglab/issues/488),
still open) about `rawler/src/pixarray.rs`'s `Color2D::at`/`Color2D::at_mut` (currently lines
424-446): both use `get_unchecked`/`get_unchecked_mut` in release builds with bounds only checked
via `debug_assert!` in debug builds - the same `#[cfg(debug_assertions)]`-gated pattern already
tracked as a real bug class by candidate 001 (`Color2DPtr::at`, a related but distinct type in the
same file). This candidate's specific, narrower claim is that an out-of-bounds `Color2D::at` call
"may cause the program to hang indefinitely when running in release mode" by reading
garbage/adjacent memory that ends up used as loop-control data. The issue's own reporter explicitly
hedges this is speculative ("I'm not sure of the author's intentions... there's a high probability
I am overworried") and never actually ran their own PoC to observe a hang - it's a claim from
generic "unsound code" static scanning, not an observed symptom.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Reporter's exact PoC (`Color2D::<i32,1>::new(3,3)`, then `.at(3,3)` - width=3,height=3,row=height,col=width), called via the real public `rawler::pixarray::Color2D` API in a standalone `repro/` crate | release build (`overflow-checks=false`, `debug-assertions=false`, matching dnglab's own `[profile.release]`), literal/compile-time-constant indices, 3 repeat runs, `timeout 15` each | No hang. Deterministic **crash**, 3/3 runs, `Illegal instruction (core dumped)`, exit 132, ~0.1s wall time each - LLVM statically proves the `get_unchecked(12)` on a provably-9-element `Vec` is unreachable and compiles it to a trap instruction |
| Same call, but with `row`/`col`/`width`/`height` routed through `std::hint::black_box` (prevents the compiler from const-folding/proving the OOB-ness, matching how a real runtime-derived value would behave) - "modest" scenario, one row's worth past the end (row=3,col=3 on a 3x3 image) | release build, 3 repeat runs, `timeout 15` each | No hang. Silent **garbage read/write**, 3/3 runs, exit 0, sub-microsecond (`70-110ns` for the read, `20-41ns` for the write) - `get_unchecked` reads/writes adjacent heap slack without crashing or looping |
| Same, but with a far out-of-bounds `row` (1,000,000 on a 3-row image, `black_box`'d) - "far" scenario | release build, 3 repeat runs, `timeout 15` each | No hang. Deterministic **crash**, 3/3 runs, `Segmentation fault (core dumped)`, exit 139, ~0.1s wall time each |
| `cargo +nightly miri run` on the reporter's exact PoC (`[profile.dev] debug-assertions = false` added so miri's dev-profile build actually takes the `get_unchecked` branch instead of the `debug_assert!`-checked one) | miri, one run | Confirms real UB exists here (`error: Undefined Behavior: \`assume\` called with \`false\`` at `pixarray.rs:432`, the `get_unchecked` call) - but miri detects and reports it **instantly**, no hang; this is the same UB class candidate 001 already tracks (memory-safety, `debug_assert!`-only bounds in release), not a new "hang" mechanism |
| Manual read of every real call site of `Color2D::at`/`at_mut` in the codebase (`ppg.rs`, `bilinear.rs` x2, `iiq.rs`, `kdc.rs`, `rw2/v8decompressor.rs`, `radc.rs`, `fuji_decompressor.rs`, `fuji_rotate.rs`, `markesteijn.rs`, `raf.rs`, `xtrans/bilinear.rs` - full list in output.txt) | code reading, no build | None of these ~60 call sites use an `.at()`/`.at_mut()` return value as a loop bound, iteration count, or other control-flow value - every one either writes a computed pixel value or reads one for further arithmetic that itself feeds a pixel value. There is no code path in this codebase where a `Color2D::at`-sourced value controls a loop's iteration count, which is the specific mechanism this candidate's description hypothesizes |

All commands and their real captured output are in `output.txt`.

## Status

**Not reproducible** - specifically, the claimed "hang" behavior. Reproduces 3/3 runs each for
three different scenarios (literal, modest, far), and every single run is a fast crash or a fast
silent garbage access - never a hang, never even a slow run (all under ~0.2s wall time including
process/timeout overhead). `miri` independently confirms the underlying access is genuine UB, but
again reports it as an instant diagnostic, not a hang.

**Fully reproducible**: N/A - no `Confirmed` bug is being reported here.

## Class

N/A (Not reproducible).

## Root cause

The `#[cfg(debug_assertions)]`/`get_unchecked`-in-release pattern in `Color2D::at`/`at_mut` is
real (an out-of-bounds `row`/`col` is genuine, `miri`-confirmed undefined behavior in a release
build) - it is the same bug *class* candidate 001 already tracks in this file (`Color2DPtr::at`),
just on a sibling type (`Color2D` itself rather than its raw-pointer view `Color2DPtr`). What does
not hold up is this candidate's specific claim that it manifests as a **hang**:

- A single `get_unchecked`/`get_unchecked_mut` call is one unconditional memory dereference with
  no loop of any kind inside `at`/`at_mut` themselves - there is no mechanism internal to these
  two functions that could iterate indefinitely.
- Empirically, across three different out-of-bounds magnitudes (a few elements past the end, one
  full row past the end, and a million rows past the end), the actual observed behavior is always
  one of: (a) the compiler proves the access is always out of bounds for a given call site and
  emits an unconditional trap (`Illegal instruction`), (b) the access lands in unmapped memory and
  the OS delivers `SIGSEGV` immediately, or (c) the access lands in adjacent allocated heap memory
  and returns/overwrites garbage in well under a microsecond, no crash, no delay.
- For a hang to actually occur, some *other* code downstream of the OOB read would need to treat
  that garbage value as a loop-control input (an iteration count, a `while` condition, a recursion
  depth). A manual read of every real call site of `Color2D::at`/`at_mut` in this codebase (see
  "What was tried") found none that do this - every call site's return value feeds directly into a
  pixel-value computation or write, never into control flow.

The `#[cfg(debug_assertions)]`/`get_unchecked`-in-release pattern for `Color2D::at`/`at_mut` was
introduced in commit e22e3675 "Add PPG demosaic algorithm" (2022-04-21), later generalized from
`Color2D<T, 3>` to `Color2D<T, const N: usize>` in 33e1564a "Change Rgb2D<T> to Color2D<T, N> for
any color channel count" - noted for context only, not as an "Introduced by" entry, since that
field is reserved for `Confirmed` candidates and this one is `Not reproducible`.

## The fix

N/A - not applicable. No bounded fix was attempted this session: the specific claim under test (a
hang) does not reproduce, so there is nothing to fix for that claim. The underlying general
UB-in-release pattern is the same class already tracked by candidate 001 (which found and fixed an
analogous underflow feeding into `Color2DPtr::at`, a different type in the same file) - a
project-wide sweep of every `debug_assert!`-only-bounds-checked accessor (`Pix2D::at`/`at_mut`,
`Color2D::at`/`at_mut`, `Color2DPtr::at`) to add real bounds checks or `Result`-based fallible
accessors would be an architectural decision for the project maintainers, out of scope for a
single-candidate bounded fix, and is already noted in `IDEAS.md` from candidate 001's session.

- Project test suite: N/A - no source change was made, so there is nothing to verify against the
  test suite for this candidate.

## What's not done / open questions

- This candidate only checked whether an out-of-bounds `Color2D::at`/`at_mut` call can hang - it
  did not attempt to find a real, attacker-controlled file input that reaches `Color2D::at`/`at_mut`
  with an out-of-bounds `row`/`col` in the first place (unlike candidate 001, which found a concrete
  underflow reachable from a real DNG file). A quick read of the real call sites (`ppg.rs`,
  `bilinear.rs`, `iiq.rs`, `radc.rs`, `rw2/v8decompressor.rs`, `kdc.rs`, `fuji_decompressor.rs`,
  `fuji_rotate.rs`, `markesteijn.rs`) shows `row`/`col` values there are all computed from bounded
  loop counters (`enumerate()` over rows/cols matching the image's own `width`/`height`), not raw
  unvalidated file data - none looked like an obvious candidate for a real end-to-end OOB trigger,
  but this was not exhaustively fuzzed this session.
- The general `debug_assert!`-only-bounds pattern itself (distinct from this candidate's specific
  "hang" claim) remains a real, `miri`-confirmed UB class in this file, already tracked via
  candidate 001's `RESULT.md` and `IDEAS.md`'s note about other `Color2DPtr::at`/similar
  raw-pointer-based callers not yet checked for a concrete reachable trigger.
