# ges_track_remove_element_uaf

`remove_element_internal()` in `subprojects/gst-editing-services/ges/ges-track.c` (~line
1234-1257) looks up a `GESTrackElement`'s `GSequenceIter` in `priv->trackelements_iter` and calls
`g_sequence_remove(it)` on it, but never calls `g_hash_table_remove()` on that hash table on the
success path - leaving a stale entry that still points at the now-freed `GSequenceIter`. A second
`ges_track_remove_element()`/`_full()` call on the same still-alive `GESTrackElement` looks that
entry up again and calls `g_sequence_remove()` on the freed iterator. Source: `findings.toml`
UAF_8.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading (`remove_element_internal` vs. its own success/failure branches) | Read `ges-track.c` around lines 1234-1257 | Confirmed the missing `g_hash_table_remove()` on the `TRUE` branch, matching `findings.toml` UAF_8 exactly, before writing any repro code |
| Standalone repro (real public API: `gst_init()` + `ges_init()` + `ges_audio_track_new()` + `ges_effect_new("volume")` + `ges_track_add_element()` + `ges_track_remove_element()` called twice on the same, still-referenced element), clang ASAN debug build of a `ges`-enabled meson tree (`-Dges=enabled`, good/bad disabled, whole tree built with `-Db_sanitize=address,undefined`) | Built via `gst-env.py`'s uninstalled devenv + `pkg-config --cflags --libs gst-editing-services-1.0`; run under `gdb` once for a clean symbolized backtrace, then 5 repeat runs under plain ASAN | SEGV / ASAN "nested bug, aborting" every run; gdb backtrace pins the crash to exactly `ges-track.c:1244` (`g_sequence_remove (it);`) inside `remove_element_internal`, called from `ges_track_remove_element_full` |
| Fix verification (same ASAN build, rebuilt after the 1-line-of-logic patch), plus a wider bug-class check (3rd removal, then a genuine re-add/re-remove cycle on the same element) and the project's `gst-editing-services` meson test suite before/after | Same clang ASAN build; 3 repeat runs of the repro; `meson test --suite gst-editing-services` (27 tests) run once before, once after | First fix attempt (just adding the missing `g_hash_table_remove()`) turned the SEGV into a **different**, still-real UAF at process teardown (`ges_track_dispose()` iterating a stale sequence entry) - see "The fix" below for why. The final fix (early-return when the element isn't currently tracked) reproduces cleanly 3/3 with no crash; 3rd-removal/re-add/re-remove all behave correctly; test suite identical 4 pass/20 fail/3 skip in both runs (pre-existing failures from this workspace's narrow good/bad-disabled build, `ges_track` itself passes 1/1 both times) |

Only 2 of the soft-cap-3 escalating approaches were needed to *find* the crash (code reading +
first ASAN repro); a 3rd round of digging (still within the same "fix verification" step, not a
new escalation) was needed because the first candidate fix only closed the originally-described
symptom and exposed a second, related UAF from the same root design (see "The fix").

## Status

**Confirmed**, reproduces 5/5 runs (SEGV / ASAN "nested bug, aborting", clang debugoptimized build
with `-Db_sanitize=address,undefined`) before the fix; 3/3 clean runs after. Deterministic - no
randomness/timing involved, the freed-iterator reuse happens on the very next call.

**Fully reproducible**: **Yes**. `repro/repro.c` calls the real, unmodified, public GES API
(`gst_init()`, `ges_init()`, `ges_audio_track_new()`, `ges_effect_new()`,
`ges_track_add_element()`, `ges_track_remove_element()`) exactly as any application directly
managing track elements would - no second participant/peer to fake: the "input" is simply "call
the public remove function twice on the same element", a first-party call sequence with no
network/IPC boundary involved. Per `CONFIRMATION.md` section 11, no second-participant exception is
needed. Also verified against a genuinely fresh clone via this candidate's own `justfile`
(`just test`, `repo_url` overridden to HTTPS since this sandbox has no outbound SSH) - see
`output.txt`.

## Class

memory-safety (use-after-free, CWE-416)

## Root cause

`remove_element_internal()`:

```c
it = g_hash_table_lookup (priv->trackelements_iter, object);
g_sequence_remove (it);

if (remove_object_internal (track, object, emit, error) == TRUE) {
  ges_timeline_element_set_timeline (GES_TIMELINE_ELEMENT (object), NULL);
  return TRUE;
}

g_hash_table_insert (track->priv->trackelements_iter, object,
    g_sequence_insert_sorted (track->priv->trackelements_by_start, object, ...));
return FALSE;
```

On the first, successful call: `g_sequence_remove(it)` removes the object from
`trackelements_by_start` and frees the `GSequenceIter` `it` itself (per GLib's own
`g_sequence_remove()` contract - the iterator is invalidated/freed by the call). The function then
returns `TRUE` without ever removing the corresponding `object -> it` entry from
`priv->trackelements_iter`, so that hash table is left holding a dangling pointer to freed memory.

A second call to `ges_track_remove_element()`/`_full()` on the *same*, still-alive `object` (the
application holds its own reference, or the element is still reachable through anything other than
the track itself) does `it = g_hash_table_lookup(...)`, gets back the same freed pointer, and calls
`g_sequence_remove(it)` on it directly - a use-after-free of the `GSequenceIter`/its underlying
`GSequenceNode`, which crashes (observed as a jump to an invalid PC, `0x1`, reading through
corrupted freed-then-reused memory interpreted as sequence-node pointers).

## Introduced by

ed9dcddef47e1f974f94da3b3dd7d2963b45bbbb "track: Make sure that new gaps are filled before
removing the old ones" (2013-07-01, `ges/ges-track.c` pre-2021 monorepo-subproject-move path).
Before this commit, `ges_track_remove_element()`'s equivalent logic called
`remove_object_internal()` *first* and only touched the `GSequence`/hash table if that succeeded;
`remove_object_internal()` itself already fails fast (`ges_track_element_get_track(object) !=
track`) for an element no longer belonging to the track, so a second call on an already-removed
element was a harmless no-op that never touched `g_sequence_remove()`/the hash table again. This
commit reordered the code to call `g_sequence_remove(it)` *unconditionally, before* checking
whether the removal will actually succeed (to let a later gap-recalculation step see the
already-shrunk sequence), and added the "reinsert on failure" rollback branch - but the
already-existing gap (the hash table entry was never removed even before this commit) became
reachable for the first time here, since after this reordering `g_sequence_remove()` runs
unconditionally on every call, including a second call on an already-removed element.

## The fix

Two related issues, fixed together in one change to `remove_element_internal()`:

1. Guard against operating on an element that isn't currently tracked: if `g_hash_table_lookup()`
   returns `NULL` (the element was already removed, or was never added), return `FALSE`
   immediately instead of calling `g_sequence_remove(NULL)` (which trips a `GLib-CRITICAL`
   assertion, not a crash, but is still wrong) or - before this guard existed - reusing a stale
   freed iterator.
2. Remove the hash table entry (`g_hash_table_remove()`) at the same point the sequence entry is
   removed, on the success path, so the hash table and the sequence stay in sync.

```c
it = g_hash_table_lookup (priv->trackelements_iter, object);
if (it == NULL) {
  GST_DEBUG_OBJECT (track, "%" GST_PTR_FORMAT " is not in the track", object);
  return FALSE;
}
g_sequence_remove (it);
g_hash_table_remove (priv->trackelements_iter, object);

if (remove_object_internal (track, object, emit, error) == TRUE) {
  ges_timeline_element_set_timeline (GES_TIMELINE_ELEMENT (object), NULL);
  return TRUE;
}

g_hash_table_insert (track->priv->trackelements_iter, object,
    g_sequence_insert_sorted (track->priv->trackelements_by_start, object, ...));
return FALSE;
```

Why item 1 is required in addition to item 2 (not just a defensive extra): fixing only item 2
(add `g_hash_table_remove()` on the `TRUE` branch, leave the lookup/`g_sequence_remove()` order
unchanged) does stop the original SEGV - the second call's `g_hash_table_lookup()` now correctly
returns `NULL`, and `g_sequence_remove(NULL)` merely trips a `GLib-CRITICAL` assertion and no-ops.
But `remove_element_internal()` then falls through to `remove_object_internal()`, which returns
`FALSE` (the object no longer belongs to this track), and the existing "rollback on failure" branch
unconditionally **reinserts** `object` into both `trackelements_by_start` and
`trackelements_iter` - even though the object does not actually belong to the track anymore and the
track holds no reference on it (its one implicit ref was already dropped by the first, successful
removal). This leaves the sequence/hash table holding a non-owning dangling entry once the caller's
own reference is later dropped, which then crashes *again*, differently, inside
`ges_track_dispose()` (iterating the now-corrupted sequence) once the track itself is disposed -
confirmed by running the item-2-only fix through the same repro (see `output.txt`'s "What was
tried" row). Returning early per item 1, before ever reaching the reinsert branch, avoids this
second failure mode entirely and keeps the "reinsert on failure" rollback correctly scoped to its
original intent: genuine, still-in-the-track removal failures (e.g. the `nleobject` failing to
detach from the composition), not "already not in this track".

Verified: `repro/repro.c` re-run against the patched tree no longer crashes for a 2nd, 3rd, or Nth
removal of the same element, and a legitimate re-add-then-remove cycle on the same element
afterwards still works correctly (matches CONFIRMATION.md section 7 - checked beyond just the one
originally-triggering input).

- Project test suite: `meson test --suite gst-editing-services` (27 tests): **4 pass / 20 fail / 3
  skip before -> 4 pass / 20 fail / 3 skip after**, identical failing-test set both times. All 20
  pre-existing failures are caused by this session's deliberately narrow ASAN build (`-Dges=enabled`
  with `good`/`bad` disabled, so no `gst-plugin-scanner` external-plugin-loading path and no
  video/audio conversion elements many of those tests' fixtures need) - unrelated to this patch.
  `ges_track` (the dedicated `GESTrack` unit test, which already exercises the normal
  add/remove-once path) passes 1/1 both before and after.

## What's not done / open questions

- Not checked: whether any other GES container type with a similar
  `GHashTable`-of-`GSequenceIter` bookkeeping pattern (if any exists elsewhere in
  `gst-editing-services`) has the same "hash entry never removed on success" shape. Only
  `ges-track.c`'s `trackelements_iter`/`trackelements_by_start` pair was investigated this session,
  matching `findings.toml` UAF_8's own scope.
- The reproduction relies on the application holding (or otherwise keeping alive) its own
  reference to the `GESTrackElement` across the two `ges_track_remove_element()` calls - a normal
  and explicitly-supported pattern (the function's own `object` parameter is not
  `(transfer full)`), not a contrived setup. If an application drops its only reference to the
  element immediately after the first, successful removal, the element is finalized there and a
  second removal call would instead be operating on a dangling `GESTrackElement*` supplied by the
  caller itself - a different, caller-side bug, out of scope for this candidate.
