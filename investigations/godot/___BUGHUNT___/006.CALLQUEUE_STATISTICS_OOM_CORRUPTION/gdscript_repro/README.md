# GDScript-level reproduction - actually run, real crash captured

A real, minimal Godot project hitting `CallQueue::push_callablep()`'s out-of-memory branch
(`core/object/message_queue.cpp:94-99`), which calls `statistics()` (`:347-440`) - through
nothing but the public GDScript API: `Object.call_deferred()`, which backs `statistics()`'s only
caller in the entire engine. Built and actually run this session against a real, self-built
Godot editor binary (`scons platform=linuxbsd target=editor use_asan=yes tests=yes
debug_symbols=yes`, commit `b8420aa3a60689572a745c0f8a85c9df62d4cbfb`).

`main.gd` sets `memory/limits/message_queue/max_size_mb=1` (the smallest practical cap, in
`project.godot`) and then calls `call_deferred("_noop")` 2,000,000 times in a tight loop inside
`_ready()` - comfortably enough to exceed a 1MB/256-page queue before the engine gets a chance to
flush it (flush only happens once per frame; this whole loop runs synchronously).

**Unlike every other candidate this session, no `Thread` is involved at all** - this is a
single-threaded, 100%-deterministic bug, found as a side effect of this session's
concurrency-focused testing (heavy concurrent `call_deferred()` producers in candidate 004's
investigation hit the queue cap faster than normal, surfacing it), but reachable by any single
thread that queues enough deferred calls without yielding back to the main loop.

## Result: confirmed, 3/3 crashing runs unpatched, 3/3 clean runs patched

Unpatched, 3 of 3 runs crashed with a fully-symbolized ASan heap-use-after-free **inside the
real engine**:

```
==8635==ERROR: AddressSanitizer: heap-use-after-free on address 0x707a84c8bc30 ...
READ of size 8 at 0x707a84c8bc30 thread T0
    #0 Callable::get_object() const core/variant/callable.cpp:163
    #1 CallQueue::statistics() core/object/message_queue.cpp:368
    #2 CallQueue::push_callablep(Callable const&, Variant const**, int, bool) core/object/message_queue.cpp:96
    #3 CallQueue::push_callp(...) core/object/message_queue.cpp:69
    #4 Object::_call_deferred_bind(...) core/object/object.cpp:733
    ...
    #8 Node::_gdvirtual__ready_call() scene/main/node.h:437
    ...
0x707a84c8bc30 is located 16 bytes inside of 88-byte region [...]
freed by thread T0 here:
    #1 void memdelete<CallableCustom>(CallableCustom*) core/os/memory.h:169
    #2 Callable::~Callable() core/variant/callable.cpp:441
```

Every frame is the real engine's own code - `statistics()`, `push_callablep`, `push_callp`,
`Object::_call_deferred_bind`, all the way up through the real `Node::_ready()` virtual-call
dispatch. Full output: `gdscript_output_crash.txt`. The crash always hits within the very same
`_ready()` call that's still flooding the queue (never reaches the "Done queuing" print line on
an unpatched checkout).

With `../patch.diff` applied (remove the 3 `statistics();` calls from the out-of-memory
branches of `push_callablep`/`push_set`/`push_notification`) and the engine rebuilt (incremental,
~30-39s - only `message_queue.cpp` changed): 3/3 clean runs, printing "No crash within the time
limit" every time - see `gdscript_output_fixed.txt`.

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/006.CALLQUEUE_STATISTICS_OOM_CORRUPTION/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`).
Unlike candidate 005's narrow race, this one is deterministic - expect an ASan abort within a
couple of seconds on every unpatched run, and a clean ~1-2s run every time with `../patch.diff`
applied.

Note: pass an **absolute** path for `XDG_DATA_HOME` if you set it - a relative path is silently
ignored by Godot (falls back to `$HOME/.local/share`), which doesn't affect this particular bug
but did cause a separate, already-known, already-recorded bug (candidate 001, Logger null-deref
on an unwritable user data dir) to fire during this session's earlier test-suite runs.
