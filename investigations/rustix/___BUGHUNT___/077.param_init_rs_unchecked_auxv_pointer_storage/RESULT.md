# param_init_rs_unchecked_auxv_pointer_storage

`src/backend/linux_raw/param/init.rs` is the alternate auxv-parsing file compiled instead of
`src/backend/linux_raw/param/auxv.rs` when the `use-explicitly-provided-auxv` feature is enabled
(a mode for custom no-libc process-startup runtimes, e.g. the `origin` crate, that call the public
`rustix::param::init(envp)` themselves very early in `_start`). Its `init_from_auxp` stores
`AT_EXECFN`/`AT_PHDR`/`AT_RANDOM`/`AT_SYSINFO_EHDR` straight into `AtomicPtr` with **zero**
validation - not even candidate 034's `check_raw_pointer`-based null/alignment check that panics on
a bad value in the sibling `auxv.rs` file. This looked like it should be strictly worse than 034's
already-Confirmed finding (silent corruption/UB instead of a controlled panic).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: which `init.rs`-populated fields are actually dereferenced inside rustix itself | Traced every consumer of `EXECFN`/`PHDR`/`RANDOM`/`SYSINFO_EHDR` | Only two are dereferenced by rustix's own code: `linux_execfn()` (`src/param/auxv.rs`) does `CStr::from_ptr(EXECFN)` unconditionally, and `vdso.rs::init_from_sysinfo_ehdr()` does `let hdr = &*hdr;` on `SYSINFO_EHDR` after only an `is_null()` check. `exe_phdrs()`/`random()`/`entry()` all just return the raw pointer/usize to the caller without dereferencing it anywhere in this crate - so a bad `AT_PHDR`/`AT_RANDOM` cannot crash rustix itself, only whatever external caller later dereferences the returned pointer (out of scope for this crate's own code) |
| `miri` on a synthetic call to the real public `rustix::param::init()` | `cargo +nightly miri run`, `use-explicitly-provided-auxv,param,runtime` features, `MIRIFLAGS="-Zmiri-disable-isolation"` | **Dead end, not a false negative**: `build.rs` forces the `libc` backend whenever `CARGO_CFG_MIRI` is set (miri can't execute the `linux_raw` backend's inline `asm!` syscalls), and `init.rs` only compiles under `cfg(not(libc))` - so this candidate's own subject file is never even compiled under Miri. Confirmed via the actual compile error (`cannot find function init in module rustix::param`, "item configured out"), not assumed |
| Digging: real public API (`rustix::param::init()` + `rustix::param::linux_execfn()`), debug build, **synthetic/hand-built envp+auxv array** (not a real producer - faked second participant, digging only per `CONFIRMATION.md` section 11) | `cargo run` (debug), native x86_64 host, `AT_EXECFN` set to the deliberately-invalid pointer `0x1` | **Segfaults immediately, 5/5 runs** (1 via `just test`'s fresh-clone cycle + 4 direct repeat-runs of the same binary), no panic, no validation of any kind - confirms the code-level mechanism is real and does fail harder than 034's panic-based check |
| Real-kernel probing for a legitimate producer of an invalid `AT_EXECFN`/`AT_SYSINFO_EHDR`: normal `execve()`, and `execveat(fd, "", ..., AT_EMPTY_PATH)` via `memfd_create` (the one realistic "unusual" real exec path not covered by candidate 034's own probing) | Native host, real unmodified kernel, small C harness (`/tmp/execfn_probe{,2}.c`) printing `getauxval(AT_EXECFN)` | Both paths produce a valid, non-null, correctly-pointing `AT_EXECFN` string (`"/dev/fd/3"` for the `memfd`+`AT_EMPTY_PATH` case, the real executable path otherwise) - no invalid value observed. Combined with candidate 034's own real-kernel probing of the same auxv surface (`PR_GET_AUXV` sanity check on real exec, a real `qemu-aarch64` exec, and the `CAP_SYS_RESOURCE`-gated `PR_SET_MM_AUXV` mechanism, all already covering `AT_EXECFN`/`AT_RANDOM`/`AT_SYSINFO_EHDR`, not just `AT_PHDR`), there remains no known way for a real, unmodified Linux kernel to hand a correctly-written `use-explicitly-provided-auxv` caller an invalid value for either of the two fields `init.rs` actually dereferences internally |

## Status

**Inconclusive.** A real signal was seen (the digging harness above segfaults 5/5 runs, debug
build, deterministic, no fixed seed needed), demonstrating the code-level mechanism is genuinely
unsound - but per `CONFIRMATION.md` section 11 this does not qualify as `Confirmed`, because the
envp/auxv array fed to the real `rustix::param::init()` call is hand-built by the harness, not
produced by a real second participant (a genuine kernel `execve()` handing a genuinely-invalid
value to a correctly-written caller, the way candidate 034 achieved for `AT_PHDR` via a crafted
`e_phoff`). Extensive real-kernel probing (this session's `execveat(AT_EMPTY_PATH)` test, plus
reuse of candidate 034's broader real-exec/qemu-user/`PR_SET_MM_AUXV` probing of the same auxv
surface) found no such real producer for either of the two fields this file actually dereferences
internally (`AT_EXECFN`, `AT_SYSINFO_EHDR`). The one field that *does* have a known real-kernel
producer of an invalid value - `AT_PHDR`, via candidate 034's `e_phoff` misalignment trick - is
never dereferenced by this file's own code (`exe_phdrs()` only returns the raw pointer), so that
trick does not crash anything inside `init.rs` itself; it would only matter to whatever external,
real `use-explicitly-provided-auxv` caller later dereferences the pointer `exe_phdrs()` returns
(e.g. an `origin`-style runtime doing ELF program-header walking for TLS setup) - not reproduced
this session, out of scope (would require building and driving a second real crate, not just
rustix).

**Fully reproducible**: n/a (not Confirmed).

## Class

n/a (not Confirmed).

## Root cause

Not applicable as a standalone confirmed defect this session, but the code-level gap is real and
documented for future reference: `init_from_auxp` (`src/backend/linux_raw/param/init.rs`,
lines ~144-175) stores `AT_EXECFN`/`AT_PHDR`/`AT_RANDOM`/`AT_SYSINFO_EHDR` directly via
`.store(a_val.cast(), Ordering::Relaxed)` with no null check, no alignment check, and no
`check_raw_pointer`/`check_elf_base` call of any kind - unlike the sibling `auxv.rs` file (candidate
034's subject), which at least validates these same four fields (even though 034 found that
validation itself panics the whole process on failure instead of degrading gracefully). Of the two
fields `init.rs`'s own code later dereferences unconditionally (`linux_execfn()`'s
`CStr::from_ptr`, and `vdso.rs`'s `let hdr = &*hdr`, whose own `# Safety` comment incorrectly
assumes "the auxv initialization code does extensive checks" - true for `auxv.rs`, false for
`init.rs`), neither field's real-kernel value is influenced by anything a `use-explicitly-provided-
auxv` caller's own crafted executable controls, unlike `AT_PHDR`'s `e_phoff`. This mirrors 034's
own caveat for `AT_EXECFN`/`AT_RANDOM` ("no known real producer... that supplies a null value"),
just without 034's `AT_PHDR` anchor to fall back on, since `init.rs` doesn't dereference `AT_PHDR`
itself.

## Introduced by

Not determined / not pursued - not applicable since this candidate isn't `Confirmed`. For context
only: `git log --oneline -- src/backend/linux_raw/param/init.rs` shows the file (under its earlier
`target_vendor = "mustang"` name) predates even 034's `a2c6c7a8` auxv.rs change; the missing
validation looks original to the file's design (a minimal, allocation-free "explicit mode" parser
written before `check_raw_pointer` existed), not a regression - not pinned to one commit this
session.

## The fix

No fix attempted - not `Confirmed`. If a future session does find a real producer for an invalid
`AT_EXECFN`/`AT_SYSINFO_EHDR` (or decides the documented-but-unenforced `init()` precondition
itself is worth hardening defensively regardless of real-producer reachability), the natural fix
mirrors 034's: route `AT_EXECFN`/`AT_SYSINFO_EHDR` (and, for defense-in-depth consistency,
`AT_PHDR`/`AT_RANDOM` too even though they're not internally dereferenced here) through
`check_raw_pointer`/`check_elf_base` the same way `auxv.rs` does, but preferably with 034's own
suggested improvement (degrade to the existing safe default instead of aborting the whole parse)
rather than reproducing 034's panic-on-abort behavior in a second file.

- Project test suite: N/A, no fix was applied this session (not `Confirmed`).

## What's not done / open questions

- A real `use-explicitly-provided-auxv` consumer (e.g. an `origin`-style `_start` that calls
  `rustix::param::init()` with the genuine kernel-provided `envp` and then walks `exe_phdrs()`'s
  program headers for TLS setup) was not built. That would be the real second participant needed
  to turn candidate 034's already-real `e_phoff` trick into a `Confirmed` finding against
  `init.rs` specifically (via `AT_PHDR`, which `init.rs` itself doesn't dereference but a real
  downstream consumer would) - out of scope this session (would mean standing up a second real
  crate/runtime, not just rustix).
- `libc_auxv.rs` (the third variant of this same file, used under `use-libc-auxv`) was not
  compared against either `auxv.rs` or `init.rs` for the same validation gap - worth a follow-up
  candidate (see `IDEAS.md`).
- This session's real-kernel `AT_EXECFN` probing covered normal `execve()` and
  `execveat(AT_EMPTY_PATH)` via `memfd_create`; it did not attempt interpreter-script (`#!`) exec,
  suid/sgid binaries, or non-x86_64 architectures - none of these seemed likely to change the
  kernel's `AT_EXECFN` computation (it's always the copied-in exec pathname string, independent of
  these factors), but it wasn't exhaustively checked.
