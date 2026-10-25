# zune-jpeg `decoder.rs` test compares fn-pointer addresses of an `#[inline]` function across codegen units

Found incidentally while working `___OPTIMIZATIONS___/002.JPEG_HUFFMAN_MCU_ENTROPY_DECODE` (running
`cargo test --release -p zune-jpeg --no-fail-fast` as before/after validation for an unrelated
Huffman-bitstream-decode performance patch) - no connection to that candidate's own fix, purely a
pre-existing test-correctness defect noticed along the way. That candidate originally described this
as failing "nondeterministically"; this session re-investigated and found it is actually
deterministic (always fails) under the crate's own default `--release` profile, and always passes
under a plain debug `cargo test` - see "Root cause" below for why.

`crates/zune-jpeg/src/decoder.rs`'s `planar_layout_helpers::set_options_refreshes_cached_dispatch`
test asserts, among other things, that `decoder.idct_1x1_func as usize` equals
`choose_idct_1x1_func(&replacement) as usize` - i.e. that two independently-obtained function
pointers to what should be the same function (`idct_int_1x1`) compare bitwise-equal.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo test --release -p zune-jpeg --lib decoder::planar_layout_helpers::set_options_refreshes_cached_dispatch`, 8 consecutive runs | rustc 1.90.0, cargo 1.90.0, default release profile (no explicit `codegen-units` override, so cargo's default of 16) | Confirmed: **8/8 failed**, always the same `assert_eq!` at `decoder.rs:2783` (the `idct_1x1_func` comparison), with the `left`/`right` addresses differing by a constant offset (158720 bytes) every single run, despite ASLR randomizing the absolute base each run |
| Same test under plain `cargo test -p zune-jpeg --lib ...` (debug/dev profile) | rustc 1.90.0, unoptimized + debuginfo | Passed - confirms the failure is specific to the release build configuration, not a source-level logic bug in `set_options` itself |
| `nm -C target/release/deps/zune_jpeg-*` on the release test binary | read-only symbol dump | Found **two distinct local (`t`) symbols** both named `zune_jpeg::idct::scalar::idct_int_1x1`, at `0x673d0` and `0x8dfd0` - difference `0x26C00` = 158720 decimal, exactly matching the observed `left`/`right` gap every run. All of `idct_int`, `idct4x4`, `idct_avx2`, `idct_avx2_4x4` (the other functions this same test's other `assert_eq!`s exercise) each appear exactly **once** in the same symbol dump - consistent with only the `idct_1x1_func` assertion ever failing |
| Read `crates/zune-jpeg/src/idct/scalar.rs` attributes on all four scalar/AVX2 IDCT functions | read-only | `idct_int_1x1` is the only one of the four marked `#[inline]`; `idct_int`, `idct4x4` are plain `pub fn` with no inline hint, and the AVX2 functions are plain `pub unsafe fn` |
| `git log -L2771,2803:crates/zune-jpeg/src/decoder.rs` | read-only | Found introducing commit: `ca04b109` "fix(jpeg): reset output state across raw replays", which added this whole test (all three `assert_eq!`s plus the colorspace loop) in one commit |
| Patched the `idct_1x1_func` comparison to check output-equivalence (call both function pointers on identical input, compare output arrays) instead of address-equality, applied to a scratch copy | rustc 1.90.0, `cargo test --release -p zune-jpeg --lib`, 3 consecutive runs | Confirmed fixed: 3/3 passed; full `cargo test --release -p zune-jpeg --lib` (all 14 tests) also passed |
| `just test` against a fresh `git clone` of the public repo at `base_commit` (HTTPS, SSH unavailable in this sandbox) | real clone, real `cargo test --release` | Confirmed end-to-end, outside this working checkout: reproduces before the patch (exit 1, panic at `decoder.rs:2783`), fixed after (exit 0) |

## Status

**Confirmed**.

Reproduces deterministically under the crate's own default release test profile - 8/8 runs in this
session, plus a further 1/1 via the standalone `repro/` clone, every run with the same panic
location and the same constant 158720-byte gap between the two compared addresses. Does not
reproduce under a debug build (expected, given the root cause below - debug builds use more
codegen units by default but apparently happen to place the call sites such that the linker/ICF
merges or the duplication doesn't manifest the same way; not investigated further since the release
failure is the one that matters, as `cargo test --release` is what this project's own justfiles and
the perf-optimize workspace both actually run).

**Fully reproducible**: **Yes**. `repro/repro.sh` runs the real, public-facing check
(`cargo test --release -p zune-jpeg --lib <test name>`, the same command the project's own CI/perf
workspaces run) against a genuine fresh clone of the public repository at `base_commit`. No second
participant is involved (this is a single-crate test/codegen-configuration defect, not an
interaction between two components).

## Class

logic (test-correctness bug - the assertion methodology, not the decoder's actual decode-time
behavior, is wrong; no compiled-library/decode-output behavior is affected, only this one test).

## Root cause

`zune_jpeg::idct::scalar::idct_int_1x1` (the function `choose_idct_1x1_func` always returns,
regardless of `DecoderOptions`) is marked `#[inline]`. Under rustc's default codegen model, a
`#[inline]`-hinted function does not get a single shared definition referenced by all call sites
across codegen units (the way a plain, un-hinted function does - confirmed above: `idct_int`,
`idct4x4`, `idct_avx2`, `idct_avx2_4x4` each appear exactly once in the release test binary's symbol
table); instead, each codegen unit that references it may get its own local (internal-linkage)
copy, so the same source function can legitimately end up at two different addresses in the final
binary if its two call sites land in different codegen units. The crate's release profile does not
override `codegen-units` (stays at cargo's release default of 16), so this is easy to trigger: the
call site inside `JpegDecoder::new_with_options`/`set_options` (via `choose_idct_1x1_func`, called
while building the `decoder` fixture) and the call site inside the test itself
(`choose_idct_1x1_func(&replacement)`, called directly) end up in two different codegen units,
producing two distinct local `idct_int_1x1` symbols 158720 bytes apart. The test's `assert_eq!`
assumes function-pointer bitwise identity across these two independently-obtained pointers, which
Rust does not guarantee for an `#[inline]` function under a multi-codegen-unit build - this is
standard, documented LLVM/rustc behavior, not undefined behavior or an ASLR artifact (the *absolute*
addresses move each run because of ASLR, but their *relative* offset, 158720, stays constant every
run, confirming it's two fixed, distinct static symbols rather than randomized runtime allocation).

## Introduced by

ca04b109 "fix(jpeg): reset output state across raw replays" - identified via
`git log -L2771,2803:crates/zune-jpeg/src/decoder.rs`; this commit's own diff shows it added the
entire `set_options_refreshes_cached_dispatch` test, including the `idct_1x1_func` comparison that
fails.

## The fix

Replaced the address-comparison `assert_eq!` for `idct_1x1_func` with a behavioral-equivalence
check: call both function pointers (`decoder.idct_1x1_func` and `choose_idct_1x1_func(&replacement)`)
on identical input buffers and compare their output buffers, instead of comparing `as usize`
addresses. This still meaningfully tests "does `decoder.idct_1x1_func` behave like the function
`choose_idct_1x1_func` would currently select", without relying on the codegen-unit-fragile
pointer-identity assumption. The other two assertions in the same loop (`idct_func`, `idct_4x4_func`)
were left untouched - they compare functions/closures that are not `#[inline]`-hinted and were
observed (via the symbol-table dump above) to have exactly one instantiation each in the release
binary, so their address comparison is sound as written.

Verified against the checkout this bug was found in (not just the standalone `repro/` clone):

- `cargo test --release -p zune-jpeg --lib decoder::planar_layout_helpers::set_options_refreshes_cached_dispatch`,
  3 consecutive runs: failing before -> passing after, all 3/3.
- `cargo test --release -p zune-jpeg --lib` (full lib test set, 14 tests): 13 passed/1 failed ->
  14 passed/0 failed.
- `patch.diff` diffstat: `+11/-4` in `crates/zune-jpeg/src/decoder.rs` (`git diff --numstat` at
  capture time).

## What's not done / open questions

- Not checked: whether any other `#[inline]`-marked function elsewhere in the workspace has a test
  making the same pointer-identity assumption (only this one test, surfaced by the perf-optimize
  session's before/after run, was investigated) - a workspace-wide `grep -rn "as usize"` around
  `assert_eq!` calls near fn-pointer fields would be needed to rule this out elsewhere.
- Not investigated: exactly why the debug/dev profile doesn't reproduce the same duplication (dev
  profile's own default codegen-units is 256, even more parallel than release's 16, so a similar or
  worse duplication would naively be expected) - plausible explanations (incremental compilation
  caching, MIR-level dedup differences between opt-levels, or the two call sites simply landing in
  the same codegen unit by luck under dev's partitioning heuristic) were not distinguished, since it
  wasn't needed to confirm or fix the release-mode failure.
- Not considered: removing `#[inline]` from `idct_int_1x1` instead of fixing the test. That would
  likely also resolve the symbol duplication, but changes real (non-test) code to work around a
  test's assumption rather than fixing the test's assumption itself, and risks a (probably tiny,
  unmeasured) codegen/performance change to actual decode-path inlining - out of scope for a
  test-only fix, and not something this session measured either way.
