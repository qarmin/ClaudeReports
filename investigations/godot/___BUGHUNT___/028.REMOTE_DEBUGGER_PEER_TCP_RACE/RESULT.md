# `RemoteDebuggerPeerTCP` missing lock on `in_queue`/`connected` races its own background poll thread

Thread-focused bug-hunt candidate (batch-assigned). `RemoteDebuggerPeerTCP`
(`core/debugger/remote_debugger_peer.cpp`) spawns a background `_poll()` thread
(`_thread_func`) the moment a `--remote-debug` TCP connection is made; `has_message()` reads
`in_queue.size()` and `is_peer_connected()` reads the plain `bool connected` with **no lock at
all**, while the background thread writes both (`connected = ...` in `_poll()`, `in_queue`
mutations in `_read_in()`) and the two sibling accessors `get_message()`/`put_message()` on the
same `in_queue`/`out_queue` correctly take `mutex` first - same missing-lock-on-some-accessors-
but-not-others shape as several earlier candidates in this workspace (002, 004, 009-012, 016,
018, 027).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading + call-graph trace | n/a | Confirmed `has_message()`/`is_peer_connected()` (lines 40-46) read `in_queue`/`connected` unlocked while `get_message()`/`put_message()` (and the background thread's own `_read_in()`/`_poll()`) touch the same fields under `mutex`; traced both accessors' only real call sites to `RemoteDebugger::_poll_messages()`/`flush_output()`, called every engine frame from the main thread via `RemoteDebugger::poll_events()`/`debug()` while `--remote-debug` is active, concurrently with the background `_poll()` thread spawned in `RemoteDebuggerPeerTCP`'s constructor |
| Debug-assertions reasoning (manual boundary construction) | n/a | `List<Array>::_data` is `nullptr` while empty and gets allocated/freed on the first push / last pop (`core/templates/list.h`); reasoned this makes the unlocked `in_queue.size()` read genuinely unsafe (not just stale-value-benign) across the empty-to-non-empty transition and across `get_message()`'s pop-to-empty free - escalated to ThreadSanitizer to get a real diagnostic rather than asserting this from reading alone |
| **ThreadSanitizer, real two-process repro** (see "Status" below) | `scons platform=linuxbsd target=template_debug use_tsan=yes debug_symbols=yes disable_path_overrides=no`; real `server_project/` (a genuine TCP debug server over loopback, speaking real `var_to_bytes()`-encoded wire messages) + real `client_project/` debuggee started with `--remote-debug tcp://127.0.0.1:17891`; 5 runs total, ~12s of real traffic each (first 2 at ~1 msg/frame, last 3 bursting 50 msgs/frame) | **5/5 runs**: real TSan data race in `RemoteDebuggerPeerTCP::is_peer_connected()` (`remote_debugger_peer.cpp:41`) vs. the background thread's unlocked write in `_poll()` (`:222`). **3/3 of the high-burst runs** additionally hit the exact candidate-described `has_message()` race (`:45`, vs. `_read_in()`'s locked `push_back()` at `:168`/`list.h:312`) and a related, same-root-cause race: `_read_in()`'s own unlocked capacity-check read of `in_queue.size()` (`:134`) vs. `get_message()`'s locked `pop_front()`/`erase()` (`:54`, a real free of the list's internal `_Data` block, `list.h:247`) |

## Status

**Confirmed.**

Reproduces **5/5** runs (the `is_peer_connected()` race fires in every run) and **3/3** of the
high-burst runs additionally reproduce the `has_message()`/`in_queue` races, all via real
ThreadSanitizer diagnostics (not a single-occurrence crash) - see `output.txt` for the exact
captured `SUMMARY:` lines and full reports. Fixed build: **5/5 runs clean** (zero
`remote_debugger_peer.cpp`/`core/templates/list.h` races; the only TSan output that still
appears sometimes is two pre-existing, unrelated findings - see "What's not done" below).

**Fully reproducible**: **Yes**. The repro is a real, standalone `--remote-debug` debuggee
(`client_project/`) driven by a real, independent second participant (`server_project/`, run as
its own OS process) that opens a real `TCPServer`, accepts the debuggee's real TCP connection,
and sends real `var_to_bytes()`-encoded, length-prefixed messages over a real loopback socket -
the exact wire format `RemoteDebuggerPeerTCP::_read_in()` decodes in production. Nothing about
the second participant's behavior is hand-faked: `server_project/main.gd` is a genuine Godot
script using the engine's own `TCPServer`/`StreamPeerTCP`/`var_to_bytes()` APIs, so the bytes it
sends are produced the same way any real external debug server (e.g. the editor, or any custom
tool implementing this protocol) would produce them.

## Class

`data-race` (ThreadSanitizer-diagnosed; the `in_queue`/`_read_in()`-vs-`get_message()` pairing in
particular is a real use-after-free-shaped race, since `List::erase()` frees the list's internal
`_Data` block once it empties - not merely a stale-value read)

## Root cause

`RemoteDebuggerPeerTCP` (`core/debugger/remote_debugger_peer.cpp`) spawns a background
`_poll()`/`_thread_func()` thread in its constructor the moment a `--remote-debug` TCP
connection is established (`core/debugger/remote_debugger_peer.cpp:94`). That thread's `_poll()`
calls `_write_out()`/`_read_in()` and then writes `connected = tcp_client->get_status() == ...`
(`:227` pre-fix) with **no lock**. Concurrently, the main thread's per-frame debugger polling
(`RemoteDebugger::poll_events()`/`debug()`/`flush_output()`/`send_message()`/`send_error()`, all
reachable merely by having `--remote-debug` active, no breakpoint needed) calls
`is_peer_connected()` (`:40-42` pre-fix, `return connected;` with no lock) and, via
`RemoteDebugger::_poll_messages()`, `has_message()` (`:44-46` pre-fix, `return in_queue.size() >
0;` with no lock) once per frame. Both are plain unsynchronized reads racing the background
thread's writes - a genuine C++ data race regardless of whether a given run happens to tear a
value, confirmed directly by ThreadSanitizer rather than inferred.

The `in_queue` side is worse than a simple stale-read: `List<Array>::_data`
(`core/templates/list.h`) is `nullptr` while the list is empty and gets `memnew`'d on the first
`push_back()` and `memdelete`'d back to `nullptr` the moment `erase()`/`pop_front()` empties it
(`list.h:244-250`). `has_message()`'s unlocked `in_queue.size()` dereferences that `_data`
pointer with no synchronization establishing a happens-before edge against either the background
thread's locked `push_back()` (`_read_in()`, `:168`) or the main thread's own locked
`get_message()`/`pop_front()` (`:54`, which can free `_data`). A third, closely related instance
of the same missing lock lives inside `_read_in()` itself: its own capacity check
(`if (in_queue.size() > max_queued_messages)`, originally `:134`) reads `in_queue.size()`
*before* taking `mutex` (the lock is only acquired later, around the `push_back()` a few lines
down) - so that read races `get_message()`'s locked, potentially list-emptying `pop_front()` too.

## Introduced by

b8ddaf9c33 "Refactor ScriptDebugger." (2020-02-27) - this commit created
`core/debugger/remote_debugger_peer.cpp` and `RemoteDebuggerPeerTCP` itself; `is_peer_connected()`
and `has_message()` have had exactly their current unlocked bodies (`return connected;` /
`return in_queue.size() > 0;`) since this commit, while `get_message()`/`put_message()` have
always taken `MutexLock lock(mutex)` first. The bug has been present, unchanged, for the entire
lifetime of this class (verified via `git log --follow`/`git show` on the file).

## The fix

Three one-line-scoped additions, all using the class's own existing `Mutex mutex` (already used
by `get_message()`/`put_message()`/`_write_out()`/`_read_in()`'s push path) - no new locking
primitive introduced:

```cpp
bool RemoteDebuggerPeerTCP::is_peer_connected() {
    MutexLock lock(mutex);
    return connected;
}

bool RemoteDebuggerPeerTCP::has_message() {
    MutexLock lock(mutex);
    return in_queue.size() > 0;
}
```

- `_poll()`'s write to `connected` is now also taken under the same lock (its own self-read
  `if (connected)` a few lines above does not need one - it is only ever written by this same
  background thread, so there is no cross-thread race on that particular read; only the write
  needs to be synchronized against `is_peer_connected()`'s now-locked external readers).
- `_read_in()`'s own unlocked capacity-check read of `in_queue.size()` (the related race found
  during this session's TSan runs, not originally called out by the candidate description but
  the same field in the same file) is now wrapped in its own `MutexLock` scope, matching the
  existing `MutexLock lock(mutex); in_queue.push_back(var);` a few lines later in the same
  function.

`Mutex` is Godot's recursive mutex type (`core/os/mutex.h`), so none of these additions risk a
self-deadlock even though `_poll()` already takes the same `mutex` indirectly via `_write_out()`/
`_read_in()`'s push.

Verified by re-running the exact same two-process repro against the patched, rebuilt
(`scons ... use_tsan=yes ...`, ~20-35s incremental) binary: **5/5 clean runs**, zero TSan reports
anywhere in `remote_debugger_peer.cpp` or `core/templates/list.h` (see `output.txt`). The fix was
exercised across two different traffic shapes (the original ~1 msg/frame rate and the later
50 msgs/frame burst rate used to make the `has_message()`/`in_queue` races reproduce reliably),
satisfying CONFIRMATION.md section 7's "more than just the one triggering input" bar.

- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san`, ASan, `tests=yes`,
  `debug_symbols=yes`, absolute `XDG_DATA_HOME`): **before** `1431 test cases | 1430 passed | 1
  failed | 3 skipped`; **after** `1431 test cases | 1430 passed | 1 failed | 3 skipped` -
  identical, same pre-existing `tests/core/io/test_ip.cpp` IPv6-loopback environment failure
  already documented as this workspace's established baseline (candidates 002/003/004/010/011).
  No new failures introduced by the fix.

## What's not done / open questions

- **Two pre-existing, unrelated TSan findings surfaced incidentally in the same runs and were
  deliberately left untouched** (out of scope for this candidate, confirmed unaffected by the fix
  since both still appear sometimes in the fixed runs too): (1) a lock-order-inversion between
  `RemoteDebugger::mutex` and the global error-print lock (`_err_print_error`/`_global_lock`),
  reachable via `_poll_messages()`'s `ERR_CONTINUE_MSG` on a malformed packet racing
  `Engine::print_header()`'s own startup print; (2) a data race in
  `drivers/unix/file_access_unix.cpp:51` `FileAccessUnix::check_errors()`, unrelated to the
  debugger entirely. Both are added to `IDEAS.md` as fresh, independent leads.
- `RemoteDebuggerPeerWebSocket` (`modules/websocket/remote_debugger_peer_websocket.cpp`) and
  `RemoteDebuggerPeerMessagePort` (`platform/web/remote_debugger_peer_messageport.cpp`) implement
  the same `RemoteDebuggerPeer` interface and were not checked for the same missing-lock pattern
  this session - worth a follow-up candidate given how consistently this exact shape (locked
  mutators, unlocked accessors) has recurred across this workspace's `009`-`018`/`027`.
  `editor/debugger/script_editor_debugger.cpp`'s own `is_session_active()`/`has_message()` call
  sites (the editor's side of the same protocol, driving a `RemoteDebuggerPeerTCP` in the
  opposite direction) were traced but not independently exercised - same underlying class, so the
  same fix already covers it, but not separately re-confirmed with its own repro.
- The `in_queue`/`_read_in()` capacity-check race (the `list.h:247`/`:134` pairing) was discovered
  during this session's escalation to a higher traffic burst rate, not predicted by the original
  candidate description - included in the same bounded fix since it is the same field, same file,
  same missing-lock root cause, but noting it explicitly since it widens this candidate's scope
  slightly beyond what was originally flagged.
