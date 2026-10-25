# gstmeta_deserialize_header_size_overflow

`gst_meta_deserialize()` (`subprojects/gstreamer/gst/gstmeta.c`) parses a serialized-meta header
straight out of a caller-supplied `const guint8 *data` buffer with no prior validation of the
`name_len` field it reads from that buffer - flagged as a pre-existing lead (findings.toml OVF_8)
describing an integer-overflow-driven out-of-bounds read in the core-library deserialize path,
reachable with 16 bytes of input and no plugin involved.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + reasoning about the `guint32 header_size = 2*sizeof(guint32) + name_len + 2` assignment | Read `gst_meta_deserialize()` (subprojects/gstreamer/gst/gstmeta.c:848-903), traced the types: the RHS is computed in 64-bit (size_t promotion from `sizeof`) but truncates on assignment to the declared `guint32 header_size` | Confirmed by hand: `name_len = 0xFFFFFFFF` gives RHS `0x100000009`, truncated to `9` |
| Debug/ASAN build (clang, `-fsanitize=address,undefined`, `-fno-sanitize=function`), core-only meson scope (`base`/`good`/`ugly`/`bad` all disabled - the bug has no plugin dependency), standalone C repro calling the real public `gst_meta_deserialize()` API with a crafted 16-byte buffer (`total_size=16`, `name_len=0xFFFFFFFF`) | `meson setup --buildtype=debugoptimized -Db_sanitize=address,undefined ...`; repro built with `clang -fsanitize=address,undefined -fno-sanitize=function` against the uninstalled `gstreamer-1.0.pc` | **Reproduced**: ASAN SEGV inside `gst_meta_deserialize`, `gstmeta.c:875:22` (`name[name_len] != '\0'`), 5/5 repeat runs, identical crash site every time |
| Same ASAN build, boundary-value sweep (`name_len = 0xFFFFFFF6` which wraps `header_size` to exactly 0, and `name_len = 0` as a sane-input sanity check) | Same repro harness, 3 additional inputs | `0xFFFFFFF6` also crashes (same bug class, different wrap target: `header_size` wraps to 0 instead of 9, crash moves to `gstmeta.c:873:20`, `data[header_size - 1]`); `name_len = 0` behaves correctly (no crash, `consumed = 16`) |

Only one tool escalation was needed (manual reasoning found the exact mechanism; the ASAN build
was used to get a real, capturable crash rather than as a second independent detection method) -
no miri (C project), no fuzzing needed (the exact triggering input was already known from reading
the arithmetic).

## Status

**Confirmed.** 5/5 repeat runs of the exact same input crash identically (same signal, same crash
site `gstmeta.c:875:22`, same call stack) - only the leaked heap address differs run to run
(ASLR). A second boundary value (`name_len = 0xFFFFFFF6`) independently confirmed the same bug
class at a different crash site (`gstmeta.c:873:20`), ruling out "one specific magic input" as the
explanation.

**Fully reproducible**: **Yes**. `repro/repro.c` calls the real, public
`gst_meta_deserialize(GstBuffer *, const guint8 *, gsize, guint32 *)` API directly with a
hand-crafted 16-byte buffer. This is the genuine external input the function is designed to
parse (its own doc comment: "Recreate a #GstMeta from serialized data ... obtained from
gst_meta_serialize()") - a malformed/adversarial byte blob handed to a deserializer's real public
entry point, with no second participant (peer/caller) whose behavior needs to be faked; per
`CONFIRMATION.md` section 11 this is the same shape as "a malformed file handed to a parser's real
public `open_file` entry point."

## Class

memory-safety (out-of-bounds read via integer-overflow-driven bounds-check bypass)

## Root cause

```c
guint32 total_size = GST_READ_UINT32_LE (data + 0);
guint32 name_len = GST_READ_UINT32_LE (data + 4);
guint32 header_size = 2 * sizeof (guint32) + name_len + 2;   /* line 869 */
if (size < total_size || total_size < header_size)
  goto bad_header;

guint8 version = data[header_size - 1];                       /* line 873 */
const gchar *name = (const gchar *) (data + 2 * sizeof (guint32));
if (name[name_len] != '\0')                                   /* line 875 - crash site */
  goto bad_header;
```

`name_len` is read straight from the untrusted `data` buffer and can be any value up to
`G_MAXUINT32`. `2 * sizeof (guint32)` is a `size_t` (64-bit) expression, so
`2 * sizeof (guint32) + name_len + 2` is computed in 64-bit arithmetic without overflowing - but
the result is then assigned into `header_size`, declared `guint32`, which silently truncates it:

- `name_len = 0xFFFFFFFF` -> `8 + 0xFFFFFFFF + 2 = 0x100000009` -> truncated to `9`
- `name_len = 0xFFFFFFF6` -> `8 + 0xFFFFFFF6 + 2 = 0x100000000` -> truncated to `0`

With `header_size` wrapped down to a tiny value, `size < total_size || total_size < header_size`
is trivially satisfiable with a small, otherwise unremarkable `total_size` (16 in the repro), so
the bounds check that's supposed to guarantee `header_size <= total_size <= size` passes despite
`name_len` being close to 4 billion. Execution then reaches `name[name_len]` (`name = data + 8`),
reading at an offset up to ~4GB past the real 16-byte buffer - an out-of-bounds read that
reliably crashes under ASAN (and would be a real out-of-bounds read in a non-instrumented build
too, just less likely to immediately SIGSEGV depending on what happens to be mapped there).
`data[header_size - 1]` two lines earlier (line 873) has the same class of problem for the
`name_len = 0xFFFFFFF6` wrap-to-0 case (`header_size - 1` becomes `guint32` `0xFFFFFFFF`).

## Introduced by

06d9d934f9 "meta: Add serialize/deserialize API" (2023-09-18) - this is the commit that added
`gst_meta_deserialize()` (and `gst_meta_serialize()`) to `gstmeta.c` in the first place; the
`guint32 header_size` truncation has been present, unmodified in this respect, since that initial
commit (confirmed via `git log -S"gst_meta_deserialize" -- subprojects/gstreamer/gst/gstmeta.c`,
which shows only this commit and a later, unrelated doc-annotation-only commit,
427d0e1e8c "meta: Mark data parameter of gst_meta_deserialize() as array").

## The fix

Compute `header_size` in 64-bit (`guint64`) instead of letting it truncate to `guint32`, by
widening `name_len`'s contribution to the sum before the addition (the other operand,
`2 * sizeof (guint32)`, is already a 64-bit `size_t`, so this one cast is sufficient - the whole
expression's type follows the widest operand). Every downstream use of `header_size` (indexing
`data`, subtracting from `total_size`) stays correct: once `total_size < header_size` is false,
`header_size` is provably `<= total_size <= G_MAXUINT32`, so it fits back into 32-bit-sized
quantities safely.

```c
-  guint32 header_size = 2 * sizeof (guint32) + name_len + 2;
+  guint64 header_size = 2 * sizeof (guint32) + (guint64) name_len + 2;
```

Full diff in `patch.diff` (+4/-1 lines, including a one-line explanatory comment).

Verified:
- `repro/repro.c`'s crafted input (`name_len = 0xFFFFFFFF`) no longer crashes against the patched
  build: 3/3 runs return `meta=(nil) consumed=0` (the `bad_header` path, as intended) instead of
  segfaulting.
- The independent boundary value `name_len = 0xFFFFFFF6` (wraps the old `header_size` to exactly
  0, a different crash site) also no longer crashes against the patched build.
- `name_len = 0` (a normal, non-adversarial small value) still behaves correctly against the
  patched build (`consumed = 16`, no regression to legitimate parsing).

- Project test suite (`meson test -C build-asan --suite gstreamer`, core "gstreamer" subproject
  only - `base`/`good`/`ugly`/`bad` plugin subprojects disabled in this scoped build, so their own
  test suites aren't part of this count): **58 pass / 55 fail / 1 skip, identical before and
  after the patch** (same failing test names before and after, e.g. `gstreamer:gst_gstbus`,
  `gstreamer:gst_gstbin`, `gstreamer:pipelines_parse_launch`, `gstreamer:libs_bytereader`, ...).
  These 55 failures are a pre-existing artifact of this scoped build configuration - most visibly
  a "Spawning gst-plugin-scanner helper failed" / element-lookup failure cascading into
  `GST_IS_ELEMENT`/`G_IS_OBJECT` assertion failures across tests that need core elements
  (`fakesrc`, `fakesink`, etc.) from the disabled plugin subprojects - not caused by, or affected
  by, this patch. The one test that actually exercises `gst_meta_deserialize()`,
  `gstreamer:gst_gstmeta`, passes both before and after (`13 checks, 0 failures` in both runs).

## What's not done / open questions

- `gst_meta_serialize()` (same file, ~line 773) has a structurally similar pattern -
  `guint8 header_size = 2 * sizeof (guint32) + name_len + 2;` - an even narrower type (`guint8`,
  not `guint32`) for a header_size computed from a `name_len` that is `strlen(name)` of a
  registered `GType` name. In current call sites `name` always comes from
  `g_type_name(meta->info->type)` (an internally-registered API name, not attacker-controlled
  network/file input), so this doesn't look reachable with untrusted external data the way the
  deserialize-side bug is - but it's the same integer-width mismatch pattern in the same file and
  wasn't independently fuzzed/boundary-tested this session. Worth a look if `gst_meta_serialize()`
  is ever called with a name whose length isn't fully controlled by the local process.
- Only the exact `gst_meta_deserialize()` function was checked; sibling per-type
  `deserialize_func` implementations (`GstCustomMeta`, `GstVideoMeta`, `GstReferenceTimestampMeta`,
  `GstAudioMeta`, all added in the same original commit 06d9d934f9) receive `payload`/
  `payload_size` computed from the now-fixed `header_size`/`total_size`, so they're protected by
  this fix from *this* specific overflow - but their own internal parsing of `payload` wasn't
  independently audited this session.
- This session's ASAN build only scopes the core "gstreamer" meson subproject (no
  base/good/ugly/bad plugins) - sufficient for this candidate since the bug and its fix are
  entirely within `subprojects/gstreamer/gst/gstmeta.c`, but means the 55 pre-existing test
  failures noted above were not root-caused beyond "scoped build, missing plugins" - not
  investigated further as it's orthogonal to this candidate.
