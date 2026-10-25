# miri_cross_interp_sweep_openbsd_fuchsia_redox_wasi

Meta/systemic sweep: continuing the non-Linux-backend Miri cross-interpretation technique that
worked well for macOS/BSD (candidates 035, 039, 058) for `cfg` targets this project's sweeps hadn't
reached yet - OpenBSD, Fuchsia, Redox, and wasi. Four escalating approaches were tried across these
targets; the first three reproduced nothing new (either duplicates of already-confirmed bugs
reaching further platforms, a clean negative, or tooling-coverage gaps), but the fourth - a direct
cross-target compile check, no Miri needed - found a real, deterministic, previously-unreported bug
specific to `wasm32-wasip1`/wasi: rustix's own `#[cfg(test)]` `Stat`-layout self-check in
`src/fs/constants.rs` does not compile for this target at all.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code-reading sweep: `grep` every file gated by `target_os = "redox"/"fuchsia"/"wasi"` or (for OpenBSD, which shares the `cfg(bsd)` group with FreeBSD/NetBSD/Apple) `target_os = "openbsd"` for `unwrap()`/`assume_init`/`from_raw_unchecked`/`transmute`/raw pointer casts | static analysis, no build | Found 3 concrete leads, but all 3 turned out to be the exact same already-`Confirmed` bugs reaching platforms this project's sweeps hadn't explicitly listed yet, not new bugs: (1) `tcgetpgrp`'s `pid == 0` -> `Pid::from_raw_unchecked` (candidate 035) is gated `#[cfg(linux_kernel)]`, so the unguarded panic/UB path is reachable on Fuchsia and Redox too, not just macOS/BSD - 035's own fix (removing the Linux-only gate) already covers this since it's the same lines; (2) `tcp_congestion`'s `assert!(slice.contains(&b'\0'))` (candidate 026) is `cfg(any(linux_like, freebsd, fuchsia, illumos))` - same function body, so 026's fix already covers Fuchsia too; (3) `fstat`/`stat`/`lstat`/`statat`'s `MaybeUninit::<Stat>::uninit()` -> `assume_init()` (candidate 039, Apple-specific `st_lspare`/`st_qspare`) is actually the single shared "main version" branch used by every `libc`-backend target including all 4 in scope here - 039's fix (`uninit()` -> `zeroed()`) is already unconditional, not Apple-gated, so it already covers OpenBSD/Fuchsia/Redox/wasi too. None of the 3 justified a new candidate folder (same bug, same lines, already-recorded fix) |
| Miri cross-interpretation of OpenBSD's own hand-rolled `libc_dirent` mirror struct (`src/backend/libc/fs/dir.rs`, used because libc's real OpenBSD `dirent` has a private field) - the exact "layout mismatch" bug class that caught 039 on Apple - via a standalone `libc = "=0.2.189"` crate replicating rustix's mirror struct and its field-offset comparison logic verbatim, run with `cargo +nightly miri run --target x86_64-unknown-openbsd -Zbuild-std=std,core,alloc,panic_abort` (real vendored `libc` crate for OpenBSD, no real OpenBSD hardware needed - confirmed `-Zbuild-std` lets Miri interpret a target with no prebuilt `rust-std` at all, and that Miri doesn't need the linker to succeed, unlike plain `cargo build`/`check`, which fails at the link step for OpenBSD/Fuchsia in this sandbox: no sysroot/system libraries) | `cargo +nightly miri run --target x86_64-unknown-openbsd -Zbuild-std=...`, 1 run (deterministic pure computation, no repeat needed) | **Clean negative**: `size_of::<LibcDirent>() = 280, size_of::<libc::dirent>() = 280`; `mirror offsets = (0, 8, 16, 18, 19, 24)`; `real offsets = (0, 8, 16, 18, 19, 24)` - byte-identical. rustix's hand-rolled OpenBSD `dirent` mirror genuinely matches the real vendored `libc` crate's OpenBSD struct layout; no bug here |
| Miri shim-availability probe: can Miri actually interpret `tcgetattr`/`fstat` (the two syscalls candidates 039/050 found real UB through, on Apple) cross-interpreted for Fuchsia/Redox/OpenBSD specifically | tiny standalone bins calling raw `libc::tcgetattr`/`libc::fstat`, `cargo +nightly miri run --target <x86_64-unknown-fuchsia\|redox\|openbsd> -Zbuild-std=...` | **No coverage for any of the 3**: `tcgetattr` has no Miri shim at all for fuchsia or redox ("can't call foreign function `tcgetattr` on OS ..."); `fstat` hits a Miri-internal panic ("`fstat` should not be called on openbsd/fuchsia/redox" - these 3 targets' libc wrapper doesn't route through the `fstat` symbol name Miri's unix-fs shim expects) on all 3. Same "no shim, no hardware" outcome candidate 050 already documented for QNX/illumos/Apple-kqueue - now independently reconfirmed for 3 more targets; not pursued further per that precedent |
| Direct cross-target compile probe (no Miri, no cross-interpretation at all - just `cargo check`/`cargo test --no-run --target wasm32-wasip1`, which has a real prebuilt `rust-std` via plain `rustup target add`, no `-Zbuild-std` needed) of the full rustix test suite, after the above 3 approaches turned up nothing new for OpenBSD/Fuchsia/Redox specifically | `cargo test --no-run --target wasm32-wasip1 --features "std,fs" --lib`, stable toolchain, 3 repeat runs | **Reproduces 3/3 runs, byte-identical error set every time** (inherently deterministic - a compile error, not runtime behavior): 24 errors in the `lib test` binary, the majority (18 of 24) being `error[E0609]/[E0026]: no field \`st_atime\`/\`st_mtime\`/\`st_ctime\`(_nsec) on type \`libc::stat\`` from `src/fs/constants.rs`'s own `#[cfg(test)]` self-check module. A separate, unrelated pair of errors (`cannot find \`ugid\` in the crate root`, `cannot find function \`dup2\``) also appears and is **not** part of this candidate's fix - see "What's not done" |

## Status

**Confirmed.**

The `st_atime`/`st_mtime`/`st_ctime` (+`_nsec`) field-name compile failure reproduces 3/3 runs,
byte-identical every time - this is a `rustc` compile error over fixed source code and a fixed
`libc` crate version, not runtime behavior, so it is inherently 100% deterministic, not merely
"observed 3 times." Verified both before and after the fix (see `output.txt`).

**Fully reproducible**: **Yes**. This is a single-participant finding (rustix's own `#[cfg(test)]`
code failing to compile against the project's own real, unmodified, vendored `libc` dependency for
a real Rust target) - there is no adversarial peer/caller to simulate, so `CONFIRMATION.md` section
11's two-participant bar doesn't apply (same category as this workspace's own candidate 004/039).
`repro/check.sh` runs the real `cargo test --no-run --target wasm32-wasip1` command against a real,
freshly-cloned checkout of the project at the exact commit this candidate was found on - nothing
about the trigger path is hand-constructed or mocked.

## Class

build-failure

## Root cause

`src/backend/libc/fs/types.rs` defines `pub type Stat = c::stat;` (a direct type alias, not a
separately-defined struct) for every `libc`-backend target except `linux_like`/`hurd`/`netbsd`
(which use `c::stat64` or a custom struct instead) - this includes all 4 targets in scope for this
candidate (OpenBSD, Fuchsia, Redox, wasi).

`src/fs/constants.rs`'s `#[cfg(test)]` `test_layouts` function contains a self-check, added by
commit 7202da6e (see "Introduced by" below), that verifies `Stat`'s field offsets match `c::stat`'s
via `check_renamed_struct_field!(Stat, stat, st_atime)` (and the `st_mtime`/`st_ctime` + `_nsec`
siblings), plus a direct `assert_eq!(some_stat.st_atime, 0_i64)` a few lines earlier. Both forms
assume every target's vendored `libc::stat` exposes **flat** `st_atime`/`st_atime_nsec` etc.
integer fields - true for every `libc`-backend target the authors apparently tested against (Apple,
the BSDs, Solaris/illumos, Haiku, ...), but **not** for wasi: the `libc` crate's own WASI binding
(`libc-0.2.189`'s `src/wasi/mod.rs`) represents these as **nested** `st_atim`/`st_mtim`/
`st_ctim: timespec` sub-structs instead (matching WASI's own `__wasi_filestat_t` naming), plus an
explicit `__reserved: Padding<[c_longlong; 3]>` field at the very end that nothing in rustix reads.
Since `Stat` is a direct alias for `c::stat` on wasi, `Stat.st_atime` is not merely a mismatched
offset to assert about - the field doesn't exist under that name at all, so `rustc` rejects it at
compile time (`E0609`/`E0026`), not at runtime. This only affects `#[cfg(test)]` code - no shipped,
non-test code path is affected, since the real `fstat`/`stat`/`lstat`/`statat` implementations
operate on `Stat`/`c::stat` through its real (nested, for wasi) field layout via raw pointer casts,
never naming `st_atime` directly outside this test module. Practical impact: `cargo test
--target wasm32-wasip1` (or `wasm32-wasip2`/`wasm32-wasip3`) has never been able to compile for this
crate - this project's own CI (`.github/workflows/main.yml`) only ever runs `cargo check` (not
`--tests`, not `cargo test`) for `wasm32-wasip1`/`wasm32-wasip2`, which is why this has gone
unnoticed: CI genuinely never compiles this code path for wasi.

## Introduced by

7202da6e "Reorganize Stat, Statx, and SockaddrStorage (#1282)" (2025-01-29) - this commit
introduced the current `check_renamed_struct_field!(Stat, stat, st_atime/_mtime/_ctime...)` call
chain and the `assert_eq!(some_stat.st_atime, ...)` block in their present form (verified via
`git log -S st_atime_nsec -- src/fs/constants.rs`, the earliest/only hit touching this exact
pattern). wasi support in the crate at large predates this commit by years, so this is a regression
introduced by the Stat-reorganization commit itself, not something wasi-specific that was never
supported - the self-check simply wasn't exercised against the wasi target at the time (consistent
with CI never running `--tests` for wasi, as noted above).

## The fix

Gated the flat `st_atime`/`st_mtime`/`st_ctime` (+`_nsec`) checks - both the `assert_eq!` block
(lines ~84-90) and the `check_renamed_struct_field!`/`check_renamed_struct_renamed_field!` call
chain (lines ~160-174) - behind `#[cfg(not(target_os = "wasi"))]`, mirroring the existing style
already used to special-case NetBSD's differently-named fields two lines above. This is sufficient
and safe because `Stat` is a direct type alias for `c::stat` on wasi (not a separately-defined
struct rustix controls the layout of) - there is no "renaming" to verify in the first place on this
target, so skipping the check loses no real coverage; the struct's offsets are tautologically
identical to themselves regardless.

**Checked against more than the one triggering symptom (CONFIRMATION.md section 7)**: fixed both of
the two distinct code locations that reference the flat field names (not just the first one hit),
and re-ran the full `cargo test --no-run --target wasm32-wasip1 --features "std,fs" --lib` 3 times
after the fix to confirm the specific error signature (`st_atime`/`st_mtime`/`st_ctime` field
errors) is gone deterministically, not just absent on one lucky run.

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host `x86_64-unknown-linux-gnu`): **411 passed, 1 failed, identical before and after** - the one
  failure is the pre-existing, environment-caused `ioctl::test_int_setter` (`/dev/net/tun` missing
  in this sandbox, documented in `projects/rustix.md`), not a regression from this patch.
- `wasm32-wasip1` compile check (`cargo test --no-run --target wasm32-wasip1 --features "std,fs"
  --lib`): 24 errors before (18 of them the `st_atime`/`st_mtime`/`st_ctime` family) -> 3 errors
  after (only the unrelated `ugid`/`dup2` errors remain, see below), reproduced 3/3 runs each way.

## What's not done / open questions

- **Two separate, pre-existing compile errors remain for `wasm32-wasip1` after this fix** -
  `cannot find \`ugid\` in the crate root` (`src/fs/constants.rs:103-104`, accessing
  `crate::ugid::RawUid`/`RawGid`) and `cannot find function \`dup2\`` (`src/backend/libc/fs/
  dir.rs:496`, in the `dir_iterator_handles_io_errors` test). Both are a **different** bug class
  (a `#[cfg]` gate on a module/function that's narrower than what the test code calling it assumes
  - `mod ugid;` in `src/lib.rs:429` is gated off for `target_os = "wasi"` outright, and `dup2` is
  similarly `#[cfg(not(target_os = "wasi"))]`) - not fixed here, kept as a separate, single-bug-per-
  candidate concern per this workspace's own convention; worth a follow-up candidate.
- Not checked: whether `wasm32-wasip2`/`wasm32-wasip3`/`wasm32-wasip1-threads` have the identical
  three remaining errors or something additionally different - only `wasm32-wasip1` was used for
  this candidate's repro (arbitrarily, as the first one with a prebuilt `rust-std`).
- The OpenBSD `libc_dirent`-layout and Fuchsia/Redox/OpenBSD Miri-shim-availability findings above
  (both negative/tooling-gap results) are recorded here for the record but didn't yield a second
  candidate - see `projects/rustix.md` for where this is folded into durable project knowledge.
- Per "Critical pitfalls" #7: the same "flat timestamp field name assumed, wasi uses nested
  `timespec` sub-structs" class could in principle recur anywhere else in the crate that names
  `st_atime`/`st_mtime`/`st_ctime` directly rather than going through `Stat`'s own (correct, pointer
  -cast-based) conversion path - a `grep -rn "\.st_atime\b\|\.st_mtime\b\|\.st_ctime\b"` across
  `src/` beyond `src/fs/constants.rs` was not done this session.
