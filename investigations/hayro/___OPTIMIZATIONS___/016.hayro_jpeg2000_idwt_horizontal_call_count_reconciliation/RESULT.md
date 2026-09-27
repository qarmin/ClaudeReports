# JPEG2000 IDWT horizontal filter: reconciling candidate 013's 92-call count against candidate 014's 28-call re-measurement

Follow-up to `013` (found `irreversible_filter_97i_horizontal_simd::<Avx512>` - candidate 002's
never-applied deinterleave/SIMD rewrite - compiles to 92 real out-of-line
`__fearless_simd_kernel` calls instead of inlining) and `014` (re-measured the identical unsplit
function, same session, and got only 28 - a ~3.3x discrepancy neither candidate root-caused).
Per `IDEAS.md`'s backlog entry, this candidate reconciles that discrepancy before any further
restructuring-based fix is attempted.

**No patch to real source is proposed here** - like 013/014, this is a pure
measurement/characterization. `patch.diff` is identical to candidate 013's own patch (candidate
002's rewrite + 013's temporary `#[cfg(test)] mod suspect_investigation` diagnostic module),
reapplied and then reverted from the real working tree before this folder was written (`git
status` on the main project is clean).

## What was tried

Both of 013's and 014's own call-count measurements were re-run **in this session, on this
machine's current toolchain**, using each candidate's *own* original methodology as closely as
possible, to isolate which of the two candidates' methodological differences (if either)
explains the discrepancy - rather than guessing from the RESULT.md text alone:

1. **013's own methodology**: apply 013's full patch (002's rewrite + the diagnostic test module,
   in the same source file) directly to `hayro-jpeg2000/src/j2c/idwt.rs`, then build via
   `cargo test -p hayro-jpeg2000 --release --lib --no-run` (in-workspace test binary, default
   release profile, `cargo clean -p hayro-jpeg2000 --release` first to force a from-scratch
   build). Counted real calls in the compiled test binary via `objdump -d --demangle`.
2. **013's *other* own methodology** (013's RESULT.md states its 92-call number actually came
   from building candidate 002's standalone `bench/idwt/` crate, not the test binary): with the
   same patch still applied to the real source, pointed candidate 002's own preserved
   `bench/idwt/Cargo.toml` path-dependency at the live, patched `hayro-jpeg2000` checkout,
   deleted its `Cargo.lock` to force fresh dependency resolution, and built fresh
   (`cargo build --release`, `[profile.release] debug = 1`, exactly as 002/013/014 all used).
   Counted real calls the same way.

| Build method | `fearless_simd` version resolved | Real `__fearless_simd_kernel` calls in `irreversible_filter_97i_horizontal_simd::<Avx512>` |
|---|---|---|
| `cargo test -p hayro-jpeg2000 --release --lib` (in-workspace, diagnostic module present, matching 013's source-level setup) | 0.6.0 (fresh `cargo clean -p hayro-jpeg2000` rebuild) | **28** |
| Candidate 002's own `bench/idwt/` crate (external path-dependency, matching 013's *stated* measurement source for the 92 figure) | 0.6.0 (fresh `Cargo.lock`, `Compiling fearless_simd v0.6.0` on this build) | **28** |

Both binaries' extracted disassembly for the function is 2237 lines long in each case; after
normalizing away absolute addresses and per-object `.LCPIn_m` constant-pool label numbering
(`sed`, see `evidence/`), the two disassemblies are **instruction-for-instruction identical** -
same opcodes, same operands, same branch/call structure, just placed at different absolute
addresses in the two different binaries (`evidence/diff_between_two_build_methods.txt`).

Neither build reproduces 013's originally-reported 92. Both reproduce 014's 28, using *both* of
013's own stated build methods (the in-workspace test binary containing the diagnostic module,
and re-fetching + rebuilding candidate 002's exact original `bench/idwt/` crate against `0.6.0`
of `fearless_simd`).

## Result

The 92-vs-28 discrepancy does **not** reproduce on this session's toolchain via either of
candidate 013's own stated build methods, and does not depend on the diagnostic test module's
presence (method 1 includes it; method 2's `bench/idwt/` crate never contained it, since it calls
into `hayro-jpeg2000` as an ordinary dependency). Both give the identical compiled result: 28
real calls, matching candidate 014's number. This rules out, for this specific pair of
candidates, all three of the alternatives 014 hedged on: build harness choice
(`cargo test --lib` vs. an external path-dependency crate), the diagnostic module's mere
presence in the same source file, and `fearless_simd` version (fresh dependency resolution
against the workspace's pinned `"0.6.0"` requirement gives exactly `0.6.0` in both cases, `Cargo.lock`
confirms 002/013/014 all recorded the same version too).

That leaves **toolchain (`rustc`/LLVM) version drift between the two original sessions** as the
remaining, unruled-out explanation - and there is concrete circumstantial evidence for it: neither
013's nor 014's `RESULT.md`/evidence recorded `rustc --version` at the time, and this machine's
`stable` rustup toolchain directory (`~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu`) has an
mtime of 2026-08-22 21:25 - i.e. it was updated at some point, and no toolchain is pinned by the
project (no `rust-toolchain.toml`, confirmed via `find`), so any session run before that update
used a different `rustc`/LLVM than any session (including this one, `rustc 1.98.0`) run after it.
This is consistent with, but does not by itself prove, the two original sessions straddling that
update - it wasn't otherwise recorded which session ran when relative to it.

## Does it change behavior/output?

No patch is proposed; nothing in this candidate touches real behavior. The same reversion/cleanup
already performed for 013/014 applies here too (patch applied only to reproduce the functions
under test, then `git checkout` before writing this folder).

## Why the result looks the way it does

Both of 013's own build methods, re-run today, converge on 014's number rather than 013's
original one - meaning whatever produced 92 originally is a property of *when* 013 ran (its
toolchain), not *how* it built. The project doesn't pin a `rust-toolchain.toml`, so `cargo`/`rustc`
silently follow whatever `rustup`'s "stable" alias currently resolves to, which can (and per the
toolchain directory's mtime, apparently did) change between sessions without anyone in this
workflow explicitly deciding to upgrade. LLVM's inlining heuristics for `#[target_feature]`-gated
generic functions like `fearless_simd`'s per-op kernels are exactly the kind of internal,
version-to-version-tunable heuristic that would plausibly produce a large call-count swing (92 to
28, not a small percentage change) across a `rustc`/LLVM bump, without any source-level change at
all.

## What's not done / open questions

- The exact `rustc`/LLVM version 013 actually ran under is not known and cannot be recovered
  after the fact (rustup's `stable` alias is a moving target and this machine no longer has
  whatever `stable` pointed to before 2026-08-22 21:25 installed under that name - only pinned
  version directories like `1.97.1` remain, none independently confirmed to be the one 013 used).
  This candidate's "toolchain drift" conclusion is therefore the best-supported remaining
  hypothesis after ruling out the other three, not a directly confirmed root cause.
- Added to `IDEAS.md`: record `rustc --version` (and `cargo --version`) in every future
  candidate's `RESULT.md` that measures compiled-code artifacts (`nm`/`objdump` call counts,
  instruction counts, etc.) so a future session doesn't hit the same unreconcilable gap - and
  consider whether this project should pin a `rust-toolchain.toml` for exactly this reason (a
  decision for the project maintainer, not made here, since it affects real contributors too, not
  just this optimization workspace).
- Since the underlying inlining behavior (92 vs 28 calls) is itself toolchain-dependent and not
  under this project's control, and candidate 014 already found no wall-clock difference between
  the "many-real-calls" and "fewer-real-calls" shapes it could actually construct (28 vs 32, +0.7%
  to +1.7%, not a meaningful win), whether chasing the call count down further via restructuring
  is worth it at all remains an open question this candidate doesn't resolve either way - it only
  removes one candidate explanation (build method/module/dependency version) from consideration.
- `evidence/` has both binaries' extracted, normalized disassembly for
  `irreversible_filter_97i_horizontal_simd::<Avx512>` and the address/label-normalized diff
  between them.
