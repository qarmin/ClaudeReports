# gst_event_new_sink_message: dangling `name` pointer stored inside the event's GstStructure

`gst_event_new_sink_message()` (`subprojects/gstreamer/gst/gstevent.c`, ~line 1675-1691) builds
the event's `GstStructure` via `gst_structure_new_static_str (name, "message",
GST_TYPE_MESSAGE, msg, NULL)`, where `name` is a plain `const gchar *` caller parameter, not a
literal. `gst_structure_new_static_str()`'s own doc comment says its `name` argument "need[s] to
be valid for the remaining lifetime of the process, e.g. have to be a static string" - but
`gst_event_new_sink_message()`'s own doc comment says nothing of the kind about its `name`
parameter, and a real in-tree caller
(`subprojects/gst-plugins-bad/sys/ipcpipeline/gstipcpipelinecomm.c`,
`gst_ipc_pipeline_comm_read_bus_message()`) passes a pointer straight into a `GstAdapter`-mapped
buffer that gets unmapped/flushed right after the call returns. Flagged by `findings.toml`
(`UAF_17`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `gst_structure_new_static_str()` -> `gst_id_str_set_static_str()` -> `_gst_id_str_set_static_str_with_len_inline()` in `gstidstr-private.h` | Confirmed the exact mechanism: for a name `len <= 15` it's copied inline into a 16-byte fixed buffer (safe); for `len > 15` it stores `sp->s.pointer_string.s = (gchar *) value` verbatim, type `2` ("statically allocated, needs no freeing") - the raw caller pointer, kept forever, never duplicated | Confirmed the exact bug shape described in `findings.toml` |
| Code reading: the one real in-tree caller, `gst_ipc_pipeline_comm_read_bus_message()` (`gstipcpipelinecomm.c` ~line 850-913) | `name = (const char *) payload;` points into `gst_adapter_map()`'s mapped region; `gst_event_new_sink_message (name, message)` is called, then (at the `done:` label) `gst_adapter_unmap()` + `gst_adapter_flush (comm->adapter, mapped_size)` run unconditionally right after | Confirms a real caller hands a transient, buffer-backed pointer as `name` today - not just a hypothetical misuse |
| Debug-assertions/ASAN+UBSAN build (clang, core-only `subprojects/gstreamer`, `-fsanitize=address,undefined`) + a standalone C program using only the real public API (`gst_event_new_sink_message()`, `gst_event_has_name()`) with a heap-allocated (`g_strdup`) name longer than 15 bytes, freed immediately after the call, then read back via `gst_event_has_name()` | 5/5 runs: deterministic `AddressSanitizer: heap-use-after-free` (`READ of size 1` in `strcmp`, via `_gst_id_str_is_equal_to_str_inline` <- `gst_structure_has_name` <- `gst_event_has_name`) - see `output.txt` | **Reproduced**, deterministically, on the first attempt |

One code-reading pass plus one runtime reproduction that reproduced deterministically on the
first try - well within the soft cap of 3, no further escalation needed.

## Status

**Confirmed.** Deterministic, 5/5 runs (debug-optimized clang ASAN+UBSAN build, no fixed seed
needed - the bug doesn't depend on memory layout, only on the freed allocation being reused/
poisoned, which ASAN's redzone/quarantine always detects for a plain `g_free()` immediately
followed by a read). After the fix: 3/3 clean runs (exit 0, no ASAN report).

**Fully reproducible**: **Yes.** The repro drives the real, public, documented entry points
(`gst_event_new_sink_message()` to create the event, `gst_event_has_name()` to read its name back
- the same function used internally whenever anything checks a custom event's name) against a
real ASAN-instrumented build of gstreamer's own core library - no hand-built stand-in for
`GstStructure`/`GstIdStr` internals. This finding is not fundamentally a "how does participant A
react to bytes participant B put on the wire" scenario in the sense `CONFIRMATION.md` section 11
is aimed at (a depayloader mishandling a crafted RTP/IPC packet): the vulnerable mechanism is
`gst_event_new_sink_message()`'s own contract with *any* caller that passes a transient string,
reachable in a single process with no peer involved at all - passing a `g_strdup()`'d name and
freeing it before the event is next touched is completely ordinary, unsurprising API usage given
that nothing in `gst_event_new_sink_message()`'s own doc comment warns against it (unlike
`gst_structure_new_static_str()`'s own doc comment, which does). The real ipcpipeline caller
(`gstipcpipelinecomm.c`) is cited above only as evidence that a transient-buffer `name` is not a
contrived scenario - not as something this repro needed to simulate a second participant for.

## Class

`memory-safety`

## Root cause

`_gst_id_str_set_static_str_with_len_inline()` (`gstidstr-private.h`) stores any `name` longer
than 15 bytes as a raw, unduplicated pointer (`sp->s.pointer_string.s = (gchar *) value`, type
`2`, "statically allocated, needs no freeing" - i.e. never freed, and never copied either).
`gst_structure_new_static_str()` calls straight through to this for its `name` argument, and its
own doc comment correctly requires `name` to be "valid for the remaining lifetime of the
process". `gst_event_new_sink_message()` calls `gst_structure_new_static_str (name, ...)` with its
own `name` parameter - a plain, ordinary `const gchar *` with no such requirement stated in its
own doc comment (contrast `gst_event_new_stream_start()`'s and `gst_event_new_toc_select()`'s
sibling functions in the same file, which pass caller strings as plain *field values*, not as the
structure's *name*, so this specific hazard doesn't apply to them). Any caller that builds `name`
dynamically and doesn't keep the buffer alive for the rest of the process's lifetime - which
nothing in the API's own documentation asks for - creates an event whose internal structure name
is a dangling pointer the moment that buffer is freed or overwritten. Any later access to the
event's name (`gst_event_has_name()`, `gst_structure_get_name()`, `GST_EVENT_TYPE`-style debug
logging that prints custom event names) then reads/compares through that dangling pointer.

## Introduced by

0c1611d31d95a3bc7bb644e8b53aba01ed9df8dc "common: Stop using GQuark-based GstStructure name API"
(2024-09-26). Before this commit, `gst_event_new_sink_message()` used
`gst_structure_new_id (g_quark_from_string (name), ...)` - `g_quark_from_string()` interns the
string into GLib's own quark table, which copies it internally, so a transient `name` was safe
under the pre-commit code. This commit's mechanical replacement of the quark-based API with
`gst_structure_new_static_str()` (part of a broad, otherwise-correct migration touching nearly
every `gst_structure_new_id()`/`gst_structure_new()` call site in `gstevent.c`, almost all of
which pass a `name` that genuinely is a string literal) missed that `name` here is a real
parameter, not a literal, turning a previously-safe interning call into an unsafe raw-pointer
store for this one call site.

## The fix

One-line change in `gst_event_new_sink_message()`: use `gst_structure_new()` instead of
`gst_structure_new_static_str()`. `gst_structure_new()` (via `gst_structure_new_valist()` ->
`gst_structure_new_str_empty_with_size()` -> `gst_id_str_set()`) duplicates `name` into a
heap-allocated (or inline, for <=15 bytes) owned copy, exactly like the pre-regression
`g_quark_from_string()` path did, at the cost of one small allocation per sink-message event
(negligible - this is not a hot path) instead of storing the caller's transient pointer.

Re-running the standalone repro (`output.txt` before; a fresh 3/3-clean run after) confirms:
`AddressSanitizer: heap-use-after-free` before the fix, no ASAN report and exit code 0 after.

- Project test suite: **core `gstreamer` suite (`meson test -C build-asan-core --suite
  gstreamer`), same scoped ASAN+UBSAN core-only build used for the repro: 105 passed / 8 failed /
  1 skipped, identical before and after the fix.** The 8 pre-existing failures
  (`gst_gsttracerdeinit`, `gst_gstbuffer`, `gst_gstvecdeque`, `libs_bytewriter`,
  `libs_bytewriter_noinline`, `gst_gstutils`, `libs_bytereader`, `libs_bytereader_noinline`,
  `libs_queuearray`) are unrelated to `gstevent.c`/`gststructure.c` and reproduce identically with
  the patch reverted, confirming they're pre-existing artifacts of this scoped ASAN/UBSAN build
  environment, not caused by this change. `gst_gstevent`'s own test (`gstreamer:gst_gstevent`,
  which exercises `gst_event_new_sink_message()` directly via `send_custom_events`) passes both
  before and after.

## What's not done / open questions

- Only `gst_event_new_sink_message()` was checked for this specific "pass a transient string as a
  structure *name* through the `_static_str` API" shape. A broader sweep of every other
  `gst_structure_new_static_str()`/`gst_structure_new_static_str_empty()`/
  `gst_event_new_custom()`-adjacent call site in the tree that takes a non-literal `name` (as
  opposed to a hardcoded string, which all the other call sites inside `gstevent.c` itself use)
  was not performed this session - `gst_event_new_sink_message()` is the only one this session
  found that takes an unqualified `const gchar * name` parameter and forwards it as a structure
  *name* rather than a field value.
- Did not build or run the real two-process `ipcpipeline` element pair (`ipcpipelinesink`/
  `ipcpipelinesrc`) to reproduce the exact real-world trigger path end-to-end over an actual pipe;
  the standalone single-process repro above is sufficient to confirm the bug because the
  vulnerable contract (`gst_event_new_sink_message()` storing a non-static `name` unduplicated) is
  entirely inside core and reachable by any single caller, not something only visible through the
  ipcpipeline peer. A maintainer wanting an ipcpipeline-specific end-to-end demonstration would
  additionally need to confirm `ipcpipelinesrc` actually reads the resulting event's name back out
  after the underlying adapter buffer is reused - not attempted here.
- Did not check whether the `message` field argument (also passed through the same
  `gst_structure_new_static_str()` varargs list) has a matching hazard - it doesn't: `"message"`
  is a literal, and `GST_TYPE_MESSAGE` field values are handled by `gst_structure_id_set_valist()`
  (boxed value dup/ref semantics), unrelated to the `_static_str` name-storage mechanism this
  finding is about.
