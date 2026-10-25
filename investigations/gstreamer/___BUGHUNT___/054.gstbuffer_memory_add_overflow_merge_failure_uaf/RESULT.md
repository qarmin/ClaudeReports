# gstbuffer_memory_add_overflow_merge_failure_uaf

`subprojects/gstreamer/gst/gstbuffer.c` `_memory_add()`'s `GST_BUFFER_MEM_MAX` (16) overflow path
force-merges all 16 existing memory blocks into one via `_get_merged_memory()`, then
unconditionally sets the local `len = 1` regardless of whether the merge actually succeeded.
Flagged by `findings.toml` (`UAF_6`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Standalone C program using only public core-lib API (`gst_allocator_alloc`, `gst_buffer_append_memory`, `gst_buffer_get_size`, `gst_buffer_unref`): build a buffer with exactly 16 memories via the real default allocator, then temporarily override the *default allocator's* `mem_map` function pointer (a public, per-instance field on `GstAllocator`) to always fail - simulating a real allocation/mapping failure under memory pressure - and push a 17th memory in, then restore the real map function and inspect the buffer | Core-only meson build (`-Dbase=disabled -Dbad=disabled -Dgood=disabled` etc., plain debug), run under `valgrind --error-exitcode=99` | **Reproduces**: valgrind reports "Invalid read of size 8" in `gst_buffer_get_size() (gstbuffer.c:1573)` on memory "104 bytes inside a block of size 159 free'd" - the freed block traced directly to `_replace_memory (gstbuffer.c:342)` called from `_memory_add (gstbuffer.c:476)` called from the public `gst_buffer_insert_memory`/`gst_buffer_append_memory`. Buffer's memory count after the overflow reports 2 (should be 1), with slot 0 holding the dangling freed pointer. Further invalid reads/an assertion failure occur during `gst_buffer_unref()` (a subsequent double-unref of the same freed memory), consistent with `findings.toml`'s description. |

## Status

**Confirmed** - reproduces 3/3 runs, identical valgrind findings each time (7 invalid-read/invalid-free reports per run, at the same lines).

**Fully reproducible**: **Yes**. The repro drives the real, unmodified `_memory_add()`/`_replace_memory()`/`_get_merged_memory()` internals purely through the public `gst_buffer_append_memory()` API - no internal/static function is called directly. The only non-organic part is inducing the *map failure* itself: rather than constructing genuine system-level memory pressure (which would be non-deterministic and impractical to force reliably in a short session), the repro overrides the default allocator's public `mem_map` function pointer to return failure, which exercises exactly the same code path (`_actual_merged_memory()`'s `gst_memory_map(result, &dinfo, GST_MAP_WRITE)` failing) that a real allocation/mapping failure would trigger. This is a standard, reasonable way to test an error-handling path deterministically; the underlying `_memory_add()`/`_replace_memory()` logic exercised afterward is entirely real and unmodified.

## Class

memory-safety (use-after-free)

## Root cause

When a `GstBuffer` already holds `GST_BUFFER_MEM_MAX` (16) memory blocks and a 17th is added,
`_memory_add()` calls `_get_merged_memory(buffer, 0, len)` to collapse all 16 into a single
merged memory, then passes the result (which can legitimately be `NULL` - see
`_actual_merged_memory()`'s two failure paths: `gst_allocator_alloc()` returning `NULL`, or
`gst_memory_map()` failing on either the new merge-target memory or any one of the 16 source
memories) straight into `_replace_memory()`. `_replace_memory()` always unrefs all 16 old
memories regardless of whether the replacement (`mem`) is `NULL`; when it is `NULL`, it correctly
sets the buffer's tracked memory length to `len - length = 16 - 16 = 0` - but it does **not**
clear the now-stale pointer values remaining in the buffer's fixed 16-slot array. Back in
`_memory_add()`, the local variable `len` is then unconditionally set to `1`
(`/* we now have 1 single spanned buffer */ len = 1;`), **ignoring** whether the merge actually
produced anything. The subsequent insertion logic uses this bogus `len` to compute the insert
index and sets `GST_BUFFER_MEM_LEN(buffer) = len + 1 = 2`, leaving the buffer's own bookkeeping
claiming 2 valid memories: slot 0 still holds the pointer to the first of the 16 just-`unref`'d
(and, since nothing else held a reference, actually freed) memories, and slot 1 holds the
genuinely new 17th memory. Any later access to slot 0 (`gst_buffer_get_size()`,
`gst_buffer_map()`, `gst_buffer_unref()`, etc.) reads or unrefs already-freed memory.

## Introduced by

38b96e6c8c7438441ec81e2372a888ca1bd9ee85 "buffer: improve the buffer memory methods" (2012-03-30) -
this is when `_get_merged_memory()`/`_actual_merged_memory()` (with their `NULL`-on-failure
return paths) were introduced and wired into `_memory_add()`'s overflow branch. The unconditional
`len = 1;` itself predates this (4e32ac8463a, 2011-03-29, when the overflow-merge concept was
first added, at which point `_get_merged_memory()` had no failure path yet) - the bug is the
2012-03-30 commit not updating that pre-existing `len = 1` to account for the failure path it was
introducing.

## The fix

Check whether `_get_merged_memory()` actually returned a non-`NULL` merged memory before touching
the buffer's memory-length bookkeeping. On failure: log an error, `gst_memory_unref()` the new
memory that couldn't be inserted (since the caller transfers ownership of it, refusing the
insertion without unreffing would leak it), and return without modifying the buffer's existing 16
memories at all - i.e. the merge-failure case now safely refuses to grow the buffer past 16
memories instead of corrupting its bookkeeping.

Verified: rebuilt with the fix and re-ran the same repro (same simulated map-failure technique) -
valgrind now reports 0 errors, and the buffer correctly retains all 16 original memories
(`gst_buffer_get_size()` returns 256 = 16 × 16 bytes, matching the original content exactly, with
the attempted 17th memory cleanly refused and a `GST_CAT_ERROR` logged instead of silently
corrupting state).

- Project test suite: `meson test -C build-core-repro gstreamer:gst_gstbuffer
  gstreamer:gst_gstbufferlist gstreamer:gst_gstbufferpool` - 3/3 passed both before and after the
  fix (these tests do not exercise the 16-memory overflow path at all, which is exactly why this
  bug went unnoticed for over a decade).

## What's not done / open questions

- Only the allocation/mapping failure path of `_get_merged_memory()` was exercised (via the
  `gst_memory_map()` override). The other `NULL`-return path - `gst_allocator_alloc()` itself
  failing - was not separately tested, but shares the exact same `_replace_memory()` call site and
  fix, so it's covered by the same patch.
- Not checked whether any other caller of `_replace_memory()` elsewhere in `gstbuffer.c` has a
  similar "assume success" assumption about its `mem` argument possibly being `NULL` from a
  different source.
