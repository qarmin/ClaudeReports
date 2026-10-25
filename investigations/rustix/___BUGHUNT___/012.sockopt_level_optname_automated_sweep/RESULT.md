# sockopt_level_optname_automated_sweep

Scripted cross-check of every `(level, optname)` pair passed to `getsockopt`/`setsockopt` in both
backends' `src/backend/{linux_raw,libc}/net/sockopt.rs`, per the backlog note: candidate 003's
confirmed bug (`IPPROTO_IP` used instead of `IPPROTO_IPV6`) went undetected for ~2 years via manual
review alone, so an automated sweep across every call site might catch other silent mismatches of
the same shape.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Custom Python script: regex-extracts every `getsockopt(...)`/`setsockopt(...)` call (including generic `getsockopt::<T>(...)`) from both backend files, parses out the `level`/`optname` arguments (handling nested parens/generics), and flags any pair where the optname's constant-name prefix (`IP_`/`IPV6_`/`TCP_`/`SO_`/`UDP_`/`ICMP6_`/`XDP_`/`NETLINK_`) doesn't match the expected prefix for the given level constant (`SOL_SOCKET`->`SO_`, `IPPROTO_IP`->`IP_`, `IPPROTO_IPV6`->`IPV6_`, `IPPROTO_TCP`->`TCP_`, etc.) | Ran over `src/backend/linux_raw/net/sockopt.rs` (102 calls extracted) and `src/backend/libc/net/sockopt.rs` (107 calls extracted) - 209 call sites total, effectively every `(get\|set)sockopt` call in both backends | Found exactly 4 mismatches, all already candidate 003's confirmed bug: `linux_raw` lines 583/588 and `libc` lines 621/626, all `setsockopt(fd, c::IPPROTO_IP, c::IPV6_MULTICAST_HOPS, ...)` / the matching `getsockopt`. No other mismatch anywhere in either file. |
| Distinct-level census (grouping every call by its `level` argument, per backend) | Same 209 calls | Only 5 distinct `level` values are ever used across both backends: `SOL_SOCKET`, `IPPROTO_IP`, `IPPROTO_IPV6`, `IPPROTO_TCP`, `SOL_XDP` - all covered by the prefix heuristic above. One `libc`-backend call site uses a `level` *variable* (not a literal constant) - manually confirmed this is a generic parameterized helper function (its own `level` parameter forwarded to `setsockopt`/`getsockopt`), not a call site that could itself be individually mismatched. |

## Status

**Not reproducible** (beyond the already-confirmed candidate 003 bug, which this sweep re-found as
expected - a sanity check that the tooling actually works, not a new finding). No other
`(level, optname)` mismatch exists in either backend's `sockopt.rs` as of this checkout.

## Root cause

N/A - no new bug found. Candidate 003's bug was apparently the only instance of this specific
mistake in the file; the sweep's re-discovery of exactly that bug (and nothing else) is itself
evidence the heuristic isn't just failing to fire - it correctly flags the one known-bad pair and
stays silent on the other 205 call sites, which is the expected result for a codebase where this
was a one-off copy-paste slip rather than a systemic issue.

## Introduced by

N/A - not applicable, no new bug confirmed (the one pair this sweep did flag is candidate 003's
already-recorded bug, introduced by 77c4aaea per that candidate's own `RESULT.md`).

## The fix

N/A - no new bug found, nothing to fix.

## What's not done / open questions

- The prefix-matching heuristic is necessarily approximate: it can't catch a mismatch between two
  constants that happen to share the same naming prefix but are still semantically wrong (e.g. two
  different `IP_*` options confused for each other under the same, correct `IPPROTO_IP` level - a
  within-family mix-up, not a cross-family one). Candidate 003's bug happened to be a cross-family
  slip (`IP_*` level with an `IPV6_*` name), which is exactly what this heuristic targets; a
  within-family mistake would need actual numeric-value verification (e.g. compiling a small probe
  that prints every constant's real integer value and diffing against the kernel's own
  `<linux/in.h>`/`<linux/in6.h>`/`<linux/tcp.h>` headers) rather than name-prefix matching.
- Options using a raw numeric level or optname literal (none currently exist in this codebase, per
  the census above - all call sites use named constants) would bypass this heuristic entirely,
  since it operates on identifier text, not resolved values.
- Not extended to non-Linux/non-Windows `sockopt.rs` files, if any exist elsewhere in the crate for
  other Unix targets - only the two backends' Linux-focused files (`linux_raw`, `libc`, the latter
  covering the same Linux socket-option surface via libc bindings) were in scope.
