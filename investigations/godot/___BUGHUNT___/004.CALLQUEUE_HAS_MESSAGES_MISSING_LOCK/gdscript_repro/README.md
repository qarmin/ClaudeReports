# GDScript-level reproduction attempt - inconclusive, see RESULT.md

This candidate's bug (`CallQueue::has_messages()`/`get_max_buffer_usage()`/`is_flushing()`
missing a lock, `core/object/message_queue.cpp:440-458`) has **no** scripting-bound entry
point for `has_messages()`/`is_flushing()` (confirmed by grep - never bound via
`ClassDB::bind_method` anywhere), but `get_max_buffer_usage()` *is* indirectly reachable from
GDScript via `Performance.get_monitor(Performance.MEMORY_MESSAGE_BUFFER_MAX)`
(`main/performance.cpp:265-266`), which reads the exact same global
`MessageQueue::get_singleton()` that `Object.call_deferred()` mutates.

`main.gd` drives exactly that pairing: writer `Thread`s call `call_deferred("_noop")` on the
scene root, reader `Thread`s poll `Performance.get_monitor(Performance.MEMORY_MESSAGE_BUFFER_MAX)`
in a tight loop - both against the real engine binary built with this candidate's fix (see
`../patch.diff`).

## What actually happened

Both the **unfixed** and the **fixed** binary crash identically within ~1-2s of this script
running (`gdscript_output_crash.txt` / `gdscript_output_oom_crash_persists_after_fix.txt`),
always in the same place:

```
Failed method: Node::_noop. Message queue out of memory. ...
handle_crash: Program crashed with signal 11
...
CallQueue::push_callablep(...) core/object/message_queue.cpp:97
CallQueue::push_callp(...) core/object/message_queue.cpp:69
```

This is **not** this candidate's bug: `push_callablep`/`push_callp` are the *locked* mutator
side, unaffected by this candidate's fix (which only touches the three unlocked *readers*) -
hence the identical crash before and after. The real cause is that 4 unthrottled writer
`Thread`s under ASan's slowdown can out-produce the single main-thread flush-per-frame fast
enough to exhaust `memory/limits/message_queue/max_size_mb` (bumped to 256MB in
`project.godot`, still not enough) within a couple of seconds, and something in the
`ERR_FAIL_COND_V_MSG`-guarded out-of-memory path at `message_queue.cpp:94-99` then crashes
instead of cleanly returning `ERR_OUT_OF_MEMORY`. That looks like its own, separate, real bug
(possibly another unlocked-read race, or a use of `p_callable`/`error_text` after something
else frees it under contention) - **not investigated further this session**, added to
`IDEAS.md` as a fresh lead instead of being conflated with this candidate.

## Status for this candidate

**Inconclusive as a GDScript-level confirmation.** This candidate's "Confirmed" status rests
entirely on the standalone mirror repro (`../repro/`, 3/3 unfixed crashes / 3/3 fixed clean,
also independently re-verified via `../justfile`'s `just test` against a fresh clone) plus the
code-reading reachability argument above (`has_messages()` is hit every frame from
`SceneTree::process()` on `pg->call_queue`, used by the real "process thread groups" feature;
`get_max_buffer_usage()` is reachable from GDScript via `Performance.get_monitor()`) - not from
an actually-run, actually-crashing real-engine GDScript reproduction, unlike candidates 002 and
003. See `../RESULT.md` for the full picture.
