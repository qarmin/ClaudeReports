# `ConfirmationDialog::dialog_text` race - shares candidate 039's `Label` root cause exactly

From the coordinator's priority list (the `Window`-family dialogs flagged in the external sweep).
`AcceptDialog::set_text()`/`get_text()` (the base class `ConfirmationDialog` inherits) are thin
wrappers calling `message_label->set_text(p_text)`/`message_label->get_text()` directly on an
internal child `Label` - **this is not a new bug, it's candidate 039's already-confirmed-and-fixed
`Label::set_text()`/`get_text()` race (missing `_THREAD_SAFE_METHOD_`), reached through a different
caller.**

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/gui/dialogs.cpp` - `AcceptDialog::set_text()`/`get_text()` call straight through to `message_label->set_text()`/`get_text()` with no guard of their own | Static read | Confirmed the crash path is identical to candidate 039's, just one call frame deeper |
| GDScript-level repro (`repro/`: shared `ConfirmationDialog`, added to tree, 3 threads alternating `.dialog_text` assignment between a long and a short string, 3 threads reading `.dialog_text`/`.length()` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/050`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 attempts** - the identical ASan signature as candidate 039: `heap-use-after-free ... in SafeNumeric<unsigned long>::conditional_increment()`, reached through `String::operator=`/`CowData<char32_t>::_ref()` |
| **Verification that candidate 039's existing patch alone fixes this** - applied `039.LABEL_SET_TEXT_GET_MINIMUM_SIZE_RACE/patch.diff` (unmodified, touches only `scene/gui/label.cpp`/`.h` - zero changes to `dialogs.cpp`/`AcceptDialog`/`ConfirmationDialog`) to the same worktree, rebuilt | Same build/run | **Clean 8/8 runs**, "survived" every time - confirms the fix lives entirely in `Label` itself, nothing `AcceptDialog`-specific needs to change |

## Status

**Confirmed** - but **not a distinct bug**: this is candidate 039's `Label::set_text()`/`get_text()`
race, reachable through `AcceptDialog`/`ConfirmationDialog`'s pass-through wrappers. 3/3 unfixed
crashes (identical ASan signature to 039), 8/8 clean using 039's own unmodified patch.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `ConfirmationDialog`'s public GDScript API.

## Class

`memory-safety` (identical to candidate 039 - `String`'s `CowData<char32_t>` refcount race).

## Root cause

Identical to candidate 039 - see that candidate's `RESULT.md` for the full mechanism. The only
difference is the call path: `ConfirmationDialog`/`AcceptDialog::set_text()`/`get_text()` are thin
wrappers with no state or locking of their own, calling straight into the same unguarded
`Label::set_text()`/`get_text()` that candidate 039 already found and fixed.

## Introduced by

Same as candidate 039 - the bug lives in `Label`, not in `dialogs.cpp`. `AcceptDialog`'s own
`set_text()`/`get_text()` wrappers have existed in their current pass-through form since
`scene/gui/dialogs.cpp`'s earliest tracked history (not independently dated, since the fix and the
bug both live in `Label`, not here).

## The fix

**No new fix - this candidate's `patch.diff` is an unmodified copy of candidate 039's own
`patch.diff`** (`+8/-0` against `scene/gui/label.cpp`, `+3/-0` against `scene/gui/label.h`).
Nothing in `dialogs.cpp` needs to change; fixing `Label` (already done) transitively fixes every
caller that forwards to it, including `AcceptDialog`/`ConfirmationDialog`.

Verified: 8/8 clean runs of `repro/` with candidate 039's patch applied (vs. 3/3 crashing
unfixed). Test suite: `1431 | 1430 passed | 1 failed | 3 skipped` with the patch applied - same
baseline already established and verified in candidate 039 itself (not independently re-run
unfixed here, since the patch is byte-for-byte identical to the one candidate 039 already
validated against the same baseline).

## What's not done / open questions

- `FileDialog` (the other `Window`-family dialog from the same sweep entry) is investigated
  separately - worth checking whether it shares this exact mechanism too (it likely has its own
  `Label`/`LineEdit` children that could hit the same already-fixed or still-open races) before
  writing its own essay.
- This confirms the general principle the coordinator flagged: when a `Window`-family dialog's
  crash routes through a plain child `Control` (`Label`, `LineEdit`, etc.) that's already been
  independently confirmed-and-fixed this session, the fast path is to verify the existing patch
  resolves it, not to re-derive or duplicate the fix.
