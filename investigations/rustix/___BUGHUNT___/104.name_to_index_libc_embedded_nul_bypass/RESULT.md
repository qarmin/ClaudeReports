# name_to_index_libc_embedded_nul_bypass

`src/backend/libc/net/netdevice.rs`'s `name_to_index` (the libc backend of
`rustix::net::netdevice::name_to_index`, a public API wrapping `ioctl(SIOCGIFINDEX)`) copies an
interface-name `&str` straight into `ifreq.ifr_name` after only checking `len() >= IFNAMSIZ`. It
never rejects an embedded NUL byte. The sibling `linux_raw` backend
(`src/backend/linux_raw/net/netdevice.rs`) does: `if if_name_bytes.contains(&0) { return
Err(io::Errno::NODEV); }`. Since the kernel's `dev_get_by_name()` matches up to the first NUL (or
`IFNAMSIZ`), a caller passing e.g. `"lo\0evil"` (a valid Rust `&str` - Rust strings may contain
interior NUL bytes) gets `NODEV` from `linux_raw` but silently resolves to the real `lo` interface
on the `libc` backend - a cross-backend behavioral divergence in a documented, identical operation.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + real syscall | Built `rustix` with default features (linux_raw backend) and with `use-libc` (libc backend), both linked against the real public API `rustix::net::netdevice::name_to_index`, called through a real socket fd against the real kernel `SIOCGIFINDEX` ioctl on the real `lo` interface, with input `"lo\0evil"` | Confirmed: linux_raw -> `Err(NODEV)`; libc -> `Ok(1)` (the real `lo` ifindex), reproduced 5/5 runs, release build, deterministic |

A single approach (manual boundary-value construction against the real syscall) was sufficient -
no sanitizer/fuzzing was needed since this is a straightforward missing-validation logic bug with
a trivially constructible trigger input, matching `HEURISTICS.md`'s guidance for this bug class.

## Status

**Confirmed.** Reproduces deterministically, 5/5 runs, release build, both before the fix (libc
backend wrongly returns `Ok(1)`) and the fix verified to correct it (libc backend now returns
`Err(NODEV)`, matching linux_raw).

**Fully reproducible**: **Yes.** `repro/` calls the real public API
(`rustix::net::netdevice::name_to_index`) through a real socket fd, which issues the real
`SIOCGIFINDEX` ioctl to the real kernel against the real `lo` interface - no hand-authored
ifreq/ioctl response is faked. This finding is not about one participant reacting to another's
produced input (no peer/caller/wire data is involved) - it is about which code path a single
real call takes depending on which of rustix's own backends is compiled in - so
`CONFIRMATION.md` section 11's two-participant requirement does not apply here.

## Class

logic (also a latent security/validation-bypass hazard: any caller that uses the `Err(NODEV)`
result on one backend to drive a security decision - e.g. "reject this configured interface name
if it doesn't resolve" - sees that validation silently bypassed on the other backend).

## Root cause

The libc backend's `name_to_index` only validates `if_name_bytes.len() >= IFNAMSIZ`, then copies
every byte (including an embedded NUL) into `ifreq.ifr_name` before calling the ioctl. The kernel's
own name-matching (`dev_get_by_name()` / `netdev_name_node_lookup()`) stops at the first NUL (or
`IFNAMSIZ`), so `"lo\0evil"` is indistinguishable, from the kernel's point of view, from `"lo"`.
The linux_raw backend rejects embedded NULs explicitly before doing the same copy; the libc
backend has no equivalent check, so the two backends disagree on whether `"lo\0evil"` names an
interface at all.

## Introduced by

f546573c "Fix errors with char signedness. (#984)" (2024-01-12). This commit added the
`if_name_bytes.contains(&0) { return Err(io::Errno::NODEV); }` check, but only to the linux_raw
backend's `name_to_index`, as an incidental side effect of fixing an unrelated `c_char`
signedness bug (issue #983) - the libc backend's sibling function, added one day earlier in
e4609840 "Add `name_to_index()` & `index_to_name()` (#972)" (2024-01-11) with no NUL check in
either backend, was never touched by f546573c and has carried the gap ever since.

## The fix

Added the identical check already present in the linux_raw backend to the libc backend's
`name_to_index`, immediately after the existing length check:

```rust
if if_name_bytes.contains(&0) {
    return Err(io::Errno::NODEV);
}
```

This is sufficient because it makes the libc backend reject exactly the same class of input
(any name containing an embedded NUL byte) that the linux_raw backend already rejects, before any
byte is copied into the `ifreq`, with the identical `Errno::NODEV` the linux_raw backend already
returns - no other behavior changes (a plain, NUL-free name within `IFNAMSIZ` is untouched, as
`repro/` confirms by also checking plain `"lo"` still resolves correctly on both backends,
before and after the fix).

Before/after of `repro/` (release build, 5/5 runs each):
- Before: `libc` backend -> `name_to_index("lo\0evil")` = `Ok(1)` (wrongly resolves to the real
  `lo` interface).
- After: `libc` backend -> `name_to_index("lo\0evil")` = `Err(Os { code: 19, ..., "No such
  device" })`, matching `linux_raw`.

- Project test suite: `cargo test --workspace --features "std,all-apis,use-libc"` -
  **410 passed, 2 failed, 5 ignored, both before and after** (identical failures, unrelated to this
  change):
  `io::ioctl::test_int_setter` (fails in this sandbox because `/dev/net/tun` isn't present - a
  pre-existing, environment-only failure already noted in this workspace's `README.md`) and
  `process::rlimit::test_setrlimit` (pre-existing, unrelated to networking). No new failures and
  no newly-passing test introduced by the patch; `net::main`'s own `test_name_to_index` (which
  only exercises the plain `"lo"` case, not an embedded-NUL name) passed identically before and
  after.

## What's not done / open questions

- `index_to_name` (the reverse direction, `SIOCGIFNAME`) was not checked for an analogous
  divergence - it reads a kernel-supplied name back out rather than validating caller input, so
  the same bug class (accepting untrusted input the other backend would reject) does not obviously
  apply, but a dedicated check was not performed this candidate.
- No other netdevice-style ioctl wrapper in this crate (if any exists outside `net/netdevice.rs`)
  was swept for the same missing-NUL-check pattern; this candidate only covers `name_to_index`
  across the two backends named in the suspect.
- Not reported upstream as a GitHub issue/PR this session (per this skill's "no severity ranking,
  facts only" scope) - left for the maintainer/reader of this `RESULT.md` to decide whether to
  file it.
