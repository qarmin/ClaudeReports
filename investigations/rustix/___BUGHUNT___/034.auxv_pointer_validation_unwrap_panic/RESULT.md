# auxv_pointer_validation_unwrap_panic

`src/backend/linux_raw/param/auxv.rs`'s `init_from_aux_iter` validates the kernel-provided
`AT_EXECFN`/`AT_PHDR`/`AT_RANDOM` pointers via `check_raw_pointer::<T>(...)?` - the `?` aborts the
*entire* aux-vector parse (discarding every other already-parsed value: page size, hwcap, clock
ticks, etc.) and returns `None`, which every one of `init_auxv_impl`'s three call sites then
`.unwrap()`s, panicking the whole process. This looked suspicious because the function has a
documented, deliberate `/proc/self/auxv` fallback design (`init_auxv_impl`'s own doc comment, and
the file's neighboring qemu-specific comment about `/proc/self/fd` checks breaking under emulation)
that a hard panic on a single bad pointer value defeats outright - and because the sibling
`AT_SYSINFO_EHDR` field, validated via the same `check_raw_pointer`-based `check_elf_base`, is
already handled gracefully (`if let Some(value) = check_elf_base(...) { ... }`, falling back to a
null pointer instead of aborting), showing the surrounding code already knows how to do this
correctly for one field but not the other three.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Native `prctl(PR_GET_AUXV)` sanity check | Host kernel 7.0.0-34-generic, real process | Kernel supports `PR_GET_AUXV`; real auxv is always fully populated (no null/misaligned `AT_EXECFN`/`AT_PHDR`/`AT_RANDOM`) under normal exec - confirms the bug needs an unusual auxv producer, not everyday use |
| `qemu-aarch64`/`qemu-user` 10.2.1 real-exec test (hand-written aarch64 asm, static binary) | Real `qemu-aarch64` binary, both `PR_GET_AUXV` and `/proc/self/auxv` paths probed | `PR_GET_AUXV` unimplemented under this qemu version (`-EINVAL`, falls back correctly); `/proc/self/auxv` under qemu is fully, validly populated (`AT_PHDR`/`AT_RANDOM`/`AT_EXECFN`/`AT_PLATFORM` all real, non-null) for a simple static guest binary - this specific qemu version shows no sign of the originally-suspected qemu auxv quirk |
| `PR_SET_MM_AUXV` (the real, kernel-sanctioned way a process can overwrite its own saved auxv, used by CRIU-style checkpoint/restore tools) | Native host, unprivileged uid 1000 | `EPERM` - requires `CAP_SYS_RESOURCE`, unavailable in this sandbox (confirmed via the prctl's own errno, not inferred); no privilege-escalation path available either (`sudo -n` blocked by no-new-privs, `unshare --user` blocked, no docker/podman) |
| Internal unit-level call to the private `init_from_aux_iter` with a hand-built synthetic iterator (digging only, not the final repro - fakes the real auxv producer, so not usable for Confirmed per `CONFIRMATION.md` section 11) | `cargo test`, debug build, default features + `param,runtime,fs,net,stdio` | Confirmed the mechanism directly: `None` returned for a null `AT_EXECFN`/`AT_PHDR`/`AT_RANDOM`, and the exact production `.unwrap()` call pattern panics - 3/3 runs, deterministic. Superseded by the real repro below once found |
| **Real end-to-end repro**: a standalone, statically linked, non-PIE binary calling the actual public API `rustix::runtime_448b8ad740e2a26f::exe_phdrs()`, whose own compiled ELF is corrupted (via `repro/corrupt_phdr.py`) to give it a non-8-byte-aligned `e_phoff` in its real ELF header, then executed by the real, unmodified host Linux kernel | `x86_64-unknown-linux-musl` target, `-C relocation-model=static` (needed so nothing but rustix's own code dereferences `AT_PHDR` before `main`; a normal dynamic/PIE binary's own `ld.so`/TLS setup dereferences it first and crashes before reaching the test) | **Panics**, 5/5 runs at `e_phoff % 8 == 1`, reproduced again at a different misalignment (`e_phoff % 8 == 3`) to rule out one specific byte offset being special - see "Status" |

## Status

**Confirmed**, for the `AT_PHDR` case specifically (misaligned pointer). Reproduces 5/5 runs at one
misalignment residue and again (separately, 3/3 after the fix showing it's gone) at a second,
different residue, debug build, host x86_64, real kernel, no fixed seed needed (the corruption is
deterministic, not layout/ASLR-dependent - `align_of::<Elf_Phdr>() == 8` is a fixed ABI fact, and the
kernel's own `load_addr` bias is always page-aligned, so the parity of the reported `AT_PHDR` is
fully determined by the ELF file's own `e_phoff` field, nothing random).

The `AT_EXECFN`/`AT_RANDOM` siblings share the *identical* code pattern and would panic the same way
given a bad value (`align_of::<c_char>() == 1` and `align_of::<[u8; 16]>() == 1` mean only the
*null*-pointer branch of `check_raw_pointer` can fail for those two, not the alignment branch used
for `AT_PHDR`) - not independently confirmed this session, since neither field's value is derived
from anything a userspace-crafted ELF file controls (unlike `e_phoff`, which directly drives the
kernel's `AT_PHDR` computation): `AT_EXECFN` is a kernel-computed stack pointer, `AT_RANDOM` likewise,
with no known real producer (checked: normal exec, a current qemu-user version, and the
`CAP_SYS_RESOURCE`-gated `PR_SET_MM_AUXV` mechanism - all real, non-null) that supplies a null value
for either in this environment. The fix below is applied identically to all three fields regardless,
since it's the same one-line-root-cause code shape in all three places.

**Fully reproducible**: **Yes**. The repro runs the real public API
(`rustix::runtime_448b8ad740e2a26f::exe_phdrs()` - a genuinely `pub mod`, if deliberately
obscurely-named, part of the crate's real API surface) through its real end-to-end path, driven by
the real, unmodified Linux kernel's own ELF loader genuinely computing a misaligned `AT_PHDR` from a
crafted-but-kernel-spec-valid ELF file (the ELF spec places no alignment requirement on `e_phoff`,
and the kernel's loader - `fs/binfmt_elf.c` - doesn't add one either). This is the "a malformed file
handed to a parser's real public entry point" case `CONFIRMATION.md` section 11 describes as not
needing a second participant to fake: the "parser" here is the kernel's own ELF loader, and it's
real and unmodified; nothing about the auxv-processing side (rustix) or the producing side (the
kernel) is simulated or hand-waved.

## Class

crash

## Root cause

`init_from_aux_iter`'s match arms for `AT_EXECFN`, `AT_PHDR`, and `AT_RANDOM` call
`check_raw_pointer::<T>(a_val as *mut _)?` - `check_raw_pointer` (`src/utils.rs`) returns `None` if
the pointer is null, misaligned for `T`, or would overflow computing `value + size_of::<T>()`. The
`?` here is on `init_from_aux_iter`'s own `Option<()>` return type, so any single bad value for any
of these three fields propagates a `None` out of the *entire* function - discarding every other
field already parsed from the same aux vector (page size, clock ticks, hwcap, etc.), not just the
one bad field. All three of `init_auxv_impl`'s call sites (`init_from_aux_iter(...).unwrap()`, used
whether the aux data came from the fast `PR_GET_AUXV` path or either of the two `/proc/self/auxv`
fallback paths) then panic on that `None`.

This is inconsistent with how the function already treats the sibling `AT_SYSINFO_EHDR` field
(validated through the same `check_raw_pointer`, wrapped in `check_elf_base`): that one already
degrades gracefully via `if let Some(value) = check_elf_base(a_val as *mut _) { sysinfo_ehdr =
value.as_ptr(); }`, leaving `sysinfo_ehdr` at its default null and continuing to parse the rest of
the vector. `AT_EXECFN`/`AT_PHDR`/`AT_RANDOM` have no such guard.

For `AT_PHDR` specifically: `align_of::<Elf_Phdr>() == 8`, and the real Linux kernel's own `AT_PHDR`
computation (`load_addr + hdr.e_phoff`, in `fs/binfmt_elf.c`) places no alignment requirement at all
on the ELF file's own `e_phoff` field - it's simply copied through. A real, valid-per-spec ELF file
with an odd `e_phoff` therefore makes the real kernel report a misaligned `AT_PHDR` to a real
process, which rustix's own `check_raw_pointer::<Elf_Phdr>` correctly flags as suspicious - but
instead of ignoring just that one field (as it already does for `AT_SYSINFO_EHDR`), it aborts the
whole parse and crashes.

## Introduced by

a2c6c7a8 "Switch from `.init_array` constructors to /proc/self/auxv. (#385)" (2022-08-08) -
introduced `check_raw_pointer::<c::c_char>(a_val as *mut _)?` for `AT_EXECFN` and
`check_raw_pointer::<Elf_Phdr>(a_val as *mut _)?` for `AT_PHDR` in their current abort-the-whole-parse
form (confirmed via git log -S on both exact call patterns in this file - no earlier match). The same
commit also introduced the *original*, equally strict version of the `AT_SYSINFO_EHDR`/`AT_BASE`
handling (`check_elf_base(a_val as *mut _)?`), i.e. all of these fields started out equally strict.

000814ca "Don't let a mismatch AT_SYSINFO_EHDR image preclude use of AUX values. (#1484)"
(2025-07-15, fixing issue #1465) later changed *only* the `AT_SYSINFO_EHDR`/`AT_BASE` handling to the
current graceful `if let Some(value) = check_elf_base(...) { ... }` form, with the explicit stated
intent: "If the ELF image pointed to by the `AT_SYSINFO_EHDR` AUX record doesn't match the
architecture rustix is compiled for, don't use it, but do continue to use the rest of the AUX
fields." This is exactly the same principle this candidate's fix applies to `AT_EXECFN`/`AT_PHDR`/
`AT_RANDOM` - PR #1484 just never extended it to those three sibling fields, which still have
a2c6c7a8's original 2022 behavior as of current HEAD (287214b8).

## The fix

Apply the same graceful-degradation pattern PR #1484 already established for `AT_SYSINFO_EHDR` to
the three remaining fields: `if let Some(value) = check_raw_pointer::<T>(a_val as *mut _) { field =
value.as_ptr(); }` instead of `field = check_raw_pointer::<T>(a_val as *mut _)?.as_ptr()`. An invalid
value for any one of these now just leaves that one field at its default (null), instead of
discarding every other already-parsed aux value and panicking three call sites up.

Additionally, `linux_execfn()`'s own doc comment already promises "Returns an empty string if the
string is not available" - but its implementation unconditionally called `CStr::from_ptr` on
whatever `EXECFN` held, which would still be a null-pointer dereference once `AT_EXECFN` is allowed
to stay null per the fix above. Added an explicit null check there returning a real empty `CStr`,
so this function now actually honors its own documented contract instead of relying on `AT_EXECFN`
always being present and valid (true in every environment checked this session, but not something
the type system or the parser enforces).

**Checked against more than the one triggering input (CONFIRMATION.md section 7)**: re-ran the
real end-to-end repro at a second, different misalignment residue (`e_phoff % 8 == 3`, not just the
first one found, `== 1`) against both the pristine and patched trees - panics before, doesn't after,
in both cases.

- Real end-to-end repro (`repro/`, via `just test` from this folder): before the patch, panics
  5/5 runs (`called Option::unwrap() on a None value` at `auxv.rs:301:68`, exit code 101); after
  `git apply patch.diff`, no panic, 3/3 runs (prints `phdr=0x0 phent=0 phnum=10`, exit code 0).
- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host x86_64): **411 passed, 1 failed, identical before and after** - the one failure is the
  pre-existing, environment-caused `ioctl::test_int_setter` (`NotFound` opening `/dev/net/tun`,
  absent in this sandboxed container; documented in `___BUGHUNT___/projects/rustix.md`). No new
  failures introduced by the fix.

## What's not done / open questions

- `AT_EXECFN`/`AT_RANDOM`'s own bad-pointer case (as opposed to `AT_PHDR`'s) was not independently
  reproduced end-to-end this session - no real producer (normal exec, this qemu-user version,
  `PR_SET_MM_AUXV`) was found that supplies a null value for either in this environment (see
  "Status"). The fix is applied to both anyway since it's the same code shape and the same
  maintainer-established precedent (#1484) applies equally; a future session with `CAP_SYS_RESOURCE`
  available (a real root-capable host, not this sandbox) could use `PR_SET_MM_AUXV` to craft a
  genuine repro for these two directly, the same way CRIU-style tools legitimately use that prctl.
- Not checked: whether any *other* field in this same function could independently be forced to an
  inconsistent state by a crafted ELF/auxv without going through `check_raw_pointer` at all (e.g.
  `AT_PHENT`/`AT_PHNUM` are taken as raw `a_val as usize` with no validation - `exe_phdrs()`'s
  callers get whatever is reported, including a `phnum` that doesn't match the real number of
  headers at `phdr`; not exercised or assessed this session, and not the same panic-shaped bug this
  candidate targeted).
- New, unrelated bug noticed incidentally while building the repro (not part of this candidate,
  not fixed here): `exe_phdrs()` always returns `phent == 0` regardless of the real `AT_PHENT`
  value - `init_from_aux_iter` parses the real `phent` local correctly (the
  `assert_eq!(phent, size_of::<Elf_Phdr>())` a few lines down never fires), but the final
  `#[cfg(feature = "runtime")]` block that stores `PHDR`/`PHNUM`/`ENTRY`/`RANDOM`/`SECURE` into their
  statics never includes a corresponding `PHENT.store(phent, Relaxed);` line - confirmed via
  `LD_SHOW_AUXV=1` showing the kernel really does report `AT_PHENT: 56` while `exe_phdrs()` still
  returns `phent=0`, consistently, every run. Worth its own candidate.
