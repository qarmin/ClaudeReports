# termios_speed_decode_unwrap_panic

`Termios::input_speed()`/`output_speed()` (src/termios/types.rs:109,121,142,154) call
`speed::decode(<raw encoded speed_t from the real tcgetattr/cfgetispeed/cfgetospeed result>).unwrap()`
on illumos/Solaris, newlib, AIX, and every other non-Linux/non-BSD platform rustix supports (Haiku,
GNU/Hurd, QNX Neutrino, Redox, Cygwin, ...). `speed::decode` (src/termios/types.rs:844) is a
`match` over the `B0..B4000000` constants *rustix itself* defines, returning `None` for anything
else - same unwrap-on-untrusted-external-value shape as the already-confirmed
`Errno::from_raw_os_error(0)` bug (candidate 002), in the termios subsystem instead of errno.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `speed::decode`'s full match table (src/termios/types.rs:844-978) and cross-check every arm's `#[cfg(...)]` gate against the vendored `libc-0.2.189` crate's real per-platform `B*` constant definitions (`~/.cargo/registry/.../libc-0.2.189/src/unix/{solarish,aix,haiku,hurd,nto,redox,cygwin}/mod.rs`) | reading real vendored source, no build | Found concrete, real gaps: illumos/Solaris ("solarish") define `B76800`(17)/`B153600`(19)/`B307200`(21) that rustix's table has **no arm for at all**; Haiku defines `B31250`(0x13, the standard MIDI serial baud rate) with no arm; GNU/Hurd defines `B7200`/`B14400`/`B28800`/`B76800` (all real-value encodings) with no arm. AIX's own real constant set (`B0..B38400`) is fully covered - no gap found there specifically, even though the code path is structurally the same `unwrap()`. Redox/Cygwin/Fuchsia's own real constant sets are already fully covered by rustix's existing table |
| Inventory real cross/emulation tooling for the affected OSes | `rustup target list --installed`, `dpkg -l`, `which qemu-*` | No illumos/Solaris/Haiku/GNU-Hurd/AIX/QNX rustc target installed, no cross-gcc/binutils for any of them, no qemu-system image for any of them - only `qemu-sparc64` (a Linux/sparc64 target, unrelated OS) was present |
| Escalate: add the real illumos rustc target directly (`rustup target add x86_64-unknown-illumos`, a tier-2-without-host-tools target with prebuilt `rust-std`) and `cargo check --target x86_64-unknown-illumos` | both the pristine tree and this candidate's patched tree | **Real compiler confirmation, not just manual reading**: the pristine tree type-checks clean (the match genuinely has no `B76800`/`B153600`/`B307200` arm for real `target_os = "illumos"`), and the patched tree (with those 3 arms added) also type-checks clean, proving `c::B76800`/`c::B153600`/`c::B307200` genuinely exist at those exact names/values in the real illumos `libc` module for this rustc/libc version - this is stronger evidence than the manual source grep alone |
| Check whether a real runnable illumos binary (any binary, not rustix-specific) is possible here at all | throwaway `fn main(){println!("hi")}` bin crate, `cargo build --target x86_64-unknown-illumos` | **Fails at the link step**: `cc`/`ld.bfd` can't find `-lsocket`/`-lposix4`/`-lnsl`/`-lumem` - no illumos sysroot/system libraries exist in this sandbox. Confirms a genuine end-to-end illumos execution (real `tcgetattr`/`cfgetispeed` call against a real illumos kernel) is not achievable here, even though type-checking the real target is |
| Same target-add attempt for Haiku/GNU-Hurd (`x86_64-unknown-haiku`, `x86_64-unknown-hurd-gnu`) | `rustup target add` | Both report "no prebuilt artifacts available for target" (tier-3, no prebuilt `rust-std`) - strictly less testable here than illumos, same category as candidate 028's arm-vita/m68k/esp-idf "genuinely untestable" targets |
| Standalone `repro/`: verbatim copy of rustix's real `speed::decode` match-arm logic (byte-for-byte, only the `#[cfg(...)]` gates pre-resolved by hand for a fixed `target_os`, exactly as `rustc` itself would resolve them for a real build), fed the real numeric `B76800`(illumos)/`B31250`(Haiku) encodings copied from the vendored `libc-0.2.189` source | host x86_64, debug build, 3 repeat runs | **Panics 3/3 runs**, deterministically (pure `const fn` match over plain integers - no timing/environment dependence to even make non-determinism possible), with the exact `called Option::unwrap() on a None value` message real callers would see |

## Status

**Confirmed.** Reproduces deterministically (3/3 runs, and inherently 100% of runs - the triggering
match arms are a `const fn` over plain integers with no external state) via the real, unmodified
`speed::decode` match logic fed the real encoded `B76800`/`B31250` values illumos/Haiku's own libc
headers define for completely standard baud rates.

**Fully reproducible**: **No** - the hardware/OS-availability exception (CONFIRMATION.md section
11), the same category candidate 028 already established in this project: this bug's "second
participant" is each target OS's own kernel/libc genuinely reporting a standard, non-adversarial
baud-rate encoding (illumos's real `B76800`, Haiku's real `B31250` - the standard MIDI serial rate
- GNU Hurd's real `B7200`/`B14400`/`B28800`, all completely ordinary configurations, not fabricated
exotic input), not a hand-authored stand-in for what that participant would send. Demonstrating it
fully end-to-end genuinely requires booting one of those OSes and calling the real
`tcgetattr`/`cfgetispeed` there. This session went one step further than code-reading alone and
verified directly (not assumed) what's actually possible in this sandbox: the real illumos rustc
target *does* type-check both the bug and the fix correctly (`cargo check --target
x86_64-unknown-illumos`, prebuilt `rust-std` available) - but a real runnable illumos binary is not
achievable here (linking fails outright: no illumos sysroot, missing `-lsocket`/`-lposix4`/`-lnsl`/
`-lumem`), and Haiku/GNU-Hurd don't even have a prebuilt `rust-std` to type-check against. The only
thing faked in `repro/` is *which OS this process is compiled for*; the decode logic (copied
verbatim) and the triggering input values (copied verbatim from the real vendored `libc` crate) are
both the real, independently-sourced things - not invented.

## Class

crash

## Root cause

`speed::decode` (src/termios/types.rs:844) is a hardcoded `match` that only recognizes the `B*`
encoded-speed constants rustix itself chose to define in its own `speed` module. On illumos/
Solaris, Haiku, and GNU/Hurd specifically, those OSes' own `libc` headers define additional
standard `B*` constants (illumos: `B76800`=17, `B153600`=19, `B307200`=21; Haiku: `B31250`=0x13, the
standard MIDI serial baud rate; GNU/Hurd: `B7200`, `B14400`, `B28800`, `B76800`, all real-value
encodings) that rustix's table simply never added. `input_speed()`/`output_speed()` then call
`speed::decode(<raw value from the real system>).unwrap()` unconditionally - so a tty/pty/serial
device on any of those OSes that is (or was ever) configured to one of these completely standard,
non-exotic baud rates makes the safe public API panic the whole process. Unlike candidate 028's
sparc bug (a pure per-architecture array-length constant mismatch with no external "driver"
involved at all), this one's trigger is a value a real OS component (the kernel/libc) is expected to
legitimately report for ordinary configurations - the same "safe API panics on a value only the
match table's author anticipated" shape as candidate 002's `Errno::from_raw_os_error(0)` bug.

## Introduced by

58152e9d "Overhaul the `termios` API." (#615), 2023-06-19 - this is the commit that introduced
`speed::decode`/`encode` and the `.unwrap()` call sites in `input_speed()`/`output_speed()` in
their current form (`git log -S "pub(crate) const fn decode"` and `git show 58152e9d` both confirm
the `.unwrap()` calls and the `solarish`/generic-platform branching were added together in this
commit; same introducing commit candidate 028 already identified for the sibling sparc
`tcgetattr`/`tcsetattr` bug in the same file). The specific missing `B76800`/`B153600`/`B307200`/
`B31250`/`B7200`/`B14400`/`B28800` constants were never added in any later commit either - this is
an omission present since the module's creation, not a later regression.

## The fix

Add the concretely-identified missing constants to `speed`'s public `B*` list and to both
`decode()`'s and `encode()`'s match tables, gated to exactly the platforms whose real `libc` module
defines them (verified against the vendored `libc-0.2.189` source, and for illumos additionally
verified by `cargo check --target x86_64-unknown-illumos` type-checking clean both before and after
this exact set of additions):

- `B76800` (76,800) - illumos/Solaris (`solarish`), GNU/Hurd, QNX Neutrino (`nto`)
- `B153600` (153,600), `B307200` (307,200) - illumos/Solaris only
- `B31250` (31,250) - Haiku only
- `B7200` (7,200), `B14400` (14,400), `B28800` (28,800) - GNU/Hurd only

**Checked against more than the one triggering input (CONFIRMATION.md section 7)**: all three
illumos/Solaris gaps, the Haiku gap, and all four GNU/Hurd gaps were added together (not just the
single `B76800`/`B31250` values repro/ happens to exercise), and symmetrically to both `decode()`
(kernel-to-rustix direction) and `encode()` (rustix-to-kernel direction, so `set_speed`/
`set_input_speed`/`set_output_speed` can now also *set* these rates on the affected platforms,
closing the same gap in the reverse direction).

**This does not close the bug class entirely** - `decode()` can still panic on any encoding no OS's
real `libc` module happens to define a named `B*` constant for at all (a genuinely nonstandard/
vendor-specific baud rate a real serial driver might report), or on any future OS this match table
doesn't yet enumerate. A fully panic-proof fix would require changing `input_speed()`/
`output_speed()`'s public return type to `Option<u32>` (or similar), which is a semver-breaking API
change out of scope for a bounded fix this session - noted here as the architectural fix a
maintainer would need to decide on, same as candidate 006's `io_uring_register` `*const`/`*mut`
mismatch being deliberately deferred to a semver-major bump.

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host x86_64-unknown-linux-gnu - the new match arms are entirely `#[cfg(...)]`'d out on this
  target, so this is a build-regression sanity check, not an exercise of the actual fix):
  **411 passed, 1 failed, both before and after** - identical failure in both runs,
  `ioctl::test_int_setter` (`NotFound` opening `/dev/net/tun`), the same pre-existing,
  environment-caused failure candidate 028 already documented. No new failures introduced by the
  fix. Additionally, `cargo check --features "use-libc,all-apis,std" --target
  x86_64-unknown-illumos` type-checks clean both before and after.

## What's not done / open questions

- AIX's own real `B*` set (`B0..B38400`) is already fully covered by rustix's table - no concrete
  gap was found there in this session, even though the same `unwrap()` is structurally reachable
  (AIX's `CBAUD`/extended-baud mechanisms weren't investigated further; worth a dedicated follow-up
  if AIX hardware/emulation ever becomes available).
- `newlib` (embedded targets: espidf, vita, horizon, rtems, arm, powerpc) define **no** `B*`
  constants at all in the vendored `libc` crate - not investigated further whether that branch is
  even reachable/compilable for any real newlib sub-target, or whether it gets its speed constants
  from elsewhere entirely.
- The deeper architectural fix (non-panicking `Option<u32>` return type) was not attempted - left
  as a maintainer decision per "The fix" above.
- A real illumos/Haiku/GNU-Hurd/AIX execution (not just type-checking) remains undone - would need
  either real hardware or a from-scratch sysroot/cross-toolchain build for one of these OSes, which
  this session judged out of scope (see "What was tried" for exactly what was verified instead).
