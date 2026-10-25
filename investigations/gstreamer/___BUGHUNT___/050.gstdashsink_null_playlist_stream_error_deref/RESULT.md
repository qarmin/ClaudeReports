# gstdashsink_null_playlist_stream_error_deref

`gst-plugins-bad/ext/dash/gstdashsink.c` `gst_dash_sink_write_mpd_file()` (~line 940-982)
checks whether the `"get-playlist-stream"` signal returned a NULL `GOutputStream` and logs a
`GST_ELEMENT_ERROR`, but does not `return` - execution falls through into
`g_output_stream_write_all(file_stream, ...)` with `file_stream == NULL` anyway. This was
`findings.toml` `PANIC_12`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real repro: compile `ext/dash/*.c` (minus `gstplugin.c`/`gstdashdemux.c`) fresh with default visibility alongside a small harness, construct a real `GstDashSink`, connect a `"get-playlist-stream"` handler (`g_signal_connect_after`, needed - see below) that returns NULL, then drive the real `GstBinClass::handle_message` vfunc with a real `GST_MESSAGE_EOS` | Scoped meson build (core+base+bad, dash only), plain debug, clang | **Reproduces immediately, deterministic SIGSEGV**, 3/3 runs |

The first repro attempt used a plain `g_signal_connect()`, and the handler was confirmed to run
(via temporary debug `g_printerr()`s added to a working copy of `gstdashsink.c`, reverted before
finishing) but the observed `file_stream` was a real, valid, non-NULL pointer and no crash
occurred. Root cause of that false start: `"get-playlist-stream"` is `G_SIGNAL_RUN_LAST`, and its
class closure (`gst_dash_sink_get_playlist_stream`, the real default handler that opens an actual
file with `g_file_replace()`) is the *class* handler. For a RUN_LAST signal, plain (non-`_after`)
`g_signal_connect()` handlers run *before* the class closure; with no accumulator, the signal's
final return value is whatever the *last*-invoked handler returned - i.e. the class closure's
real, valid stream overwrote our NULL. Switching to `g_signal_connect_after()` makes our handler
the last one invoked, so its NULL becomes the signal's real return value - this is exactly what a
real application does when using this public API to reject a playlist write (e.g. a permission or
quota check).

## Status

**Confirmed** - reproduces 3/3 runs (fully deterministic, no memory corruption or timing
dependency; the crash is a straightforward NULL-pointer dereference).

**Fully reproducible**: **Yes**. The repro drives the real, unmodified
`gst_dash_sink_write_mpd_file()` (indirectly, via the real `handle_message` vfunc and a real
`GST_MESSAGE_EOS`) and the real, public `"get-playlist-stream"` signal exactly as a genuine
application would when supplying a custom output-stream handler (a documented, intended use of
this signal - see the property docs at `gstdashsink.c` line ~574). No internal/static function is
called directly.

## Class

memory-safety (NULL pointer dereference)

## Root cause

1. `gst_dash_sink_write_mpd_file()` emits `"get-playlist-stream"` and checks `if (!file_stream)`,
   but only logs a `GST_ELEMENT_ERROR` - it does not `return`.
2. Execution falls through to `g_output_stream_write_all(file_stream, mpd_content,
   bytes_to_write, NULL, NULL, &error)` with `file_stream == NULL`. `g_output_stream_write_all()`
   itself has a `g_return_val_if_fail (G_IS_OUTPUT_STREAM (stream), FALSE)` guard, which fires a
   `GLib-GIO-CRITICAL` and returns `FALSE` **without touching `error`** (still `NULL`).
3. Back in `gst_dash_sink_write_mpd_file()`, the `!write_ret` branch runs `GST_ERROR ("Failed to
   write mpd content: %s", error->message)` - dereferencing `error->message` on a NULL `GError`,
   crashing.

A NULL-returning `"get-playlist-stream"` handler is a legitimate, documented use of this signal
(rejecting a playlist write, e.g. on a permission or quota check) - no malicious/malformed input
is required, only a normal application customizing this API as intended.

## Introduced by

506bd90bf77 "dashsink: fix leaks, unref properly the file/dir streams" (Stéphane Cerveau,
2021-03-09) - this commit added both the `if (!file_stream) { GST_ELEMENT_ERROR(...); }` check
(without a `return`) and the later `g_object_unref (file_stream)` it was trying to guard against
leaking; the missing `return` means the NULL-check exists but doesn't actually prevent the
NULL from reaching `g_output_stream_write_all()` two statements later.

## The fix

Add `g_free (mpd_content); g_free (mpd_filepath); return;` right after the `GST_ELEMENT_ERROR()`
call in the `if (!file_stream)` branch, so a NULL playlist stream is treated as fatal for this
write attempt instead of falling through to use it anyway. See `patch.diff`.

Verified: rebuilt with the fix and re-ran the exact same repro - `gst_dash_sink_write_mpd_file()`
now returns cleanly (exit 0, no crash, no GIO-CRITICAL) instead of segfaulting.

- Project test suite: `meson test -C build-bad-devtools gst-plugins-bad:elements_dashsink` - 0/1
  passed both before and after the fix (pre-existing, unrelated failure: `test_dashsink_video_ts`
  aborts on `gst_dash_sink_add_splitmuxsink: assertion 'stream->splitmuxsink != NULL' failed`,
  reproducible on a clean checkout with this workspace's scoped build - looks like a plugin-scope/
  registry issue in this environment, not something this fix touches). The fix introduces no new
  failures and does not change this pre-existing result.

## What's not done / open questions

- The same missing-`return` shape does not affect `"get-fragment-stream"` (the sibling signal
  for per-fragment writes) since that path is inside `splitmuxsink`'s callback machinery, not
  `gst_dash_sink_write_mpd_file()` - not checked whether an analogous issue exists there.
- Not investigated whether `gst_dash_sink_send_mpd_update_msg()` (called unconditionally after the
  write, even on the write-failure path) has any assumptions that a successful write occurred.
