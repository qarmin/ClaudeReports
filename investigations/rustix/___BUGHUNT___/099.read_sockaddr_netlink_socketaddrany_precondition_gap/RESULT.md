# read_sockaddr_netlink_socketaddrany_precondition_gap

`read_sockaddr_netlink` (`src/backend/linux_raw/net/read_sockaddr.rs:152` and
`src/backend/libc/net/read_sockaddr.rs:261`) `assert!`s `addr.addr_len() >= size_of::<sockaddr_nl>()`
(12 bytes) before dereferencing. The public `unsafe fn SocketAddrAny::new`/`read`
(`src/net/socket_addr_any.rs:99-101,119-120`) that produce every `SocketAddrAny` only enforce
`len >= size_of::<sockaddr_header>()` (2 bytes - just `sa_family`). A value that satisfies the
constructor's own documented and runtime-checked contract (family tag `NETLINK`, `len` anywhere in
`[2, 11]`) therefore panics the safe, infallible-looking `TryFrom<SocketAddrAny> for
SocketAddrNetlink` instead of returning `Err`. This candidate follows up on 020's open question
about this exact "sibling assert family" (`read_sockaddr_v4`/`_v6`/`_xdp`/`_netlink`), narrowed to
the netlink constructor-vs-consumer precondition gap specifically.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct, single-process call through rustix's own real public API (`repro/src/main.rs`): build the smallest `SocketAddrStorage` that satisfies `SocketAddrAny::new`'s own documented+enforced precondition (2-byte `sa_family` header only, tagged `AF_NETLINK`), call the real public `unsafe fn SocketAddrAny::new(storage, 2)`, then hand the result to the safe `SocketAddrNetlink::try_from` a normal (non-`unsafe`) caller would use | `cargo run --release`, both backends (default `linux_raw` and `--features use-libc`), release build | **Reproduced on both backends**: `SocketAddrAny::new(storage, 2)` succeeds cleanly (as its own contract promises), but `SocketAddrNetlink::try_from(any)` panics with `assertion failed: addr.addr_len() as usize >= size_of::<c::sockaddr_nl>()` instead of returning an `Err` |
| Repeat-run (same build, same input) | 3 runs each on `linux_raw` and `libc` backends (6 total) | 6/6 deterministic panics, identical assertion message and panic location each time |

No second/third escalation tool was needed - the first approach reproduced deterministically and
the mechanism is fully explained by reading the two preconditions side by side (not a
timing/layout-sensitive bug class that would benefit from miri/ASAN/fuzzing).

## Status

**Confirmed.** Reproduces 6/6 runs (3 on the `linux_raw` backend, 3 on the `libc` backend),
release build, deterministic (no fixed seed, no flakiness - the panic is a straightforward
boundary-value assertion failure, not a layout/timing-dependent one).

**Fully reproducible**: **Yes**. This is a single-participant finding, not a two-peer one: the
"input" in question is a `SocketAddrAny` built by directly calling rustix's own real, documented,
public `unsafe fn SocketAddrAny::new` with a storage buffer/length that satisfies that function's
own stated and runtime-enforced contract to the letter. There is no second participant (no peer, no
wire, no upstream caller) being stood in for here - the external caller of this public constructor
*is* the genuine external input this finding is about, the same way CONFIRMATION.md section 11
treats "a malformed file handed to a parser's real public `open_file` entry point" as a complete,
non-faked repro with no second participant to impersonate. `repro/` calls the real, unmodified
public API end to end (`SocketAddrAny::new` -> `SocketAddrNetlink::try_from`), nothing hand-waved or
internal-only.

## Class

logic (an API-contract mismatch that turns a safe, `Result`-returning conversion into a panic for
input that is valid per the producing constructor's own documented and enforced precondition). Not
classified as memory-safety/UB: the `assert!` fires and panics *before* the `unsafe` dereference of
`c::sockaddr_nl` is reached, so no out-of-bounds read or uninitialized-memory read actually occurs
on this path - the consequence is "clean panic for a non-exceptional, contract-satisfying value",
not memory unsafety.

## Root cause

`SocketAddrAny` is one generic container shared by every address family (`V4`/`V6`/`UNIX`/`XDP`/
`NETLINK`/unrecognized), so its own public constructors (`new`/`read`) can only enforce the
*smallest* universally-valid lower bound across all families - `size_of::<sockaddr_header>()`, i.e.
just the 2-byte `sa_family` tag. Each per-family decoder (`read_sockaddr_netlink` and its siblings
`read_sockaddr_v4`/`_v6`/`_xdp`) independently knows a *larger*, family-specific minimum length is
actually required to interpret the bytes as that family's struct, and enforces it with a hard
`assert!` rather than returning an `Err`. Nothing connects the two: a caller who satisfies the
constructor's own contract (2 bytes, correct family tag) is never told, by that constructor, that a
specific downstream per-family decoder expects more - the mismatch is only discovered at the
`assert!` inside the decoder, as a panic rather than a recoverable error. Every per-family decoder
already returns `Result<_, Errno>` precisely to handle "this isn't valid for this family" - a
too-short length for the tagged family is exactly that case, and should flow through the same
`Result` channel as a family mismatch does, not bypass it via a panic.

## Introduced by

5654c117 "Add trait to reduce duplicated code between socket address types, implement netlink
addresses (#1004)" (2025-02-10). This single commit introduced both halves of the mismatch at once:
it replaced the old enum-based `SocketAddrAny` with today's generic safe wrapper (`new`'s own
assert only requiring `len >= size_of::<sockaddr_header>()`, confirmed via
`git show 5654c117 -- src/net/socket_addr_any.rs`) and, in the same diff, added
`read_sockaddr_netlink` with its own `assert!(addr.len() >= size_of::<sockaddr_nl>())`
(confirmed via `git show 5654c117 -- src/backend/linux_raw/net/read_sockaddr.rs`). The two
preconditions have never agreed since. (The exact assert lines were later touched by
19037c36b/8a027c3f4 on 2025-02-14, both purely renames/clippy formatting - `git blame` on those
lines points there, but the actual gap - a generic 2-byte-minimum constructor paired with a
12-byte-minimum consumer - already existed as of 5654c117.)

## The fix

Changed the `assert!` in both `read_sockaddr_netlink` implementations
(`src/backend/linux_raw/net/read_sockaddr.rs`, `src/backend/libc/net/read_sockaddr.rs`) into an
explicit length check that returns `Err(Errno::INVAL)` instead of panicking, mirroring how a family
mismatch two lines above already returns `Err(Errno::AFNOSUPPORT)` rather than asserting:

```rust
if (addr.addr_len() as usize) < size_of::<c::sockaddr_nl>() {
    return Err(Errno::INVAL);
}
```

This is sufficient for every input in the previously-panicking range (`len` from 2 up to 11
inclusive, the entire gap between the two preconditions), not just the specific `len = 2` value the
repro happens to use - verified by rerunning `repro/` (which constructs exactly `len = 2`, the
narrowest/most adversarial case in that range) against the patched tree, 3/3 runs, clean `Err`
instead of a panic, on both backends.

- Project test suite (patched worktree, `cargo test --features=all-apis --workspace --no-fail-fast`):
  **413 passed, 1 failed, both before and after the patch** - the one failure
  (`ioctl::test_int_setter`, `ENOENT` opening a device node) is a pre-existing container/environment
  issue unrelated to this change (confirmed identical on the unpatched tree via `git stash`/
  `git stash pop` around the same test run). No new failures introduced by the fix.

## What's not done / open questions

- The sibling decoders flagged by candidate 020 (`read_sockaddr_v4`, `read_sockaddr_v6`,
  `read_sockaddr_xdp`) have the exact same shape of `assert!(addr.addr_len() >= size_of::<T>())`
  guarding a per-family minimum larger than the constructor's generic 2-byte floor, and were not
  independently fixed or given their own dedicated repro this session - only the netlink one named
  in this candidate. A future candidate could apply the identical direct-construction repro and fix
  to each of them.
- Not tested on a non-Linux target (this whole function is `#[cfg(linux_kernel)]`-gated so the
  scope is inherently Linux/Android only; both backends available on this Linux container were
  covered).
- The `Errno::INVAL` choice mirrors the existing convention in these same files (e.g.
  `read_sockaddr_unix`'s own short-length check), but is a judgment call, not something mandated by
  a spec - a maintainer could reasonably prefer a different errno.
