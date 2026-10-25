# `MultiMesh` instance-count resize vs. instance-transform access race

From `IDEAS.md`'s backlog: `MultiMesh` was flagged (alongside `BitMap`, `Image`, `CryptoKey`,
`GLTFDocument`, `NavigationPolygon`, `ENetConnection`, `StreamPeerGZIP`) as a plain unsynchronized
container with zero `Mutex` anywhere in its header (confirmed: `grep -n "Mutex" scene/resources/
multimesh.h` returns nothing), suspected to share candidates 016/027's `Curve`/`Curve2D`/`Curve3D`
shape (a resize racing a reader/writer on the same backing storage).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/resources/multimesh.cpp` (`set_instance_count()`, `set_instance_transform_2d()`/`get_instance_transform_2d()`, etc.) | Static read | Unlike `Curve`, `MultiMesh` holds **no local backing buffer** of its own - `instance_count`/`visible_instance_count`/`transform_format`/etc. are plain unguarded members, but the actual per-instance data lives entirely server-side behind a `RID` (`RenderingServer::get_singleton()->multimesh_*`); every accessor's only local state is the bounds-check inputs (`instance_count`), not the data itself. This changes the shape of the hypothesis from "local buffer UAF" (candidates 016/027) to "stale `instance_count` read lets an out-of-range index reach the RenderingServer call", which is a different mechanism to actually trigger |
| GDScript-level repro attempt #1 (`repro/`: 2 threads oscillating `instance_count` between 1000/1 , 4 threads calling `set_instance_transform_2d(50, ...)`/`get_instance_transform_2d(50)` in a tight loop, `wait_to_finish()` blocking the main thread in `_ready()`), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/034`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 25/90 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **No crash in 3 attempts** (25s and 90s timeouts) - instead a high rate of expected `ERR_FAIL_INDEX` log+backtrace prints (index 50 legitimately out of range half the time by design) that, per the already-documented "fixed repro log-print-volume" pitfall (`projects/godot.md`), made the run too slow to finish inside either timeout. Inconclusive - the bounds check appears to be doing its job, not a crash under this access pattern |
| GDScript-level repro attempt #2 (same shape, but touch index fixed at `0`, always within the smaller of the two oscillating counts (1/1000), to eliminate the expected-`ERR_FAIL_INDEX` noise and isolate the real race) | Same binary, same harness pattern, `timeout -k 5 20-90`, 1-2 worker threads | **Hung (no crash, no completion) up to 90s wall-clock on every attempt.** `/proc/<pid>/task/*/wchan` while hung showed the dedicated `Node::_touch` thread parked in `futex_do_wait` (genuinely blocked, not busy-spinning) - a real deadlock, but traced to the test harness itself blocking the main thread in `wait_to_finish()` inside `_ready()`, which stops the engine's own per-frame loop from ever running again; `MultiMesh`'s RS calls appear to go through a bounded command queue that needs the main thread's frame loop to drain it, so any worker thread pushing enough RS traffic while the main thread is parked like this blocks forever on queue backpressure - an artifact of how the repro blocks the main thread, not a `MultiMesh`-specific defect (a caller that doesn't block its own main thread like this would never hit it) |
| GDScript-level repro attempt #3 (identical worker threads, but the main thread yields via `await get_tree().create_timer(4.0).timeout` instead of `wait_to_finish()`, so the engine's frame loop keeps running and drains the RS command queue normally while the race window is exercised) | Same binary, 2 resize + 4 touch threads, 3s stress window, 4s main-thread await | **Clean 3/3 runs, "survived" every time** - no ASan error, no crash, no hang |

## Status

**Not reproducible.** Three distinct approaches tried (naive out-of-range index access, same-index
access with a blocked main thread, same-index access with a live main thread/frame loop) - none
produced a memory-safety crash. The one approach that did hang (attempt #2) was traced to the test
harness blocking the main thread rather than to `MultiMesh` itself, and disappeared once the
harness stopped doing that (attempt #3).

Root-cause read for why this differs from candidates 016/027's confirmed `Curve`/`Curve2D`/
`Curve3D` bugs: those classes keep their actual mutable data in a local `LocalVector`/cache that
every accessor touches directly and unsynchronized, so a concurrent resize-vs-read genuinely races
on the same memory. `MultiMesh` instead only keeps bookkeeping scalars locally; the actual
per-instance array lives behind a `RID` inside `RenderingServer`, reached only through calls that
(at least in this build/configuration) appear to serialize through the engine's own command-queue/
frame-loop mechanism rather than being touched directly by the calling thread. This doesn't rule
out a race entirely (a build using `--rendering-driver`/threading options not tried this session
might behave differently, or the bounds-check-vs-resize race on the scalar `instance_count` member
itself could still be a torn/stale read worth a narrower, more targeted investigation), but no
approach tried this session reached an actual memory-safety failure.

## What's not done / open questions

- Not tried: forcing single-threaded rendering (`--rendering-driver` variations) or checking
  whether a non-headless run with an active render thread changes the command-queue draining
  behavior enough to expose a real race.
- Not tried: a narrower repro targeting just the scalar bookkeeping fields (`instance_count`,
  `transform_format`, `use_colors`) for a plain torn-read race independent of any RS call at all
  (e.g. racing `set_transform_format()` against `set_instance_transform()`'s
  `ERR_FAIL_COND_MSG(transform_format == TRANSFORM_2D, ...)` check) - lower-value than the
  originally-suspected buffer race, not pursued under this candidate's time budget.
- The other seven siblings named in the same `IDEAS.md` backlog entry (`BitMap`, `Image`,
  `CryptoKey`, `GLTFDocument`, `NavigationPolygon`, `ENetConnection`, `StreamPeerGZIP`) were not
  investigated this session - `MultiMesh` was picked as the most `Curve`-like of the eight, and
  turned out structurally different (RID-backed, not a local buffer) once actually read in detail;
  the remaining seven should each be checked for whether they keep a *local* buffer (closer to the
  `Curve` shape) before assuming the same "not reproducible" result transfers.
- **Worth remembering for future repros that stress `RenderingServer`-backed resources from worker
  threads while blocking the main thread in `_ready()`**: this can deadlock on command-queue
  backpressure for reasons unrelated to whatever bug is being investigated (see attempt #2 above) -
  prefer `await get_tree().create_timer(N).timeout` (or any other main-thread-yielding wait) over
  `Thread.wait_to_finish()` directly inside `_ready()` whenever the class under test funnels
  through `RenderingServer`/similar queued singletons, to avoid a false "hang" obscuring the real
  result.
