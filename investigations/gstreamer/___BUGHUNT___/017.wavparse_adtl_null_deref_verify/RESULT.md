# wavparse adtl NULL-deref fix (010db8a224) - verified complete

Found while scanning git log for `wavparse`/`audioparsers` (in-scope, ASAN-built plugins).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `010db8a224` "wavparse: Avoid NULL pointer dereference when parsing adtl in push mode" diff (`gst_wavparse_stream_headers()`, `GST_RIFF_LIST_adtl` case, `subprojects/gst-plugins-good/gst/wavparse/gstwavparse.c`) | Added `if (!gst_wavparse_peek_chunk (wav, &tag, &size)) goto exit;` before flushing 12 bytes and calling `gst_adapter_map (wav->adapter, data_size)` in streaming/push mode - without it, `gst_adapter_map()` can return `NULL` when fewer than `data_size` bytes are currently buffered (normal in push mode, where data arrives incrementally), and the following `gst_wavparse_adtl_chunk(wav, data, data_size)` call would dereference that `NULL` | Fix looked plausible; needed to confirm `peek_chunk`'s own size check actually covers the *total* bytes this code path goes on to consume (12 header bytes + `data_size` payload bytes), not just the 8-byte chunk header |
| Code reading: `gst_wavparse_peek_chunk()` / `gst_wavparse_peek_chunk_info()` (~line 666-723) | `peek_chunk_info` reads the chunk's 4-byte tag + 4-byte size fields (8 bytes) via a checked `gst_adapter_available() < 8` guard. `peek_chunk` then requires `gst_adapter_available() >= 8 + GST_ROUND_UP_2(*size)` before returning `TRUE`. For the `adtl` LIST case, `*size` (the whole LIST chunk's declared size, including its 4-byte list-type fourcc) relates to `data_size = size - 4` (the payload alone) by `size = data_size + 4`, so `8 + size = 12 + data_size` - exactly the total byte count this code path consumes (12 bytes flushed for header+listtype, then `data_size` bytes mapped) | **Confirms the fix is sufficient**: `peek_chunk` returning `TRUE` guarantees enough data is buffered for both the flush and the subsequent map, so `gst_adapter_map(wav->adapter, data_size)` can no longer return `NULL` on this path |

## Status

**Not reproducible** - no bug, already correctly fixed in current `HEAD`.

## Root cause

N/A (not a bug in current code) - see "What was tried" for the historical NULL-deref this commit
already fixed and why the fix's size arithmetic is sufficient.

## What's not done / open questions

- Did not check the sibling `pull`-mode branch (the `else` block a few lines below, using
  `gst_wavparse_pull_range_exact`) for an analogous issue - it doesn't use `gst_adapter_map` at
  all (pull mode fetches exactly the requested range via a different, blocking API), so the same
  bug class doesn't obviously apply there, but this wasn't independently verified in as much
  depth as the push-mode path this candidate focused on.
