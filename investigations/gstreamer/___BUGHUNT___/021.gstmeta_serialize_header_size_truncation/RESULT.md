# gstmeta_serialize_header_size_truncation

`subprojects/gstreamer/gst/gstmeta.c`'s `gst_meta_serialize()` (~line 763-786, core GStreamer
public API, no plugin/media file involved). Backlog item OVF_7 in `findings.toml`: `header_size`
is declared `guint8` but the real header length is `name_len + 10`; a custom meta name >= 246
bytes wraps `header_size` modulo 256, under-allocating the serialization buffer while the later
`memcpy(header + 8, name, name_len + 1)` still uses the real, untruncated `name_len`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug-assertions-equivalent (clang ASAN+UBSAN, `-fno-sanitize=function`) core-only build (new `tools/build_asan_core.sh`, core `gstreamer` lib only, all plugins disabled) | Standalone C repro calling the real public API `gst_meta_register_custom_simple()` -> `gst_buffer_add_meta()` -> `gst_meta_serialize_simple()` with a registered custom-meta name of `name_len` bytes; swept `name_len` in {245, 246 x3, 300, 500, 1000, 4096} | `name_len=246` (`header_size` wraps `256 mod 256 == 0`): heap-buffer-overflow WRITE, reproduced 3/3 runs. `name_len=245` (`header_size=255`, no wrap): no crash. `name_len` in {300, 500, 1000, 4096} (header_size wraps to a smaller-but-nonzero value): ASAN doesn't trip on this build, because GLib's `GArray`/`g_array_set_size` over-allocates capacity beyond the requested logical length, so a modest under-allocation lands inside real malloc slack rather than past the actual heap chunk boundary - the underlying bug (writing `name_len+1` bytes into a buffer only grown by the wrapped `header_size`) is present at every one of these values, but only `name_len=246`'s exact-256 wrap reliably lands the overflow write outside the allocator's slack in this environment. |

Only one approach was needed - the bug was already fully understood from reading the code (a
`guint8` truncation of an arithmetic expression that can exceed 255), so this was pure
boundary-value confirmation, not exploratory fuzzing.

## Status

**Confirmed.** Reproduces 3/3 runs at `name_len=246` (debugoptimized build, clang
`-fsanitize=address,undefined -fno-sanitize=function`, no fixed seed needed - deterministic:
every run with a custom meta name whose length makes `(name_len + 10) mod 256 == 0` crashes the
same way, at the same line, every time).

**Fully reproducible**: **Yes**. `repro/run.sh` builds and runs a standalone C program that calls
only real, public GStreamer core API (`gst_meta_register_custom_simple`, `gst_buffer_add_meta`,
`gst_meta_serialize_simple`) - there is no second participant to fake here (no peer, no upstream
caller, no wire format): the "attacker-controlled input" *is* the meta name string handed
directly to the library's own public registration API by whichever code (application or plugin)
calls it, the same as a malformed file handed to a parser's own public entry point. Verified via
`just test` from this folder against a genuine fresh clone of `repo_url` at `base_commit` (see
`output.txt` for the captured run): `reproduce-before` crashes with the ASAN diagnostic below,
`apply` + `reproduce-after` no longer crashes.

## Class

memory-safety (heap-buffer-overflow, WRITE)

## Root cause

```c
guint8 header_size = 2 * sizeof (guint32) + name_len + 2;   /* == name_len + 10, but truncated to 8 bits */
if (!gst_byte_array_interface_set_size (data, data->len + header_size))
  return FALSE;
...
memcpy (header + 8, name, name_len + 1);   /* still uses the real, untruncated name_len */
```

`name_len` is `strlen (g_type_name (meta->info->type))` - the length of the meta implementation's
own registered type name, which for a custom meta (`gst_meta_register_custom()` /
`gst_meta_register_custom_simple()`) is exactly the caller-supplied `name` string, with no length
limit enforced anywhere in that registration path. `header_size` is declared `guint8`, so for any
`name_len` such that `name_len + 10 >= 256` (i.e. `name_len >= 246`), the assignment silently
truncates modulo 256. The buffer is then grown by only this truncated (too small) amount via
`gst_byte_array_interface_set_size()`, but the subsequent `GST_WRITE_UINT32_LE (header + 4,
name_len)` and, most severely, `memcpy (header + 8, name, name_len + 1)` still operate with the
real, un-truncated `name_len` - writing up to ~`name_len` bytes past the space actually reserved.
The sibling deserialize path, `gst_meta_deserialize()` a few dozen lines below in the same file,
already computes the identical expression into a `guint32 header_size` - the serialize side's
`guint8` is simply the narrower of the two, with no reason found in the code or its commit history
for the width mismatch (looks like a plain oversight, not a deliberate constraint).

## Introduced by

06d9d934f990d29384955b6a577b9ab581b2f084 "meta: Add serialize/deserialize API" (2023-09-18) - the
commit that added `gst_meta_serialize()`/`gst_meta_deserialize()` in the first place; the `guint8
header_size` in the serialize function has this exact width from its very first version
(`git log -S "guint8 header_size" -- subprojects/gstreamer/gst/gstmeta.c` finds only this one
commit).

## The fix

Widen `header_size` in `gst_meta_serialize()` from `guint8` to `guint32`, matching the type
already used for the identical computation in `gst_meta_deserialize()`:

```diff
-    guint8 header_size = 2 * sizeof (guint32) + name_len + 2;
+    guint32 header_size = 2 * sizeof (guint32) + name_len + 2;
```

`name_len` is itself a `guint32` (`strlen()` result), so `2 * sizeof(guint32) + name_len + 2` can
never overflow a `guint32` in practice (`name_len` would need to be within 10 of `G_MAXUINT32`,
i.e. a ~4 GB type name string, to even approach it) - a `guint32` header_size is sufficient here,
unlike the deserialize side where `name_len` comes directly from 4 untrusted wire bytes and can
genuinely be `G_MAXUINT32` (a separate, already-tracked finding, `OVF_8` in `findings.toml`, not
addressed by this patch).

Verified: re-ran `repro/run.sh` against the patched build - no longer crashes at `name_len=246`
(3/3 runs). Per CONFIRMATION.md section 7, also re-checked at `name_len` = 245, 300, 500, 1000
(spanning several different pre-patch wrap points, not just the original 246) - all pass cleanly
under the same ASAN/UBSAN build after the fix.

- Project test suite (`meson test -C build-asan-core --suite gstreamer`, core-only ASAN+UBSAN
  build, no plugins): Before (pristine): **55 Ok / 55 Fail** (110 total). After (patched):
  **56 Ok / 54 Fail** (110 total). Diffing the full per-test result sets shows exactly one line
  changed:
  `gstreamer:gst_gstdevice` FAIL -> OK - re-checked independently and unrelated to `gstmeta.c`
  (device-enumeration test, no relation to meta serialization); treated as pre-existing flakiness
  in this environment, not caused by the patch. All other 109 test outcomes are identical before
  and after. The 54-55 pre-existing failures themselves are caused by this being a
  core-library-only build (all plugins disabled per `tools/build_asan_core.sh`, so any test
  requiring a `gst-plugins-base`/`good` element - `fakesink`, `queue2`, etc. - fails to find its
  element), present identically before and after this patch, unrelated to `gstmeta.c`.

## What's not done / open questions

- `gst_meta_deserialize()`'s own `header_size` (already `guint32`, not `guint8`) has a related but
  distinct bug: it's computed directly from an untrusted 4-byte `name_len` read off the wire,
  which *can* wrap a `guint32` (e.g. `name_len = 0xFFFFFFF8` wraps `header_size` to 2) and defeat
  its own bounds checks, causing an out-of-bounds read. This is already tracked separately as
  `OVF_8` in `findings.toml` and is not fixed by this patch - see "New suspects" in the final
  report for this session. While investigating this candidate, an unrelated leftover `git stash`
  entry was found in the main project's shared `.git` (not this worktree's own working tree)
  containing what looks like an in-progress fix for exactly that OVF_8 issue (a `guint64
  header_size` in `gst_meta_deserialize`); it was left untouched in the stash stack (re-pushed
  after an accidental pop during this session - see below) rather than adopted into this
  candidate, since it belongs to a separate finding.
- Did not check every other `serialize_func` implementor (`GstVideoMeta`, `GstAudioMeta`,
  `GstReferenceTimestampMeta`, per the introducing commit's own description) for a similar
  fixed-width-truncation pattern - none of them compute their own `header_size`, they all go
  through this same shared `gst_meta_serialize()` header-writing code, so they're covered by this
  one fix, but this wasn't independently re-verified against each of them.
- Operational note for future sessions using worktrees: `git stash` operates on the whole
  repository's shared `.git` directory, not per-worktree - a `git stash pop` inside one
  worktree can retrieve a stash entry that was pushed by a *different* worktree/session sharing
  the same main `.git`. This session hit exactly that (see above) while trying to revert this
  candidate's own in-progress edit; the foreign stash entry was identified before use and restored
  immediately via `git stash push` before proceeding with `git checkout --` /direct `Edit` calls
  instead of `git stash` for the rest of the session. Future sessions in this project should avoid
  `git stash` for save/revert inside a worktree entirely - use `git diff > tmp.patch` +
  `git checkout -- <file>` (and `git apply tmp.patch` to restore) instead, which stays entirely
  within that worktree's own working tree.
