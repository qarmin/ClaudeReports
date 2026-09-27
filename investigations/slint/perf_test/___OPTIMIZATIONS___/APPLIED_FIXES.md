# The applied set, as a patch

`APPLIED_FIXES.patch` is `git diff internal/` - every fix this workspace has applied, in one file.
Regenerate it after applying or changing a fix:

```sh
git diff internal/ > ___OPTIMIZATIONS___/APPLIED_FIXES.patch
```

## Why this exists

`git checkout <file>` to undo a temporary instrument also undoes any *fix* living in the same file.
That happened here: reverting an instrument in `partial_renderer.rs` silently took candidate 35 with
it, and the only reason it was noticed is that the modified-file count in `git status` dropped by
one. Reconstructing it from the write-up took twenty minutes and a re-measurement to confirm the
-11.5% came back.

Two habits that prevent it:

- **Never `git checkout` a file that also holds a fix.** Save the instrument as a patch, apply it,
  and `git apply -R` to remove it - or copy the file to `target/` first (not `/tmp`, which is wiped).
- **Keep this patch current.** It is the difference between "reconstruct from prose" and
  `git apply`.

## What is in it

| candidate | file |
| --- | --- |
| 03, 19, 23 | `internal/core/textlayout/sharedparley/` |
| 08, 14 | `internal/core/properties.rs` |
| 09 | `internal/core/graphics/brush.rs`, femtovg |
| 10 | `internal/renderers/femtovg/itemrenderer.rs` |
| 26 | `internal/core/textlayout/sharedparley/cache.rs`, `item_rendering.rs` |
| 29 | `internal/core/model/adapters.rs` |
| 35 | `internal/core/partial_renderer.rs` |
| 36 | `internal/core/item_rendering.rs` |
| 41 | `internal/renderers/software/lib.rs` |

`tools/isolate.py list` is the authoritative list of what can be toggled off individually.
