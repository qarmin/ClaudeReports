# 023.print_target_features_empty

`rustc --print target-features` prints nothing under cg_clif while plain rustc prints 241 rows for
`x86_64-unknown-linux-gnu`. Noticed while working candidate #015 (`target_config` dropping
`-C target-feature`): this print request goes through a *different* backend hook
(`CodegenBackend::print`), so #015's area does not touch it, and it had never been checked.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Direct differential check of the print request, LLVM vs cg_clif | `dist/rustc-clif --print target-features` vs `rustc +nightly-2026-08-24 --print target-features`, main tree at 2bc05d45 | Reproduced immediately: 0 lines / 0 bytes / exit 0 vs 241 lines / 19597 bytes |
| 2. Widened to every print request rustc routes through the backend | The six kinds `rustc_driver_impl` dispatches to `CodegenBackend::print`: `relocation-models`, `code-models`, `tls-models`, `stack-protector-strategies`, `target-cpus`, `target-features`; both the bare form and the `--print KIND=<path>` file form | All six empty under cg_clif (and `--print KIND=<path>` writes a 0-byte file), all six non-empty under LLVM. Turned into `repro/` |
| 3. Searched for a real downstream consumer that this actually breaks | `grep` over `~/.cargo/registry` for build scripts shelling out to `--print target-features`; read `scripts/test_rustc_tests.sh` | The only registry hit (`target-features 0.1.6`, the `pulp`/`gemm`/`faer` dependency) uses a *pre-generated* `target-features.txt`, so it is unaffected. The real hit is in this project itself: `scripts/test_rustc_tests.sh:116` carries `rm -r tests/run-make/print-to-output # requires --print relocation-models`, i.e. an upstream rustc test is deleted specifically because of this gap |
| 4. Whole-target sweep for regressions after the fix | Every tuple in `--print target-list` (~290), three print kinds, unpatched vs patched cg_clif | Caught a real regression in the first version of the patch (48 new hard errors + 6 Cranelift `assert!` panics on `aarch64_be-*`, because the helper went through `target_tuple()`/`isa::lookup()`); after making the ISA lookup infallible the counts match the unpatched backend exactly (1 vs 1, and that one is the pre-existing `amdgcn-amd-amdhsa` "LTO is not supported" failure) |

## Status

**Confirmed** - 3/3 consecutive runs of the repro against the unpatched main-tree
`dist/rustc-clif` exit 1 with the identical 12 findings; the underlying observation
(`--print target-features` -> 0 bytes, exit 0) is a pure function of the binary, no seed, no
timing, no memory layout involved.

**Fully reproducible**: **Yes** - `repro/` invokes the real `dist/rustc-clif` binary (the real
affected entry point: rustc's own `--print` handling with cg_clif loaded as the codegen backend)
and the real oracle `rustc`, both as separate processes, and compares their actual stdout and the
actual files written by `--print KIND=<path>`. Nothing is hand-asserted or simulated.

Note on scope, stated plainly: this is **not** a miscompilation and **not** an ICE. Under the
strict definition in `___BUGHUNT___/README.md` ("the backend crashes on it, refuses it, or emits
code that computes the wrong answer") it is none of the three - no generated program computes a
wrong answer here. What it is: a backend hook cg_clif never implements, so six documented rustc CLI
queries silently answer "nothing" with exit status 0, and one upstream rustc test
(`tests/run-make/print-to-output`) has to be deleted from cg_clif's rustc-test-suite run because of
it. A reader who only cares about wrong codegen can skip this candidate.

## Class

missing-backend-hook (CLI/diagnostic output divergence from LLVM; silent empty answer, not an error)

## Root cause

`rustc_driver_impl/src/lib.rs` routes six print requests to the codegen backend:

```rust
RelocationModels | CodeModels | TlsModels | TargetCPUs | StackProtectorStrategies
| TargetFeatures => { codegen_backend.print(req, &mut crate_info, sess); }
```

`CodegenBackend::print`'s declaration in `rustc_codegen_ssa/src/traits/backend.rs:42` is

```rust
fn print(&self, _req: &PrintRequest, _out: &mut String, _sess: &Session) {}
```

- an **empty default body**. `rustc_codegen_cranelift`'s `impl CodegenBackend for
CraneliftCodegenBackend` (`src/lib.rs`) does not override it, so all six requests append nothing to
`out`, and rustc then prints/writes that empty string and exits 0. There is no "unsupported print
request" diagnostic anywhere on the path, which is why this never surfaced as an error.

For `target-features` specifically the empty answer is also avoidable rather than inherent: the
"Features supported by rustc for this target" half of LLVM's output is derived from
`sess.target.rust_target_features()`, which lives in `rustc_target` and is completely
backend-independent - cg_clif has the same data available. Only the second half (the
code-generation feature list) is genuinely backend-specific, and Cranelift exposes its equivalent
through `isa::Builder::iter()` (`has_*` boolean settings). Likewise `target-cpus`: cg_clif already
feeds `-Ctarget-cpu` to Cranelift as a settings *preset*, and the presets are enumerable from the
same iterator.

## Introduced by

not determined as a single commit, and the reason is concrete: this repository's history is a chain
of `git subtree` syncs from rust-lang/rust, and `git log -S` shows cg_clif never had a handler for
any of the six kinds (`git log -S RelocationModels` returns nothing at all;
`git log -S PrintRequest` returns only the three 2025-12-08 commits below). The two commits that
bracket the current state:

- ed7d97e4 Merge commit '3270432f4b0583104c8b9b6f695bf97d6bbf3ac2' into sync_cg_clif-2024-05-13 -
  earliest evidence of the gap in this repo: it adds
  `rm -r tests/run-make/print-to-output # requires --print relocation-models` to
  `scripts/test_rustc_tests.sh`.
- d38a5a6d Make `--print=backend-has-zstd` work by default on any backend (2025-12-08) - removed
  cg_clif's last `fn print` impl. That impl only handled `PrintKind::BackendHasZstd` and had a
  bare `_ => {}` arm, so it already swallowed all six of these kinds; deleting it changed nothing
  about the behaviour reported here.

## The fix

`patch.diff` (+163/-2, all in `src/lib.rs`; `cargo fmt --check` clean) implements
`CodegenBackend::print` for
`CraneliftCodegenBackend`:

- `relocation-models` / `code-models` / `tls-models` / `stack-protector-strategies`: the same
  backend-independent lists `rustc_codegen_llvm` hardcodes (`RelocModel::ALL`, `TlsModel::ALL`, and
  the literal code-model / stack-protector texts), so the output is byte-identical to LLVM's for
  these four - which is what `tests/run-make/print-to-output` asserts.
- `target-cpus`: `native` (only when host arch == target arch and the target doesn't require a
  consistent CPU, matching cg_llvm's rule) plus Cranelift's own settings **presets** for the
  target's ISA, sorted. 70 lines for x86_64, versus LLVM's 138 - deliberately Cranelift's real
  list, not LLVM's, because those are the names `-Ctarget-cpu` is actually forwarded to.
- `target-features`: the rustc-known feature names from `sess.target.rust_target_features()`
  filtered by `gate.in_cfg()` plus `crt-static` (this is the section that has to contain `sse2` on
  x86_64), then a "Code-generation features supported by Cranelift for this target" section built
  from the ISA's `has_*` boolean settings with Cranelift's own descriptions.
- a new `isa_settings()` helper, deliberately infallible: it parses the tuple itself instead of
  calling `target_tuple()` (which `fatal`s on an unparsable tuple) and refuses to hand
  `isa::lookup` an architecture whose per-ISA `isa_builder` would `assert!` rather than return
  `Err`. Without that guard the patch turned 48 targets' `--print target-features` from
  "empty, exit 0" into a hard error, six of them (`aarch64_be-*`) into an outright Cranelift panic
  at `cranelift-codegen-0.135.0/src/isa/aarch64/mod.rs:267` - see approach 4 in the table.

Verified against more than the one triggering input:

- `repro/` against the patched build: exit 1 -> exit 0, all six kinds non-empty and containing the
  expected substrings, in both the bare and the `--print KIND=<path>` form (see `output.txt`).
- Sweep over **every** tuple in `--print target-list` for `target-features`, `target-cpus` and
  `relocation-models`: nonzero-exit count identical before and after (1 vs 1 each, the pre-existing
  `amdgcn-amd-amdhsa` LTO failure).
- Spot-checked non-x86 targets by hand: `aarch64-unknown-linux-gnu`,
  `riscv64gc-unknown-linux-gnu`, `s390x-unknown-linux-gnu`, `wasm32-unknown-unknown`,
  `x86_64-unknown-none`, `nvptx64-nvidia-cuda`, plus the four no-Cranelift-ISA cases
  (`aarch64_be-unknown-linux-gnu`, `aarch64-apple-ios-sim`, `thumbv6-none-eabi`,
  `nvptx64-nvidia-cuda`), all exit 0 with the rustc feature list present and the
  code-generation section reduced to "No code-generation features are known for this target."

- Project test suite: `./y.sh test` in the worktree (`tmp/wt/023.print_target_features_empty`,
  with `download/` seeded so the external repos run too). **Before** the patch: exit code 0.
  **After** the patch: exit code 0, and every one of the 62 `test result:` lines it prints reads
  `ok. ... 0 failed` (the largest being `2798 passed`, `1400 passed`, `1222 passed`, `1200 passed`,
  `1020 passed`, `924 passed`, `825 passed`). No new failures; no failures at all on either side.
  Note that `./y.sh test` aborts on the first failure, so the identical exit status on both sides
  is itself the before/after comparison.

## What's not done / open questions

- `scripts/test_rustc_tests.sh:116` still deletes `tests/run-make/print-to-output`. With this
  patch that test should pass, but the rustc test suite is not available in this environment
  (`rustc-src` ships the compiler, not `tests/`), so **the line was deliberately left in the
  patch's blast radius alone rather than removed unverified**. Whoever takes this patch should try
  dropping that line and running `scripts/test_rustc_tests.sh`. `tests/run-make/print-request-help-stable-unstable`
  (line 162) is removed for an unrelated reason (needs a stable toolchain) and is not affected.
- Honesty caveat on two of the four copied lists: cg_clif ignores `-Ccode-model` and has no stack
  protector support at all (`scripts/test_rustc_tests.sh` removes `tests/ui/abi/stack-protector.rs`
  for exactly that reason). The patch prints LLVM's lists for those two because `--print` documents
  the *values the option accepts* (rustc parses them regardless of backend) and an empty list is
  strictly less informative - but a maintainer may prefer to print an explicit "not supported by
  this backend" note instead. Same question for `relocation-models`/`tls-models`, which cg_clif
  also only partially honours (`build_isa` sets `is_pic` from JIT mode and `tls_model` from the
  binary format, ignoring `-Crelocation-model` / `-Ztls-model`).
- `target-features` output now advertises the rustc feature namespace, but candidates #015 and the
  `-C target-feature`-is-dropped `IDEAS.md` row still stand: cg_clif reports only
  `fxsr/sse/sse2/x87` from `target_config` and never forwards `-C target-feature` to Cranelift. So
  `--print target-features` listing `avx2` does not mean asking for `+avx2` does anything. This
  patch does not change that and does not pretend to.
- The `aarch64_be-*` Cranelift `assert!` this patch had to route around is candidate #022's
  subject; the guard here is local to the print path and does not fix the underlying panic on the
  codegen path.
- Not checked: whether any other `CodegenBackend` method with an empty/`false` default body is
  similarly unimplemented in cg_clif and silently answers wrong (e.g. `has_zstd`, which d38a5a6d
  moved to a default of `false`).
