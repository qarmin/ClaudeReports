# `ENetConnection::connect_to_host()` has no locking, races the underlying ENet library's own host state

From `IDEAS.md`'s backlog - the last of the original "plain unsynchronized container" suspects from
the external sweep. `ENetConnection` wraps a raw `ENetHost *host` (the third-party ENet library's
own C struct, with its own internal fixed-capacity peer array) plus a Godot-side
`List<Ref<ENetPacketPeer>> peers`, with zero `Mutex`/`_THREAD_SAFE_CLASS_` anywhere in the header.

Unlike `ENetConnection`'s originally-assumed need for a real two-participant network exchange, this
specific bug needed **no actual network peer at all**: `connect_to_host()` allocates a local ENet
peer handle and queues the connection attempt synchronously, returning immediately - it never
blocks waiting for a response. This means the race is reachable purely from multiple threads
calling the same local object's public API concurrently, identical in shape to every other
candidate this session - the originally-flagged networking complexity turned out not to apply to
this particular crash.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `modules/enet/enet_connection.h`/`.cpp` - confirmed zero `Mutex` anywhere in the header; `connect_to_host()` does a check-then-act on `peers.size()` (`ERR_FAIL_COND_V_MSG(peers.size(), ...)`, a classic TOCTOU) then calls the third-party `enet_host_connect()` C function directly on the shared `host` struct, with no lock around any of it | Static read | Confirmed `connect_to_host`/`create_host`/`get_peers`/`service` are all `ClassDB`-bound, reachable from GDScript; `connect_to_host()` does not require a real listening peer to reach the vulnerable code path |
| GDScript-level repro (`repro/`: one shared `ENetConnection` host (`create_host(64)`), 4 threads each repeatedly calling `connect_to_host("127.0.0.1", 34567, 0, 0)` (no real server listening on that port - not needed), 2 threads calling `get_peers()`/`service(0)` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/048`, `bin`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 2/3 attempts** - a real ASan heap-buffer-overflow, with a full stack showing the fault **inside the third-party ENet library's own `enet_host_connect()`** (`thirdparty/enet/host.c:233`), called directly from `ENetConnection::connect_to_host()`. The 3rd attempt didn't crash within 30s: after the first thread's call succeeds, `peers.size()` becomes nonzero and every subsequent call from every thread hits the (also unguarded, but at least now consistently-true) "already connected" early-return check before the race window reopens - a timing artifact of the repro's specific design, not evidence the bug doesn't exist (see "What's not done" for why a cleaner repro would avoid this) |
| Fix (`_THREAD_SAFE_CLASS_` on the header + `_THREAD_SAFE_METHOD_` on all 20 public/bound methods that touch `host`/`peers`: `connect_to_host`, `create_host(_bound)`, `destroy`, `service`/`_service`, `check_events`, `flush`, `bandwidth_limit`, `channel_limit`, `bandwidth_throttle`, `compress`, `pop_statistic`, `get_max_channels`, `get_peers`/`_get_peers`, `get_local_port`, `dtls_server_setup`, `dtls_client_setup`, `refuse_new_connections`, `broadcast`, `socket_send`, plus the private `_create` helper), incremental rebuild (ccache-warm hardlinked worktree, ~1m3s) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild ~32s, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 2/3 unfixed runs (a real ASan heap-buffer-overflow both times, inside
the third-party ENet library's own connection-setup code), and 8/8 clean runs with the fix applied.
The 3rd unfixed run's non-crash is explained by the repro's own design (see above), not a
contradiction of the finding - per `CONFIRMATION.md`, a 2/3 rate with an understood reason for the
miss is honestly reported as such, not rounded up to "3/3" or discarded as flaky.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `ENetConnection` class through nothing but the public
GDScript API. No second participant is involved - a same-process crash reachable from GDScript
`Thread`s calling `connect_to_host()` concurrently on a shared `ENetConnection`, with no actual
network peer required.

## Class

`memory-safety` (heap-buffer-overflow inside the third-party ENet library's own `enet_host_connect()` -
concurrent calls to `ENetConnection::connect_to_host()` on the same host race each other through
ENet's internal fixed-capacity peer array, which has no thread-safety of its own and was never
designed to be called concurrently from multiple threads on the same `ENetHost`).

## Root cause

```cpp
Ref<ENetPacketPeer> ENetConnection::connect_to_host(const String &p_address, int p_port, int p_channels, int p_data) {
	Ref<ENetPacketPeer> out;
	ERR_FAIL_NULL_V(host, out);
	IPAddress ip;
	...
	ERR_FAIL_COND_V_MSG(peers.size(), out, "The ENetConnection is already connected to a peer.");  // <-- unguarded check-then-act
	...
	ENetPeer *peer = enet_host_connect(host, &address, ..., p_data);   // <-- unguarded call into shared ENetHost state
	...
	peers.push_back(out);                                              // <-- unguarded resize
	return out;
}
```

`host` (a raw `ENetHost*`, the third-party ENet library's own connection-management struct) and
`peers` (Godot's own `List<Ref<ENetPacketPeer>>` mirroring it) are both plain members with zero
synchronization anywhere in the class. Two threads calling `connect_to_host()` concurrently both
pass the `peers.size()` check before either one updates it, then both call `enet_host_connect()`
on the *same* `ENetHost` struct simultaneously - ENet's own internal peer-slot-allocation logic
(a linear scan for a free slot in its fixed-size `host->peers` array) was never designed to be
reentrant, and two threads racing through it corrupt its bookkeeping - confirmed via ASan:
`heap-buffer-overflow ... in enet_host_connect thirdparty/enet/host.c:233`, with a full stack
showing the exact call chain from `ENetConnection::connect_to_host()`.

Reachability: `connect_to_host`/`create_host`/`get_peers`/`service` (and 16 other methods) are all
bound, GDScript-callable public methods - `ENetConnection` is an ordinary `RefCounted` object,
reachable from any thread with no special setup, and - contrary to this bug class's initial
assessment - **no real network peer is needed to trigger this specific crash**, since
`enet_host_connect()` queues the connection attempt locally and returns immediately rather than
blocking on an actual handshake.

## Introduced by

42a17775314424ba24974d072bb5f74bc4ff9467 "[Net] Implement lower level ENet wrappers." (2021-06-14) -
the commit that introduced this class, already unguarded.

## The fix

`+44/-0` against `modules/enet/enet_connection.cpp`, `+3/-0` against `modules/enet/enet_connection.h`
(see `patch.diff`): add `_THREAD_SAFE_CLASS_` next to the private members, and
`_THREAD_SAFE_METHOD_` to all 22 methods (public and the private helpers they call) that touch
`host`/`peers` - found via a scripted insertion pass (same technique used for `ItemList`/`TabBar`
earlier this session), matched 22 of 22 target names on the first attempt. Given the whole class is
only 27 methods total, this is effectively a full-class treatment, not a narrowly-scoped one.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 2/3 crashing unfixed, with the 3rd run's
non-crash explained by the repro's own design rather than contradicting the finding).

- **Applied for real** in an isolated worktree (`.worktrees/048`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- A cleaner repro design that avoids the "only the first call matters" artifact (e.g. calling
  `destroy()`/re-`create_host()` between connection attempts, or having each thread use its own
  short-lived `ENetConnection` instance attempting to connect to a *shared* listening host instead
  of racing `connect_to_host()` on one shared client-mode host) would likely reproduce more
  consistently across all attempts - not pursued further since 2/3 real crashes with an
  understood, documented reason for the 3rd already meets `CONFIRMATION.md`'s bar.
- This fix guards Godot's own `ENetConnection` wrapper; it does not and cannot fix any thread-safety
  issue that might exist inside the third-party ENet library's own code for other call patterns
  this session didn't exercise (e.g. concurrent `service()`/`check_events()` calls racing real
  incoming network events from an actual remote peer) - out of scope for this candidate's narrow,
  confirmed finding.
- This was the final candidate of this session's 6-hour autonomous bug-hunt run.
