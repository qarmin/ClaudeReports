# qnx_termios_reserved_field_uninitialized

Follow-up to candidate 050, which flagged this as "the structurally most promising remaining
lead" but could not test it: on `target_os = "nto"` (QNX) only, rustix's own `Termios` struct
(`src/termios/types.rs:49`) carries a `pub(crate) __reserved: [ffi::c_uint; 3]` field that is
never explicitly written anywhere in rustix's own code - unlike the sibling `futex_waitv::
__reserved` field (`src/thread/futex.rs:535,546`), which IS explicitly zeroed in its own
constructor. The non-Linux `tcgetattr` path (`src/backend/libc/termios/syscalls.rs:100-109`)
builds the whole `Termios` via `MaybeUninit::<Termios>::uninit()` -> real `c::tcgetattr()` FFI
call -> `assume_init()`, which is instant UB on Rust's abstract machine if `__reserved`'s 12 bytes
are not actually written by the real QNX syscall.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Re-run the Miri cross-interpretation attempt from candidate 050, on the current nightly (rustc 1.101.0-nightly 2026-09-29, one session newer than candidate 050's), to check whether a shim gap had since closed | `RUSTFLAGS="--cfg rustix_no_linux_raw" MIRIFLAGS="-Zmiri-disable-isolation" cargo miri run --target x86_64-pc-nto-qnx710`, rustix's real public `tcgetattr()` API, both directly and via `repro/`'s self-contained `just test` (fresh clone from GitHub, independently re-verified) | Identical outcome to candidate 050: `error: unsupported operation: can't call foreign function 'tcgetattr' on OS 'nto'` - Miri still has no shim for this function on this OS at all. No execution reached the question of whether `__reserved` ends up initialized. See `output.txt` section 1. |
| Documentary/archaeological investigation of the `libc` crate's own treatment of this exact field, via its real GitHub history (`git blame`/`git log -S`/GitHub commit-search API, all real network queries, not recollection) | `libc` 0.2.189 (the version this repo's `Cargo.toml` constraints resolve to), `src/types.rs`, `src/unix/nto/mod.rs`; GitHub API search across `rust-lang/libc`'s real commit/issue history | Found that `libc` 0.2.189 defines QNX's `struct termios`'s `reserved` field as `__reserved: Padding<[c_uint; 3]>` - a **private** (no `pub`), `MaybeUninit`-backed wrapper type libc's own maintainers created specifically because, per the seminal issue (#1453, 2019, quoted in full in `output.txt` section 2): "libc uses integers and aggregates of integers for explicit padding byte fields. When these fields are returned from C, they might be uninitialized." This is the exact bug class rustix's own `Termios::__reserved` is NOT protected against - rustix mirrors the same field as a plain, directly-readable `[ffi::c_uint; 3]`, not wrapped in `MaybeUninit`/`Padding`. See `output.txt` sections 2-3. |
| Source-archaeology for QNX's own real `tcgetattr()` implementation, via the OpenQNX open-source archive (`vocho/openqnx` on GitHub - QNX 6.3.0's userspace libc, genuinely released under a QNX source license in 2007) and QNX's own official documentation pages (fetched live via `WebSearch`, not recalled) | Cloned `vocho/openqnx` (sparse checkout), read `trunk/lib/c/1/tcgetattr.c` in full; read QNX's own `termios`/`tcgetattr`/`DCMD_CHR_TCGETATTR` doc pages (qnx.com, versions 6.4.1-7.1) | QNX6's real userspace `tcgetattr()` is a 3-line wrapper that forwards the caller's own buffer pointer straight into `_devctl(fd, DCMD_CHR_TCGETATTR, termios_p, sizeof(*termios_p), ...)` - the actual byte-fill behavior lives entirely in the kernel-side io-char/tty resource-manager driver, whose source is **not** part of the open-sourced subset (`devc-ditto`, the only `devc-*` driver present in this mirror, has no `DCMD_CHR_TCGETATTR` handling of its own). QNX's own official docs name and place the field (`reserved[3]`, between `c_cc` and `c_ispeed`/`c_ospeed` - exactly matching rustix's placement) but state no contract about what `tcgetattr()` writes into it. This closes off the one remaining avenue that could have settled the question definitively (reading the real fill logic) - it is simply not available to inspect, open-source or otherwise, in this or any publicly accessible environment. See `output.txt` sections 4-5. |

## Status

**Not reproducible.** No execution - real, Miri-interpreted, or otherwise - was ever obtained
that actually observed the `__reserved` field's contents after a real or simulated QNX
`tcgetattr()` call. Per `CONFIRMATION.md` section 1, a claim with zero observed occurrences
cannot be written `Confirmed`; this is a stronger case than a typical "not reproducible" in that
every avenue tried converged on evidence that *tightens the suspicion* (see "Root cause" below)
without ever crossing into an actual observation. This is the same bottom line candidate 050
reached for this exact lead, now backed by substantially more evidence: candidate 050 could only
state that the field "really is a pub(crate) field rustix's own code never writes" and that Miri
had no shim; this session additionally establishes that (a) the sibling `libc` crate's own
maintainers independently modeled this *exact kind* of field (an OS-returned reserved/padding
slot) as potentially-uninitialized as a matter of policy, across every platform they support, not
specifically verified for QNX but applied as a blanket rule to exactly this bug shape, and (b) the
real QNX6 kernel-side fill logic is not available to inspect in any publicly accessible source,
open or closed - there is no further documentary avenue left to try with the tools available in
this environment.

**Fully reproducible**: **N/A** (no Confirmed finding this session).

## Class

N/A (Not reproducible)

## Root cause (why this remains suspicious but could not be settled)

`src/termios/types.rs:48-49` declares `#[cfg(target_os = "nto")] pub(crate) __reserved: [ffi::
c_uint; 3]` as a plain, directly-typed integer array - introduced by commit c564105e (2023-07-14,
"Convert `assert_eq`+`size_of` asserts to `static_assertions`", #738), whose own stated purpose
was to "adjust the layout of the `Termios` struct to fix some differences with the libc `termios`
on some platforms" (see `output.txt` section 6 for the real diff) - i.e. the field was added
purely to make `size_of::<Termios>()` match the real OS struct's size (fixing what was likely an
out-of-bounds-write risk from a too-small buffer being handed to `tcgetattr()`), with no apparent
consideration given to whether the field's contents are ever actually written by the OS.
`src/backend/libc/termios/syscalls.rs:100-109`'s non-Linux `tcgetattr()` path then builds the
*entire* `Termios` (including this field) via `MaybeUninit::<Termios>::uninit()` -> a real,
opaque `c::tcgetattr()` FFI call that receives only a raw, type-erased pointer and byte count ->
`assume_init()`. `assume_init()` requires every byte of the referent to be genuinely initialized,
regardless of whether all bit patterns of the field's own type (`c_uint`) would otherwise be
"valid" - if the real OS leaves these 12 bytes untouched, this is unconditionally UB under Rust's
current validity rules, the identical bug class candidate 039 confirmed for Apple's `fstat`
`st_lspare`/`st_qspare` fields (there, independently corroborated by a real upstream bug report on
real hardware; here, no equivalent corroboration exists or could be obtained).

What keeps this from crossing into `Confirmed`: unlike candidate 039's Apple case, there is no
real-hardware report, no emulator, and no available kernel-side source to check what QNX's
`tcgetattr()` actually does with these 12 bytes. Two plausible, mutually exclusive, both-reasonable
scenarios remain open: (a) the io-char driver maintains one persistent, fully-populated `struct
termios` per device from the moment the device is created (the same architectural pattern that
made candidate 050's `uname`/`sysinfo` suspicion a false alarm on Linux - a single `memcpy`-style
copy-out of an already-fully-initialized struct naturally fills every byte, reserved or not), in
which case this is sound in practice despite being unsound per the type system; or (b) the driver
fills only the fields it recognizes by name/offset and leaves `reserved` genuinely untouched
(exactly what `libc`'s own `Padding` policy assumes as the default-safe posture for this class of
field), in which case every `tcgetattr()` call on QNX is real, exploitable UB. Nothing available in
this environment distinguishes the two.

## Introduced by

N/A (Not reproducible - there is no confirmed bug to attribute). For reference, the commit that
introduced the specific `Termios::__reserved` field this candidate is about is c564105e "Convert
`assert_eq`+`size_of` asserts to `static_assertions`. (#738)" (2023-07-14) - see "Root cause"
above; this is not an "Introduced by" in the confirmed-bug sense, just the most relevant commit a
future session would want to look at first.

## The fix

No fix attempted - no bounded, verifiable fix can be written and verified against an actual
reproduction when no reproduction was ever obtained (see `CONFIRMATION.md` section 6: a fix must
be re-verified against the same reproduction, which does not exist here). No source changes were
made to the main project in this session.

A defensive change *would* be easy to write if a future session wants to apply the same
precautionary posture `libc`'s own maintainers already apply to this exact field shape, without
waiting for QNX-specific confirmation: change the non-Linux `tcgetattr` path
(`src/backend/libc/termios/syscalls.rs:100-109`) from `MaybeUninit::<Termios>::uninit()` to
`MaybeUninit::<Termios>::zeroed()`, mirroring candidate 039's fix for the structurally identical
Apple `fstat` case and rustix's own existing `futex_waitv::__reserved` precedent. This was
deliberately **not** applied as `patch.diff` this session, per the workspace rule that a patch is
only attached for a `Confirmed` finding with a verified fix - writing one here with nothing to
verify it against would be exactly the "fix that looks done but isn't actually verified" trap this
skill's own quality checklist warns against.

- Project test suite: not run (no code change to validate, per `SKILL.md` step 3's "Not
  reproducible" path - steps 4-8, including the mandatory before/after test-suite run, apply only
  once a candidate reaches a reproduction).

## What's not done / open questions

- **The actual kernel-side QNX io-char/tty resource-manager source that implements
  `DCMD_CHR_TCGETATTR`** remains the one thing that would settle this definitively, and it is not
  available anywhere this session could reach: not in the OpenQNX 2007 open-source release (which
  covers QNX6 userspace libc and utilities, but not the proprietary device-driver framework), not
  in QNX's own public documentation (which documents the field's existence and placement but not
  its initialization contract), and not accessible without a licensed QNX SDP installation and
  real or emulated QNX hardware, neither of which exists in this environment.
- **Re-attempt if a future Miri version ships an `nto` `tcgetattr` shim** - this is the second
  consecutive session (050, then this one) to hit the identical "unsupported foreign function"
  wall on this exact call, now confirmed stable across at least one nightly-toolchain bump
  (candidate 050's session -> this session's 2026-09-29 nightly). Treat a third identical attempt
  with no new Miri version and no new QNX access as low-value - the same judgment `projects/
  rustix.md` already records for the unrelated io_uring opcode wall (candidates 008/030) after it
  held stable across two consecutive sessions on this same container.
- **The defensive `zeroed()` change described above was not applied.** A future session that
  decides the documentary evidence here is strong enough to act on without further confirmation
  could apply it directly - it is a pure safety hardening with no behavioral downside even in the
  scenario where QNX already fully initializes the field (writing zero bytes that are about to be
  overwritten by the real `tcgetattr()` call is free), so unlike most of this skill's findings, the
  usual "verify it actually fixes something" bar may reasonably be relaxed for this one specific
  kind of change if a maintainer wants to take it as a precaution rather than a confirmed fix.
- **Other QNX-specific reserved/padding fields in rustix's own mirrored structs were not swept
  this session** - this candidate was scoped specifically to `Termios::__reserved`, the one lead
  carried forward from candidate 050. `libc` 0.2.189's own `src/unix/nto/mod.rs` lists over a dozen
  other `Padding<T>`-wrapped fields across other structs (`dirent.d_reserved`, `sembuf`-adjacent
  `reserved[10]` fields, `msqid_ds.msg_pad1-4`, `sockaddr_storage.__ss_pad1/2`, etc.) - whether
  rustix mirrors any of these with its own plain (non-`MaybeUninit`) field and reads them via an
  uninitialized-then-`assume_init()` path the way `Termios` does was not checked; this is a
  reasonable next candidate for a future session to pick up (see `IDEAS.md`).
