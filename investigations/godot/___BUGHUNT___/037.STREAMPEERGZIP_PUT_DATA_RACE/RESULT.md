# `StreamPeerGZIP` has no locking anywhere in the class

From `IDEAS.md`'s backlog, picked as the next sibling after candidates 035 (`BitMap`, confirmed)
and 036 (`Image`, confirmed): `StreamPeerGZIP` keeps local mutable state (`void *ctx` - a raw
`z_stream*`, `RingBuffer<uint8_t> rb`, `Vector<uint8_t> buffer`) touched directly by every public
method, with zero `Mutex` anywhere in its header - confirmed before investigating.

(Note: `NavigationPolygon`, also named in the same original backlog entry, was checked first and
turned out to already have thorough `RWLock`/`Mutex` protection on every accessor - the original
external sweep's "zero Mutex" claim for that class was incorrect, likely from a grep that missed
`RWLock`-based locking. No candidate was opened for it; it's removed from the backlog.)

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `core/io/stream_peer_gzip.cpp`/`.h` - confirmed zero `Mutex` anywhere in the header; `put_partial_data()`/`finish()` resize `buffer` (a plain `Vector<uint8_t>`) and call `_process()`, which drives the raw `z_stream` pointed to by `ctx` and writes into `rb` (`RingBuffer<uint8_t>`), all with no lock; `clear()` frees `ctx` via `_close()` | Static read | Same "local mutable buffer/handle, zero locking, reachable off the main thread" shape as candidates 016/027/035/036; confirmed `start_compression`/`put_data` (via `StreamPeer` base)/`clear`/`finish` are all `ClassDB`-bound, reachable from GDScript |
| GDScript-level repro (`repro/`: shared `StreamPeerGZIP` after a single `start_compression()` call on the main thread, 6 worker threads each alternating `put_partial_data()` with a 4096-byte and a 16-byte payload plus a `get_partial_data()` drain, for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/037`, `bin/`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed or hung on 5/5 attempts**: 3 real ASan `attempting double-free` reports (`core/os/memory.cpp:160` in `Memory::realloc_static`, one with a full stack through `zlib`'s `deflate`/`fill_window`/`read_buf` into `StreamPeerGZIP::_process`/`put_partial_data`), 2 timeouts with no "survived" printed (consistent with the race corrupting the shared `z_stream`'s internal state and `zlib` spinning rather than crashing outright - not independently diagnosed further, noted as a secondary symptom of the same missing lock rather than a separate bug) |
| Fix (`_THREAD_SAFE_CLASS_` on the header + `_THREAD_SAFE_METHOD_` on `clear()`, `_start()` (covers both `start_compression`/`start_decompression`), `put_partial_data()` (covers `put_data`), `get_partial_data()` (covers `get_data`), `get_available_bytes()`, `finish()`), incremental rebuild (ccache-warm hardlinked worktree, ~70s) | Same build/run | **Clean 8/8 runs**, "survived" printed every time, no hangs |
| Project test suite before/after (revert to unfixed source, rebuild, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 5/5 unfixed runs (3 real, independently-triggered ASan
`attempting double-free` crashes and 2 hangs consistent with the same corrupted shared state), and
8/8 clean runs with the fix applied (no crashes, no hangs).

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `StreamPeerGZIP` class through nothing but the public
GDScript API (`put_partial_data`/`get_partial_data`, exposed via the `StreamPeer` base class). No
second participant is involved - a same-process crash reachable from GDScript `Thread`s calling
`put_partial_data()` concurrently on a shared `StreamPeerGZIP`.

## Class

`memory-safety` (heap corruption, manifesting as an ASan double-free - concurrent
`put_partial_data()` calls race each other's `buffer.resize()`/`rb.write()` and the shared raw
`z_stream` dereferenced through `ctx`, corrupting `zlib`'s internal allocator-tracked buffers).

## Root cause

`StreamPeerGZIP::put_partial_data()`:

```cpp
Error StreamPeerGZIP::put_partial_data(const uint8_t *p_data, int p_bytes, int &r_sent) {
	ERR_FAIL_NULL_V(ctx, ERR_UNCONFIGURED);
	...
	if (buffer.size() < p_bytes) {
		buffer.resize(p_bytes);                                    // <-- unguarded resize
	}
	...
	RETURN_IF_ERROR(_process(buffer.ptrw(), ..., p_data + r_sent, ..., sent, to_write));  // <-- unguarded z_stream use
	...
	int wrote = rb.write(buffer.ptr(), to_write);                  // <-- unguarded ring buffer write
	...
}
```

`ctx` (a raw `z_stream*`), `buffer` (`Vector<uint8_t>`), and `rb` (`RingBuffer<uint8_t>`) are all
plain members with zero synchronization anywhere in the class. Two threads calling
`put_partial_data()` concurrently race each other's `buffer.resize()` (same `CowData`-corruption
shape as candidates 035/036) and drive the *same* `z_stream` through `zlib`'s `deflate()`
concurrently - `zlib` itself is not thread-safe for concurrent calls on the same stream, so this
corrupts its internal state, observed as ASan double-frees inside `Memory::realloc_static` (one
with a full backtrace through `zlib`'s own `fill_window`/`read_buf`) or, in two attempts, a hang
consistent with `zlib`'s internal state becoming inconsistent enough to loop rather than crash
outright.

Reachability: `put_data`/`put_partial_data`/`get_data`/`get_partial_data` are inherited, bound,
GDScript-callable public methods (`core/io/stream_peer.cpp:406-410`), and `start_compression`/
`finish`/`clear` are bound directly on `StreamPeerGZIP` itself - an ordinary `RefCounted` object,
reachable from any thread with no special setup.

## Introduced by

273ba0794f022748ed5084ee74020849443af7d7 "[IO] Add StreamPeerGZIP for streaming
compression/decompression." (2022-07-30) - the commit that added this class; it has never had any
locking since.

## The fix

`+12/-0` against `core/io/stream_peer_gzip.cpp`, `+3/-0` against `core/io/stream_peer_gzip.h` (see
`patch.diff`): add `_THREAD_SAFE_CLASS_` next to the private members, and `_THREAD_SAFE_METHOD_` at
the top of `clear()`, `_start()` (the shared implementation behind both
`start_compression`/`start_decompression`), `put_partial_data()` (covers `put_data`, which calls
it), `get_partial_data()` (covers `get_data`), `get_available_bytes()`, and `finish()`. `_process()`
and `_close()` are private helpers only ever called from within an already-locked public entry
point, and `Mutex` (`core/os/mutex.h`) is recursive, so no additional lock is needed there - same
scoping convention as candidates 035/036.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 5/5 crashing-or-hanging unfixed).

- **Applied for real** in an isolated worktree (`.worktrees/037`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- The two unfixed-run hangs were not independently root-caused beyond "consistent with corrupted
  `zlib` internal state" - a future session could attach a debugger mid-hang to confirm exactly
  where `zlib` gets stuck, though this isn't necessary to confirm or fix the underlying missing-lock
  bug itself.
- The destructor (`~StreamPeerGZIP()`, which calls `_close()` directly) was deliberately left
  without its own lock - guarding a destructor against concurrent use by other threads is a
  different, harder problem (the object may be mid-destruction while another thread still holds a
  reference) that's out of scope for this candidate; this fix addresses concurrent calls on a
  *live* object, not destruction racing concurrent use.
- The remaining siblings named in the original `IDEAS.md` backlog entry (`CryptoKey`,
  `GLTFDocument`, `ENetConnection`) were not investigated this session.
