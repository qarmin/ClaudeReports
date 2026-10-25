# hls: use-after-free in gst_hls_master_playlist_new_from_data() when an EXT-X-I-FRAME-STREAM-INF URI fails to resolve

`gst_hls_master_playlist_new_from_data()` (`subprojects/gst-plugins-bad/ext/hls/m3u8.c`) frees a
freshly-allocated `GstHLSVariantStream` in the middle of parsing an
`#EXT-X-I-FRAME-STREAM-INF:` line's attributes whenever its `URI` attribute fails to resolve
against `base_uri` (`uri_join()` returns `NULL`), but the surrounding attribute-parsing loop and
the code right after it keep using the now-freed pointer. Flagged by findings.toml UAF_11.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `gst_hls_master_playlist_new_from_data()` (~line 1628-1700) and `uri_join()` (~line 1192-1241) | Traced how `uri_join()` can return `NULL` (the "relative" branch returns `NULL` when `base_uri` has no `/` at all; the "absolute" branch returns `NULL` when `base_uri` has no `:`) and confirmed `gst_hls_variant_stream_unref()` is a real refcounted free (`refcount` starts at 1 in `gst_hls_variant_stream_new()`, so one `unref()` frees it) | Confirmed the mechanism by inspection: `stream` is freed at line 1669 while `data`/`a`/`v` (owned by the still-running `while (data && parse_attributes (&data, &a, &v))` loop) can still reference `stream` on a later attribute, and the post-loop `if (stream->iframe)` block (line ~1686) unconditionally dereferences `stream` again |
| Built a standalone repro (`repro/repro.c`, compiles the project's own unmodified `m3u8.c` and calls its real public entry point `gst_hls_master_playlist_new_from_data()`) against a debug-optimized clang ASAN/UBSAN build (`repro/run.sh`, links system `gstreamer-1.0` core via pkg-config - `m3u8.c` needs nothing beyond glib + stable core API) | 5 crafted master playlists, see `repro/repro.c`: (1) `URI` fails, `AUDIO` parsed afterward, (2) same but the "absolute" `uri_join()` branch, (3) `URI` is the *last* attribute (exercises the post-loop dereference directly), (4) two consecutive failing i-frame streams, (5) a valid i-frame + valid regular variant, as a regression check | **Reproduced immediately** on case (1): real AddressSanitizer heap-use-after-free, `READ of size 4` at `m3u8.c:1663` (inside `parse_attributes`' next iteration, reading `stream->iframe` for the `AUDIO` attribute), `freed by ... gst_hls_variant_stream_unref ... m3u8.c:1669` - see `output.txt` |

One code-reading pass plus an immediately successful runtime reproduction - well within the soft
cap of 3, no further escalation needed.

## Status

**Confirmed.** Deterministic: 5/5 runs of `repro/run.sh` against the unpatched checkout hit the
identical AddressSanitizer heap-use-after-free report (same crash site, same free site - only the
heap addresses/PIDs differ between runs, as expected under ASLR). After the fix, 3/3 runs of the
same binary complete cleanly (exit 0) on all 5 crafted cases, including the regression case.

**Fully reproducible**: **Yes.** `gst_hls_master_playlist_new_from_data(gchar *data, const gchar
*base_uri)` is the real public entry point `gsthlsdemux.c` calls with the raw bytes of a master
playlist it fetched over HTTP (`data`) and the playlist's own URL (`base_uri`) - the repro calls
it directly with a crafted playlist string and base URI, taking ownership of a heap-allocated
`data` exactly as the real caller does. This is a pure text-parsing finding (a malformed file
handed to the parser's real public entry point *is* the genuine external input - see
`CONFIRMATION.md` section 11's own carve-out for this shape), not a peer-to-peer protocol
interaction, so there is no second participant to fake here.

## Class

`memory-safety`

## Root cause

Inside the `#EXT-X-I-FRAME-STREAM-INF:`/`#EXT-X-STREAM-INF:` attribute-parsing branch, when the
`URI` attribute belongs to an i-frame stream (`stream->iframe` true), the code does:

```c
} else if (stream->iframe && g_str_equal (a, "URI")) {
  stream->uri = uri_join (base_uri, v);
  if (stream->uri != NULL) {
    ...
  } else {
    gst_hls_variant_stream_unref (stream);   /* <-- frees `stream` right here */
  }
}
```

`uri_join()` returns `NULL` whenever it can't resolve `v` against `base_uri` - e.g. `base_uri`
has no `/` at all and `v` is a relative, non-absolute URI (the "relative" branch, ~line
1211-1214), or `base_uri` has no `:` and `v` starts with `/` (the "absolute" branch, ~line
1225-1228). `stream` was just allocated by `gst_hls_variant_stream_new()` with `refcount = 1`, so
`gst_hls_variant_stream_unref(stream)` here actually frees it (`g_free (stream)` at the end of
`gst_hls_variant_stream_unref()`).

The enclosing `while (data && parse_attributes (&data, &a, &v))` loop, however, keeps iterating
over the same line's remaining attributes and keeps writing into `stream` for each one it
recognizes (`AUDIO`, `SUBTITLES`, `VIDEO`, `RESOLUTION`, ...) - all of which dereference the
already-freed `stream` pointer. Once the loop ends, `if (stream->iframe) { ... }` right after it
(~line 1686) dereferences `stream` a second time (and, if a `URI` attribute never appeared at
all - which can't reach this specific `NULL` path - or if it did resolve, calls
`find_variant_stream_by_uri(playlist->iframe_variants, stream->uri)`, again reading the freed
struct). Any attribute after a failing `URI`, or simply reaching the end of the attribute list,
turns this into a live heap-use-after-free, not just a theoretical one.

## Introduced by

4df6f1ee930af0ec7961ade57e21bd904f199fa9 "hlsdemux: move variant list handling over to new master
playlist code" (2016-08-03). Before this commit, the equivalent i-frame `URI` failure path did not
allocate a separate heap struct up front - it worked directly on caller-owned lists and, on a
`uri_join()`/unquote failure, simply logged a warning and `continue`d the loop without freeing
anything. This commit's refactor introduced the standalone, refcounted `GstHLSVariantStream`
(`gst_hls_variant_stream_new()`/`_unref()`) but paired the new failure path with an immediate
`gst_hls_variant_stream_unref(stream)` while `stream` was still the loop's live working variable -
turning "skip this attribute" into "free the struct out from under the still-running loop".

## The fix

`subprojects/gst-plugins-bad/ext/hls/m3u8.c`, `patch.diff`: stop freeing `stream` inside the
attribute-parsing loop when the i-frame `URI` fails to resolve - just leave `stream->uri` `NULL`
(already its default from `g_new0()`) and let parsing continue normally. Then, in the single
post-loop cleanup block (`if (stream->iframe) { ... }`), check for `stream->uri == NULL` first
and drop the stream there, in the one place that already owns the "is this stream worth keeping"
decision for every other i-frame-stream outcome (duplicate URI, successfully added). This mirrors
how the non-iframe path already handles an incomplete `pending_stream` - cleanup happens once,
after parsing has fully finished, never mid-loop.

Verified against `repro/run.sh`: heap-use-after-free (5/5) before, clean run (3/3) on all 5 cases
after - including the added regression case (a valid i-frame stream plus a valid regular variant,
to confirm the normal success path is unaffected) and boundary variations of the trigger itself
(the "absolute" `uri_join()` branch, `URI` as the last attribute on the line, two consecutive
failing streams back to back) per `CONFIRMATION.md` section 7.

- Project test suite: **N/A**. `subprojects/gst-plugins-bad/tests/check/elements/hlsdemux_m3u8.c`
  exists and does exercise `m3u8.c`, but it is unconditionally skipped by the project's own
  `subprojects/gst-plugins-bad/tests/check/meson.build` (`not hls_dep.found()`): `hls_dep` in
  `subprojects/gst-plugins-bad/ext/hls/meson.build` is declared with
  `dependency('', required : false)` and is never reassigned to a real, found dependency anywhere
  else in that file, so `hls_dep.found()` is always false and this test is always built-disabled -
  regardless of this patch, regardless of environment (confirmed by reading the meson.build, not
  an environment quirk: no `libcheck`/system `check` package is installed in this container
  either, which would independently block the whole check-based suite, but this specific test is
  dead code even where `check` is available). No i-frame-stream test case exists in that file at
  all, so this bug class also had zero coverage even when the suite is reachable.

## What's not done / open questions

- Only the i-frame (`EXT-X-I-FRAME-STREAM-INF`) `URI` attribute path was checked for this exact
  "free while the attribute loop is still running" pattern. The regular (non-iframe)
  `EXT-X-STREAM-INF` path resolves its own URI (`name`/`uri` at line ~1602-1608) *outside* the
  attribute-parsing loop, using a separate `pending_stream` variable that is only freed after the
  loop in existing, already-correct code (`pending_stream != NULL` check at line ~1695 and
  ~1737) - not affected by this same bug.
- The dead `hls_dep`/disabled `hlsdemux_m3u8.c` check-test noted above is a separate, pre-existing
  build-system defect worth its own look (see `NewSuspectsNoticed` in the session report) - fixing
  it was out of scope for this candidate, but it directly explains why this bug had no test
  coverage at all.
- Didn't fuzz `gst_hls_master_playlist_new_from_data()` beyond the 5 hand-crafted cases; a short
  harness feeding random attribute permutations against `base_uri` values with/without `/`/`:`
  could plausibly find sibling issues in the same function (e.g. in the non-iframe path's own
  `uri_join()` call at line 1603, which does check for `uri == NULL` correctly via `goto
  next_line` before touching `pending_stream` - looked safe on inspection, not independently
  fuzzed).
