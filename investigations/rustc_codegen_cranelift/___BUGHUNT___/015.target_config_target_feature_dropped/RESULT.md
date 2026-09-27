# 015. `-C target-feature` is dropped by `target_config`, so the generated program takes different `#[cfg]` branches than LLVM's - and valid programs stop compiling

`src/lib.rs`'s `target_config` returns a hardcoded per-architecture feature list
(`fxsr`/`sse`/`sse2`/`x87` on x86_64) and never looks at `sess.opts.cg.target_feature`. The FIXME
on that line has been there for years; what was untested is the *consequence*. That set becomes
`sess.internal_target_features`, which drives `cfg(target_feature = "...")` and rustc's
`abi_required_features` check - so `-C target-feature=+avx2` does not merely fail to make the
program faster, it makes the **same source with the same flags produce a program that runs a
different code path** than the one plain rustc builds, and it makes some valid programs
**fail to compile at all**. Candidate 012 confirmed the second half in passing and left it to this
row.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the backend for where `-C target-feature` could take effect | `src/lib.rs` `target_config` + `build_isa`, `git log -S 'cg.target_feature'` over the whole history | `sess.opts.cg.target_feature` has **never** been referenced anywhere in cg_clif's history. `build_isa` only consults `-C target-cpu` (which does reach Cranelift's `isa_builder`), so the flag is dropped on both channels: rustc is told the wrong feature set *and* Cranelift never enables the feature. |
| Differential test vs LLVM, single file (`tools/diffcheck.sh`) | opt-level 0, `-Ctarget-feature=+avx2,+fma,+bmi2,+popcnt`, a program branching on `cfg!(target_feature = ...)` | **Reproduced.** LLVM prints `path=avx2 / avx=true avx2=true fma=true bmi2=true popcnt=true`, cg_clif prints `path=baseline / avx=false avx2=false fma=false bmi2=false popcnt=false`. Same source, same flags, different program output. |
| Same, for the compile-time half (candidate 012's complementary case) | opt-level 0, `extern "C" fn(__m256, __m256) -> __m256` with **no** per-function `#[target_feature]`, under `-Ctarget-feature=+avx`, `-Ctarget-cpu=native`, `-Ctarget-cpu=x86-64-v3` | **Reproduced, all three.** LLVM compiles and prints `[1.5, 2.5, ... 8.5]`; cg_clif hard-errors at rustc's `abi_required_features` check - and the diagnostic advises "consider enabling it globally (`-C target-feature=+avx`)", which is exactly what the build already does. |
| Boundary-value construction on the flag syntax itself | opt-level 0, 10 flag forms: none, `+avx2`, `+avx512vl`, `+avx2,-avx`, `-popcnt,+popcnt`, `+sse4.2,+aes,+pclmulqdq`, `+bogus_feature_xyz,+avx`, `avx2` (no sign), `+crt-static,+avx2`, `+avx512f,+cmpxchg16b,+adx,+rdrand,+sha,+f16c,+lzcnt`; 27 features printed per run | 8 of 10 diverge from LLVM unpatched (the no-flag and unsigned-`avx2` forms agree because both are no-ops). Also checks LLVM's *removal* semantics (`+avx2,-avx` must drop `avx2` too) and its handling of unknown names. |
| Generated sweep over the whole feature space (`tools/target_feature_cfg_sweep.sh`, written for this candidate) | 230 features rustc knows for `x86_64-unknown-linux-gnu`, one `-Ctarget-feature=+F` compilation per feature, via `--print cfg` on both compilers; plus a `MODE=cpu` one-shot for `-Ctarget-cpu=native` | Unpatched: **72 features disagree**, 4 agree (the hardcoded base), 154 are not enabled by LLVM either (nightly-gated). `-Ctarget-cpu=native`: LLVM reports 47 features, cg_clif 4 - **43 disagree**. |
| `-C target-feature=+crt-static`, the one target feature with non-ISA semantics | opt-level 0, `cfg!(target_feature = "crt-static")` + real static link | **Not affected.** Both print `true`; rustc computes `crt-static` from `sess.crt_static()` directly, not from the backend's feature set. Recorded so a future session doesn't re-check it. |
| `#[target_feature]` call-safety, the other thing `internal_target_features` gates | opt-level 0, safe call of a `#[target_feature(enable = "avx2")] fn` and of `_mm256_add_epi32` from an ordinary fn, under `-Ctarget-feature=+avx2` | **Not affected today.** Current rustc rejects both identically on either backend ("the avx2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`"). This channel was closed on the rustc side, so it is not a cg_clif divergence. |
| Extended-sysroot test crates, as the candidate asked | `git grep 'target_feature ='` over `download/graviola`, `download/rand`, `download/regex` | Exactly one hit: `rand/src/distributions/utils.rs` picks between two `WideningMultiply` implementations under `#[cfg(target_feature = "sse2"/"avx2"/"avx512bw")]` (`_mm{,256,512}_mulhi_epu16` vs a widening-cast fallback). Under cg_clif the `avx2`/`avx512bw` arms are never compiled. Both arms compute the same values, so for `rand` the consequence is a **never-exercised code path**, not a wrong answer - the wrong-answer/failed-build cases above are hand-written. |
| Non-x86 targets, `--print cfg` only (no cross toolchain in this sandbox) | `aarch64-unknown-linux-gnu`, `riscv64gc-unknown-linux-gnu`, `s390x-unknown-linux-gnu`, `x86_64-unknown-none`, with and without `-Ctarget-feature=+sve,+aes,+v,+neon` | Same class on every arch: e.g. riscv64gc reports 0 features where LLVM reports 10 (no flags) / 19 (with flags). Not a symptom unique to x86_64. |
| Standalone `repro/` end-to-end via the real `dist/cargo-clif` | real cargo crate, two binaries, opt-level 0, real binaries actually run and diffed against the LLVM-built ones; 4 runs unpatched, 2 patched | Unpatched **4/4**: `cfgpath` prints `kernel=baseline` where LLVM prints `kernel=avx2`, and `abi` fails to compile. Patched **2/2**: `cfgpath` matches LLVM line for line; `abi` is a clean `error:`. |
| Project test suite | `./y.sh test` in the isolated worktree, on pristine `2bc05d45` and on the patched tree | Both exit 0, 61 `test result: ok.` lines, zero `FAILED` - see "The fix". |
| Full `just test` cycle from a pristine clone | `just repo_url=https://github.com/rust-lang/rustc_codegen_cranelift.git test` (no GitHub SSH key in this sandbox, so the documented https fallback): clone @ 2bc05d45 -> `./y.sh build` -> `reproduce-before` -> apply both diffs -> `./y.sh build` -> `reproduce-after` | before: `cfgpath: DIFFERENT`, `abi: DIFFERENT`, run.sh exit 1; after: `cfgpath: SAME`, `abi: DIFFERENT` (clean error), run.sh exit 2 - see `just_test` note at the end of this file. |

## Status

**Confirmed** - deterministic, no timing or layout dependence.

- `repro/run.sh` through the real `dist/cargo-clif`: **4/4 runs** against the unpatched `dist/` of
  cg_clif @ 2bc05d45, with `rm -rf target-llvm target-clif` at the start of every run so nothing
  is served from cargo's cache. Every run: `cfgpath` prints `kernel=baseline` /
  `fused_multiply_add=false` where the LLVM-built binary prints `kernel=avx2` /
  `fused_multiply_add=true`, and `abi` fails to compile while LLVM's runs and prints the sum.
- `tools/target_feature_cfg_sweep.sh`: 72/72 disagreeing features, reproduced on two separate runs
  (once against the main tree's `dist/`, once against the pristine worktree).
- The 10-form boundary sweep: 8/10 diverge, twice.
- Machine: Ryzen 9 7900 (Zen 4, AVX-512), rustc nightly-2026-08-24, cg_clif @ 2bc05d45
  (Cranelift 0.135), opt-level 0 (the divergence is a `#[cfg]`/`--print cfg` effect, so it is
  present at every optimization level - also checked at `-O`).

**Fully reproducible**: **Yes**. `repro/` is a real cargo crate compiled by the real
`dist/cargo-clif` wrapper; the `kernel=baseline` line in `output.txt` is a real generated binary
actually running, and the `abi` error is the real compiler actually refusing the program. There is
no second participant to fake - the input is Rust source plus a documented rustc flag, handed to
the real compiler entry point.

## Class

logic (the generated program takes a different, `#[cfg]`-selected code path than the same source
built by any other backend) - with a compile-time facet (valid Rust that LLVM accepts is rejected).

## Root cause

`src/lib.rs`:

```rust
fn target_config(&self, sess: &Session) -> TargetConfig {
    // FIXME return the actually used target features. this is necessary for #[cfg(target_feature)]
    let target_features = match sess.target.arch {
        Arch::X86_64 if sess.target.os != Os::None => {
            vec![sym::fxsr, sym::sse, sym::sse2, Symbol::intern("x87")]
        }
        ...
    };
    TargetConfig { internal_target_features: UnordSet::from_iter(target_features), .. }
}
```

The returned set is a constant function of the *target*, with no input from the *command line*.
rustc stores it in `sess.internal_target_features`, and that one set feeds at least three separate
things:

1. **`cfg(target_feature = "...")` and `--print cfg`.** So a crate that statically selects a SIMD
   implementation with `#[cfg(target_feature = "avx2")]` - the standard pattern, used by `rand` in
   cg_clif's own extended-sysroot test set - always compiles its fallback arm under cg_clif, no
   matter what the user asked for. When the two arms are not observationally identical (a
   different printed kernel name, an FMA-vs-mul+add rounding difference, a different lane count in
   a debug dump) the generated program's *output* differs from LLVM's on the same source.
2. **rustc's `abi_required_features` check** (`extern` fn signatures containing SIMD vectors). This
   is a hard error, so `-C target-feature=+avx` / `-C target-cpu=native` / `-C target-cpu=x86-64-v3`
   plus an `extern "C" fn(__m256)` is valid Rust that LLVM compiles and runs and cg_clif refuses -
   while telling the user to enable the feature globally, which is what they already did.
3. **`#[target_feature]` call safety**, which happens to be inert right now because current rustc
   no longer lets a globally-enabled feature make such a call safe.

`build_isa` has the mirror-image half of the same gap: it forwards `-C target-cpu` to Cranelift's
`isa_builder` but not `-C target-feature`, so even the pure performance intent of the flag is lost.

Note the asymmetry this creates for `-C target-cpu=native`: it *does* reach Cranelift (via
`cranelift_native::builder_with_options`), so codegen changes, but it does not reach
`target_config`, so `cfg(target_feature)` still says SSE2 - 43 of the 47 features LLVM reports for
that flag are missing.

## Introduced by

Never worked - `git log -S 'cg.target_feature'` over the full history returns no commit, so
`-C target-feature` has not been read by this backend at any point. The closest thing to an
introducing commit is 431cebdc "Introduce #[cfg(target_feature = \"cg_clif\")]" (2020-06-20), which
first implemented the backend's `target_features` hook and implemented it as a hardcoded constant
(`vec![Symbol::intern("cg_clif")]`) instead of deriving anything from the session options.
45d8c121 "Return architecturally mandated target features to rustc" (2024-01-02) replaced the
then-empty `vec![]` with today's baseline list and wrote the FIXME that is still there
("In the future the actual target features that Cranelift enables should be returned here, but for
now this works."). Found with `git log -S` on `cg.target_feature`, `Symbol::intern("sse2")` and
`target_features`; no bisect needed.

## The fix

`patch.diff` (+91/-17, all in `src/lib.rs`, `cargo fmt --check` clean) makes `target_config` honor
`-C target-feature`. The hardcoded per-arch list becomes the *base* of a new
`crate_target_features(sess)` helper, which then walks `sess.opts.cg.target_feature`:

- `+feat` adds `feat` plus its transitive implied features, using rustc's own table
  (`sess.target.rust_target_features()` gives `(name, stability, implied)` per feature), so
  `+avx512f` also enables `avx2`/`fma`/`f16c`/... exactly as LLVM does.
- `-feat` removes `feat` **and every feature that transitively implies it**, matching LLVM's
  semantics (`+avx2,-avx` must leave `avx2` off, not just `avx`).
- Names rustc does not know, names whose `Stability` says `!toggle_allowed()` or `!in_cfg()`, and
  the unsigned/empty forms are skipped, so cg_clif never claims a feature LLVM would refuse to
  enable - that would be a divergence in the opposite direction.

Verified against more than the single triggering input (`CONFIRMATION.md` section 7):

| case | before | after |
|---|---|---|
| `repro/` `cfgpath` via the real `dist/cargo-clif` (`-Ctarget-feature=+avx2,+fma,+bmi2,+popcnt`) | `kernel=baseline`, all 5 cfgs false | matches LLVM exactly: `kernel=avx2`, all 5 true |
| `tools/target_feature_cfg_sweep.sh`, 230 features one at a time | 72 disagree / 4 agree | **0 disagree / 76 agree** |
| the 10-form boundary sweep (implied features, `-feat` removal, unknown name, unsigned form, `+crt-static` mixed in, a 7-feature list) | 8/10 diverge | **0/10 diverge** |
| `--print cfg` on `aarch64-unknown-linux-gnu` with `-Ctarget-feature=+sve,+aes,+v,+neon` | 1 of LLVM's 5 features | 4 of 5 |
| `--print cfg` on `riscv64gc-unknown-linux-gnu`, same flags | 0 of LLVM's 19 | 12 of 19 |
| `-Ctarget-cpu=native` (43 disagreeing features) | 43 disagree | 43 disagree - **deliberately out of scope**, see below |
| `repro/` `abi` (`extern "C" fn(__m256)` under `-Ctarget-feature=+avx`) | rustc `abi_required_features` hard error | not fixed: a clean cg_clif `error:` (with candidate 012's patch) - see below |

**Two things this patch deliberately does not fix**, both stated here rather than papered over:

- **The `abi` case still does not compile.** Once `avx` is correctly reported, rustc *accepts* the
  `extern "C" fn(__m256)` signature and hands cg_clif a 256-bit vector parameter - which Cranelift
  has no type for. That is candidate 012's bug, and candidate 012's patch is a genuine
  **prerequisite** for this one: measured in this worktree, `patch.diff` applied *alone* on top of
  `2bc05d45` turns the hard error into 012's ICE
  (`src/base.rs:222 ... Unsupported("Unexpected SSA-value type: f32x8")`). With
  `prereq-candidate-012.diff` (a verbatim copy of candidate 012's `patch.diff`) applied first, it
  is a clean diagnostic instead. The justfile applies both, in that order.
- **`-C target-cpu` is still not expanded into a feature set.** LLVM derives 47 features from
  `-Ctarget-cpu=native`; doing the same would need a per-CPU feature table, which Cranelift does
  not have (its `isa_builder.enable("znver4")` only sets its own ~19 `has_*` predicates and there
  is no rustc-name mapping in tree). Deriving the reported set from the built `TargetIsa`'s
  `isa_flags()` is the obvious follow-up and is noted below.

- Project test suite: `./y.sh test` in the isolated worktree (in-tree items plus the `download/`
  repos: rand, regex, graviola, portable-simd). **Before (pristine 2bc05d45): exit 0, 61
  `test result: ok.` lines, 0 `FAILED`, 2m54s. After (012's patch + this one): exit 0, 61
  `test result: ok.` lines, 0 `FAILED`, 2m56s.** No new failures; the suite aborts on the first
  failure and neither run aborted.

## What's not done / open questions

- **`-C target-feature` still does not reach Cranelift.** After this patch
  `cfg(target_feature = "avx2")` is true while `build_isa` still hands Cranelift a plain `x86-64`
  ISA, so the generated code is SSE2. That is *correct* (cg_clif's `llvm.x86.*` handlers lower
  independently of the ISA flags, and Cranelift legalizes what it cannot do natively) but it means
  the flag is still cosmetic for performance. Forwarding it needs a rustc-name -> Cranelift
  `has_*` mapping (19 x86 settings exist: `has_sse3`/`has_ssse3`/`has_sse41`/`has_sse42`/`has_avx`/
  `has_avx2`/`has_fma`/`has_avx_vnni`/`has_avx512{f,vl,dq,bitalg,vbmi,vnni}`/`has_popcnt`/
  `has_bmi1`/`has_bmi2`/`has_lzcnt`/`has_cmpxchg16b`). It was left out to keep the patch bounded:
  it is a codegen change, not a correctness one, and it needs benchmarking plus a decision about
  what to do with the ~200 rustc features that have no Cranelift equivalent.
- **Deriving the reported set from the built `TargetIsa` instead** (`isa.isa_flags()`) is what the
  FIXME literally asks for and would fix `-C target-cpu` too. Not attempted here: it would report a
  set *narrower* than LLVM's for the same flags (only the 19 mappable features), i.e. it trades one
  kind of divergence for another, and it is the maintainer's call which is preferable.
- **A newly-truthful `cfg(target_feature)` can expose the unimplemented-intrinsic gap.** With
  `-C target-feature=+avx2` a crate like `rand` (with `simd_support`) now compiles its
  `_mm256_mulhi_epu16` arm, and `llvm.x86.avx2.pmulh.w`/`pmulhu.w` are among the 701 unimplemented
  `llvm.x86.*` intrinsics, which means a compile warning plus a run-time abort where the crate
  previously silently took a working fallback. This is not new behavior (any crate using
  `#[target_feature(enable = "avx2")]` attributes already hits it) and it only happens when the
  user explicitly passes the flag, but it is a real consequence of making the flag work and a
  maintainer should know about it. `./y.sh test` does not pass these flags, so the suite does not
  see it.
- **The target spec's own baseline `features` string is still ignored.** The residual aarch64
  (`outline-atomics`) and riscv64 (`+m,+a,+f,+d,+c`) gaps in the table above come from
  `sess.target.options.features`, which the hardcoded base list does not reproduce. The same
  parsing loop could be run over it first; not done here because it changes behavior on targets
  this sandbox cannot build or run a program for (no cross toolchain), including the deliberate
  `x86_64-unknown-none` / `Os::None` special cases.
- **`rustc --print target-features` prints nothing under cg_clif** (plain rustc prints 237 rows for
  x86_64). Not touched by this patch - a separate hook, and a separate suspect.
- **No wrong-*arithmetic* case was found**, only wrong-code-path and failed-compile ones. Whether a
  real crate exists whose `#[cfg(target_feature)]` arms are not observationally equivalent was not
  established beyond `rand` (whose arms are equivalent).
