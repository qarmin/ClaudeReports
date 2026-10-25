# cross_platform_github_issues_untestable_env

Re-checked a 13-issue backlog item from `projects/rustix.md`'s "Risk map" (session 2026-09-18):
open upstream issues affecting macOS/BSD/sparc/riscv/loongarch/m68k/vita/esp/Android/exotic Linux
ABIs (#1668, #1678, #1682, #1518, #1461, #1462, #1496, #1519, #1521, #1576, #1626, #1645, #1652),
noted then as "not investigated - this environment is x86_64 Linux only, no cross-compilation/
emulation set up for those targets." That assumption no longer fully holds: this session's
environment actually has `qemu-user` (aarch64/arm/riscv64/sparc/sparc64), cross-`gcc`/`binutils`
for aarch64/arm/riscv64-linux-gnu, `rustup`-installed Android/aarch64/riscv64gc/armv7 Rust targets,
network access, and an authenticated `gh`. This candidate re-verifies the premise issue-by-issue
with what's actually available now, rather than assuming the original "untestable" note still
applies, and fully investigates the one issue (#1518) that turned out to be a genuine,
still-present logic bug reachable with what this environment has.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Inventory real tooling now available (`which`/`apt`/`rustup target list --installed`/`dpkg -l`) | qemu-user, cross-gcc, rustup targets, network, `gh auth status` | Found real cross/emulation tooling the 2026-09-18 note didn't have (or didn't check for): qemu-{aarch64,arm,riscv64,sparc,sparc64}, gcc-15-{aarch64,arm,riscv64}-linux-gnu-base, Android/aarch64/riscv64gc/armv7 rust-std already installed |
| Fetch all 13 issues' current bodies via `gh issue view --repo bytecodealliance/rustix` | network, ~1 min | All still `OPEN`; triaged each into: runtime logic bug (#1518, #1678, #1682, #1668), build-only failures (#1461, #1462, #1496, #1519, #1576, #1626, #1645, #1652), feature request/not-a-bug (#1521) |
| Code-read #1518's exact panic site (`src/backend/libc/termios/syscalls.rs:94-95`) against the vendored `libc` 0.2.189 crate's sparc/sparc64 type definitions | reading `~/.cargo/registry/.../libc-0.2.189/src/unix/linux_like/linux/{arch/sparc,gnu/b32,gnu/b64/sparc64}/mod.rs` | Confirmed exact root cause: `termios2.c_cc` is a hardcoded `[cc_t; 19]` (kernel uapi size, shared by sparc+sparc64), but `NCCS == 17` for both sparc ABIs specifically (every other Linux glibc/musl arch has `NCCS >= 19`) - rustix's own `SpecialCodes([c::cc_t; c::NCCS])` is therefore only 17 wide, so `result.special_codes.0[..19]` always panics on sparc/sparc64. Same bug in the reverse direction in `tcsetattr` (lines ~243-248) |
| Attempt a real cross-compiled+qemu-run reproduction: `rustup target add sparc64-unknown-linux-gnu` (succeeded, prebuilt std exists) then `cargo build --target sparc64-unknown-linux-gnu` on a trivial `hello world` | stable 1.98, `rust-lld`, no sparc cross-`gcc`/`binutils` package available via `apt-cache search`/`apt-get install -s` | **Failed as expected**: `rust-lld: error: ... is incompatible with elf64-x86-64` - `rust-lld` cannot link ELF64 sparc64 objects at all, and this Ubuntu-based container has no `gcc-sparc64-linux-gnu`-style cross toolchain package (confirmed via empty `apt-cache search`). Real end-to-end execution of a sparc/sparc64 rustix binary under `qemu-sparc64` genuinely isn't possible in this environment - the original "no cross-compilation" note was right for sparc specifically, even though it's now wrong for aarch64/arm/riscv64/Android |
| Boundary-value construction: standalone `repro/` executing rustix's own two lines verbatim (copied, not reimplemented), with the real sparc/sparc64 sizes (19, 17) substituted for the per-target `libc`-crate-derived constants | host x86_64, debug build, 3 repeat runs of the panic + 2 of the fix | **Panics 3/3 runs**, with the exact message the original issue reporter saw: `range end index 19 out of range for slice of length 17`. Patched version (clamped `nccs`) succeeds 2/2 runs |
| Quick `cargo check` sweep of the now-testable build-failure issues against current HEAD (287214b8, v1.1.5) with each issue's own reported feature set, using the newly-available targets | `aarch64-linux-android`/`i686-linux-android`/`armv7-linux-androideabi`/`x86_64-linux-android` (#1519), `loongarch64-unknown-linux-gnu` (#1461 `mm`, #1496 `fs`), `riscv64gc-unknown-linux-musl` (#1462 `mm,use-libc`), `x86_64-unknown-linux-gnux32` (#1645) - all targets added fresh via `rustup target add`, all had prebuilt std | All 8 checks **pass cleanly on current HEAD** - these 5 issues' reported compile errors no longer reproduce (see "What's not done" - not independently confirmed *why*, e.g. whether rustix's own fixes or a newer vendored `libc` resolved them; not closed on GitHub as of this session) |
| Toolchain-availability check for the remaining build-only issues (#1576 arm-vita, #1626 m68k, #1652 riscv32imc-esp-espidf) | `rustup target add armv7-sony-vita-newlibeabihf` / `m68k-unknown-linux-gnu` | Both **genuinely have no prebuilt rust-std** ("no prebuilt artifacts available... low-tier target") and no matching cross-`gcc`/libc/linker exists in this environment either - confirmed still untestable here, unlike the Android/loongarch/riscv64-musl/gnux32 set above |

## Status

**Confirmed** (for issue #1518's exact bug, still present in current HEAD 287214b8). Reproduces
3/3 runs (panic, debug build, host x86_64) with the identical panic message the original reporter
saw; fix verified 2/2 runs with no panic.

The other 12 issues in this backlog item were triaged but not pursued to a full Confirmed/Not-
reproducible verdict each in this candidate (see "What's not done" - one bug per candidate folder,
per `SKILL.md`). Their current status as of this session: 5 build-failure issues (#1461, #1462,
#1496, #1519, #1645) no longer reproduce against current HEAD with the newly-available targets; 3
(#1576, #1626, #1652) remain genuinely untestable here (no prebuilt std and no matching cross-
toolchain for those targets exist in this environment); 3 are runtime/logic bugs on an OS this
environment cannot run at all (#1668, #1678, #1682 - macOS/Apple-specific, no macOS hardware, no
way to actually execute code for that OS, Miri real-syscall interposition is host-OS-specific and
can't stand in for real macOS kernel behavior); #1521 is a feature-request question, not a bug.

**Fully reproducible**: **No**, for a specific, concrete reason - *not* the two-participant
exception in `CONFIRMATION.md` section 11 (this bug has no second participant: it's a pure
architecture-dependent array-size mismatch within a single process, not a peer/caller sending
adversarial input), but the hardware/toolchain exception the same section calls out: a real
end-to-end run requires either genuine sparc/sparc64 hardware or a working sparc cross-compiler
*and* cross-linker under `qemu-sparc64`, and this session verified directly (not assumed) that
neither exists here - `rustup target add sparc64-unknown-linux-gnu` succeeds (prebuilt std exists),
but the actual link step fails outright (`rust-lld` refuses sparc64 ELF objects, no cross-`gcc`
package available). The repro instead runs rustix's own two source lines verbatim (not
reimplemented), with the real, independently-verified sparc/sparc64 array-length constants
substituted in place of what's otherwise computed per-target by the `libc` crate - the only thing
faked is *which CPU architecture this process is compiled for*, not any external input or any other
participant's behavior. See `repro/src/main.rs`'s header comment for the exact citations.

## Class

crash

## Root cause

`src/backend/libc/termios/syscalls.rs`'s `tcgetattr`/`tcsetattr` (the `libc`-backend, `linux_kernel`
cfg path) both compute `let nccs = termios2.c_cc.len();` and then slice-copy `nccs` elements into/
out of rustix's own `SpecialCodes([c::cc_t; c::NCCS as usize])` array, implicitly assuming
`termios2.c_cc.len() <= NCCS` always holds. On every Linux glibc/musl architecture this crate ships
prebuilt rust-std for *except* sparc and sparc64, that assumption is true (`c_cc.len()` is 19 or 23,
`NCCS` is 32). On sparc/sparc64 specifically, the kernel's `arch/sparc/include/uapi/asm/termbits.h`
defines the ioctl-facing `termios2.c_cc` as 19 elements, but glibc's own userspace `<termios.h>`
defines `NCCS` (the *user-facing* control-code count) as only 17 for that same architecture - a
real, independently-documented kernel/libc discrepancy specific to sparc, not something rustix
introduced. `tcgetattr` then panics copying the kernel's 19-element response into rustix's 17-
element array; `tcsetattr` panics the same way trying to fill 19 kernel slots from rustix's 17-
element array.

## Introduced by

58152e9d "Overhaul the `termios` API." (#615), 2023-06-19 - this commit introduced both the
`tcgetattr`/`tcsetattr` functions and the unclamped `let nccs = termios2.c_cc.len();` pattern in
their current form (verified via `git log -S` on the surrounding comment text and the `nccs`
computation; the file has had later unrelated changes, e.g. 16dcb8db's `TCGETS` fallback, but the
unclamped `nccs` line itself traces to 58152e9d).

## The fix

Clamp `nccs` to the shorter of the two arrays on both sides (`tcgetattr`'s kernel-to-rustix copy
and `tcsetattr`'s rustix-to-kernel copy), and slice *both* operands to `[..nccs]` rather than just
one: `let nccs = termios2.c_cc.len().min(result.special_codes.0.len()); result.special_codes.0[..nccs].copy_from_slice(&termios2.c_cc[..nccs]);`
(symmetric change in `tcsetattr`). This is sufficient because rustix's own public `SpecialCodes`
type is itself bounded to `NCCS` entries on every architecture (it's `[c::cc_t; c::NCCS as usize]`,
`src/termios/types.rs:1124`) - on sparc/sparc64, control codes at kernel indices 17 and 18 (`c_cc[17]`,
`c_cc[18]`) are therefore never representable in rustix's own type on this architecture regardless
of this fix (the kernel still only exposes NCCS-worth of codes via sparc's own libc `<termios.h>`
too, so no application code path on sparc is losing something it previously had access to - this
purely stops a crash, it doesn't newly truncate a code an sparc caller could previously read back
through the normal, non-`TCGETS2`-specific API).

**Checked against more than the one triggering input (CONFIRMATION.md section 7)**: the same
clamp-and-slice pattern was applied identically to both `tcgetattr` (kernel->rustix direction,
the original issue's exact panic site) and `tcsetattr` (rustix->kernel direction, the symmetric but
previously unreported sibling bug in the same file) - both now compile and behave the same way,
not just the one function the original report happened to hit.

**Build-verified** (patched tree, libc backend): `cargo build --features "use-libc,all-apis,std"` -
succeeds, on host x86_64 (`nccs` never exceeds `NCCS` there, so this cannot exercise the actual
panic path natively - it's a compile/regression sanity check only, not a stand-in for the real
sparc repro above).

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host x86_64): **411 passed, 1 failed, both before and after** - identical failure in both runs,
  `ioctl::test_int_setter` (`NotFound` opening `/dev/net/tun`), the same pre-existing, environment-
  caused failure documented in `projects/rustix.md`'s "Gotcha (2026-09-18, still true 2026-10-01)".
  No new failures introduced by the fix.

## What's not done / open questions

- The other 12 issues in this backlog item were triaged (see "What was tried") but not carried to
  their own full candidate loop (repro/fix/verify) in this session - "one bug per candidate folder"
  per `SKILL.md`. Worth splitting into their own `IDEAS.md` backlog entries for a future session,
  now that this one has established which are actually testable here:
  - #1678 (`tcgetpgrp` unsound on macOS: `Pid::from_raw_unchecked(0)` reachable via the
    non-Linux-guarded `pid == 0` path, src/backend/libc/termios/syscalls.rs:166-179) reads as a
    real, still-present bug from code alone (the `#[cfg(linux_kernel)]` guard on the `pid == 0`
    check is the entire fix the issue asks for, generalized to all platforms) - genuinely
    untestable without real macOS/BSD hardware (no amount of qemu-user emulates a different OS's
    kernel behavior), a strong candidate for `CONFIRMATION.md` section 11's hardware exception if
    picked up standalone.
  - #1682 (Apple `fallocate` partial-`F_PREALLOCATE` handling) and #1668 (Miri+macOS `fstat`) are
    both macOS-kernel-behavior-specific; same "needs real Apple hardware" limitation.
  - #1461, #1462, #1496, #1519, #1645 no longer reproduce against current HEAD with the newly-
    available targets (see "What was tried") - not independently confirmed *why* (rustix fix vs.
    upstream `libc`-crate fix vs. version drift since the issue was filed); worth a maintainer
    closing them with a repro attempt against current release, not re-litigated here.
  - #1576, #1626, #1652 remain genuinely untestable in this environment specifically because no
    prebuilt rust-std AND no matching cross-toolchain exist for their targets (arm-vita, m68k,
    esp-idf) - distinct from sparc (which has prebuilt std, just no linker) and distinct from the
    5 now-testable ones above.
  - #1521 is a design/roadmap question ("plans to support Android for linux_raw"), not a bug - not
    actionable by this skill at all.
- Not checked: whether sparc 32-bit (`sparc-unknown-linux-gnu`, no prebuilt std at all per
  `rustup target add`'s own error) has any *additional* divergence beyond the shared
  `arch/sparc/mod.rs`/`NCCS=17` already covered - the root cause applies identically to both ABIs
  per the vendored `libc` source, so this isn't expected to differ, just not independently run.
- A fresh `gh-issues-prs-check`-style sync of this repo's issues/PRs since the `#1682` watermark
  recorded in `projects/rustix.md` would likely be worth doing on a future session - not done here,
  this candidate only re-fetched the 13 already-known numbers.
