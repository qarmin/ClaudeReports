# auxv_phent_store_missing_exe_phdrs_stride_zero

`src/backend/linux_raw/param/auxv.rs`'s `init_from_aux_iter` parses `AT_PHENT` into a local
`phent` and even asserts it against `size_of::<Elf_Phdr>()`, but unlike every sibling field
(`pagesz`, `hwcap`, `phdr`, `phnum`, `entry`, `random`, ...) the final block of
`FOO.store(foo, Relaxed)` calls never includes `PHENT.store(phent, Relaxed)`. Flagged in candidate
034's RESULT.md as "worth its own candidate" but never written up until now.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | Read `init_from_aux_iter` end to end, compared the final storage block against every `#[cfg(feature = "runtime")]` static it's supposed to populate | Confirmed statically: `PHENT` is declared, loaded in `exe_phdrs()`, parsed into the `phent` local, and even used in `assert_eq!(phent, size_of::<Elf_Phdr>())` a few lines above the storage block - but no `PHENT.store(...)` call exists anywhere in the file. `PHENT`'s `AtomicUsize::new(0)` initializer is therefore permanent. |
| Real end-to-end repro, release build | Standalone crate, public API `rustix::runtime_448b8ad740e2a26f::exe_phdrs()`, compared against ground truth read independently from this same process's own `/proc/self/auxv` | Reproduces 5/5 runs (plus 3 more before/after captured into `output.txt`): `exe_phdrs()` always returns `phent=0` while the kernel-reported `AT_PHENT` for the exact same process is `56` (the real `size_of::<Elf64_Phdr>()` on this x86_64 host). Deterministic, not layout/seed-dependent - this is a straight missing-store logic bug, not UB, so a third escalating tool (miri/ASAN/fuzz) would add nothing beyond what's already fully explained by reading the code and observing the real public API's output - soft cap intentionally not exhausted. |

## Status

**Confirmed**: reproduces 5/5 runs in the initial check, plus a further 3/3 runs captured as the
"before" half of `output.txt` - deterministic, release build, host x86_64 Linux (the local dev
container). No sanitizer/miri needed: this is a plain missing `.store()` call, not memory-unsafety
or UB, and the symptom is 100% deterministic once a `runtime`-feature caller's auxv genuinely
contains a nonzero `AT_PHENT` (true for every normal Linux exec of a real ELF binary - `AT_PHENT`
is supplied by the kernel's own ELF loader itself, it isn't something a caller can omit).

**Fully reproducible**: **Yes**. `repro/` calls the real, public
`rustix::runtime_448b8ad740e2a26f::exe_phdrs()` API exactly as any real caller (e.g. a from-scratch
TLS-setup routine walking program headers) would, running in the real process the kernel handed a
real auxv to at exec time. The "ground truth" side isn't hand-authored either - it's this exact
process's own `/proc/self/auxv`, i.e. the kernel's own record of what it actually gave this process,
read completely independently of rustix. There's no second participant being faked here: the single
external input (the kernel's real auxv) is genuine, not synthesized - this falls under
`CONFIRMATION.md` section 11's explicit "single-process repro remains completely fine" case, not its
"must run a real second participant" case, since there's no peer/caller whose reaction is being
impersonated.

## Class

logic

## Root cause

`init_from_aux_iter` builds up a set of local variables from the real auxv, then stores each one
into its corresponding `static` at the end of the function so later calls to `page_size()`,
`linux_hwcap()`, `exe_phdrs()`, etc. can read them without re-parsing auxv. For `phent` specifically,
every step up to the last is done correctly: the `AT_PHENT => phent = a_val as usize` match arm
fires, and the value is even consumed further down by
`assert_eq!(phent, size_of::<Elf_Phdr>())`. But the final storage block
(lines ~450-466 of the pristine file) stores `PHDR`, then jumps straight to `PHNUM`, skipping
`PHENT` entirely - `PHENT.store(phent, Relaxed)` simply doesn't exist anywhere in the file. Since
`PHENT` is a `static AtomicUsize = AtomicUsize::new(0)`, every read of it (in `exe_phdrs()`) returns
the initializer value `0` forever, regardless of what the real auxv said. `exe_phdrs()`'s public
contract is "`(phdr, phent, phnum)` describing the real program-header table of this process" - it
silently breaks that contract for `phent` only, while `phdr`/`phnum` remain correct, which is what
makes the bug easy to miss: two of the three return values are always right.

## Introduced by

2c56e441 "Don't panic if `AT_PHENT` doesn't match our expectations." (#808) (2023-08-31)

Before this commit, `exe_phdrs()` returned only `(phdr, phnum)` (no `phent` at all) and there was no
`PHENT` static - the `phent` local existed purely to feed the (then-panicking)
`assert_eq!(phent, size_of::<Elf_Phdr>())`. This commit changed `exe_phdrs()`'s signature to
`(phdr, phent, phnum)`, added the `PHENT` static, and added the `PHENT.load(Relaxed)` reads in
`exe_phdrs()` - but never added the matching `PHENT.store(phent, Relaxed)` in `init_from_aux_iter`'s
storage block, so the newly-added read side was wired up to a static that nothing ever writes.
Verified directly: `git show 2c56e441^:...auxv.rs` has no `PHENT` static and a 2-tuple
`exe_phdrs()`; `git show 2c56e441:...auxv.rs` has the 3-tuple, the `PHENT` static, and the two
`PHENT.load(Relaxed)` call sites, but the storage block in that same commit's diff is untouched
apart from `PHDR.store`/`PHNUM.store` staying exactly as they were - `PHENT.store` was never added
in this commit or any later one (confirmed via `git log -S "static PHENT"` showing only this one
commit touching the `PHENT` static's declaration).

## The fix

Add the one missing line, in the same position its sibling `PHDR`/`PHNUM` stores already occupy:

```rust
#[cfg(feature = "runtime")]
PHDR.store(phdr, Relaxed);
#[cfg(feature = "runtime")]
PHENT.store(phent, Relaxed);
#[cfg(feature = "runtime")]
PHNUM.store(phnum, Relaxed);
```

This is the only change in `patch.diff` (+2/-0 lines, including the `#[cfg(...)]` attribute line).
It's sufficient because `phent` is already correctly parsed and validated earlier in the same
function (the `assert_eq!` two lines above it already proves the local holds the real value by the
time the storage block runs) - the only gap was wiring that already-correct local to its static.

Checked against more than the one input that first surfaced it (CONFIRMATION.md section 7): this
isn't an adversarial-input bug with a "boundary value" to vary - the triggering condition is simply
"a real process's auxv contains a nonzero `AT_PHENT`", which is true unconditionally for every
normal exec on this host (re-confirmed across all 8 repro runs captured in `output.txt`, all showing
the identical real `AT_PHENT=56`/`Elf64_Phdr` size on this x86_64 host) - there is no narrower input
class the fix could accidentally be overfit to.

- Real end-to-end repro (`repro/`, via `just test` from this folder): before the patch, `phent=0`
  reported while the real kernel-provided `AT_PHENT=56`, 5/5 runs (plus 3/3 more captured in
  `output.txt`), repro exits 1 ("BUG CONFIRMED..."). After `git apply patch.diff`, `phent=56`
  (matches ground truth) reported, 3/3 runs, repro exits 0.
- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host x86_64, run inside this candidate's own git worktree): **411 passed, 1 failed, identical
  before and after** the patch. The one failure is the pre-existing, environment-caused
  `ioctl::test_int_setter` (`NotFound` opening `/dev/net/tun`, absent in this sandboxed container;
  same failure already documented against candidate 034 and in `___BUGHUNT___/projects/rustix.md`).
  No new failures introduced by the fix, no previously-failing test newly passes (this bug has no
  test coverage either way currently).

## What's not done / open questions

- No test currently exercises `exe_phdrs()`'s `phent` return value at all (the whole bug existed
  silently since 2023-08-31 precisely because nothing checks it) - a regression test comparable to
  this candidate's `repro/` (compare against `/proc/self/auxv`'s own `AT_PHENT`) would be a natural
  thing for a maintainer to add alongside this fix; not added here since the candidate loop scopes
  the fix to the bug itself, not new test infrastructure.
- Only the default linux_raw auxv backend (`init_from_aux_iter`, used by
  `pr_get_auxv_dynamic`/`init_from_auxv_file`) was checked. Per candidate 034's own note and
  confirmed again by reading the sibling files directly: `libc_auxv.rs`'s `exe_phdrs()` calls libc's
  `getauxval(AT_PHENT)` itself on every call (no caching, no missing store, not affected), and the
  explicit-auxv `init.rs` backend (`use-explicitly-provided-auxv` feature) stores its parsed
  `AT_PHENT` into its own `PHENT` static correctly inline, in the same function, right next to
  `PHDR`/`PHNUM` - also not affected. This bug is specific to the default `linux_raw` backend's
  `auxv.rs` file.
- Not checked: whether any downstream consumer of `rustix::runtime::exe_phdrs()`'s `phent` field
  (none in-tree in this crate - it's purely part of the public, docs-hidden `runtime` API surface
  for external crates to use, e.g. a from-scratch TLS-setup routine) is affected in a way beyond
  "would stride by 0 instead of 56" - that's inherently about how an out-of-tree caller uses the
  value, which is outside the scope of what this repository's own tests can exercise.
