# hls_m3u8_generate_media_seqnums_null_deref

`generate_media_seqnums()` in `subprojects/gst-plugins-bad/ext/hls/m3u8.c` (~line 422-481) is
missing the `if (!self->files) return ...;` guard that its sibling `check_media_seqnums()`
(right above it in the same file) already has, so a live media-playlist update whose new
playlist has zero media segments and no `#EXT-X-MEDIA-SEQUENCE` tag dereferences a NULL
`GstM3U8MediaFile *`. Source: findings.toml PANIC_2.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + code reading | Read `check_media_seqnums()`/`generate_media_seqnums()` side by side; confirmed the asymmetric guard by reading the commit history of both functions | Confirmed the missing-guard hypothesis before writing any repro code |
| Standalone repro against a clang ASAN+UBSAN core-only build, driving the real `gst_m3u8_update()` public entry point twice (first playlist populates `previous_files`, second playlist has no segments and no `#EXT-X-MEDIA-SEQUENCE`) | `___BUGHUNT___/tools/build_asan_fuzz.sh`-produced build (also independently re-verified against a freshly-built `build-asan-core`-style tree through `repro/run.sh`), 5 repeat runs | SEGV/UBSan null-member-access, deterministic 5/5 |
| Fix verification: re-run the same repro against a patched copy of `m3u8.c`, 5 repeat runs; plus a full `gst-plugins-bad` `meson test` run before and after the patch | Same ASAN core-only compile; `meson test --suite gst-plugins-bad` (110 tests) run once before, once after | No crash, 5/5 clean after the fix; test suite identical 100 pass/5 fail/5 skip in both runs, same 5 pre-existing unrelated failures (`elements_dashsink`, `elements_rtpsrc`, `elements_rtpsink`, `elements_srtp`, `elements_vmaf`) |

Only 2 of the soft-cap-3 escalating approaches were needed: the root cause was unambiguous from
reading the code and history, and the first repro attempt reproduced deterministically.

## Status

**Confirmed**, reproduces 5/5 runs (deterministic - `self->files == NULL` is set unconditionally
whenever the new playlist text contains no `#EXTINF`/URI lines, no randomness/timing involved).

**Fully reproducible**: **Yes**. `repro/repro.c` calls the real, unmodified `gst_m3u8_update()`
(the exact function `gsthlsdemux.c` calls with the text of a playlist downloaded over HTTP) with
hand-crafted-but-well-formed M3U8 playlist text - the genuine external input format for this
parser (a downloaded playlist file), not a stand-in for some other participant's behavior. Per
`CONFIRMATION.md` section 11, a malformed/edge-case file handed to a parser's real public entry
point is the genuine external input on its own; no second network participant needs to be faked
or actually run here.

## Class

memory-safety (NULL-pointer dereference / crash-on-input, `CWE-476`)

## Root cause

`gst_m3u8_update()` reparses a downloaded playlist's text into a fresh `self->files` list, then
(if there was a previous playlist, i.e. this is a live-playlist update, not the first fetch) either
calls `check_media_seqnums()` (if the new playlist declared `#EXT-X-MEDIA-SEQUENCE`) or
`generate_media_seqnums()` (if it didn't), *before* the later `if (self->files == NULL)` check
that would otherwise reject an empty playlist (m3u8.c line ~813-817). If the new playlist text has
no media-segment lines at all, `self->files` stays `NULL` after parsing.

`check_media_seqnums()` explicitly handles this:
```c
if (!self->files) {
  /* Empty playlists are trivially consistent */
  return TRUE;
}
```
`generate_media_seqnums()` has no equivalent check. Its "no URI match" loop
```c
for (l = self->files; l; l = l->next) { ... f1 = l->data; ... if (match) break; }
```
never executes when `self->files` is `NULL`, so `l` stays `NULL` and `f2` stays at its `NULL`
initializer. The function then falls into the `else` branch (meant for "reached the end of the new
playlist without finding a matching URI"):
```c
} else {
  mediasequence = f2->sequence + 1;   /* f2 is NULL here -> NULL deref */
  l = self->files;
}
```
which dereferences `f2->sequence` on a NULL pointer.

## Introduced by

f80ad048e6f "m3u8: Split gst_m3u8_update_check_consistent_media_seqnums" (2017-09-15) is where
`generate_media_seqnums()` took its current standalone shape (split out of a single combined
`gst_m3u8_update_check_consistent_media_seqnums()` function) without this guard. However, the
underlying missing-guard logic already existed identically in *both* branches of that original
combined function, introduced by 075ceffd9b "hlsdemux: Detect media sequence number
inconsistencies and fail" (2017-01-18) - so the bug itself predates the split. The asymmetry
between the two now-separate functions was created by 025633b162 "m3u8: Ignore empty playlists in
check_media_seqnums" (2017-09-27), which added the guard to `check_media_seqnums()` only,
apparently overlooking that its `generate_media_seqnums()` sibling (the `!have_mediasequence`
branch of the same original function) needed the identical fix.

## The fix

Add the same guard `check_media_seqnums()` already has, adapted for `generate_media_seqnums()`'s
`void` return type (an empty new playlist has nothing to assign sequence numbers to, so the
function can simply return early):

```c
  g_return_if_fail (previous_files);

+ if (!self->files) {
+   /* Empty playlists are trivially consistent, and there's nothing to
+    * assign sequence numbers to */
+   return;
+ }
+
  /* Find first case of same URI in new playlist.
   * From there on we can linearly step ahead */
```

Verified: `repro/repro.c` re-run against the patched source no longer crashes (5/5 clean runs);
`gst_m3u8_update()`'s second call now falls through to the pre-existing
`if (self->files == NULL) { GST_ERROR(...); return FALSE; }` check right after, which is the
already-correct behavior `check_media_seqnums()`'s sibling call site gets for the same
empty-playlist case.

- Project test suite: `meson test --suite gst-plugins-bad` (110 tests, core-only-scope build with
  `hls` unit test force-enabled - see "What's not done" below): **100 pass / 5 fail / 5 skip
  before -> 100 pass / 5 fail / 5 skip after**, identical failing-test set both times
  (`elements_dashsink`, `elements_rtpsrc`, `elements_rtpsink`, `elements_srtp`, `elements_vmaf` -
  all pre-existing and unrelated to HLS/m3u8, consistent with this sandbox's environment: missing
  optional deps/network access for those elements' own tests). No new failures, no newly-fixed
  failures beyond the target bug itself (which had no existing automated test coverage - see
  below).

## What's not done / open questions

- The project's own `ext/hls`'s check test (`elements_hlsdemux_m3u8`,
  `subprojects/gst-plugins-bad/tests/check/elements/hlsdemux_m3u8.c`) never actually ran in this
  container's build before this session: `subprojects/gst-plugins-bad/ext/hls/meson.build` sets
  `hls_dep = declare_dependency(include_directories: include_directories('.'))` (a real,
  found dependency) and then, a few lines later, unconditionally overwrites it with
  `hls_dep = dependency('', required: false)` (a comment above it literally says
  `# used for unit test`), before the plugin's own enable/disable logic runs - so
  `tests/check/meson.build`'s `not hls_dep.found()` skip condition is always true and this test
  is never configured, regardless of build options. Confirmed via `meson-info/intro-tests.json`
  containing no `hls`-named test in a build with `-Dhls=enabled` (`gst-plugins-bad` still built
  `libgsthls.so` successfully). This looks like an independent, second bug (a permanently-disabled
  unit test) - noted in `IDEAS.md`, not investigated further as part of this candidate. The
  before/after test-suite run above only got to exercise this test at all by locally commenting
  out that one reset line for the duration of this session's verification (not part of
  `patch.diff` - only `subprojects/gst-plugins-bad/ext/hls/m3u8.c`'s 6-line guard is).
  Even with the test force-enabled, its own fixture playlists (`LIVE_PLAYLIST`,
  `LIVE_ROTATED_PLAYLIST`, `ON_DEMAND_PLAYLIST`, etc.) never happen to hit this exact empty-new-
  playlist edge case, so it stayed green both before and after this candidate's fix regardless of
  the `hls_dep` issue - it does not itself demonstrate the bug either way.
- `gst-plugins-good/ext/adaptivedemux2/hls/m3u8.c` (the newer adaptivedemux2-based HLS
  implementation, ~3500 lines) was checked for the same function pair: it does not have
  `generate_media_seqnums()`/`check_media_seqnums()` at all - it computes/tracks
  `media_sequence`/`discont_sequence` with substantially different logic - so it is not a
  duplicate of this exact bug. Not otherwise audited for its own NULL-deref-shaped issues in this
  session.
