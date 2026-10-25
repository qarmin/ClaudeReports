# xdp_statistics_v1_fallback_wrong_field

`xdp_statistics()` in both backends (`src/backend/linux_raw/net/sockopt.rs:1044-1083` and
`src/backend/libc/net/sockopt.rs:1247-1286`) handles the pre-Linux-5.9 `xdp_statistics_v1`
fallback (triggered when the kernel's `getsockopt(SOL_XDP, XDP_STATISTICS)` only knows the smaller,
3-field struct). Candidate description flagged a suspected copy-paste bug: the fallback branch
assigns `xdp_statistics.rx_dropped` into both the `rx_invalid_descs` and `tx_invalid_descs` output
fields, instead of the struct's own `rx_invalid_descs`/`tx_invalid_descs` fields - contrasted with
the adjacent, correctly-shuffled `xdp_mmap_offsets` v1 fallback in the same files.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Source + struct-layout reading | Read both backends' `xdp_statistics()` functions; cross-checked `linux-raw-sys` 0.12.1's real `xdp_statistics`/`xdp_statistics_v1` struct definitions (all target arches, identical layout) and the real kernel UAPI header (`/usr/include/linux/if_xdp.h`) | Confirmed: both `xdp_statistics_v1` and the first 3 fields of `xdp_statistics` are, in order, `rx_dropped, rx_invalid_descs, tx_invalid_descs` (all `u64`) - the fallback branch's `MaybeUninit::<xdp_statistics>::zeroed()` + a v1-sized kernel write means `xdp_statistics.rx_invalid_descs`/`.tx_invalid_descs` already hold the real, correct values at that point; the code discards them and substitutes `rx_dropped` instead. Plain copy-paste bug, not a struct-layout misunderstanding. |
| Real-environment feasibility check for a real `AF_XDP` + old-kernel repro | `socket(AF_XDP, SOCK_RAW, 0)` via a small C program; `/proc/self/status` capability-bounding-set inspection; `sudo -n` | `socket()` fails with `EPERM` - `CapBnd` (`00000000000000cb`) excludes `cap_net_raw` (bit 13) entirely, so this is not fixable by privilege escalation inside this container; `sudo` additionally refuses to run at all (`no_new_privs` is set). Independently, this container's kernel is `7.0.0-34-generic` (2026), far newer than the Linux 5.9 cutoff (kernel commit 77cd0d7b3f25, which added `rx_ring_full`/`rx_fill_ring_empty_descs`/`tx_ring_empty_descs`) - rustix's own doc comment on `XdpStatistics` independently states "kernel version (<5.9)" for the exact same cutoff. Two independent, compounding reasons a real end-to-end repro is infeasible here. |
| Manual boundary-value construction (the project's own public `XdpStatistics` type + a verbatim transcription of the real branch body, synthetic input bytes) | `repro/` - see below | Reproduces the bug 3/3 runs (deterministic - pure computation, no flakiness expected or observed); fix verified 3/3 runs |

## Status

**Confirmed**. Reproduces deterministically, 3/3 runs (pure computation over caller-supplied bytes,
no source of nondeterminism), `cargo run` in `repro/`.

**Fully reproducible**: **No** - justified per `CONFIRMATION.md` section 11's documented exception.
The bug is about how rustix (one participant) interprets data written by the kernel's
`getsockopt(2)` implementation (a second, independent participant) - but driving that second
participant for real requires *both* (a) a genuine `AF_XDP` socket, which this sandboxed container
cannot create under any privilege level (`cap_net_raw` is excluded from the capability *bounding
set*, confirmed via `/proc/self/status`, and `sudo`/root escalation is itself blocked by
`no_new_privs`), *and* (b) a kernel built before Linux 5.9 (commit 77cd0d7b3f25) - this container's
kernel (`7.0.0-34-generic`) is drastically newer and will always report the full struct. Neither
constraint is something this session's environment can route around (not a time-budget shortcut -
both are hard environmental walls), matching the "needs hardware"/unavailable-specific-version
class of exception the skill documents. `repro/` instead feeds synthetic bytes - laid out exactly
as a genuine pre-5.9 kernel's `copy_to_sockptr(optval, &stats, sizeof(struct xdp_statistics_v1))`
would deposit them (3 sequential `u64`s, rest zeroed, matching `MaybeUninit::<xdp_statistics>::
zeroed()`'s own documented invariant in the real source) - through a function that is a verbatim,
cited-by-line transcription of the real (pre-patch/post-patch) branch body, using the real, public
`rustix::net::xdp::XdpStatistics` type (pinned to the exact published release, `rustix = "=1.1.5"`,
cut at this candidate's own `base_commit`).

## Class

logic

## Root cause

Both backends build a zeroed `MaybeUninit<xdp_statistics>` (the full, 6-field struct), ask the
kernel for the full size, then call `getsockopt_raw`. A kernel that predates the 3 extra fields
(Linux <5.9) can only fill in the leading 3 `u64`s and reports back `optlen ==
size_of::<xdp_statistics_v1>()` (24 bytes); the rest of the buffer stays at the zero the
`MaybeUninit::zeroed()` call put there. Because `xdp_statistics_v1`'s 3 fields are, in order,
identical to the first 3 fields of the full `xdp_statistics` struct, `value.assume_init()` in the
`optlen == size_of::<xdp_statistics_v1>()` branch already yields a struct whose
`.rx_invalid_descs`/`.tx_invalid_descs` fields hold the real kernel-reported values - no cast or
reinterpretation is needed to recover them correctly. The code nonetheless constructs the returned
`XdpStatistics` with:

```rust
rx_dropped: xdp_statistics.rx_dropped,
rx_invalid_descs: xdp_statistics.rx_dropped,   // should be .rx_invalid_descs
tx_invalid_descs: xdp_statistics.rx_dropped,   // should be .tx_invalid_descs
```

silently substituting the drop counter for both of the other two counters - a plain copy-paste
mistake (the adjacent `xdp_mmap_offsets` v1 fallback in the same files shuffles fields correctly,
making this look like an isolated slip rather than a systemic misunderstanding). Both backends
(`linux_raw` and `libc`) contain byte-for-byte the same mistake.

## Introduced by

5648e33e "Add AF_XDP syscalls, constants and types (#946)" (2024-01-11). `git log -S
"rx_invalid_descs: xdp_statistics.rx_dropped"` on `src/backend/linux_raw/net/sockopt.rs` returns
only this one commit - the bug has been present, unchanged, since AF_XDP support was first added,
confirming the candidate description's claim.

## The fix

`patch.diff`: in both `src/backend/linux_raw/net/sockopt.rs` and `src/backend/libc/net/sockopt.rs`,
inside `xdp_statistics()`'s `optlen == size_of::<xdp_statistics_v1>()` branch, change:

```rust
rx_invalid_descs: xdp_statistics.rx_dropped,
tx_invalid_descs: xdp_statistics.rx_dropped,
```

to:

```rust
rx_invalid_descs: xdp_statistics.rx_invalid_descs,
tx_invalid_descs: xdp_statistics.tx_invalid_descs,
```

Verified against 3 distinct synthetic values (`rx_dropped=111, rx_invalid_descs=222,
tx_invalid_descs=333`, chosen to be individually recognizable so a field swap/mixup would be
visually obvious, not just "any nonzero value") - not just the single input shape originally
suspected - per `CONFIRMATION.md` section 7: `repro/`'s default build (pre-patch) reports
`rx_invalid_descs=111, tx_invalid_descs=111` (wrong, both equal to `rx_dropped`); the
`--features fixed` build (post-patch) reports `rx_invalid_descs=222, tx_invalid_descs=333`
(correct, matching the synthetic kernel-supplied values). Both re-run 3/3 times with identical,
deterministic results.

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc"` + `cargo test
  --doc --features "std,all-apis,use-libc"`), applied directly to the main worktree, then reverted
  (`git checkout --`) before writing this candidate up:
  - Before: 411 passed / 1 failed / 5 ignored (lib + integration tests) + 31 passed / 1 ignored
    (doctests). The 1 failure is `ioctl::test_int_setter`, a pre-existing, environment-only failure
    (`/dev/net/tun` doesn't exist in this sandbox) unrelated to this candidate - see
    `projects/rustix.md`'s own dated note on this.
  - After: identical - 411 passed / 1 failed / 5 ignored + 31 passed / 1 ignored. Same single
    pre-existing failure, no new failures, no regressions.

## What's not done / open questions

- No real `AF_XDP` socket could be created in this sandbox at all (see "Status" above), so this
  finding was never observed against a real syscall in this session - only against synthetic bytes
  shaped exactly like what a real pre-5.9 kernel's `getsockopt(2)` implementation deposits. A
  future session with access to an actual Linux <5.9 kernel (or kernel build) and a container with
  `cap_net_raw` available could close this gap with a fully end-to-end repro.
- The sibling `xdp_mmap_offsets()` v1 fallback (same two files, immediately above
  `xdp_statistics()`) was re-read specifically as a contrast case and found correct (its field
  shuffle between `xdp_mmap_offsets_v1`'s 4-ring/3-field-each layout and the full 4-ring/4-field
  layout is handled properly) - not a second instance of the same bug class.
- Not checked: whether any other `getsockopt`/`setsockopt` wrapper in the crate has a similar
  "two struct versions, fields assigned from the wrong one" shape outside the two xdp-related
  functions in `src/backend/*/net/sockopt.rs` - no other versioned-struct getsockopt call site was
  found while reading this file, but a dedicated sweep wasn't performed this session.
