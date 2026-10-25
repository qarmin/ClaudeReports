# ges_validate_bin_element_added_uaf

`bin_element_added()` in `subprojects/gst-editing-services/tools/ges-validate.c` (~line 131-171),
the `bin-add-post` tracer hook `ges_validate_activate()` registers via `gst_tracing_register_hook()`
when `ges-launch-1.0`/GES apps run with `--validate`, walks the parent chain of a just-added
element with a do/while loop that does, per ancestor:

```c
gst_object_unref (parent);
parent = gst_object_get_parent (parent);
```

If that `gst_object_unref()` drops `parent`'s last reference, the following
`gst_object_get_parent(parent)` dereferences freed memory. Source: `findings.toml` UAF_9.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading (refcounting semantics of `GstObject`/`GstBin` containment) | Manual reasoning about which reference the walk's own `gst_object_unref()` call can legitimately drop | Established the bug is real but **race-only**: under single-threaded execution every ancestor bin retains an independent owning reference from its own parent bin, so the walk's own temporary ref/unref pair never actually drops an object's last reference on its own - a second thread must concurrently drop that ancestor's other last reference (e.g. a real `gst_bin_remove()`) inside the few-instruction gap between the two calls above. Decided the fix needed a real 2-thread repro, not a single-threaded one |
| Real 2-thread ASAN repro, clang debugoptimized build with `-Db_sanitize=address,undefined` (`ges=enabled`, `devtools=enabled` (gst-validate), `base=enabled`, good/bad/ugly disabled, tracer_hooks left at their project default of enabled) | Repro compiles the project's own unmodified `tools/ges-validate.c.o` (the exact object meson produces for `ges-launch-1.0`) into a standalone harness that calls the real, public `ges_validate_activate()` (the same entry point `ges-launch-1.0 --validate` calls), then races a real "walker" thread's `gst_bin_add()` against a real "remover" thread's `gst_bin_remove()` on the same ancestor bin, synchronized via a small LD_PRELOAD shim (`delay_shim.c`) that widens the real timing window by intercepting only the walker thread's own `gst_object_unref()` call from inside `bin_element_added()` (identified via `dladdr()` on the caller's return address, not by pointer alone - see "Root cause" below for why that distinction mattered) - 5/5 runs | Clean, deterministic **AddressSanitizer: heap-use-after-free** every run, backtrace pinpointing exactly `ges-validate.c:168` (the fatal `gst_object_unref(parent)`) as the free site and `ges-validate.c:169` (`gst_object_get_parent(parent)`) as the UAF read, both inside `bin_element_added()`, both called from the real `gst_bin_add()` (`gstbin.c:1531`, `GST_TRACER_BIN_ADD_POST`) |
| Fix verification: same ASAN build, `ges-validate.c.o` rebuilt after the patch, harness re-linked, same race, 5/5 runs; plus the project's own `gst-editing-services` meson test suite before/after | Same clang ASAN build; `meson test -C build-asan-ges --suite gst-editing-services` (45 tests) run once with the original source, once with the patch applied | 5/5 clean runs (`RACE_COMPLETED_NO_CRASH`, exit 0) with the fix, vs. 5/5 crashes without it. Test suite: **identical 4 pass / 41 fail both before and after** - the 41 pre-existing failures are from this scoped build missing playback-capable plugins (good/bad disabled), unrelated to `tools/ges-validate.c` (not exercised by the library-level test suite at all - `ges-validate.c` is `ges-launch-1.0`-tool-only code) |

Only 2 of the soft-cap-3 escalating approaches were needed: code reading identified the exact
mechanism and why a naive single-threaded repro wouldn't show it, and the real 2-thread ASAN repro
confirmed it directly and repeatably on the first attempt (after iterating on the repro's own
synchronization - see "What's not done" for the debugging detour that was needed to get the timing
right).

## Status

**Confirmed**, reproduces 5/5 runs (clang debugoptimized build, `-Db_sanitize=address,undefined`,
project default `tracer_hooks` setting). Reliably reproducible on demand, not merely observed once
- the LD_PRELOAD synchronizer (see "Root cause") makes the underlying race land essentially
deterministically given the intended interleaving, rather than depending on incidental thread
scheduling luck.

**Fully reproducible**: **Yes**. `repro/harness.c` calls the real, unmodified
`ges_validate_activate()` (the same entry point `ges-launch-1.0 --validate` calls) and links in the
project's own compiled, unmodified `ges-validate.c.o`; the real `bin_element_added()` runs through
its real, documented trigger mechanism (`gst_tracing_register_hook()`'s `bin-add-post` hook, fired
from within the real, public `gst_bin_add()`). The race itself is driven by two genuine, valid,
unmodified GStreamer public API calls from two real pthreads (`gst_bin_add()` on the "walker"
thread, `gst_bin_remove()` on the "remover" thread) - nothing about the finding involves an
external peer/caller whose input could be faked, so `CONFIRMATION.md` section 11's second-participant
bar doesn't strictly apply here, but the repro satisfies its spirit anyway: the second "participant"
is a second real thread making a second real, unmodified API call, not hand-constructed bytes/state.
The one contrivance is `delay_shim.c`'s `usleep()` after the walker's targeted `gst_object_unref()`
call - a standard delay-injection technique to make an already-real, narrow race land reliably
instead of depending on incidental scheduling luck (see "Root cause"); it does not change what
`bin_element_added()` does or what data flows through it, and CONFIRMATION.md section 8 (the
concurrency bar) is satisfied with an explicit 5/5 reproduction count under this synchronization.

## Class

memory-safety (use-after-free, CWE-416)

## Root cause

`bin_element_added()`'s do/while loop walks `element`'s ancestor chain looking for a `GESTrack`:

```c
parent = gst_object_get_parent (GST_OBJECT (element));
do {
    if (GES_IS_TRACK (parent)) {
      ...
      gst_object_unref (parent);
      break;
    }

    gst_object_unref (parent);              /* ges-validate.c:168 */
    parent = gst_object_get_parent (parent); /* ges-validate.c:169 */
} while (parent);
```

Each `gst_object_unref(parent)` (line 168) drops only the walk's own temporary reference (the one
`gst_object_get_parent()` just took). Under normal, non-racing execution this is safe: every
non-top-level `GstObject` in a running pipeline is kept alive by an independent, separate reference
held by its *own* parent bin (GstObject containment refs the child from the parent, not the other
way around), so dropping the walk's temporary ref never drops the object to zero by itself. The bug
is a genuine but narrow **data race**: if a second thread concurrently removes `parent` from its own
parent bin (a real `gst_bin_remove()`) in the handful of instructions between line 168's
`gst_object_unref()` and line 169's `gst_object_get_parent()`, and no third reference is keeping
`parent` alive, then line 168's unref becomes the one that drops the refcount to zero, freeing
`parent` - and line 169 immediately dereferences that freed memory (`GST_IS_OBJECT()`'s
`g_type_check_instance_is_a()`, then `GST_OBJECT_LOCK()`, both read the freed `GTypeInstance`).

This is reachable in real GES/gst-validate usage because `gst_tracing_register_hook()`'s
`"bin-add-post"` hook fires for *every* `gst_bin_add()` call anywhere in the process, on whatever
thread makes that call - and a real GES pipeline legitimately has multiple such calls happening
concurrently across different streaming/demuxer threads (e.g. `decodebin`/`uridecodebin` autoplug
callbacks firing from different streaming threads for different tracks), any of which can overlap
with another thread's `gst_bin_remove()` on a shared ancestor bin during dynamic
reconfiguration/teardown.

One debugging detour worth recording: an intermediate version of the repro synchronized on
`gst_object_unref(mid_bin_pointer)` matched *only by pointer*, which reliably mis-fired on an
unrelated, legitimate `gst_object_unref()` call inside `_validate_bin_element_added()`
(`gst-validate-bin-monitor.c:337`, a temporary `gst_validate_monitor_get_target()` +
`gst_object_unref()` pair from the *monitor's own* `"element-added"` signal handling, which touches
the very same bin from the very same thread, just from different code, milliseconds earlier in the
same `gst_bin_add()` call) - consuming the synchronizer's one-shot trigger before
`bin_element_added()`'s own loop ever ran, so the race harness completed without crashing despite
being armed correctly. Filtering the trigger to *also* require the immediate caller (via `dladdr()`
on the return address from `backtrace()`) to be code linked into the main executable (i.e.
`bin_element_added()` itself, the only such caller this repro links in) resolved this and made the
repro land 5/5.

## Introduced by

b612d015cad0133b6d1df34e9634041cded740b4 "ges:validate: Check that no extra decoding happens"
(2016-07-26). `git blame` on `bin_element_added()`'s full body (lines 131-171) attributes every
line to this single commit; the function has never been touched since.

## The fix

Reorder the two calls so the walk's own reference on `parent` is only dropped *after* the next
parent has already been obtained (and ref'd) while `parent` is still guaranteed alive:

```c
{
  GstObject *next_parent = gst_object_get_parent (parent);

  gst_object_unref (parent);
  parent = next_parent;
}
```

This is sufficient because the only unsafe access was line 169's read of `parent` *after* it could
have been freed; calling `gst_object_get_parent(parent)` while the walk still holds its own
reference on `parent` makes that read safe regardless of what any other thread does concurrently -
`parent` cannot be freed while the walk holds a live reference on it. The `GES_IS_TRACK` branch
(unref-then-break, no further dereference of `parent`) was already safe and is untouched.

Per CONFIRMATION.md section 7, the fix was checked against the same race construction (not just the
one input/interleaving that first triggered it) - the identical harness/synchronization exercises
the same code path (any ancestor bin, any depth) each of the 5 verification runs, not a single
fixed byte sequence, so there is no narrower "just this one input" variant to additionally check
here (unlike a parser bug, the vulnerable state space is "which ancestor bin races a concurrent
removal", already covered by the repro's own construction).

- Project test suite (`gst-editing-services`, 45 tests): **4 pass / 41 fail before the fix, 4 pass
  / 41 fail after the fix** - identical failure set both times (the 41 failures are pre-existing,
  caused by this scoped build's good/bad plugins being disabled, not by `tools/ges-validate.c`,
  which the library-level test suite doesn't exercise at all).

## What's not done / open questions

- The do/while loop can walk through an arbitrary number of ancestor bins before finding a
  `GESTrack` (or exhausting the chain); the repro exercises a 2-level chain
  (`pipeline -> mid-bin -> leaf`) for the race's first (and only, in this repro) non-`GESTrack`
  ancestor. The fix is structural (applies uniformly to every loop iteration), so a deeper chain
  isn't expected to behave differently, but a deeper chain wasn't separately exercised this
  session.
- This is the only tracer-hook-style ancestor walk in `ges-validate.c` - `process_ges_meta()` and
  `max_compositor_factory_rank()` don't walk object hierarchies this way, so no sibling instance of
  this exact pattern was found elsewhere in the same file. A broader sweep of the codebase for the
  same "unref-then-dereference-same-pointer" idiom elsewhere in GStreamer/GES was not attempted
  this session (out of scope for a single-candidate investigation).
- A longer, dedicated concurrency-fuzzing campaign (e.g. via `fuzz-bug-repro`'s loom-style tooling,
  if adapted for C, or repeated TSan runs of a real multi-track `ges-launch-1.0 --validate`
  playback/edit scenario) was not attempted - this session's repro deliberately engineers the exact
  interleaving via a synchronizer rather than searching for it under real playback load, which
  would be a reasonable next step to additionally confirm real-world reachability without the
  synchronizer's assistance.
