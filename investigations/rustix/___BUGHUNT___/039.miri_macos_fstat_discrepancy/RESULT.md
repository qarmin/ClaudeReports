# miri_macos_fstat_discrepancy

Upstream bytecodealliance/rustix#1668 ("Miri + macOS fstat discrepancy"), part of a stale blanket
"macOS-kernel-behavior-specific, needs real Apple hardware" list from candidate 028's re-triage
(grouped with #1682) - never independently investigated to a verdict before this candidate.
`src/backend/libc/fs/syscalls.rs::fstat` reads an `fstat(2)` result into a
`MaybeUninit::<Stat>::uninit()` and then calls `assume_init()`, trusting the libc call to have
initialized every byte of the struct. On Apple targets, `libc::stat` has reserved/spare fields
(`st_lspare: i32`, `st_qspare: [i64; 2]`) that a shim for the syscall is not obligated to write a
defined value into - the original issue's reporter hit exactly this under Miri on real macOS
hardware (aarch64-apple-darwin). The "needs real Apple hardware" assumption turned out to be
false: Miri cross-interprets MIR directly, so it doesn't need to run on the target OS at all -
`cargo +nightly miri run --target x86_64-apple-darwin` works from this Linux x86_64 environment,
using Miri's own built-in libc shim for the apple target (which is the actual thing responsible
for not fully initializing the struct, same as it would be on real Apple hardware).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Check whether a `rust-std`/sysroot exists for `x86_64-apple-darwin` and whether Miri can build a sysroot for it | `rustup target list --installed` (yes, `rust-std-x86_64-apple-darwin` present), `rustup run nightly cargo miri setup --target x86_64-apple-darwin` | Sysroot built successfully in ~12s from `rust-src` (Miri compiles `core`/`alloc`/`std` from source per target, doesn't need the prebuilt `rust-std` component at all - confirms cross-interpretation is genuinely possible here, not just "looks possible") |
| Real end-to-end repro: a standalone example calling rustix's own public `rustix::fs::fstat` on a real `std::fs::File`'s real fd, run under `cargo +nightly miri run --target x86_64-apple-darwin` with `MIRIFLAGS="-Zmiri-disable-isolation"` | debug build, 4 total runs (1 initial + 3 repeats) | **Reproduces 4/4 runs**, byte-identical panic message and exact source location (`src/backend/libc/fs/syscalls.rs:1580:20`, `.st_lspare, encountered uninitialized memory, but expected an integer`) to the original issue reporter's real-macOS-hardware output |
| Checked the 3 sibling call sites with the identical `MaybeUninit::<Stat>::uninit()` -> `assume_init()` pattern (`stat`, `lstat`, `statat` - all in the same file) | Same technique, a second standalone example calling `rustix::fs::stat`/`lstat`/`statat` | `stat` and `lstat` **independently reproduce the identical UB** (confirmed via Miri, not just code-reading); `statat`'s own `fstatat` call could not be independently exercised at all - Miri has no shim for `fstatat` on apple targets (`unsupported operation: can't call foreign function 'fstatat$INODE64' on OS 'macos'`), a Miri limitation unrelated to this bug, not evidence the fix is wrong there (same source pattern, fixed identically by code symmetry) |
| Applied a bounded fix (`uninit()` -> `zeroed()` at all 4 call sites) and re-ran both examples under Miri | Same build/target/flags | UB gone: `fstat`/`stat`/`lstat` all return the correct, real result (`fstat ok, st_size = 0`, `stat: 1`, `lstat: 1`) - 3/3 repeat runs, fully deterministic |

## Status

**Confirmed**. Reproduces 4/4 runs (debug build, Miri cross-interpreting `--target
x86_64-apple-darwin`, `-Zmiri-disable-isolation`), fully deterministic - byte-identical panic
message and source line to the original reporter's real macOS hardware run. Fix verified 3/3 runs,
no flakiness either way.

**Fully reproducible**: **Yes**. This is a single-process, single-participant finding (no
peer/caller producing adversarial input - the "second participant" is Miri's own interpreter
faithfully emulating the apple target's libc, not something this repro fakes), so
`CONFIRMATION.md` section 11's two-participant bar doesn't apply (same category as this
workspace's own candidate 004, the futex Miri UB finding). `repro/` runs rustix's real, public
`fstat` API on a real file descriptor from a real `std::fs::File` - nothing about the trigger path
is hand-constructed; the only thing not real-hardware is which CPU/OS the process is compiled for,
and Miri's whole design point is that this doesn't matter for UB detection (it interprets MIR, it
doesn't natively execute target code).

## Class

UB

## Root cause

`src/backend/libc/fs/syscalls.rs`'s `fstat` (and identically, `stat`/`lstat`/`statat`) each do:

```rust
let mut stat = MaybeUninit::<Stat>::uninit();
ret(c::fstat(borrowed_fd(fd), stat.as_mut_ptr().cast()))?;
let stat = stat.assume_init();
```

This assumes the underlying C `fstat`/`stat`/`lstat`/`fstatat` call initializes every byte of the
destination buffer. On Apple targets, `libc::stat` (`libc` crate's
`src/unix/bsd/apple/mod.rs`) has two reserved/spare fields at the end of the struct -
`st_lspare: i32` and `st_qspare: [i64; 2]` - which exist purely as padding/reserved space for
future kernel use and are not meaningful `stat` output. Miri's libc shim for `fstat` on
`x86_64-apple-darwin`/`aarch64-apple-darwin` does not write a defined value into `st_lspare`
(the original issue's exact field), so `assume_init()` constructs an `i32` from uninitialized
memory, which Rust's abstract machine (and Miri's validity checker, correctly) treats as
instant UB - integers are never allowed to be uninitialized, unlike e.g. padding bytes inside a
struct that are never read as a typed value. This is not a Miri false positive: regardless of
whether a real XNU kernel syscall happens to zero this field in practice on every macOS version,
rustix's own code has no actual guarantee that it does, and `assume_init()` on a value the
compiler cannot prove is fully initialized is unsound by construction - Miri is the tool that
makes this visible, not the cause of it.

## Introduced by

eb827162 "Implement `fstat`." (2021-06-17) - this commit introduced `fstat`'s
`MaybeUninit::<Stat>::uninit()` / `assume_init()` pattern in its original form (verified via `git
log -S "MaybeUninit::<Stat>::uninit"` across the whole history, including pre-rename paths - the
earliest hit). The `stat`/`lstat` siblings were added later by 00edd161 "Add non-at filesystem
functions." (2023-05-24), copying the same pattern; `statat`'s own copy of the pattern predates
both (present since at least 1d0dfeac's codebase reorganization).

## The fix

Changed all 4 call sites (`stat`, `lstat`, `statat`, `fstat` in
`src/backend/libc/fs/syscalls.rs`) from `MaybeUninit::<Stat>::uninit()` to
`MaybeUninit::<Stat>::zeroed()`. This is sufficient because `Stat`'s own representation
(`c::stat`-compatible, `#[repr(C)]`-equivalent layout) permits an all-zero bit pattern for every
field it has (integers, not references/enums/`NonZero*`), so zeroing is always a valid starting
state regardless of target; the real syscall then overwrites every field it actually produces,
and any reserved/spare field it doesn't touch is left at a well-defined `0` instead of
indeterminate bits. `fix_negative_stat_nsecs` (the apple-only nanosecond-rounding postprocessing
already applied after `assume_init()`) is unaffected - it only reads fields the syscall does
populate.

**Checked against more than the one triggering input (CONFIRMATION.md section 7)**: fixed `stat`
and `lstat` identically (re-verified independently via Miri - both now succeed) in addition to the
`fstat` the original issue reported, and `statat` by code symmetry (same pattern, could not be
independently Miri-verified due to the unrelated `fstatat` apple-target shim gap noted above).

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host x86_64-unknown-linux-gnu): **411 passed, 1 failed, identical before and after** - the one
  failure (`ioctl::test_int_setter`, `NotFound` opening `/dev/net/tun`) is the same pre-existing,
  environment-caused failure documented in `projects/rustix.md`'s "Gotcha (2026-09-18, still true
  2026-10-01)", unrelated to this fix. No new failures introduced.

## What's not done / open questions

- `statat`'s own fix could not be independently exercised through Miri (no `fstatat` shim for
  apple targets in this Miri version) - the fix is applied there by code symmetry with the other
  3 sites, not independently confirmed to clear the identical UB the way `fstat`/`stat`/`lstat`
  were.
- Not checked: whether any other Apple-only syscall wrapper in this codebase has the same
  "`MaybeUninit::uninit()` then trust the syscall to fill every byte" shape for a struct with
  reserved/spare fields (e.g. `statfs`/`statvfs` at lines 1615/1624 of the same file use the same
  pattern for different structs - not checked for apple-specific reserved fields this session).
- The sibling, still-untriaged issue #1682 ("Apple `fallocate` partial `F_PREALLOCATE` handling")
  that was grouped with this one in the stale blanket list remains unchecked - it's a logic/
  behavior question about `fallocate`'s return semantics on Apple, not a Miri/UB finding, so this
  candidate's "needs real Apple hardware" re-derivation doesn't carry over to it the same way; it
  would need its own investigation.
- aarch64-apple-darwin (the original reporter's exact architecture) was not independently
  cross-interpreted this session - only x86_64-apple-darwin was (no `rust-src`-built sysroot
  attempt was made for aarch64-apple-darwin here). The `st_lspare`/`st_qspare` struct layout is
  identical for both Apple ABIs in the vendored `libc` crate, so this isn't expected to differ,
  just not independently run.
