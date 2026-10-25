# align_for_cmsghdr_oob_slice_panic

`align_for_cmsghdr` in `src/net/send_recv/msg.rs` (used internally by the public
`SendAncillaryBuffer::new` and `RecvAncillaryBuffer::new`) rounds a caller-supplied
`&mut [MaybeUninit<u8>]` buffer's start address up to `align_of::<cmsghdr>()` and then slices the
buffer by that many bytes of padding. It guards the empty-buffer case but not the case of a
short, non-empty buffer whose address needs more padding than the buffer is long - flagged by
code reading per candidate 083's lead.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction via the real public API | `#[repr(align(32))]` backing storage so every `addr % 32` offset 0..32 is reachable deterministically; a 2-byte `&mut [MaybeUninit<u8>]` sub-slice at each offset passed straight into `rustix::net::SendAncillaryBuffer::new` (release build, `cargo run --release`) | Panicked ("range start index N out of range for slice of length 2") at 21 of the 32 offsets, every single run |
| Repeat-run check | 3 independent `./target/release/repro` invocations, same binary, same backing storage layout | Identical outcome every time (same 21 offsets, same panic messages modulo PID) - deterministic, not a flaky one-off |
| Fix + re-run | Same repro against the patched tree (`padding.min(len)`) | 0 of 32 offsets panicked |
| Project test suite before/after | `cargo test --features=all-apis --workspace --no-fail-fast` | Identical pass/fail/ignored counts before and after (see below) |

No miri/ASAN/fuzzing needed here - this is a pure boundary-value arithmetic bug (padding
computation vs. buffer length) reachable directly by constructing the right-sized/right-aligned
input, which the manual approach demonstrates deterministically and repeatably; escalating to a
heavier tool would not add signal beyond what's already a 3/3 deterministic repeat.

## Status

**Confirmed.** 3/3 runs against the release-built unpatched tree panic identically (same 21 of 32
tested alignment offsets every time, only the thread's PID differing in the panic message). This
is address-dependent in the sense the candidate note described (a real caller's buffer might
happen to land at a "safe" offset or not), but the repro forces every possible offset
deterministically via a 32-byte-aligned backing allocation, so it does not rely on chance.

**Fully reproducible**: Yes. `repro/` calls the real, unmodified public API
(`rustix::net::SendAncillaryBuffer::new`) with a real `&mut [MaybeUninit<u8>]` buffer backed by
real stack memory at a real (forced, but genuine) address - nothing about `align_for_cmsghdr`
itself is reimplemented or faked. This finding is not about one participant reacting to another
participant's input (no peer/wire/upstream-caller is involved) - it is a single-process
input-construction bug in a buffer-alignment helper, so CONFIRMATION.md section 11's two-participant
bar does not apply here.

## Class

crash

## Root cause

```rust
let align = align_of::<c::cmsghdr>();      // 8 on x86_64 Linux (libc and linux_raw backends)
let addr = buffer.as_ptr() as usize;
let adjusted = (addr + (align - 1)) & align.wrapping_neg();
&mut buffer[adjusted - addr..]
```

`adjusted - addr` is the number of padding bytes needed to reach the next `align`-byte boundary -
anywhere from 0 to `align - 1` (e.g. up to 7 when `align == 8`). The code never checks this
padding against `buffer.len()`. If the caller passes a buffer shorter than the padding needed for
its own runtime address (e.g. a 2-byte buffer whose address is `8k + 3`, needing 5 bytes of
padding), `&mut buffer[5..]` on a 2-byte slice panics with "range start index out of range" - a
plain Rust slice-bounds panic, reachable with a perfectly ordinary, safe caller-supplied buffer
(no `unsafe`, no malformed kernel data involved). The existing `if buffer.is_empty()` guard only
handles the *zero-length* buffer case; it does nothing for a short-but-nonzero buffer whose
address happens to need more padding than it has bytes.

## Introduced by

ff9c7fbe "Make `cmsg_space!` usable in const contexts. (#889)" (2023-10-22) - this commit is what
introduced `align_for_cmsghdr` itself (verified via `git log -S "align_for_cmsghdr"` and reading
the commit's diff directly: the function and both of its two call sites in `SendAncillaryBuffer::new`/
`RecvAncillaryBuffer::new` are added here, unguarded, exactly in the form that panics). A later
commit, 31dfad15 "Fixes for empty and multiple-entry control message buffers. (#915)", added the
`if buffer.is_empty()` early-return but did not add a check for the short-non-empty case, so the
bug has persisted from ff9c7fbe through the current `HEAD` (287214b8).

## The fix

Compute the padding once, then clamp it to the buffer's actual length before indexing, so a
buffer too short to hold any padding degrades to an empty slice (same as the already-handled
fully-empty-buffer case) instead of panicking:

```rust
let padding = adjusted - addr;
let len = buffer.len();
&mut buffer[padding.min(len)..]
```

Verified sufficient across the full range of possible alignment offsets, not just the one that
happened to trigger it first: re-running `repro/` (which tries all 32 possible `addr % 32`
offsets against a 2-byte buffer) against the patched tree produces 0 panics across all 32 offsets,
versus 21/32 before the fix (see `output.txt`).

- Project test suite: `cargo test --features=all-apis --workspace --no-fail-fast`
  - Before: 413 passed, 1 failed, 5 ignored
  - After: 413 passed, 1 failed, 5 ignored
  - The 1 failure (`io::ioctl::test_int_setter`, `ENOENT` opening a tty device) is identical
    before and after, and is a pre-existing environment limitation of this sandboxed container
    (no suitable tty device present) unrelated to this change - not a regression.

## What's not done / open questions

- Not checked whether any other code in this crate does similar unchecked
  "round address up, then slice by the padding" arithmetic outside `src/net/send_recv/msg.rs` - a
  repo-wide grep for `wrapping_neg()` combined with slice indexing would be the next step if this
  class of bug is suspected elsewhere.
- The fix clamps to an empty resulting slice when the buffer is too short to hold even one
  aligned byte; callers that then try to push an ancillary message into a `SendAncillaryBuffer`
  backed by that empty slice will simply find `push` returns `false` (no space), which is the
  same behavior already relied upon for the pre-existing empty-buffer case - no new behavior class
  introduced by this fix.
