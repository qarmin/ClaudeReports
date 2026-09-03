# aarch64_be_cranelift_assert_ice

All six `aarch64_be-*` rustc targets make cg_clif ICE inside Cranelift itself
(`cranelift-codegen-0.135.0/src/isa/aarch64/mod.rs:267`, `assertion failed:
triple.architecture == Architecture::Aarch64(Aarch64Architecture::Aarch64)`) rather than producing
the "can't compile for &lt;target&gt;" diagnostic cg_clif already has for targets Cranelift does not
support. Found by candidate #014's 331-target sweep (6 of the 15 ICEs it recorded) and left alone
there.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Minimal `#![no_core]` crate, `--target aarch64_be-unknown-linux-gnu --emit=obj`, plain rustc vs `dist/rustc-clif` | main tree @ 2bc05d45, opt-level 0 | rustc: big-endian (`ELF 64-bit MSB`) object emitted. cg_clif: ICE at `cranelift-codegen .../isa/aarch64/mod.rs:267`, exit 101. Reproduced on the first attempt. |
| Same probe over all six `aarch64_be-*` targets, plus `armeb-unknown-linux-gnueabi` and `s390x-unknown-linux-gnu` as controls | main tree @ 2bc05d45 | All six aarch64_be targets ICE. `armeb` (an architecture Cranelift has no backend for at all) gives the clean `error: can't compile for armeb-...: Support for this target has not been implemented yet`. `s390x` (big-endian *and* supported) compiles fine - so the failure is specifically the unsupported *sub-variant*, not big-endianness or cross-compilation per se. |
| Real end-to-end path: an ordinary `#![no_std]` lib crate built with `cargo -Zbuild-std=core --target aarch64_be-unknown-linux-gnu` vs `dist/cargo-clif` with the same flags | opt-level 0, `panic=abort` | plain cargo: `libbe_probe.rlib` built. `cargo-clif`: ICEs while codegening `core` itself, "error: the compiler unexpectedly panicked. This is a bug", no output. |
| Real end-to-end run: bare-metal `#![no_std]`/`#![no_main]` aarch64_be binary (raw `write`/`exit` syscalls), `--target aarch64_be-unknown-none-softfloat`, linked static by rust-lld, executed under `qemu-aarch64_be` | opt-level 0, `panic=abort`, `repro/run.sh` | LLVM: builds, links (`ELF 64-bit MSB executable, statically linked`), runs, prints `7890598189674834828`. cg_clif: ICE, no binary produced. |
| `-C target-cpu=native` variant of the same probe (this path bypasses `isa::lookup` entirely) | main tree @ 2bc05d45 | No ICE, but cg_clif silently emits an **x86-64 little-endian** object for `--target aarch64_be-unknown-linux-gnu`. Recorded as a separate suspect below; the fix in this candidate happens to also cover it. |
| Code reading of `cranelift-codegen` 0.135 `isa::lookup` / `isa::aarch64::isa_builder` and cg_clif `build_isa` | - | Root cause pinned exactly (see below): `lookup` dispatches on the architecture *family*, `aarch64::isa_builder` then `assert!`s the exact variant. |
| 331-target blast-radius sweep (`repro/sweep_targets.sh`, reused from candidate #014) against the pristine tree and the patched worktree | `rustc --print target-list`, LLVM as the go/no-go filter | Before: 90 CLIF_OK / 224 ERROR / 15 ICE / 2 LLVM_FAIL. After: 90 CLIF_OK / 230 ERROR / 9 ICE / 2 LLVM_FAIL. Exactly the six aarch64_be targets flip ICE -> clean diagnostic; **no other target's status changes**. |

## Status

**Confirmed**

Deterministic: the ICE reproduced 5/5 direct `dist/rustc-clif` invocations, plus 2/2 full
`repro/run.sh` runs and 2/2 whole-target-list sweeps, all on the release build of cg_clif @
2bc05d45 (`./y.sh build`, rustc nightly-2026-08-24 / 1.100.0-nightly fb6531d55). After the patch
the clean diagnostic appeared 5/5 runs and `repro/run.sh` exits 0.

**Fully reproducible**: **Yes**. `repro/run.sh` drives the real `dist/rustc-clif` and
`dist/cargo-clif` on ordinary valid Rust (a `#![no_std]` lib crate, plus `core` itself via
`-Zbuild-std=core`) and observes the real backend's real ICE. On the LLVM side it links the emitted
big-endian object into a real static `aarch64_be` ELF and *executes* it under `qemu-aarch64_be`,
printing `7890598189674834828`, which establishes that the target genuinely works and that the
failure is cg_clif's. Nothing is hand-asserted.

Note on the toolchain plumbing inside the repro: there is no rustup `rust-std` for any
`aarch64_be-*` target, so `core` is compiled from the `rust-src` component - by
`cargo -Zbuild-std=core` in part 1 and by a direct `rustc` invocation on `library/core/src/lib.rs`
in part 2. Part 2 also supplies a 4-function `compiler_builtins` stub (`repro/cb_stub.rs`), because
rustc demands a `compiler_builtins` crate for a `no_std` target and cargo's `-Zbuild-std` output is
a metadata stub that `--extern` rejects. Both are ordinary bare-metal build plumbing (copied from
candidate #014's repro); the crate under test and the code cg_clif has to compile are real.

## Class

crash (backend ICE on valid Rust - the failure mode is an ICE instead of the diagnostic the backend
already emits for every other target Cranelift cannot handle)

## Root cause

`cranelift_codegen::isa::lookup` (`cranelift-codegen-0.135.0/src/isa/mod.rs:108`) dispatches on the
architecture **family**, deliberately ignoring the sub-variant:

```rust
Architecture::Aarch64 { .. } => isa_builder!(aarch64, (feature = "arm64"), triple),
```

The aarch64 backend's own entry point then asserts the exact variant
(`.../src/isa/aarch64/mod.rs:265`):

```rust
pub fn isa_builder(triple: Triple) -> IsaBuilder {
    assert!(triple.architecture == Architecture::Aarch64(Aarch64Architecture::Aarch64));
```

`target_lexicon::Aarch64Architecture` has two variants, `Aarch64` and `Aarch64be`, and all six
`aarch64_be-*` rustc targets parse to `Architecture::Aarch64(Aarch64Architecture::Aarch64be)`. So
`lookup` routes them into the little-endian aarch64 backend, which panics - it never returns
`Err(LookupError::Unsupported)`.

cg_clif's `src/lib.rs` `build_isa` is written on the assumption that `lookup` reports unsupported
targets by returning `Err`:

```rust
let mut builder = cranelift_codegen::isa::lookup(target_tuple.clone()).unwrap_or_else(|err| {
    sess.dcx().fatal(format!("can't compile for {}: {}", target_tuple, err));
});
```

That `unwrap_or_else` is what turns `armeb` (a family Cranelift has no backend for) into a clean
diagnostic. It cannot help here, because the panic happens *inside* `lookup` before any `Result` is
produced. Big-endian aarch64 is the only case of this shape among the architectures Cranelift knows
about: `s390x`'s `isa_builder` asserts on a variant-less `Architecture::S390x`, `riscv64`'s accepts
`Riscv64(..)` with any sub-variant, and `target-lexicon` has no big-endian x86 or RISC-V variants.

Cranelift has no big-endian AArch64 support at all, so nothing can be codegen'd for these six
targets either way - the bug is purely that the refusal arrives as an ICE ("the compiler
unexpectedly panicked. This is a bug", with a request to file a rustc issue) rather than as the
`error: can't compile for ...` cg_clif already produces for every other unsupported target.

## Introduced by

60340d44 "Don't panic when the target is not supported by Cranelift" (2021-07-17) - the commit that
introduced exactly this handling, replacing `isa::lookup_variant(...).unwrap()` with
`unwrap_or_else(|err| sess.fatal(format!("can't compile for {}: {}", ...)))`. It covers only the
`Err` channel, which is why the assert-inside-`lookup` case still ICEs. Before that commit the
`unwrap()` panicked for unsupported targets anyway, so 60340d44 is where the intent to diagnose
came in and where the gap dates from - it is not a regression point. There is no commit that
"broke" aarch64_be: the ICE became reachable when Cranelift grew an aarch64 backend and cg_clif
started routing aarch64-family triples into it, which `git blame` on these lines cannot date.

## The fix

`patch.diff` (1 file, +13/-0): `src/lib.rs` `build_isa` rejects a non-`Aarch64` value of
`Aarch64Architecture` with `sess.dcx().fatal(...)` before any Cranelift lookup happens:

```rust
if let target_lexicon::Architecture::Aarch64(aarch64_arch) = target_tuple.architecture {
    if aarch64_arch != target_lexicon::Aarch64Architecture::Aarch64 {
        sess.dcx().fatal(format!(
            "can't compile for {}: big-endian AArch64 is not supported by Cranelift",
            target_tuple
        ));
    }
}
```

The comparison is against the one supported variant rather than against `Aarch64be`, so a future
`target-lexicon` variant cannot silently reach the assert again. It is placed before the
`match sess.opts.cg.target_cpu` block on purpose, so it also covers the `-C target-cpu=native`
path, which bypasses `isa::lookup` entirely.

Verification, not just code reading:

- `repro/run.sh` against the pristine tree: exit 1, the Cranelift assert ICE in all three parts, no
  object/binary produced. Against the patched worktree: exit 0, every aarch64_be target reports
  `error: can't compile for aarch64_be-...: big-endian AArch64 is not supported by Cranelift`, and
  the LLVM side still runs its big-endian binary under `qemu-aarch64_be`. Full captured output in
  `output.txt`.
- Varied within the bug class (CONFIRMATION.md section 7): all **six** aarch64_be targets (three
  libcs, hermit, netbsd, bare-metal softfloat, plus the ILP32 ABI variant), two entry paths
  (`rustc-clif --emit=obj` and `cargo-clif -Zbuild-std=core`), and the `-C target-cpu=native`
  variant - which before the patch silently emitted an `ELF 64-bit LSB relocatable, x86-64` object
  for `--target aarch64_be-unknown-linux-gnu` and now errors out.
- Controls unaffected: `aarch64-unknown-linux-gnu`, `aarch64-apple-darwin`,
  `s390x-unknown-linux-gnu`, `x86_64-unknown-linux-gnu`, `riscv64gc-unknown-linux-gnu` stay
  CLIF_OK, and the 331-target sweep shows the six aarch64_be rows as the only changes.

- Project test suite: `./y.sh test` was run in the worktree both before and after the patch.
  Before: exit 0, last reported item `test result: ok. 1200 passed; 0 failed`. After: exit 0, same
  `1200 passed; 0 failed`. `./y.sh test` aborts on the first failing item, so exit 0 means every
  item in `config.txt` (example/, the in-tree AOT/JIT smoke tests and the external repos seeded in
  `download/`) passed in both runs. No new failures.

## What's not done / open questions

- The patch makes the refusal a clean error; it does **not** add big-endian aarch64 support. That
  would have to happen in Cranelift first (there is no big-endian aarch64 backend), so the six
  targets remain uncompilable by cg_clif either way.
- Whether this belongs in cg_clif at all is a maintainer decision: the more durable fix is for
  Cranelift's `isa::lookup` to return `Err(LookupError::Unsupported)` for
  `Architecture::Aarch64(Aarch64be)` instead of asserting inside `aarch64::isa_builder`, at which
  point cg_clif's existing `unwrap_or_else` would handle it with no cg_clif change. The patch here
  is the cg_clif-side workaround that can land without an upstream Cranelift release.
- `-C target-cpu=native` silently ignores `--target` for *every* cross target, not just
  aarch64_be: `Some(NATIVE_CPU) => cranelift_native::builder_with_options(true)` returns the host
  ISA builder unconditionally, so
  `rustc-clif --target aarch64-unknown-linux-gnu -C target-cpu=native` produces an
  `ELF 64-bit LSB relocatable, x86-64` object (verified for `aarch64-unknown-linux-gnu` and
  `s390x-unknown-linux-gnu` too). This candidate's patch only shadows the aarch64_be corner of
  that; the general problem is untouched and is a separate suspect.
- The other 9 ICEs remaining in the 331-target sweep after this patch were not investigated here.
  One of them (`x86_64-unknown-trusty`, `src/lib.rs:378` `builder.enable(cpu).unwrap()` ->
  `BadName("generic")`) is candidate #021.
