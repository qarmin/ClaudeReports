# aarch64_none_binary_format_unknown_ice

`cg_clif` derives the Cranelift target triple by string-parsing rustc's LLVM target tuple with
`target-lexicon`, and `target-lexicon` cannot infer an object format from a bare-metal tuple like
`aarch64-unknown-none`. The resulting `BinaryFormat::Unknown` makes `cranelift_object::ObjectBuilder::new`
return `Err`, which `src/driver/aot.rs` `unwrap()`s - so every codegen unit for those targets ICEs.
Spotted while looking at `make_module`'s unconditional `unwrap()` (candidate #008's leftover note).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Minimal `#![no_core]` crate, `--target aarch64-unknown-none --emit=obj`, plain rustc vs `dist/rustc-clif` | main tree @ 2bc05d45, opt-level 0 | rustc: object emitted. cg_clif: ICE `aot.rs:52` `Err(Backend(binary format is unknown))`. Reproduced on the first attempt. |
| Generated sweep over the whole rustc target list (331 targets), same `no_core` crate, LLVM as the go/no-go filter | script over `rustc --print target-list`; classifies CLIF_OK / ERROR / ICE | 90 CLIF_OK, 224 clean diagnostics, 15 ICE, 2 targets LLVM itself refuses. **8 of the 15 ICEs are this bug**: aarch64-unknown-none, aarch64-unknown-none-softfloat, aarch64v8r-unknown-none, aarch64v8r-unknown-none-softfloat, aarch64-unknown-nuttx, aarch64-unknown-teeos, aarch64-kmc-solid_asp3, aarch64-nintendo-switch-freestanding. |
| Real end-to-end differential run: bare-metal aarch64 `#![no_std]` binary (raw `write`/`exit` syscalls), linked static, executed under `qemu-aarch64`; `cargo` + `-Zbuild-std=core` and direct-`rustc` variants | opt-level 0, `panic=abort`, `repro/run.sh` | LLVM: builds, links, runs, prints `7890598189674834828`. cg_clif: ICE, no binary produced. `cargo-clif build -Zbuild-std=core --target aarch64-unknown-none` ICEs too (while codegening `core`). |
| Code reading of `target-lexicon` 0.13.5 `default_binary_format` + `cranelift-object` `ObjectBuilder::new` | - | Root cause pinned exactly (see below): `OperatingSystem::None_` + `Environment != Eabi/Eabihf` -> `BinaryFormat::Unknown`. |
| Fix, then re-run the 331-target sweep and the qemu differential | patched worktree, `./y.sh build` | Exactly those 8 targets flip ICE -> CLIF_OK; no other target's status changes. Both aarch64 bare-metal targets now run under qemu and print the same value as LLVM. |

## Status

**Confirmed**

Deterministic: the ICE reproduced 5/5 direct `rustc-clif` invocations, plus 2/2 full `repro/run.sh`
runs and 2/2 sweeps, all on the release build of cg_clif @ 2bc05d45 (`./y.sh build`, rustc
nightly-2026-08-24). After the patch, the patched backend produced a running binary printing the
correct value 5/5 runs.

**Fully reproducible**: **Yes**. `repro/run.sh` drives the real `dist/rustc-clif` and
`dist/cargo-clif` on real valid Rust; on the LLVM side and (after the fix) on the cg_clif side it
links the emitted object into a real static aarch64 ELF and *executes* it under `qemu-aarch64`,
comparing the actual printed value. Nothing about the failure is hand-asserted.

Note on the toolchain plumbing inside the repro: there is no rustup `rust-std` for
`aarch64-unknown-none`, so `core` is compiled from the `rust-src` component - either by
`cargo -Zbuild-std=core` (Part 1) or by a direct `rustc` invocation on `library/core/src/lib.rs`
(Part 2). Part 2 also supplies a 4-function `compiler_builtins` stub (`repro/cb_stub.rs`), because
rustc demands a `compiler_builtins` crate for a `no_std` target and cargo's `-Zbuild-std` output is
a metadata stub that `--extern` rejects. Both are ordinary toolchain plumbing for a bare-metal
build; the crate under test and the code cg_clif has to compile are real.

## Class

crash (backend ICE on valid Rust - a whole target family cannot be compiled at all)

## Root cause

`src/lib.rs` `target_tuple` builds the `target_lexicon::Triple` that Cranelift uses purely by
parsing rustc's LLVM target *string*:

```rust
match back::versioned_llvm_target(sess).parse() { ... }
```

`target-lexicon` then infers the object format from the parsed os/env
(`target-lexicon-0.13.5/src/targets.rs` `default_binary_format`):

```rust
OperatingSystem::None_ => match triple.environment {
    Environment::Eabi | Environment::Eabihf => BinaryFormat::Elf,
    _ => BinaryFormat::Unknown,
},
```

All eight affected rustc targets have the LLVM tuple `aarch64-unknown-none` - os `none`, no
environment field - so they land on the `_ => BinaryFormat::Unknown` arm. `cranelift-object`'s
`ObjectBuilder::new` maps that straight to
`Err(ModuleError::Backend(anyhow!("binary format is unknown")))`, and `src/driver/aot.rs:52`
`unwrap()`s it, so the whole compiler panics on the *first* codegen unit.

Two details explain why the neighbouring bare-metal targets are fine: `x86_64-unknown-none`'s LLVM
tuple is `x86_64-unknown-none-elf` (the `-elf` suffix is parsed as an explicit binary format), and
the `*-none-eabi(hf)` ARM targets hit the `Eabi`/`Eabihf` arm above. It is exactly the tuples that
spell neither that break.

The information cg_clif needs is already available and authoritative: rustc's own target spec
carries `sess.target.binary_format` (`BinaryFormat::Elf` for all eight). cg_clif simply never
consulted it and relied on `target-lexicon`'s inference instead. The same inferred value is also
used to pick Cranelift's `tls_model` in `build_isa`, which silently selected `"none"` instead of
`"elf_gd"` for these targets.

## Introduced by

0471ed27 "Add support for object::write" (2019-10-18) - the commit that introduced `make_module`
with `ObjectBuilder::new(...).unwrap()`; in the current file layout the same `unwrap()` arrived in
`src/driver/aot.rs` with 5d3b8e7c "Merge commit '61667ded...' into sync_cg_clif-2021-09-19"
(2021-09-19). The other half, deriving the Cranelift triple by parsing the LLVM target string, has
been there since d078fde0 (2019-05-25). No single commit "broke" bare-metal aarch64 - the ICE
became *reachable* only once Cranelift grew an aarch64 backend and cg_clif started accepting the
target, which git blame on these lines cannot date; treat the two hashes above as where the
unwrapped-inferred-format pattern came from rather than as a regression point.

## The fix

`patch.diff` (2 files, +21/-3):

1. `src/lib.rs` `target_tuple` now overwrites the parsed triple's `binary_format` with rustc's own
   `sess.target.binary_format`, mapped variant-for-variant
   (Coff/Elf/MachO/Wasm/Xcoff -> Coff/Elf/Macho/Wasm/Xcoff). This is a total match, so no target
   can fall back to `Unknown` any more.
2. `src/driver/aot.rs` replaces the `unwrap()` with `tcx.sess.dcx().fatal(...)`, so that if
   `ObjectBuilder::new` ever rejects a format again (e.g. wasm) the user gets
   `error: can't emit object files for <llvm-target>: ...` instead of an ICE. Defensive only - with
   (1) in place no currently-supported target reaches it.

Verification, not just code reading:

- `repro/run.sh` against the pristine tree: exit 1, `Backend(binary format is unknown)` ICE, no
  binary. Against the patched tree: exit 0, cg_clif's own binary runs under `qemu-aarch64` and
  prints `7890598189674834828`, byte-identical to LLVM's. Full captured output in `output.txt`.
- Varied within the bug class (CONFIRMATION.md section 7): the repro covers two targets
  (`aarch64-unknown-none` and `aarch64-unknown-none-softfloat`, i.e. a different float ABI), and
  the 331-target sweep re-run against the patched backend shows all 8 affected targets flipping
  ICE -> compiles-clean, with **no** status change for any of the other 323 targets (98 CLIF_OK
  after vs 90 before; 224 clean diagnostics unchanged).

- Project test suite: `./y.sh test` was run in the worktree both before and after the patch.
  Before: exit 0, last reported item `test result: ok. 1200 passed; 0 failed`. After: exit 0, same
  `1200 passed; 0 failed`. `./y.sh test` aborts on the first failing item, so exit 0 means every
  item in `config.txt` (example/, the in-tree AOT/JIT smoke tests and the external repos seeded in
  `download/`) passed in both runs. No new failures.

## What's not done / open questions

- The fix makes the eight targets *compile*; it does not claim the whole bare-metal aarch64 path is
  usable. Compiling `core` itself for aarch64 with cg_clif still fails on unrelated, pre-existing
  gaps, now as proper diagnostics rather than an ICE: `error: 128bit atomics not yet supported`
  (in `core`) and `error: attempt to declare __gttf2 with signature ... returns [I64] ... but it
  was already declared with ... returns [I32]` (in `compiler_builtins`). The second one looks like
  a real libcall-signature bug in cg_clif and is worth its own candidate.
- Two other ICE families showed up in the same sweep and were *not* investigated: the six
  `aarch64_be-*` targets ICE on a Cranelift-internal assert
  (`cranelift-codegen-0.135.0/src/isa/aarch64/mod.rs:267`) instead of cg_clif rejecting big-endian
  aarch64 with a diagnostic, and `x86_64-unknown-trusty` ICEs at `src/lib.rs` on
  `builder.enable(sess.target.cpu).unwrap()` -> `BadName("generic")`.
- `build_isa` now derives `tls_model = "elf_gd"` for these targets where it previously derived
  `"none"`. That is what LLVM does for bare-metal ELF and none of these targets sets
  `has_thread_local`, so nothing in the test suite or the repro exercises it - but a maintainer may
  want to confirm the intent.
- Whether an `os = "none"` target should be *supported* at all by cg_clif is a maintainer decision;
  this patch takes the position that if cg_clif accepts the target it must not ICE. If the answer
  is "unsupported", the `aot.rs` half of the patch already provides the diagnostic shape for it.
