# cvt_msg_cmsg_data_stacked_borrows_violation

`AncillaryDrain::cvt_msg` (`src/net/send_recv/msg.rs:551`, the function that parses a received
ancillary (`cmsghdr`) message off a `recvmsg()` result) derives a payload pointer via
`c::CMSG_DATA(msg)` from a reference to just the `cmsghdr` header, then immediately builds a
*mutable* slice from that pointer via `slice::from_raw_parts_mut` - a Stacked Borrows violation
under Miri, on every single received ancillary message (valid or not), regardless of message type
or content. This was explicitly named as an open, unresolved lead in candidate 063's own RESULT.md
("The unrelated Stacked-Borrows violation ... hit on every SCM_CREDENTIALS message under strict
Miri aliasing checking, worked around here with `-Zmiri-disable-stacked-borrows` ... was not
investigated - noted as a new suspect, not fixed or further characterized here").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Plain debug build, real `AF_UNIX` `socketpair`, real `sendmsg`/`recvmsg` round trip carrying a real `SCM_RIGHTS` message via rustix's own public API (`socketpair`, `sendmsg`, `recvmsg`, `RecvAncillaryBuffer::drain()`) | `cargo run`, debug, x86_64-unknown-linux-gnu | No crash - silently receives the fd and prints it. Confirms the bug is real but invisible without stricter tooling in a plain build (expected per CONFIRMATION.md section 2). Also captured the genuine kernel-produced `cmsghdr`+payload bytes for reuse under Miri |
| `cargo +nightly miri run`, libc backend (`RUSTFLAGS="--cfg rustix_no_linux_raw"`, `linux_raw`'s inline-`asm!` syscalls can't run under Miri at all, per this workspace's `README.md`), the exact genuine bytes captured above (zero bytes patched - the finding doesn't depend on payload content at all), fed through the real public `AncillaryDrain::parse` entry point, no `-Zmiri-disable-stacked-borrows` (unlike candidate 063 - Stacked Borrows checking is exactly what this candidate is about) | Miri (nightly-x86_64-unknown-linux-gnu), 3 separate runs against the pristine tree | **Confirmed**: `error: Undefined Behavior: trying to retag from <N> for Unique permission at alloc...[0x10], but that tag does not exist in the borrow stack for this location` at `src/net/send_recv/msg.rs:558:43` (the `slice::from_raw_parts_mut` call), identical location on all 3 runs, exit code 1 every time |
| Attempted to run the *same* real socketpair/sendmsg/recvmsg round trip directly under Miri (not the captured-bytes workaround), to see whether Miri could execute the real end-to-end path without any byte-capture step at all | `RUSTFLAGS="--cfg rustix_no_linux_raw" MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri run --bin end_to_end` | Confirms Miri cannot execute rustix's real `sendmsg`/`recvmsg` syscalls at all: aborts with `error: unsupported operation: can't call foreign function \`sendmsg\` on OS \`linux\`` before `cvt_msg` is ever reached. (Along the way, before even reaching `sendmsg`, it separately also aborted on a different, unrelated Stacked Borrows violation in `push_ancillary`'s own `CMSG_DATA` usage on the *send* side - see "What's not done" below.) This confirms the captured-genuine-bytes-via-`AncillaryDrain::parse` approach (the previous row) is the correct, already-established methodology for this exact function under this project's toolchain, not a shortcut |

2 escalating approaches were used (plain debug build, then Miri), plus the one direct attempt at a
fully-real Miri round trip that determined why the captured-bytes workaround is necessary (same
reasoning, and same conclusion, as candidate 063's own investigation of this exact limitation). A
third independent tool (ASAN/UBSAN) wasn't needed - Miri already produced an unambiguous, precisely
located, deterministic diagnostic (CONFIRMATION.md section 5) for exactly the bug class (Stacked
Borrows aliasing violation) Miri is built to catch.

## Status

**Confirmed**

Reproduces deterministically under Miri: 3/3 runs, nightly Miri, libc backend
(`--cfg rustix_no_linux_raw`), same exact diagnostic and source location (`msg.rs:558:43`,
`slice::from_raw_parts_mut`) every time, exit code 1 every time. Does not produce an externally
visible crash in a plain (non-Miri) debug or release build - the ancillary message is parsed and
returned normally; this is expected and does not weaken the finding (CONFIRMATION.md section 2: a
bug needing instrumentation to manifest is not less real for needing it).

**Fully reproducible**: **Yes**.

Unlike candidate 063's `pid == 0` finding (which needed a specific, hard-to-reproduce kernel
behavior - cross-PID-namespace credential delivery), this bug is a pure Rust-level aliasing
violation in rustix's own pointer handling inside `cvt_msg`. It does not depend on any unusual or
adversarial behavior from a peer: `C::CMSG_DATA`/`slice::from_raw_parts_mut` run unconditionally at
the top of `cvt_msg` for *every* received ancillary message, regardless of its type or payload
content. The real end-to-end repro (`repro/src/bin/end_to_end.rs`) demonstrates this directly: two
fully cooperative, ordinary ends of a real `AF_UNIX` `socketpair` exchange one file descriptor via
`SCM_RIGHTS` - completely standard fd-passing usage, no edge case constructed at all - and the
resulting genuine kernel-written bytes, fed unmodified through the real public `AncillaryDrain`
parsing path, trigger the violation under Miri every time (`repro/src/bin/miri_check.rs`). The only
reason `miri_check.rs` exists as a separate binary from `end_to_end.rs`, rather than running the
real `sendmsg`/`recvmsg` syscalls directly under Miri, is that Miri itself cannot execute those
syscalls at all (confirmed directly this session - see "What was tried" row 3); this is a Miri
tooling limitation, not a gap in what's being demonstrated - every byte `miri_check.rs` uses is a
verbatim, unmodified capture from a real `recvmsg()` delivery in `end_to_end.rs`.

## Class

UB, memory-safety (Stacked Borrows violation: retagging a pointer for `Unique`/write access at a
location the pointer's own tag was never granted for)

## Root cause

`Messages::next()` (the internal iterator `AncillaryDrain` walks) used to yield `&'a mut
c::cmsghdr` - a reference typed as just the fixed-size `cmsghdr` header struct
(`size_of::<c::cmsghdr>()`, 16 bytes on x86_64 Linux). `cvt_msg` then called `c::CMSG_DATA(msg)`,
which computes `cmsg.offset(1)` - i.e. a pointer to the variable-length *payload* that starts
immediately after the header and can extend arbitrarily far beyond it (up to `cmsg_len`). Under
Stacked Borrows, a reference's retag only grants read/write permission for the byte range covered
by its own pointee type's size - the reference to the 16-byte header was never granted any
permission over memory past those 16 bytes at all, regardless of whether that reference was shared
(`&c::cmsghdr`, the original code) or unique (`&mut c::cmsghdr`, tried and found insufficient
during this session's own fix attempt - see "The fix" below). `slice::from_raw_parts_mut` on the
resulting out-of-footprint pointer then tries to retag for `Unique` (write) access at a location
where the header reference's tag was never present in the borrow stack at all, which is exactly
what Miri's Stacked Borrows checker flags.

The same pattern independently exists in `push_ancillary` (`src/net/send_recv/msg.rs:325`, the
*send*-side counterpart that writes a `cmsghdr` header and its payload when building an outgoing
message) - also fixed as part of this candidate's patch, see "The fix" below, since both share the
same root mechanism and the same `Messages` iterator.

## Introduced by

2cdbb1d4 "feat: Add sendmsg/recvmsg (#505)" (2023-04-28) - this commit added the whole
`send_recv/msg.rs` module from scratch, including `AncillaryDrain::cvt_msg`'s
`CMSG_DATA`/`slice::from_raw_parts_mut` pattern and `Messages`'s `&'a mut c::cmsghdr` iterator item
type, exactly as they stood (modulo unrelated refactors) before this candidate's fix. `git log -S
"CMSG_DATA(msg)" -- src/net/send_recv/msg.rs` returns only this one commit as having introduced
that call pattern.

## The fix

Two related changes, both needed because narrowing to a typed reference at any point before the
`CMSG_DATA` pointer-arithmetic step reintroduces the same footprint problem regardless of
mutability:

1. `messages::Messages`'s `Iterator::Item` changed from `&'a mut c::cmsghdr` to `NonNull<c::cmsghdr>`
   (a raw pointer, no reference ever constructed over just the header). `next()` now returns the
   `NonNull` directly instead of `&mut *header.as_ptr()`.
2. `AncillaryDrain::advance`/`cvt_msg` changed to accept `NonNull<c::cmsghdr>` instead of
   `&c::cmsghdr`, and work with `*mut c::cmsghdr` throughout (`(*msg).cmsg_len` etc. via raw
   deref, `c::CMSG_DATA(msg.cast_const())`) rather than ever creating a `&`/`&mut c::cmsghdr`
   reference. `push_ancillary` (the send-side counterpart, using the same `Messages` iterator) was
   updated the same way, since it hit the identical pattern (confirmed directly: the real
   end-to-end repro, run under Miri, aborted on this exact pattern in `push_ancillary` before even
   reaching the receive side).

Note on the fix's own development: an intermediate attempt changed only the reference's mutability
(`msg: &c::cmsghdr` -> `msg: &mut c::cmsghdr`, keeping `CMSG_DATA` fed through a reference) and was
**re-verified to still reproduce the identical failure** under Miri (3/3 runs) - the SharedReadOnly
tag became a SharedReadWrite tag, but the violation persisted at the same line, because the
reference's retag footprint is bounded by its pointee's `size_of` regardless of mutability. Only
switching to a raw-pointer-only representation of `msg`, with no reference to the header ever
constructed, resolved it. This matters for the next reader: "use `&mut` instead of `&`" is *not*
sufficient here, unlike the more common shared-vs-unique Stacked Borrows fix pattern.

Verified: re-ran the same `miri_check` binary against the patched tree, 3/3 runs, all exit code 0,
printing `FIXED: reached past cvt_msg without a Stacked Borrows abort, ...` - no UB diagnostic. Also
re-ran `end_to_end` (the real socketpair round trip) against the patched tree - unaffected,
continues to send/receive the real fd correctly (this binary never demonstrated the bug directly in
the first place, since it's invisible outside Miri, but confirms the fix doesn't change observable
behavior).

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`):
  **411 passed, 1 failed, both before and after** the patch - identical counts, identical single
  failure (`ioctl::test_int_setter`, pre-existing and unrelated: `/dev/net/tun` doesn't exist in
  this sandbox, already noted in this workspace's `README.md` and in candidate 063's own
  RESULT.md). No new failures introduced by the fix.
- `cargo clippy --features "std,all-apis,use-libc"`: no new warnings in `src/net/send_recv/msg.rs`
  before or after (2 pre-existing, unrelated `wrong_self_convention` warnings in
  `src/timespec.rs` both before and after).
- Builds cleanly under both backends (`cargo build --features "std,net,time,use-libc"` for
  `linux_raw`, and the same with `RUSTFLAGS="--cfg rustix_no_linux_raw"` for `libc`).

## What's not done / open questions

- `push_ancillary`'s own, separate Stacked Borrows violation (discovered this session while trying
  to run the real end-to-end repro under Miri - distinct call site, same root pattern) is fixed as
  part of this same patch rather than split into its own candidate, since it's the exact same
  mechanism (`Messages` iterator -> `CMSG_DATA` -> raw pointer arithmetic past a too-narrow
  reference's footprint) in the same file, sharing the same iterator this candidate already had to
  change. Not spending a separate candidate slot re-deriving the identical root cause.
- The same "reference derived, then `.offset()` past its own footprint" pattern might exist
  elsewhere in the codebase wherever C-style variable-length structures (anything using
  `CMSG_NXTHDR`-style header-plus-trailing-data access, or similar) are parsed via a typed
  reference rather than a raw pointer - not swept for this session; `src/net/send_recv/msg.rs` was
  the only file checked.
- Did not investigate whether Tree Borrows (Miri's newer, stricter-by-default-in-some-configs
  aliasing model, `-Zmiri-tree-borrows`) flags the same or a different diagnostic here - Stacked
  Borrows (the default) was sufficient to confirm and fix this candidate.
