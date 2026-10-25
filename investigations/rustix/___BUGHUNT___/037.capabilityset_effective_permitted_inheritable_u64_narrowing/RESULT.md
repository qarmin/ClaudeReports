# capabilityset_effective_permitted_inheritable_u64_narrowing

`src/thread/libcap.rs` represents `CapabilitySet` as a `u64`-backed `bitflags!` type, but the
kernel's `_LINUX_CAPABILITY_VERSION_3` ABI (`__user_cap_data_struct`) only has 32-bit
`effective`/`permitted`/`inheritable` fields, so `capget`/`capset` split each 64-bit value across a
2-element `[__user_cap_data_struct; 2]` array using `bits() as u32` (low half) and
`(bits() >> u32::BITS) as u32` (high half). The candidate suspected this is an "unchecked narrowing
cast near a type's capacity" that would silently drop bits if a future `CAP_*` constant were ever
defined at bit position >= 64.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: standalone Rust program reimplementing the exact low/high split-and-reconstruct expressions used in `capget`/`capset`, checked against representative boundary `u64` values (`0`, `1`, `u32::MAX`, `u32::MAX+1`, `1<<32`, `1<<63`, `u64::MAX`, a mixed-bit-pattern value) plus every individual bit position `0..64` | `rustc -O`, release-opt build, single run, <1s | No data loss for any value: every boundary case round-trips exactly, and the bit-position loop (testing each of the 64 individual bit positions) reported zero lost bits - see `output.txt` |
| Real end-to-end round-trip through rustix's own public API (`rustix::thread::capabilities`/`set_capabilities`), standalone crate depending on this worktree's rustix with `std,thread` features, issuing a real `capget` syscall then a real `capset` syscall with the retrieved value written straight back | `cargo run`, debug build, this container's actual (capability-less) process | Compiles and runs correctly, round-trip is value-identical (`CapabilitySet(0x0)` both times, since this container holds no capabilities) - confirms the real syscall code path works, but can't exercise bit positions beyond `CAP_LAST_CAP` since the kernel itself rejects unknown capability bits with `EINVAL` regardless of which 32-bit half they'd land in |

Two approaches were enough here, not the full soft cap of 3: approach 1 is not a sample of
"clean negatives" that might still be hiding a rare case further out - it is a closed-form proof.
The low/high split (`v as u32`, `(v >> 32) as u32`) and reconstruction
(`u64::from(low) | (u64::from(high) << 32)`) is a bitwise-linear transform where each output bit
depends on exactly one input bit and no others; testing every one of the 64 individual bit
positions independently (in addition to several multi-bit boundary values) is therefore an
exhaustive proof that the identity holds for all `2^64` possible `u64` values, not just the ones
tried - there is no room left for a 3rd escalating tool to turn up a case the first one missed.

## Status

**Not reproducible.** The suspected narrowing does not occur: a 64-bit value split into a low
32-bit half (`v as u32`) and a high 32-bit half (`(v >> 32) as u32`) and reconstructed as
`u64::from(low) | (u64::from(high) << 32)` is lossless for every possible `u64` value, proven
exhaustively over all 64 bit positions (see `output.txt`). There is no narrower integer type
involved anywhere in this path that could actually drop a bit - `CapabilitySet`'s own backing
storage is `u64` (exactly 64 bits), and the kernel's own ABI already represents exactly 64 bits
per field via its 2-element `__user_cap_data_struct` array (`VFS_CAP_U32 == 2`, hardwired to
`_LINUX_CAPABILITY_VERSION_3`). The split captures all 64 of those bits; it is not "approaching a
capacity limit and anticipating future growth" - the kernel's 2x32 layout and the Rust side's
`u64` are exactly the same width, by construction, not by coincidence.

The candidate's own premise ("the kernel's own `cap_user_data_t` struct already uses a 2-element
array anticipating >32-bit growth") does not hold up: the 2-element array is not forward-looking
headroom beyond the current ABI, it *is* the current, final ABI for `_LINUX_CAPABILITY_VERSION_3`
(introduced for exactly this reason: to carry 64 bits across the pre-existing 32-bit-only
`__user_cap_data_struct` fields). A `CAP_*` bit at position >= 64 would require an entirely new
capability-header version (there is no `_LINUX_CAPABILITY_VERSION_4`/wider array today, nor any
upstream kernel proposal for one as of this check), which would need its own new wire struct
layout and its own new Rust-side handling - not a silent truncation of the existing `u64`/2x`u32`
path, since `CapabilitySet` itself (being `u64`-backed) could not represent such a bit at all; the
`1 << linux_raw_sys::general::CAP_WHATEVER` bitflag definitions would hit a shift-amount panic (in
debug builds) or UB (in release, if ever unchecked) long before `capget`/`capset`'s own `as u32`
casts were reached.

**Fully reproducible**: N/A (Not reproducible - no reproduction to characterize).

## Class

N/A (Not reproducible).

## Root cause

Not a bug: the 32/32 bit split used by `capget`/`capset` is a lossless representation of the full
64-bit `u64` range by construction (see "Status" above for the closed-form argument), and it
matches the real width of both sides of the kernel ABI (`__user_cap_data_struct`'s 2-element array
= 64 bits total) and the Rust side (`CapabilitySet` = `u64` = 64 bits). There is no code path
where a bit is dropped; the candidate's "unchecked narrowing cast" framing conflates "casts to a
narrower integer type" (true, syntactically) with "loses information" (false, since the two casts
together cover disjoint, exhaustive halves of the source value).

## Introduced by

N/A (not a bug - no fix to attribute). For reference, this exact split/reconstruct logic has been
present since the `capget`/`capset` syscalls were first added: d9429b50 "Add cap{get,set} syscalls
(#514)".

## The fix

No fix needed or applied - nothing to fix. No `patch.diff` in this folder.

- Project test suite: N/A, not run (no code change was made; nothing to verify before/after).

## What's not done / open questions

- This only examined the `linux_raw` backend's `capget`/`capset` implementation in
  `src/thread/libcap.rs`; the project also has a `libc`-backend variant of the same calls
  (`src/backend/libc/thread/syscalls.rs` or similar) which was not separately re-derived here,
  since it necessarily goes through the identical kernel ABI (`__user_cap_data_struct`) and would
  be subject to the exact same closed-form argument - not expected to differ, but not independently
  re-checked line-by-line this session.
- If Linux ever does introduce a wider capability ABI (a hypothetical `_LINUX_CAPABILITY_VERSION_4`
  carrying more than 64 bits per field), this code would need new handling regardless - that would
  be a new feature to add, not a latent bug in the current code to fix now.
