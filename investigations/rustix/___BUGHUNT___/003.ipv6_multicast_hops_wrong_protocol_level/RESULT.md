# ipv6_multicast_hops_wrong_protocol_level

`(set_)ipv6_multicast_hops` use `IPPROTO_IP` instead of `IPPROTO_IPV6` as the sockopt level in
both backends, so they silently mutate/read an unrelated IPv4-namespace option (`IP_PASSSEC`)
instead of the real IPv6 multicast hop limit. Sourced from GitHub issue #1660, cross-checked
against the still-open, unmerged fix PR #1662.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/backend/libc/net/sockopt.rs` and `src/backend/linux_raw/net/sockopt.rs`, `(set_)ipv6_multicast_hops` | Confirmed both backends use `IPPROTO_IP` where `IPPROTO_IPV6` is required, on current HEAD (line numbers re-verified, not just trusted from the older issue/PR references) |
| Real socket repro, both backends (`linux_raw` default, `libc` via `use-libc` feature) | `AF_INET6`/`SOCK_DGRAM` socket, rustix's real public `sockopt::set_ipv6_multicast_hops`/`ipv6_multicast_hops`, cross-checked via independent raw `libc::getsockopt`/`setsockopt` calls at the correct level and at the colliding `IPPROTO_IP` option 18 (`IP_PASSSEC`) | 3/3 runs per backend (6/6 total) show the real kernel-held `IPV6_MULTICAST_HOPS` never changes, while the unrelated `IP_PASSSEC` option gets silently flipped instead |
| `git log -S`/`git blame` | on the `set_ipv6_multicast_hops`/`ipv6_multicast_hops` functions | Pinned the introducing commit |
| Swept every other `IPV6_*` sockopt in both files for the same mistake | manual review | No other sibling option shares this bug - isolated to these 2 functions x 2 backends |
| `cargo test --features=all-apis --workspace --no-fail-fast -- --test-threads=1` before/after fix | | Uncovered that the existing test baked in the buggy behavior as its expected value - see "The fix" |

## Status

**Confirmed**. 3/3 identical runs on the `linux_raw` backend and 3/3 identical runs on the `libc`
backend (6/6 total), deterministic.

**Fully reproducible**: Yes. A real `AF_INET6`/`SOCK_DGRAM` socket, calling rustix's actual public
`rustix::net::sockopt::set_ipv6_multicast_hops`/`ipv6_multicast_hops`, cross-checked via an
independent raw `libc::getsockopt`/`setsockopt` call (not rustix's own code) at the correct
`IPPROTO_IPV6` level, run against both of rustix's own backends.

## Class

`security` - a wrong sockopt "level" argument silently mutates an unrelated, security-relevant
kernel socket option (`IP_PASSSEC`, which controls receiving SELinux security-context ancillary
messages) instead of the multicast hop limit the caller actually asked to set, while leaving the
real hop limit unchanged. A caller relying on `set_ipv6_multicast_hops` to scope multicast traffic
gets silent no-op instead of the intended limit.

## Root cause

In both `src/backend/libc/net/sockopt.rs` (lines 620-627) and
`src/backend/linux_raw/net/sockopt.rs` (lines 582-589), `set_ipv6_multicast_hops`/
`ipv6_multicast_hops` call `setsockopt`/`getsockopt` with level `c::IPPROTO_IP` instead of
`c::IPPROTO_IPV6`. Option numbers are only unique within a protocol level. On Linux,
`IPV6_MULTICAST_HOPS == 18`, and at the `IPPROTO_IP` level, option number `18` is `IP_PASSSEC`
(confirmed via `/usr/include/linux/in.h`: `IP_IPSEC_POLICY=16, IP_XFRM_POLICY=17,
IP_PASSSEC=18`). `IP_PASSSEC` is a boolean flag that also happens to accept a 4-byte int, so the
call doesn't error - it just silently flips `IP_PASSSEC` to whatever nonzero hop value was passed,
while the real `IPV6_MULTICAST_HOPS` is left untouched. The getter has the mirrored bug: it reads
back `IP_PASSSEC`'s value while pretending it's the hop limit.

No other `IPV6_*`-named sockopt in either backend file shares this mistake (checked
`ipv6_multicast_if`, `ipv6_multicast_loop`, `ipv6_unicast_hops`, `ipv6_v6only`, `ipv6_mtu`,
`ipv6_mtu_discover`, `ipv6_add/drop_membership`, `ipv6_recvtclass`, `ipv6_freebind`,
`ipv6_tclass`) - isolated to these two functions in both backends.

## Introduced by

77c4aaea1768866a737c55b1e0b06f9ae7076c32 "Reorganize sockopts code and add several more socket
options (#855)" (Dan Gohman, 2023-09-28). Both backends' `ipv6_multicast_hops`/
`set_ipv6_multicast_hops` were added in this commit already using the wrong level, unchanged
since (still present in 1.1.5 / HEAD).

## The fix

See `patch.diff`: change the level constant from `c::IPPROTO_IP` to `c::IPPROTO_IPV6` in all 4
call sites (getter + setter, both backends).

An important side-discovery: the pre-existing test `tests/net/sockopt.rs::test_sockopts_ipv6`
baked in the *buggy* behavior - it asserted `ipv6_multicast_hops() == 0` (the coincidental default
value of `IP_PASSSEC`, which defaults to disabled), with a comment claiming NetBSD anomalously
returned `1` there. After fixing the level to `IPPROTO_IPV6`, the correct default (`1`, per
RFC 3493 / `ipv6(7)`) is returned on all platforms - so the test needed updating too, which also
removes the now-explained NetBSD "anomaly" special-case (NetBSD's behavior was the *correct* one
all along; every other platform was wrong). The patch also adds an actual setter round-trip check
(tolerating `ENOPROTOOPT`, since Linux's kernel genuinely rejects setting multicast options on
`SOCK_STREAM` sockets - confirmed independently as unrelated to this bug).

- Project test suite: `cargo test --features=all-apis --workspace --no-fail-fast -- --test-threads=1`.
  Before fix (source only, unmodified test): `net` binary 83 passed / 1 "failed" in the sense that
  it passed while encoding wrong expected behavior (not a literal runner failure - see output.txt);
  `io` binary 16 passed / 1 failed (`ioctl::test_int_setter`, pre-existing, needs `/dev/net/tun`,
  unrelated). After fix (source + corrected test): `net` 84 passed / 0 failed; `io` unchanged (same
  pre-existing unrelated failure). No regressions.

## What's not done / open questions

- No other sockopt level mismatch was found in this file during this session's review, but the
  fact that this exact bug (copy-pasted `IPPROTO_IP` for an IPv6 option) went undetected for ~2
  years suggests a systematic automated cross-check (every `(level, optname)` pair in both backend
  files against each platform's real numeric option-namespace tables) could be worth a dedicated
  future candidate/tool, rather than manual review alone - added to `IDEAS.md`.
- This session's independent-verification repro used `IPPROTO_IP` option `18` (`IP_PASSSEC`) as
  the "what actually got hit" oracle - this is Linux-specific; the same collision on other
  platforms (BSD/macOS numeric option tables) was not checked, since the repro only ran on this
  session's Linux x86_64 host.
