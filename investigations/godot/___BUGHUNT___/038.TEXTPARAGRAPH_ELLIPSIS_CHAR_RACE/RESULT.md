# `TextParagraph::set_ellipsis_char()`/`get_ellipsis_char()` missing `_THREAD_SAFE_METHOD_`

From `IDEAS.md`'s backlog item about the ~23 files using `_THREAD_SAFE_CLASS_`/
`_THREAD_SAFE_METHOD_` not yet fully audited for the locked-writer/unlocked-reader asymmetry that
produced candidates 008 (`Input`), 009 (`CameraServer`), 010/029 (`ProjectSettings`) - a 3/3 hit
rate so far. `text_paragraph.h` was named as one of the highest-value remaining targets (widely
used, not editor-only). `TextParagraph` declares `_THREAD_SAFE_CLASS_` and locks 49 of its 65
methods - a scripted scan for `TextParagraph::` definitions missing `_THREAD_SAFE_METHOD_` found 15
unlocked methods, most touching plain scalars/enums (low risk - a torn word-sized read), but one
pair stood out: `set_ellipsis_char()`/`get_ellipsis_char()` are **both** completely unlocked
(unlike every sibling setter/getter pair in the same file, which locks at least the setter) and
touch a `String` (`el_char`, `CowData`-backed) - the same heap-corruption shape as candidates
016/027/035/036/037's confirmed buffer races, just through `String`'s refcounted backing store
instead of a `Vector`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: scripted scan of `scene/resources/text_paragraph.cpp` for every `TextParagraph::` method definition, checking whether `_THREAD_SAFE_METHOD_` appears in its body - found 15 of 65 methods unlocked, including `set_ellipsis_char()`/`get_ellipsis_char()` (both unlocked - unique among the file's setter/getter pairs, every other one locks at least the setter) | Static read (awk script, not manual grep) | `set_ellipsis_char()` directly assigns `el_char = c` (a `String`, `CowData<char32_t>`-backed) with zero lock; `get_ellipsis_char()` returns `el_char` by value (a copy, incrementing/decrementing the shared refcount) also with zero lock - a genuine race on `String`'s internal `CowData`, not just a benign scalar torn-read like the other 13 unlocked methods |
| GDScript-level repro (`repro/`: shared `TextParagraph`, 3 threads alternating `set_ellipsis_char("…")`/`set_ellipsis_char(".")`, 3 threads calling `get_ellipsis_char()` + `.length()` repeatedly, for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/038`, `bin`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on all 3 attempts** - real ASan heap-use-after-free every time, inside `SafeNumeric<unsigned long>::decrement()`/`conditional_increment()` (`core/templates/safe_refcount.h`), reached through `CowData<char32_t>::_unref()` - `String`'s own refcount, corrupted by the concurrent unguarded assignment |
| Fix (`_THREAD_SAFE_METHOD_` added to `set_ellipsis_char()` and `get_ellipsis_char()`), incremental rebuild (ccache-warm hardlinked worktree, ~62s) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (a real ASan heap-use-after-free every time, on
`String`'s own `CowData` refcount - deterministic enough to crash on the very first attempt), and
8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `TextParagraph` class through nothing but the public
GDScript API. No second participant is involved - a same-process crash reachable from GDScript
`Thread`s calling `set_ellipsis_char()`/`get_ellipsis_char()` concurrently on a shared
`TextParagraph`.

## Class

`memory-safety` (heap-use-after-free on `String`'s internal `CowData<char32_t>` refcount -
concurrent `set_ellipsis_char()` calls, and `get_ellipsis_char()`'s unguarded copy, race the
refcount increment/decrement that backs `String`'s copy-on-write semantics).

## Root cause

```cpp
void TextParagraph::set_ellipsis_char(const String &p_char) {
	String c = p_char;
	...
	el_char = c;              // <-- unguarded String assignment, races CowData refcount
	lines_dirty = true;
}

String TextParagraph::get_ellipsis_char() const {
	return el_char;            // <-- unguarded copy, also races CowData refcount
}
```

`TextParagraph` declares `_THREAD_SAFE_CLASS_` and locks 49 of its 65 methods (including every
other field's setter) via `_THREAD_SAFE_METHOD_` - but these two methods were never given the
guard. `String` is `CowData`-backed, so a plain assignment/copy increments or decrements a shared
`SafeNumeric` refcount and, when it reaches zero, frees the backing buffer. Two threads racing
`set_ellipsis_char()` against each other, or against `get_ellipsis_char()`'s copy, can both observe
the same refcount and both decide to free (or one frees while the other still holds a reference it
believes is valid) - confirmed via ASan: `heap-use-after-free ... in
SafeNumeric<unsigned long>::decrement()`, reached through `CowData<char32_t>::_unref()` from
`String::operator=`.

Reachability: `set_ellipsis_char`/`get_ellipsis_char` are bound, GDScript-callable public methods
(`scene/resources/text_paragraph.cpp:102-104`) with a bound `ellipsis_char` property -
`TextParagraph` is an ordinary `RefCounted`, reachable from any thread with no special setup.

## Introduced by

56579f397decc0d399ff3ca49f0a3abeadda682b "[Text Overrun] Add option to set custom ellipsis
character, add support for system font fallback." (2023-10-01) - the commit that added
`set_ellipsis_char()`/`get_ellipsis_char()` to the class, already missing the guard every sibling
setter/getter in the same commit's context already had.

## The fix

`+4/-0` against `scene/resources/text_paragraph.cpp` (see `patch.diff`): add
`_THREAD_SAFE_METHOD_` at the top of both `set_ellipsis_char()` and `get_ellipsis_char()`, matching
the pattern already used by every other locked setter/getter pair in the same file.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed).

- **Applied for real** in an isolated worktree (`.worktrees/038`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- **13 other unlocked methods remain** in the same file (`_shape_lines`'s caller-side exemption
  aside - that one is safe since it's only ever called from an already-locked public entry point):
  `get_rid`, `get_dropcap_rid`, `duplicate`, `get_alignment`, `get_justification_flags`,
  `get_break_flags`, `get_text_overrun_behavior`, `get_max_lines_visible`, `get_line_spacing`,
  `get_dropcap_lines`, `is_dirty`. All of these read plain scalars/enums/`RID`s (not `CowData`-backed
  containers), so the risk is a benign torn word-sized read rather than heap corruption - not
  independently repro'd or fixed this session, left as a lower-priority follow-up in `IDEAS.md`
  since they don't share `set_ellipsis_char`'s specific `CowData` heap-corruption mechanism.
- **Still open**: 22 of the original ~23 `_THREAD_SAFE_CLASS_`/`_THREAD_SAFE_METHOD_` files flagged
  in `IDEAS.md` haven't been checked for this same asymmetry pattern yet (`scene_tree.h`,
  `rendering_device.h`, `editor_settings.h`, the XR tracker headers, etc.) - this session's hit (4th
  for this specific pattern, after 008/009/010/029) suggests it's worth continuing the sweep.
