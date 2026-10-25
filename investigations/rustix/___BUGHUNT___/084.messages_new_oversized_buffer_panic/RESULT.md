# messages_new_oversized_buffer_panic

`rustix::net::send_recv::msg::messages::Messages::new` (`src/net/send_recv/msg.rs:940`, reached
through the public `unsafe fn AncillaryDrain::parse` and through the safe `RecvAncillaryBuffer::
drain`) converts a caller-supplied buffer's length into the real C `msghdr.msg_controllen` field
via `buf.len().try_into().expect("buffer too large for msghdr")`. Flagged as a candidate because
this looked like an unwrap/expect on a value only indirectly bounded by the caller (a long-lived,
reused >4GiB allocation on a 64-bit host).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: what type is `c::msghdr.msg_controllen` actually, per backend/platform | Read `libc` crate's `msghdr` struct definitions (`unix/linux_like/linux/gnu/mod.rs`, `unix/linux_like/android/mod.rs`, `unix/bsd/mod.rs`, `unix/solarish/mod.rs`) and `linux-raw-sys`' `x86_64`/`x86` `net.rs` | On every Linux target (both the libc backend and rustix's own `linux_raw` backend, 32- and 64-bit), `msg_controllen` is `size_t`/`usize` - the exact same type as `buf.len()` - so `usize::try_into::<usize>()` can never fail there; the panic's premise (candidate description said "`socklen_t` (u32 on common platforms)") does not hold for Linux at all. On BSD (incl. Apple)/Solaris/illumos, `msg_controllen` is genuinely `socklen_t` (`u32`), so the conversion is real and fallible there on a 64-bit host |
| Native execution, real allocation, Linux host (`x86_64-unknown-linux-gnu`, default `linux_raw` backend) | `release` build, `vec![0u8; u32::MAX as usize + 1]` (a real ~4.3GB zeroed allocation, backed by lazy zero-fill-on-demand pages so it completes instantly) fed to the real `AncillaryDrain::parse` | **No panic** - confirms the Linux-only part of the code-reading finding above; this is not a reachable bug on the most common 64-bit Linux host |
| Miri cross-interpretation of the REAL, unmodified `Messages::new`/`AncillaryDrain::parse` for `x86_64-apple-darwin` (libc backend, real `socklen_t` `msg_controllen`) - no mock needed at all, since the panic fires purely from `buf.len()` before any syscall or ancillary-message content is ever touched | `MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri run --target x86_64-apple-darwin`, debug build, real `vec![0u8; N]` allocations of varying size fed to the real public `rustix::net::AncillaryDrain::parse` | **Reproduced**: `N = u32::MAX as usize + 1` (and `N = 3 * u32::MAX as usize`) panics with `called \`Result::unwrap()\`/\`.expect()\` on an \`Err\` value: TryFromIntError(PosOverflow)` at `src/net/send_recv/msg.rs:940:58`, 5/5 runs, identical message/location every time; `N = 16` and `N = u32::MAX as usize` (the exact boundary) both construct cleanly with no panic, every run - confirming the panic is driven exactly by the buffer length crossing `u32::MAX`, not by any other incidental factor |

## Status

**Confirmed**, reproduces 5/5 runs (debug build, Miri cross-interpretation for
`x86_64-apple-darwin`, deterministic - no layout/seed dependency; the panic is driven entirely by
the buffer length passed in, not memory layout), plus the two boundary-value variants (`u32::MAX`
exactly, and a small 16-byte buffer) behaving correctly (no panic) in all 5 of the same runs, and a
third variant (`3 * u32::MAX`, well past the boundary) also panicking identically.

**Fully reproducible**: **No**, justified - the same narrow `CONFIRMATION.md` section 11 exception
already used by candidates 038/039/058. This bug needs a 64-bit host whose real C `msghdr.
msg_controllen` field is narrower than `usize` (a 32-bit `socklen_t`: BSD/macOS/Solaris/illumos -
**not** Linux, where it's `size_t`/`usize` and the conversion is infallible, confirmed above by a
real native run). This sandboxed environment has no macOS SDK/cross-linker (`xcrun` absent) and no
FreeBSD/Solaris/illumos hardware, matching the already-established conclusion of candidates
038/039/058 for this same container. Given that:

- Unlike several sibling candidates (e.g. 058's `getpath`), **no kernel mock of any kind was
  needed** here: the bug fires purely from the *length* of a buffer the caller itself allocates and
  passes to the real, unmodified public API (`AncillaryDrain::parse`) - no syscall, no peer, no
  kernel-shaped response is involved at all before the panic point. This makes the Miri
  cross-interpretation strictly cleaner than the mocked-syscall cases: every line executed (buffer
  allocation, `rustix::net::AncillaryDrain::parse`'s real code, `Messages::new`'s real code, the
  real `libc::CMSG_FIRSTHDR`) is the exact, unmodified source being tested, compiled for the real
  `cfg(apple)`/BSD-shaped target, with nothing hand-authored standing in for a second participant.
- The "external input" this bug reacts to is simply the size of a buffer a real caller legitimately
  allocates (e.g. a long-lived, reused >4GiB buffer, as the candidate description's own framing
  suggested) - there is no second participant to fake here at all (no peer, no upstream caller
  producing bytes on the wire), so this sits in the same "single external artifact, no second
  participant" category CONFIRMATION.md section 11 describes as not needing the two-participant
  recipe in the first place; the only reason this candidate still isn't "Yes" is the cross-platform
  execution constraint (no real BSD/Apple/Solaris hardware/linker), not anything about faking an
  interaction.

## Class

crash

## Root cause

`messages::Messages::new` (`src/net/send_recv/msg.rs:940`) builds its own local `c::msghdr` and
sets its `msg_controllen` field directly from the caller-supplied buffer's length:

```rust
msghdr.msg_controllen = buf.len().try_into().expect("buffer too large for msghdr");
```

`c::msghdr.msg_controllen`'s actual C type depends on the target: on Linux (both the `libc` and
`linux_raw` backends) it is `size_t`/`usize`, so this conversion is infallible there. On BSD
(including Apple)/Solaris/illumos it is a 32-bit `socklen_t`, so a caller-supplied buffer at or
beyond `u32::MAX + 1` bytes - plausible for a long-lived, reused allocation on a 64-bit host, not
inherently abusive input - makes the conversion fail and the `.expect()` panic, reached through two
safe-adjacent call sites: the `pub unsafe fn AncillaryDrain::parse` directly, and the fully safe
`RecvAncillaryBuffer::drain()` (bounded in practice there by whatever length a prior real `recvmsg`
call reported, but `parse` has no such bound - it is handed the caller's raw buffer length
directly).

Critically, **this exact conversion problem was already solved correctly elsewhere in the same
codebase** and simply not reused here: both backends' own `msghdr.rs` (`src/backend/libc/net/
msghdr.rs`, `src/backend/linux_raw/net/msghdr.rs`) already define a private `msg_control_len`
helper used for constructing the *outgoing*/`recvmsg`-syscall-driving `msghdr`s, which saturates
instead of panicking (`len.try_into().unwrap_or(c::socklen_t::MAX)` on BSD/Solaris/musl/etc., a
same-width `len`/cast on Linux). `messages::Messages::new` is separate, shared code (used for
parsing a buffer the caller already has in hand, not for constructing a request to the kernel) that
re-implemented its own length conversion with `.try_into().expect(...)` instead of calling the
already-correct, already-platform-aware helper.

## Introduced by

2cdbb1d49d2efd5030752df1689d621a47e2957c "feat: Add sendmsg/recvmsg (#505)" (2023-04-28) - this is
where `h.msg_controllen = buf.len().try_into().expect("buffer too large for msghdr");` was first
introduced (confirmed via `git log -S "buffer too large for msghdr" --follow -p`). The exact message
text briefly changed to a plain `.unwrap()` in 3b8ffebe "Avoid using `.expect` in release mode.
(#1125)" (2024-08-23, a code-size reduction, still panics identically) and back to `.expect(...)` in
3b0e2861 "Add uninit buffer ancillary APIs (#1108)" (2025-02-25, an unrelated `MaybeUninit`
refactor) - the underlying panic-on-overflow behavior itself has been continuously present since
2cdbb1d4 regardless of which exact panicking call was used.

## The fix

Made both backends' already-correct, already-platform-aware `msg_control_len` helper
`pub(crate)` (it was private to each backend's own `msghdr` module) and had `messages::Messages::
new` call it instead of re-implementing its own `.try_into().expect(...)`:

```rust
// src/net/send_recv/msg.rs
let mut msghdr = msghdr::zero_msghdr();
msghdr.msg_control = buf.as_mut_ptr().cast();
msghdr.msg_controllen = msghdr::msg_control_len(buf.len());
```

This reuses the exact saturating behavior the codebase already relies on for the symmetric
outgoing-`msghdr` path (`unwrap_or(c::socklen_t::MAX)` on BSD/Solaris/musl/etc., an identical-width
`len` on Linux where the field is `size_t`) - a buffer whose length can't be represented now has its
*parsed* control length clamped to the largest representable value instead of panicking; any bytes
beyond that are simply not interpreted as ancillary data by `CMSG_FIRSTHDR`/`CMSG_NXTHDR` (safe:
clamping only ever makes the recorded length smaller than the real buffer, never larger, so no
out-of-bounds access is introduced).

**Checked against more than the one triggering input (CONFIRMATION.md section 7)**: verified
against the exact triggering size (`u32::MAX as usize + 1`), the legitimate boundary just below it
(`u32::MAX as usize`, unaffected, already worked before and after), a small ordinary buffer (16
bytes, unaffected), and a size well past the boundary (`3 * u32::MAX as usize`, ~12GB) - all four
produce a clean, non-panicking construction after the fix, 1-5 runs each (see `repro/` and
`output.txt`), all via Miri cross-interpretation for `x86_64-apple-darwin`.

See `patch.diff` (+9/-4 lines across `src/net/send_recv/msg.rs`, `src/backend/libc/net/msghdr.rs`,
`src/backend/linux_raw/net/msghdr.rs`), verified by:
- Building both backends on the host (`cargo build --features "net,fs,std"` for `linux_raw`, `cargo
  build --features "net,fs,std,use-libc"` for the `libc` backend) - clean, no warnings.
- Re-running `repro/` against the patched tree via the same Miri cross-interpretation - the
  formerly-panicking cases no longer panic (see above).
- Project test suite: `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`
  (host `x86_64-unknown-linux-gnu`, libc backend) - **411 passed, 1 failed before the fix; 411
  passed, 1 failed after the fix (identical)**. The one failure (`ioctl::test_int_setter`) is the
  same pre-existing, environment-caused failure already documented in `projects/rustix.md` and in
  candidates 058/079's own `RESULT.md` files (`NotFound` opening `/dev/net/tun`, absent in this
  container) - unrelated to this change, present and identical both before (confirmed via `git
  stash`, running against genuinely pristine code) and after.

## What's not done / open questions

- Not independently confirmed on real BSD/Apple/Solaris/illumos hardware - see "Fully reproducible:
  No" above for the specific, verified reason (no such hardware/SDK/cross-linker in this
  environment).
- Not checked: whether the same "shared code re-implements its own length conversion instead of
  reusing the backend's already-correct helper" pattern recurs anywhere else in `src/net/send_recv/
  msg.rs` or elsewhere in the crate - this session only investigated the one specific site named by
  the candidate description.
- The `far_over` (`3 * u32::MAX`, ~12GB) boundary-value variant allocates real memory for the test;
  confirmed to run quickly and without excessive memory use under Miri in this environment (a few
  seconds, no OOM), but a much larger multiplier was not attempted.
