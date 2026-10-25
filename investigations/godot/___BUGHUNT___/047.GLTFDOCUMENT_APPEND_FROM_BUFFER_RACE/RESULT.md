# `GLTFDocument::append_from_buffer()` races on the unguarded `document_extensions`

From `IDEAS.md`'s backlog, one of the remaining classes named in the original external sweep
(alongside `CryptoKey`/`ENetConnection`, flagged as likely more complex than the simple-buffer
shape that worked for most of this session's confirmed candidates). `GLTFDocument` keeps its
per-import/export extension list in `Vector<Ref<GLTFDocumentExtension>> document_extensions`
(`modules/gltf/gltf_document.h:91`, `protected:`), rebuilt from scratch at the start of every
`append_from_file`/`append_from_buffer`/`append_from_scene` call - with zero
`Mutex`/`_THREAD_SAFE_CLASS_` anywhere in the header protecting it (the class does have a
`static Mutex all_document_extensions_mutex`, but that only guards the separate, global,
process-wide *registered-extension-types* list, not this per-instance working copy).

`GLTFDocument` is one of Godot's larger classes (126 `GLTFDocument::` method definitions) - this
candidate's fix is **deliberately scoped** to the 5 public entry points that directly
clear/rebuild/iterate `document_extensions`, not a full-class sweep.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `modules/gltf/gltf_document.h`/`.cpp` - confirmed the instance-level `document_extensions` has no lock of its own; `_parse()` (reached from `append_from_file`/`append_from_buffer`) and `append_from_scene()` each independently do `document_extensions.clear()` followed by rebuilding it from the (separately, correctly, mutex-protected) global registered-extensions list, then several methods (`append_from_buffer`/`append_from_file`/`generate_scene`/`generate_buffer`/internal `_parse_*`/`_generate_*` helpers) iterate it afterward | Static read | Confirmed `append_from_file`/`append_from_buffer`/`append_from_scene`/`generate_scene`/`generate_buffer` are all `ClassDB`-bound, reachable from GDScript |
| GDScript-level repro (`repro/`: shared `GLTFDocument`, 6 threads each repeatedly calling `append_from_buffer()` with a minimal valid glTF JSON payload (`{"asset":{"version":"2.0"}}`) and a fresh `GLTFState` per call, for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/047`, `bin`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on all 3 attempts** - 3 distinct real ASan reports, all inside `CowData<Ref<GLTFDocumentExtension>>` operations: `heap-use-after-free` in `Memory::realloc_static`, `use-after-poison` in `_unref()` (with a full stack through `Vector::clear() <- GLTFDocument::_parse() <- append_from_buffer()`), and `unknown-crash` in `_copy_on_write()` |
| Fix (`_THREAD_SAFE_CLASS_` on the header + `_THREAD_SAFE_METHOD_` on the 5 public entry points that touch `document_extensions`: `append_from_file`, `append_from_buffer`, `append_from_scene`, `generate_scene`, `generate_buffer`), incremental rebuild (ccache-warm hardlinked worktree, ~1m9s) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild ~1m8s, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (3 distinct, independently-triggered real ASan
crashes, all inside `CowData<Ref<GLTFDocumentExtension>>`), and 8/8 clean runs with the fix
applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `GLTFDocument` class through nothing but the public
GDScript API. No second participant is involved - a same-process crash reachable from GDScript
`Thread`s calling `append_from_buffer()` concurrently on a shared `GLTFDocument`.

## Class

`memory-safety` (heap corruption on a `Vector<Ref<GLTFDocumentExtension>>` - concurrent
`append_from_buffer()`/`append_from_file()`/`append_from_scene()` calls on the same `GLTFDocument`
instance race each other's `document_extensions.clear()` + rebuild, corrupting the `CowData`
backing the vector; manifests as a use-after-free, use-after-poison, or a bounds-check-adjacent
"unknown-crash" depending on timing - same family as every other confirmed container race this
session, just through a `Vector<Ref<T>>` of reference-counted extension objects).

## Root cause

```cpp
Error GLTFDocument::_parse(Ref<GLTFState> p_state, const String &p_path, Ref<FileAccess> p_file) {
	...
	document_extensions.clear();                              // <-- unguarded resize
	for (Ref<GLTFDocumentExtension> ext : get_all_gltf_document_extensions()) {
		...
		if (err == OK) {
			document_extensions.push_back(ext_dup);            // <-- unguarded resize
		}
	}
	...
}

Error GLTFDocument::append_from_buffer(...) {
	...
	err = _parse(p_state, p_state->base_path, file_access);
	...
	for (Ref<GLTFDocumentExtension> ext : document_extensions) {  // <-- unguarded read
		...
	}
	...
}
```

`document_extensions` is a plain member with zero synchronization anywhere in the class - the
existing `all_document_extensions_mutex` guards only the separate, static, process-wide list of
*registered extension types* (via `get_all_gltf_document_extensions()`), not this per-instance
working copy. Two threads calling `append_from_buffer()`/`append_from_file()`/`append_from_scene()`
concurrently on the same `GLTFDocument` instance race each other's `clear()`/`push_back()` calls on
`document_extensions`, corrupting its `CowData` - confirmed via ASan:
`use-after-poison ... in CowData<Ref<GLTFDocumentExtension>>::_unref()`, with a full stack showing
the exact call chain from `append_from_buffer()`.

Reachability: `append_from_file`/`append_from_buffer`/`append_from_scene`/`generate_scene`/
`generate_buffer` are all bound, GDScript-callable public methods - `GLTFDocument` is an ordinary
`Resource`, reachable from any thread with no special setup (the repro needed no actual GLTF
asset files - a minimal in-memory JSON payload was sufficient to drive the race).

## Introduced by

73c673a614b47716ce467cd43adf770505c016cd "Change the way GLTFDocumentExtension classes are
registered" (2022-09-18) - the commit that introduced the per-instance `document_extensions`
clear-and-rebuild pattern (replacing an earlier design), already unguarded.

## The fix

`+9/-0` against `modules/gltf/gltf_document.cpp`, `+3/-0` against `modules/gltf/gltf_document.h`
(see `patch.diff`): add `_THREAD_SAFE_CLASS_` next to the private members, and
`_THREAD_SAFE_METHOD_` to the 5 public entry points that directly touch `document_extensions`:
`append_from_file()`, `append_from_buffer()`, `append_from_scene()`, `generate_scene()`,
`generate_buffer()`. The many internal `_parse_*`/`_generate_*` helpers that also read
`document_extensions` are only ever reached through these 5 entry points (none are separately
`ClassDB`-bound), so locking the entry points covers the whole call graph via `Mutex`'s recursion.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed, with 3 distinct
ASan failure signatures across the attempts).

- **Applied for real** in an isolated worktree (`.worktrees/047`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- `GLTFDocument` has 126 methods; this fix covers only the 5 public entry points plus whatever
  they call internally. Any other public method that might independently read/write
  `document_extensions` outside this call graph (none were found during this session's reading,
  but the class is large enough that a full audit wasn't attempted) is not covered.
- The remaining items from the original backlog (`CryptoKey`, `ENetConnection`) were not
  investigated this session - `CryptoKey` was assessed as a weak candidate (plain scalar
  `psa_key_id_t`/`psa_key_type_t` fields, no `CowData`-backed container of its own); `ENetConnection`
  would need real two-participant networking (bind/connect/poll) to exercise its `peers` list
  realistically, raising the bar per `CONFIRMATION.md` section 11 - not attempted given the
  session's remaining time budget.
