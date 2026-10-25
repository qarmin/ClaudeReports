# `CallQueue::statistics()` corrupts the deferred-call queue when hit from the out-of-memory path

Found as a side effect of candidate 004's investigation (heavy concurrent `call_deferred()`
producer threads, used to stress-test `CallQueue`'s unlocked readers, hit the queue's
`max_pages` cap faster than normal and crashed both before *and* after that candidate's fix -
flagged at the time as "a separate, likely-real crash... not root-caused this session" and added
to `IDEAS.md`). This session root-caused and fixed it.

**Unlike every other candidate this session, this is not fundamentally a concurrency bug** - it
reproduces 100% of the time, single-threaded, the moment the deferred-call queue (`CallQueue`,
backing `Object.call_deferred()`) is filled past its configured capacity
(`memory/limits/message_queue/max_size_mb`, 32MB/8192 pages by default). It's included in this
thread-focused session because it was *discovered* through concurrency stress-testing and because
the same underlying queue is the subject of candidate 004.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | n/a | `push_callablep()`/`push_set()`/`push_notification()`'s out-of-memory branch (`pages_used == max_pages`) calls `statistics()` (`:347-440`) after logging an error. `statistics()` is not a read-only diagnostic: its loop body destructs **every pending message in every page** (`message->~Message()`, plus `args[k].~Variant()` and - via `Callable`'s own destructor - any `CallableCustom` it owns) while walking `pages`/`page_bytes` - but never resets `pages_used`/`page_bytes[]` the way `flush()`/`clear()` do. `grep`-confirmed `statistics()` has **no other caller anywhere in the engine** - these 3 OOM branches are its only call sites, so this corruption happens on every real-world queue-overflow, not just under this session's synthetic stress |
| GDScript-level repro (`gdscript_repro/`), single-threaded, against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`, `memory/limits/message_queue/max_size_mb=1` in `project.godot`, `--headless --path gdscript_repro`, `_ready()` calls `call_deferred("_noop")` 2,000,000 times in a tight loop (comfortably exceeds the 1MB/256-page cap before any frame can flush) | **Crashed 3/3 runs**, a fully-symbolized ASan heap-use-after-free with every frame inside the real engine - `Callable::get_object()` reading memory already freed by an earlier `Callable::~Callable()` call, both from within the same `CallQueue::statistics()` invocation. See "Root cause" below |
| Fix verification: same `gdscript_repro/` against the fixed, rebuilt binary | Same binary/build (incremental rebuild, ~30-39s - only `message_queue.cpp` changed), 3 runs | **Clean 3/3 runs**, "No crash within the time limit" every time |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline from candidates 002-005 (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.**

Reproduces **3/3** runs crashing (ASan heap-use-after-free, fully symbolized, identical signature
every time) against the unfixed binary; **3/3** clean runs with the fix applied. This is a
deterministic bug, not a narrow race - every run crashes, every run heals once fixed. All runs:
real, self-built `target=editor use_asan=yes tests=yes debug_symbols=yes` ASan binary,
`--headless`, single-threaded GDScript, 24-core machine (irrelevant here - no concurrency
involved).

**Fully reproducible**: **Yes** - a real Godot project (`gdscript_repro/`), run against a real,
self-built ASan editor binary, driving the actual `CallQueue::push_callablep`/`statistics()` path
through nothing but `Object.call_deferred()`, the ordinary public GDScript API every deferred call
in the engine goes through. No standalone C++ mirror was built for this candidate (per this
project's current repro policy) - this bug has a real, trivial GDScript entry point, so none was
needed.

## Class

`memory-safety` (heap-use-after-free from a function that frees live data without updating the
bookkeeping that still claims it's valid)

## Root cause

`core/object/message_queue.cpp:347-440`, `CallQueue::statistics()`. Despite the name suggesting
a read-only diagnostic dump, its loop body actually **consumes** every pending message as it
counts it:

```cpp
void CallQueue::statistics() {
    LOCK_MUTEX;
    ...
    for (uint32_t i = 0; i < pages_used; i++) {
        uint32_t offset = 0;
        while (offset < page_bytes[i]) {
            Message *message = (Message *)&page->data[offset];
            ...
            Object *target = message->callable.get_object();   // (*) read, counted below
            switch (...) { /* tally into set_count/call_count/notify_count */ }
            offset += advance;
            if (... != TYPE_NOTIFICATION) {
                for (int k = 0; k < message->args; k++) { args[k].~Variant(); }   // destructs args
            }
            message->~Message();                                // destructs the Callable too
        }
    }
    ... // fprintf the tallies
    UNLOCK_MUTEX;
}
```

Each iteration reads `message->callable.get_object()` (marked `(*)` above) - fine the first time
a given message slot is visited - but a `Callable` constructed by `call_deferred()` for a bound
method owns a heap-allocated `CallableCustom` (freed by `Callable`'s own destructor, called at the
bottom of each loop iteration via `message->~Message()`). Once enough messages have been
destructed in earlier iterations, the allocator's freed blocks can be immediately adjacent to or
reused near still-to-be-processed message slots within the SAME call to `statistics()` (ASan's own
`asan_malloc_linux.cpp`-backed allocator does not need to serve a *different* live allocation for
this - a `malloc`/`free` pair close in time is enough for the shadow-memory "freed" state to still
be active when the next iteration's unrelated read happens to land in that exact address range),
producing the heap-use-after-free seen in every crashing run: `Callable::get_object()` reading
state that `Callable::~Callable()` → `memdelete<CallableCustom>` already freed earlier in the very
same `statistics()` call.

Separately from (and compounding) that: even where no read-after-free within the same call
happens, `statistics()` never resets `pages_used`/`page_bytes[]` (unlike `flush()`/`clear()`,
which always do) - so after it returns, the queue's own bookkeeping still claims `pages_used`
pages of *valid* messages that have, in fact, all just been destructed. Any subsequent operation
that trusts that bookkeeping (another `push_*` call hitting the cap again and calling
`statistics()` a second time, or an ordinary `flush()` later in the same frame) would process
already-destructed `Message`/`Callable`/`Variant` objects as if they were live - a second,
independent source of use-after-free on top of the first.

Reachability: any code path that calls `Object.call_deferred()` (or `set_deferred`/
`notification(..., true)`'s deferred-notification path) often enough, fast enough, to fill the
queue before the next frame's `flush()` - most directly, a tight loop with no yield point, but
also plausible in real games with `memory/limits/message_queue/max_size_mb` left at a value too
small for a burst of deferred calls (e.g. instantiating/destroying many nodes in one frame, each
scheduling several deferred calls).

## Introduced by

`not determined`. `statistics()`'s destructive loop body looks original to the file (not an
accidental merge of an unrelated "drain the queue" helper); the 3 call sites that invoke it from
the out-of-memory branch all look original too. Not bisected this session.

## The fix

Remove the 3 `statistics();` calls from the out-of-memory branches of `push_callablep()`,
`push_set()`, and `push_notification()`. Each branch already prints a targeted diagnostic
(`fprintf(stderr, "Failed method/set/notification: ...")` naming the specific callable/property/
notification that couldn't be queued) immediately before the now-removed call - that's the
actionable information a developer needs to raise `memory/limits/message_queue/max_size_mb` or
fix a runaway deferred-call producer; the additional aggregate dump `statistics()` provided (total
pages, counts grouped by target method/property/notification) is a nice-to-have that is not worth
corrupting the queue over (`+0/-4` against `core/object/message_queue.cpp`, see `patch.diff`):

```cpp
if (pages_used == max_pages) {
    fprintf(stderr, "Failed method: %s. Message queue out of memory. %s\n", ...);
    UNLOCK_MUTEX;
    return ERR_OUT_OF_MEMORY;
}
```

This is the smallest, lowest-risk fix: it doesn't change `statistics()` itself (still callable
manually by an embedder/future caller, who would need to understand it's destructive - see "What's
not done" below), doesn't change any success-path behavior, and doesn't touch any other file.

Verified: 3/3 clean runs of `gdscript_repro/` against the rebuilt binary (vs. 3/3 crashing
before), identical test-suite results (`1431 | 1430 | 1 | 3`, same baseline).

- **Applied for real** to `core/object/message_queue.cpp`, rebuilt the ASan editor binary
  (incremental, ~30-39s each direction - single translation unit, no header touched), ran
  `gdscript_repro/` and the test suite against it, then reverted
  (`git checkout -- core/object/message_queue.cpp`) before finishing this candidate - main tree
  confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan+
  debug_symbols binary, absolute `XDG_DATA_HOME`): `1431 test cases | 1430 passed | 1 failed |
  3 skipped`, identical to the established baseline (same pre-existing `test_ip.cpp` IPv6
  failure, unrelated to this change).

## What's not done / open questions

- **`statistics()` itself remains destructive** - this fix only stops it from being
  auto-triggered from the error path; if anything else ever calls it in the future (it's a public
  method), the same corruption would recur. A more thorough fix would make `statistics()`
  read-only (don't destruct messages while tallying them) or have it call something
  `flush()`/`clear()`-equivalent afterward to keep the bookkeeping consistent - judged out of
  scope for this minimal, OOM-path-only fix, but worth flagging for a maintainer.
- Did not determine whether the within-single-call heap-use-after-free (reading a `Callable` just
  freed earlier in the *same* `statistics()` loop) requires a specific allocator/ASan interaction
  to manifest, or would also corrupt memory (silently, without an ASan-visible UAF) on a plain
  non-ASan release build - the double-destruct-without-bookkeeping-reset half of the bug (see
  "Root cause") is unconditionally wrong regardless of allocator behavior, so the fix is correct
  either way, but the *exact* crash signature on a non-ASan build wasn't characterized.
- This candidate's repro happens to be single-threaded; candidate 004's own GDScript repro
  attempt (which originally surfaced this crash) used concurrent `call_deferred()` producers and
  is retained as supporting evidence in `004.CALLQUEUE_HAS_MESSAGES_MISSING_LOCK/gdscript_repro/`
  - not re-verified against this candidate's fix, since this candidate's own single-threaded repro
  already isolates and confirms the bug directly.
