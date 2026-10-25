# ges_title_clip_getters_null_deref

`ges_title_clip_get_text`, `ges_title_clip_get_font_desc`, `ges_title_clip_get_halignment`,
`ges_title_clip_get_valignment`, `ges_title_clip_get_text_color`,
`ges_title_clip_get_background_color`, `ges_title_clip_get_xpos`, `ges_title_clip_get_ypos` in
`subprojects/gst-editing-services/ges/ges-title-clip.c` (~line 531-749) all dereference
`self->priv->track_titles->data` without checking whether `track_titles` is `NULL`.
`track_titles` stays `NULL` until a `GESTitleSource` track element has been added as a child of
the clip (added lazily once the clip is placed into a timeline with a video track), so a freshly
created `GESTitleClip` from the public `ges_title_clip_new()` has `track_titles == NULL`. The
sibling `ges_title_clip_get_property()` (the modern GObject property path, same file, ~line
73-103) already guards this exact case by constructing a temporary `GESTitleSource` when
`priv->track_titles` is empty - the plain getters were never updated to match. Source:
`findings.toml` PANIC_8.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading (getters vs. `get_property`'s guard) | Compared the 8 getters against `ges_title_clip_get_property` in the same file | Confirmed the missing-guard hypothesis and matched it exactly to `findings.toml` PANIC_8 before writing any repro code |
| Standalone repro (real public API: `gst_init()` + `ges_init()` + `ges_title_clip_new()` + `ges_title_clip_get_text()`), clang ASAN+UBSAN debug build of a minimal `ges`-enabled meson tree (`-Dges=enabled -Dbase=enabled`, good/bad/ugly/libav disabled) | Built via `gst-env.py`'s uninstalled devenv + `pkg-config --cflags --libs gst-editing-services-1.0`; 5 repeat runs | SEGV, deterministic 5/5, exact predicted line (`ges-title-clip.c:541:70`, `self->priv->track_titles->data` inside `ges_title_clip_get_text`) |
| Fix verification against a fuller build (`-Dgood=enabled` plus `playback`/`typefind`/`pango`/`compositor`/`tools` explicitly enabled - needed for the fix's own fallback path, `ges_title_source_new()`, to fully resolve a real compositor/colorconvert/textrender element, not just avoid the NULL deref) | Same clang ASAN+UBSAN debug build, `meson test --suite gst-editing-services` (27 tests) run once before, once after the patch | No crash, 3/3 clean repro runs after the fix, all 8 getters return sane default values; test suite identical 13 pass/11 fail/3 skip in both runs, same 11 pre-existing unrelated failures |

Only 2 of the soft-cap-3 escalating approaches were needed: the root cause was unambiguous from
reading the code, and the first repro attempt reproduced deterministically. A third iteration was
spent widening the build's plugin scope (not a new "approach" in the escalation sense) after the
first fix-verification attempt hit unrelated missing-plugin errors from the deliberately narrow
`ges`-only build, not from the bug or the fix itself (see below).

## Status

**Confirmed**, reproduces 5/5 runs in the minimal build and 3/3 runs in the fuller build
(deterministic - `track_titles` is unconditionally `NULL` immediately after
`ges_title_clip_new()`, no randomness/timing involved).

**Fully reproducible**: **Yes**. `repro/repro.c` calls the real, unmodified, public GES API
(`gst_init()`, `ges_init()`, `ges_title_clip_new()`, then the getter) exactly as any application
using GES would - no timeline, no track, no pipeline, no second participant/peer to fake: the
"input" here is simply "call this public getter before adding the clip to a timeline", which is a
trivially reachable, entirely first-party call sequence. Per `CONFIRMATION.md` section 11, no
second-participant exception is needed.

## Class

memory-safety (NULL-pointer dereference / crash-on-use, CWE-476)

## Root cause

`self->priv->track_titles` is a `GSList *` populated only by `_child_added()` (same file, called
when a `GESTitleSource` track element is added as a child of the clip - i.e. once the clip is
placed in a timeline with a compatible video track). A `GESTitleClip` freshly constructed via
`ges_title_clip_new()` has not been added to any timeline yet, so `track_titles` is `NULL`.

Each of the 8 getters does:
```c
ges_timeline_element_get_child_properties (self->priv->track_titles->data, "text", &text, NULL);
```
`self->priv->track_titles` is `NULL`, so `self->priv->track_titles->data` reads the `data` member
of a `GSList` through a `NULL` pointer - `GSList.data` is the struct's first member (offset 0), so
this is a direct read of address `0x0`, a deterministic `SIGSEGV`.

`ges_title_clip_get_property()` (lines 73-103, the modern `GObject` property-get path used by
`g_object_get()`) already handles this exact situation:
```c
if (!priv->track_titles)
  child = tmpsrc = GES_TIMELINE_ELEMENT (ges_title_source_new ());
else
  child = priv->track_titles->data;
...
if (tmpsrc)
  g_object_unref (tmpsrc);
```
The 8 plain getters are the older, pre-`GObject`-property API (see "Introduced by" below) and were
never updated to use this same fallback when `child_properties`/`get_property` later grew it.

## Introduced by

190643508f "title: Deprecate all method related to child properties" (2014-10-29) is where these
getters took their current shape, switching from returning a plain private field
(`self->priv->text`, always safe) to `self->priv->track_titles->data` with no `NULL` check.
`ges_title_clip_get_property()`'s own `!priv->track_titles` guard (the pattern this fix reuses)
was present already by the time the file was moved into `subprojects/` in 2021, and the getters
were never brought in line with it.

## The fix

Applied the identical guard-and-fallback pattern `ges_title_clip_get_property()` already uses, to
each of the 8 getters: when `self->priv->track_titles` is `NULL`, construct a temporary
`GESTitleSource` via `ges_title_source_new()`, use it as the child to query, then unref it
afterwards.

Example (`ges_title_clip_get_text`, the other 7 getters follow the same pattern):
```c
const gchar *
ges_title_clip_get_text (GESTitleClip * self)
{
  gchar *text;
  GESTimeline *_locked_timeline;
  GESTimelineElement *child, *tmpsrc = NULL;

  g_return_val_if_fail (GES_IS_TITLE_CLIP (self), NULL);

  _locked_timeline = _ges_timeline_element_lock (GES_TIMELINE_ELEMENT (self));

  if (!self->priv->track_titles)
    child = tmpsrc = GES_TIMELINE_ELEMENT (ges_title_source_new ());
  else
    child = self->priv->track_titles->data;

  ges_timeline_element_get_child_properties (child, "text", &text, NULL);

  if (tmpsrc)
    g_object_unref (tmpsrc);

  _ges_timeline_element_unlock (GES_TIMELINE_ELEMENT (self), _locked_timeline);

  return text;
}
```

`patch.diff` (+84/-16 lines) applies this to all 8 getters.

Verified: `repro/repro.c` re-run against the patched+rebuilt tree no longer crashes - all 8
getters return their documented default values (empty text, `"Serif 36"` font, default
alignments, `0xFFFFFFFF` colors, `0.5`/`0.5` position) and the process exits 0, both 3/3 and
5/5-style repeat runs clean.

- Project test suite: `meson test --suite gst-editing-services` (27 tests): **13 pass / 11 fail /
  3 skip before -> 13 pass / 11 fail / 3 skip after**, identical failing-test set both times
  (`ges_asset`, `nle_simple`, `nle_tempochange`, `nle_nleoperation`, `nle_complex`,
  `nle_nlecomposition`, `ges_uriclip`, `ges_timelineedition`, `ges_group`, `ges_clip`,
  `ges_project`) - all pre-existing and caused by this sandbox's deliberately narrow build
  (missing `gst-plugins-bad`, `opus`/`vorbis`/network-downloadable codec subprojects that those
  tests' own fixture media need), unrelated to this patch. `ges_titles` (the dedicated
  `GESTitleClip` unit test) passes both before and after. No new failures, no newly-fixed
  failures.

## What's not done / open questions

- The narrow initial `ges`-only build (no `good`, no `playback`/`pango`/`compositor`) was enough
  to demonstrate the crash itself, but *not* enough to demonstrate the fix working end-to-end: the
  fix's fallback (`ges_title_source_new()`, the same one `get_property` already uses) needs a real
  compositor/colorconvert/scaler/textrender element to fully build a `GESTitleSource`'s internal
  element tree, none of which exist in that narrow build. This is an artifact of this session's
  minimal build scope, not evidence against the fix - `ges_title_clip_get_property()` (the
  already-shipped code this fix mirrors) would hit the exact same missing-element errors in that
  same narrow build if driven through `g_object_get()`. Verification was redone against a fuller
  build (`good` plus `playback`/`pango`/`compositor`/`tools` explicitly enabled) where both the
  fix and (implicitly) `get_property`'s existing fallback fully succeed.
- Not checked: whether any *other* GES clip/source subclass has the same "old getter API predates
  a later-added `get_property` NULL-guard" asymmetry. `ges-title-clip.c` was the only file
  investigated this session (per `findings.toml` PANIC_8's own scope).
