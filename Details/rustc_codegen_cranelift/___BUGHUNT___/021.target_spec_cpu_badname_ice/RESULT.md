# target_spec_cpu_badname_ice

`build_isa` (`src/lib.rs`) feeds rustc's target-spec CPU name straight into Cranelift with
`builder.enable(sess.target.cpu.as_ref()).unwrap()`. `sess.target.cpu` is an *llvm* cpu name and is
not guaranteed to be a Cranelift ISA preset name, so any target whose spec names a cpu Cranelift
doesn't know - starting with rustc's own default, `generic` - makes the compiler panic on the very
first codegen unit. Spotted as candidate #014's leftover note about `x86_64-unknown-trusty`
(another unwrap-instead-of-diagnostic in the same ISA-construction path).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct differential run on the in-tree target: tiny `#![no_core]` crate, `--target x86_64-unknown-trusty --emit=obj`, plain rustc vs `dist/rustc-clif` | main tree @ 2bc05d45, opt-level 0 | rustc: object emitted. cg_clif: ICE `src/lib.rs:365` `called Result::unwrap() on an Err value: BadName("generic")`. Reproduced on the first attempt, 5/5 afterwards. |
| Boundary-value construction around the suspect operation: custom target-spec JSONs for x86_64 with the `"cpu"` key **omitted** (rustc then defaults it to `generic`), and with `"cpu": "znver5"` / `"pantherlake"` / `"no-such-cpu-12345"` | direct `rustc`/`rustc-clif`, `-Zunstable-options --target <spec>.json`, opt-level 0 | Every one ICEs the same way, with the offending name echoed in `BadName(...)`. This is the general form of the bug: it is not specific to trusty, any custom target spec that omits `cpu` or names a newer cpu hits it. |
| Real end-to-end differential run: `no_std`/`no_main` x86_64 program (raw `write`/`exit` syscalls, integer checksum + decimal formatting), `core` + a `compiler_builtins` stub + the program all compiled by the backend under test, linked into a static freestanding ELF by `rust-lld`, **executed natively**; plus the realistic `cargo-clif build -Zbuild-std=core --target x86_64-unknown-trusty` path | opt-level 0, `panic=abort`, `repro/run.sh` | LLVM: builds, links, runs, prints `7890598189674834828` for every spec. cg_clif: ICE, no binary produced; `cargo-clif -Zbuild-std=core --target x86_64-unknown-trusty` ICEs while codegening `core`. |
| Generated sweep of the whole input space - every x86 cpu name `rustc --print target-cpus` reports, each injected as a custom target spec's `"cpu"` field, with LLVM as the go/no-go filter (`___BUGHUNT___/tools/sweep_target_spec_cpu.sh`, new this session) | 138 entries (the omitted-`cpu` case + all 136 names `--print target-cpus` lists except `native` + one deliberately bogus name), `#![no_core]` crate, classifies CLIF_OK / ERROR / ICE | 61 CLIF_OK, **39 ICE**, 38 rejected by LLVM itself (32-bit-only cpus, plus the bogus name). The 39 include `generic`, the omitted-key case, `znver5`, `znver6`, `arrowlake*`, `pantherlake`, `lunarlake`, `novalake`, `wildcatlake`, `gracemont`, `rocketlake`, `diamondrapids`, `emeraldrapids`, `clearwaterforest`, `graniterapids-d`, every underscore-spelled llvm alias (`icelake_client`, `skylake_avx512`, `core_2nd_gen_avx`, ...) and the four `c86-4g-m*` Zhaoxin cpus. |
| Code reading of `cranelift-codegen-meta-0.135.0/src/isa/x86.rs` preset list | - | Root cause pinned exactly (see below): Cranelift's baseline preset is called `x86-64` (and `baseline`), never `generic`, and its microarchitecture preset list stops well short of llvm's. |
| Fix, then re-run all of the above plus the 331-target `rustc --print target-list` sweep from candidate #014 | patched worktree, `./y.sh build` | All 39 sweep ICEs flip to CLIF_OK, the 61 already-OK names are unchanged, and in the full target-list sweep exactly one target changes status (`x86_64-unknown-trusty` ICE -> CLIF_OK) with no change for the other 330. cg_clif's own binaries run and print the same value as LLVM's. |

## Status

**Confirmed**

Deterministic: the ICE reproduced 5/5 direct `rustc-clif` invocations on `x86_64-unknown-trusty`,
2/2 full `repro/run.sh` runs and 2/2 sweeps, all on the release build of cg_clif @ 2bc05d45
(`./y.sh build`, rustc nightly-2026-08-24). After the patch: 5/5 objects emitted, 2/2 `run.sh`
exit 0, and the binary cg_clif produced printed the correct value 5/5 runs.

**Fully reproducible**: **Yes**. `repro/run.sh` drives the real `dist/rustc-clif` and
`dist/cargo-clif`. For PART 2 the backend under test compiles `core`, the `compiler_builtins` stub
*and* the program, the result is linked into a real static x86_64 ELF and *executed natively*, and
the printed value is compared against LLVM's. Nothing is hand-asserted.

Note on the toolchain plumbing inside the repro: there is no rustup `rust-std` for
`x86_64-unknown-trusty` or for a custom target spec, so `core` is compiled from the `rust-src`
component - either by `cargo -Zbuild-std=core` (PART 1) or by a direct `rustc` invocation on
`library/core/src/lib.rs` (PART 2). PART 2 also supplies a small `compiler_builtins` stub
(`repro/cb_stub.rs`: the four `mem*` symbols plus the 128-bit div/mul-overflow libcalls that
cg_clif emits where llvm inlines them), because rustc demands a `compiler_builtins` crate for a
`no_std` target and cargo's `-Zbuild-std` output is a metadata stub that `--extern` rejects. Both
are ordinary bare-metal build plumbing; the code cg_clif has to compile is real.

## Class

crash (backend ICE on valid Rust)

## Root cause

`src/lib.rs` `build_isa`, in the arm taken when no `-Ctarget-cpu` was given:

```rust
if target_tuple.architecture == target_lexicon::Architecture::X86_64 {
    // Only set the target cpu on x86_64 as Cranelift is missing
    // the target cpu list for most other targets.
    builder.enable(sess.target.cpu.as_ref()).unwrap();
}
```

`sess.target.cpu` is the `cpu` field of rustc's target spec, i.e. an **llvm** cpu name. Cranelift's
x86 settings expose their own preset names (`cranelift-codegen-meta-0.135.0/src/isa/x86.rs`),
deliberately borrowed from llvm's list but neither complete nor identically spelled, and
`isa::Builder::enable` returns `Err(SetError::BadName(name))` for anything that isn't in it. The
`unwrap()` turns that into a panic, so the compiler ICEs on the *first* codegen unit, before any
user code is compiled.

Two specific gaps make this reachable on perfectly ordinary input:

1. **rustc's default cpu is `generic`.** `TargetOptions::default()` sets `cpu: "generic"`, and
   Cranelift spells the same thing `x86-64` (`add_preset("x86-64", "Generic x86-64
   microarchitecture.", preset!())`) or `baseline` - there is no `generic` preset. Every x86_64
   target that doesn't bother to set `cpu` therefore ICEs. Among rustc's built-in targets that is
   exactly one, `x86_64-unknown-trusty` (all the other x86_64 targets say `x86-64`, `penryn` or
   `core-avx2`), but a *custom* target spec JSON with no `"cpu"` key is a completely normal thing
   for OS/embedded work and hits it too.
2. **Cranelift's preset list lags llvm's.** 37 further cpu names that llvm accepts for x86_64 have
   no Cranelift preset - notably every recent microarchitecture (`znver5`, `znver6`, `arrowlake`,
   `lunarlake`, `pantherlake`, `novalake`, `wildcatlake`, `gracemont`, `diamondrapids`, ...) and all
   of llvm's underscore-spelled aliases (`icelake_client`, `skylake_avx512`, `core_2nd_gen_avx`, ...).

Note the neighbouring arm for `-Ctarget-cpu=<value>` already handles the same failure properly with
`sess.dcx().fatal("the specified target cpu isn't currently supported by Cranelift.")`; only the
target-spec path `unwrap()`s. The maintainers have hit this class of failure before: d9bd396c
"Workaround for incorrect target cpu rename in Cranelift" (2026-03-17) papered over one instance
with `sess.target.cpu.replace("-", "_")` and it was reverted again by 1111eb59 "Update to Cranelift
0.130" (2026-03-20); the `unwrap()` itself was never addressed.

## Introduced by

df88c118 "Use the target cpu from the target spec on x86_64 when -Ctarget-cpu isn't used"
(2024-05-12), which replaced the previously hardcoded `builder.enable("nehalem").unwrap()` with
`builder.enable(sess.target.cpu.as_ref()).unwrap()`; it reached this repo through the subtree sync
merge ed7d97e4 "Merge commit '3270432f4b0583104c8b9b6f695bf97d6bbf3ac2' into
sync_cg_clif-2024-05-13" (2024-05-13). Found with `git log -S 'builder.enable(sess.target.cpu' --all`
plus `git show` on the hit; no bisect needed. Before df88c118 the argument was a constant that was
always a valid preset, so the `unwrap()` could not fail.

## The fix

`patch.diff` (1 file, +16/-1) - `src/lib.rs` only:

1. Map rustc's default cpu name `generic` to Cranelift's `x86-64`. That preset is defined as
   `preset!()`, i.e. it enables nothing beyond Cranelift's own x86-64 defaults, so
   `x86_64-unknown-trusty` now gets bit-for-bit the same ISA settings as `x86_64-unknown-linux-gnu`
   (whose spec says `x86-64`) instead of panicking.
2. Replace the `unwrap()` with a warning: an unrecognised target-spec cpu now falls back to
   Cranelift's defaults (the x86-64 baseline) and emits
   `warning: target cpu `<name>` isn't supported by Cranelift; compiling for the x86-64 baseline
   instead`. Falling back is safe - baseline codegen is correct for every x86-64 cpu, just not
   tuned - and it is what keeps `znver5` & co. compiling instead of turning an ICE into a hard
   error. The `-Ctarget-cpu` arm is deliberately left alone: an explicit user request that can't be
   honoured should keep being a fatal error, not a silently downgraded one.

Verification, not just code reading:

- `repro/run.sh` against the pristine tree: exit 1, `BadName("generic")` / `BadName("znver5")` ICE,
  no binary. Against the patched tree: exit 0, `cargo-clif -Zbuild-std=core --target
  x86_64-unknown-trusty` builds clean, and for both custom specs cg_clif's own `core` + program
  link into a static ELF that runs and prints `7890598189674834828`, byte-identical to LLVM's. Full
  captured output in `output.txt`.
- Varied within the bug class (CONFIRMATION.md section 7) rather than fixing the one input: the
  generated sweep re-run against the patched backend turns **all 39** ICEing cpu names into
  CLIF_OK (including the omitted-key/`generic` case, the underscore aliases and the modern
  Zen/Intel cpus) while leaving all 61 previously-working names unchanged;
  `repro/spec_cpu_sweep_{before,after}.txt` hold both runs. A deliberately bogus
  `"cpu": "no-such-cpu-12345"` was checked by hand rather than through the sweep (llvm rejects that
  spec, so there is no differential to run): the patched cg_clif emits the new warning and compiles
  the crate instead of ICEing.
- No collateral damage: the 331-target `rustc --print target-list` sweep from candidate #014,
  re-run on the patched backend, differs from the pristine run in exactly one line
  (`x86_64-unknown-trusty|ICE` -> `x86_64-unknown-trusty|CLIF_OK`); the other 14 ICEs, 224 clean
  diagnostics and 90 CLIF_OK targets are untouched. `repro/target_sweep_after.txt`.

- Project test suite: `./y.sh test` was run in the worktree both before and after the patch.
  Before: exit 0, `test result: ok. 1200 passed; 0 failed`. After: exit 0, same
  `1200 passed; 0 failed`. `./y.sh test` aborts on the first failing item, so exit 0 means every
  item in `config.txt` (example/, the in-tree AOT/JIT smoke tests and the external repos seeded in
  `download/`) passed in both runs. No new failures.

## What's not done / open questions

- The fix makes these targets *compile*; it does not claim they are otherwise usable.
  `x86_64-unknown-trusty` now gets through `core` and a small `no_std` crate, but nothing here
  exercises trusty's std or its ABI.
- Whether a `warning` is the diagnostic the maintainers want is a judgement call. The alternatives
  are silence (indistinguishable from "cg_clif honoured your cpu") or a hard error (which would
  leave `x86_64-unknown-trusty` and every `znver5` custom spec uncompilable, i.e. no better than
  today for those users). It fires at most once per crate and only for names Cranelift genuinely
  lacks.
- The warning is currently unconditional for unknown names; a maintainer may prefer it gated
  behind a lint so it can be allowed, or suppressed entirely for cpu names that are merely
  *newer* than Cranelift's list rather than wrong.
- Adjacent, not fixed here (different code path, hard error rather than ICE, so it is a separate
  bug class): `-Ctarget-cpu=generic` - a completely ordinary llvm cpu name that cargo users do
  pass - hits the `Some(value)` arm and fails with `error: the specified target cpu isn't currently
  supported by Cranelift.` (checked directly: `-Ctarget-cpu=generic` and `-Ctarget-cpu=znver5`
both hard-error, `-Ctarget-cpu=x86-64` and `-Ctarget-cpu=haswell` compile). The same
  `generic` -> `x86-64` mapping would fix the `generic` case there too, but "an explicit
  `-Ctarget-cpu` Cranelift can't honour" arguably *should* stay an error, so it needs a maintainer
  decision rather than a patch from here.
- Not investigated: the aarch64/riscv64 side deliberately never calls `builder.enable(...)` at all
  ("Cranelift is missing the target cpu list for most other targets"), so `-Ctarget-cpu` /
  `target.cpu` are silently dropped there. That is a feature gap, not this ICE, and was left alone.
- The other ICE family from candidate #014's sweep is still open: the six `aarch64_be-*` targets
  ICE on a Cranelift-internal assert (`cranelift-codegen-0.135.0/src/isa/aarch64/mod.rs:267`)
  instead of cg_clif rejecting big-endian aarch64 with a diagnostic.
