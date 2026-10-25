# `Resource::set_path()` avoids a nested recursive-lock + `Ref<>` refcount churn

`Resource::set_path()` already holds `ResourceCache::lock` (a recursive `Mutex`) when it calls
`ResourceCache::get_ref(p_path)` to check for a path collision - `get_ref()` locks the *same*
recursive mutex again (safe, since `Mutex` is recursive, but not free) and constructs a full
`Ref<Resource>` (an atomic refcount increment, then a decrement when the temporary goes out of
scope) just to test whether an entry exists. Open PR **godotengine/godot#110046** (`MERGEABLE`
against current `HEAD`) replaces this with a raw `ResourceCache::resources.getptr(p_path)` pointer
lookup, avoiding both the redundant recursive lock acquisition and the `Ref<>` refcount churn in the
common (no existing entry) case.

Toolchain: `g++` (GCC) 15, SCons 4.11.1, `scons platform=linuxbsd target=editor tests=yes
debug_symbols=yes` (same debug-symbols build used for the rest of this session).

## What was tried

Benchmark: new `resource_set_path` mode added to `harness/gdbench/bench.gd` - creates a fresh
`Resource`, assigns a unique `resource_path` (`"res://bench_resource_%d.tres" % counter`, counter
incrementing across the whole run so every `set_path()` call is a genuine new-path insert, never a
collision) 50,000 times, discarding the usual 10% warmup. 3 runs per arm, `uptime` checked before
each (load average 0.45-0.62 on a 24-core box, both idle).

| Variant | us/iter (3 runs) | mean |
|---|---|---|
| baseline (pristine `HEAD`) | 0.926440, 0.946180, 0.726740 | 0.8665 |
| patched (PR #110046) | 0.897880, 0.905940, 0.782080 | 0.8620 |

## Result

**No measurable time difference**: the patched mean (0.862us/iter) is 0.5% below baseline
(0.867us/iter), well inside the ~20-30% run-to-run spread visible within each arm's own 3 runs
(e.g. baseline's own 0.727 vs 0.946 - a 30% spread with no patch involved at all). This candidate's
benchmark exercises exactly the case the patch targets (no path collision, so the "avoid `Ref<>` +
nested lock" mechanism is fully exercised on every call) - the removed operations (one recursive
mutex re-lock, one atomic refcount increment+decrement) are apparently too small relative to
`Resource::set_path()`'s other fixed costs (the `HashMap` insert/erase operations, `String`
comparisons) to show up above this benchmark's noise floor at 50,000 iterations. Time only; peak
RSS/allocation count were not measured, since the mechanism (removing one Ref<> construct/destruct
pair) is not expected to change allocation *count* - `Ref<>` refcounting doesn't allocate, it only
does atomic increment/decrement on the existing object.

## Does it change behavior/output?

**Yes - and it is a real regression, found by the mandatory full test-suite run, not by this
candidate's own micro-benchmark.** Running `bin/godot.linuxbsd.editor.x86_64 --headless --test`
prints, only on the patched build:

```
ERROR: 1 resources still in use at exit (run with --verbose for details).
   at: clear (core/io/resource.cpp:819)
```

This does **not** appear on the pristine baseline build (confirmed by rebuilding pristine and
re-running the identical command - reproduced twice on the patched build, absent both times on
baseline). `--verbose`/`-v` did not surface which specific resource leaks in this build (the
`OS::is_stdout_verbose()` gate inside `ResourceCache::clear()` did not trigger via either flag
combination tried - not investigated further, see "What's not done" below), so the exact leaked
resource's identity is not identified, only that the leak is real and reproducible.

**Root-cause hypothesis** (not confirmed, see "What's not done"): the patch's collision-handling
branch works directly with the raw `Resource **existing` pointer instead of `get_ref()`'s
`Ref<Resource>`, and `get_ref()`'s own logic has an extra step the patch's inlined version drops -
`get_ref()` checks `ref.is_null()` *after* wrapping in a `Ref<>` (which would be true if the cached
object's refcount was already 0, i.e. it is mid-destruction) and in that case removes the stale
cache entry and treats it as "no existing resource". The patch's raw-pointer version has no
equivalent "is this entry mid-destruction" check - `existing != nullptr` is true for a mid-destruction
entry just as much as a live one, so a mid-destruction resource can be treated as a live collision
(and either wrongly "taken over" or wrongly cause an `ERR_FAIL_MSG`), which could plausibly explain
an object failing to fully unregister itself from `ResourceCache::resources` before the process
exits. This is a plausible mechanism, not a confirmed one - see below.

- **Is this a breaking change?** Not to any public API/file format, but a resource-cache-integrity
  regression is a real behavioral break (a leaked entry keeps a stale pointer in a process-lifetime
  global cache).
- Project test suite: **1427 cases, 1426 passed, 1 failed, 3 skipped both before and after** (same
  known pre-existing `test_ip.cpp` IPv6 failure) - the doctest pass/fail *count* is unaffected,
  because the leak is reported via a separate `ERR_PRINT` at shutdown, not a `CHECK`/`REQUIRE`
  assertion the test suite tracks. This is exactly why "the test suite passed" is not sufficient by
  itself - the actual printed shutdown diagnostics matter too, and were read directly rather than
  inferred from the pass count alone.

## Why the result looks the way it does

The timing side: `Resource::set_path()`'s other work (String comparisons, two `HashMap` operations -
an `erase` of the old path and an `insert`/`[]` of the new one, both involving `String` hashing) is
apparently large enough relative to one recursive-mutex-relock and one atomic refcount pair that
removing the latter doesn't move the needle at this benchmark's scale. The correctness side: the
PR's own inlining of the collision-check logic drops a safety check (`get_ref()`'s "is this entry
mid-destruction" handling) that the original two-step (`get_ref()` then check `is_valid()`) had for
free, which is a plausible - though unconfirmed - explanation for the observed leak.

## What's not done / open questions

- **The leaked resource's identity was not determined.** `OS::is_stdout_verbose()`'s gate in
  `ResourceCache::clear()` did not appear to trigger with either `-v` (which doctest's own CLI
  parser seems to intercept as its own version flag, short-circuiting the whole test run before
  Godot's own arg parsing sees it) or `--verbose --test` (printed the same "run with --verbose for
  details" message, suggesting the flag wasn't registered as active by the time `clear()` runs, or
  is reset somewhere in the `--test` path) - would need a temporary debug print inside
  `ResourceCache::clear()` itself to name the leaked resource without depending on that CLI flag at
  all.
- **The root-cause hypothesis above (mid-destruction race) is not confirmed** - would need a
  targeted repro (construct a `Resource`, drop its last ref exactly while a second `Resource` is
  mid-`set_path()` to the same path on another thread, or a single-threaded equivalent if the race
  is actually reachable single-threaded during shutdown) rather than reasoning from the diff alone.
- Given the leak reproduces on an ordinary single-threaded `--test` run (no explicit multithreading
  in play from this candidate's own benchmark), the race may not require multiple threads at all -
  worth checking whether it's actually a plain logic bug in the single-threaded collision-handling
  branch rather than a genuine concurrency race, before assuming the "mid-destruction" theory above.
- Not measured: peak RSS/allocation count (see "Result" above for why this mechanism isn't expected
  to move them).
