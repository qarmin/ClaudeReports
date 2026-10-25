# Recovered partial progress for candidate #026 (pre-assigned, never completed)

Session that claimed #026 (`aes_hexstring2bytearray_unbounded_write`) died before producing a
result block. Its worktree at `/tmp/bughunt-026` (checked out from `65b9547519`, build at
`/tmp/bughunt-026/build-asan`, an existing ASAN+UBSAN meson build) survived and contained one
untracked file, `repro_manual.c` (copied into this directory), which was inspected and run before
the worktree was discarded.

## What was already established

The repro is a genuine, confirmed heap-buffer-overflow, not just a plausible read of the code:

```
cd /tmp/bughunt-026 && source build-asan/env.sh
gcc repro_manual.c -o /tmp/repro_026 $(pkg-config --cflags --libs gstreamer-1.0) -fsanitize=address,undefined
ASAN_OPTIONS=detect_leaks=0 /tmp/repro_026 aesenc key 400
```

ASAN reports a heap-buffer-overflow WRITE of size 1 in `gst_aes_hexstring2bytearray`
(`subprojects/gst-plugins-bad/ext/aes/gstaeshelper.c`), called from
`gst_aes_enc_set_property` (`gstaesenc.c:286`) via a plain `g_object_set (elem, "key", <400-char
hex string>, NULL)` - no pipeline/media needed. Full ASAN output saved as `asan_crash.log` in this
directory.

Root cause confirmed by reading the code (matches findings.toml OVF_2 exactly):
`gst_aes_hexstring2bytearray()` in `gstaeshelper.c` (~line 140-187) writes one byte to `out` per
two input hex chars with no bound on the number of input chars - it only returns 0 early on a
non-hex character, never on `out` running past its destination. The callers in `gstaesenc.c` and
`gstaesdec.c` `set_property()` pass `filter->key`/`filter->iv`, which are fixed-size fields in the
element struct, and only check the returned `hex_count` against the expected length *after* the
call already wrote past the buffer.

## What remains (candidate loop steps not reached)

- Root-cause writeup is done (see above) but no `patch.diff` was drafted.
- The natural fix is a bound check inside `gst_aes_hexstring2bytearray()` itself (needs an added
  `out` capacity parameter, or a pre-check of input string length against the destination size at
  each of the two call sites) - not yet designed or written.
- Not yet re-verified against a patched tree or the existing test suite (no test run at all this
  round).
- `iv` property and `aesdec` (as opposed to `aesenc`/`key`) were not separately re-tried, though
  the code path is identical (same helper, same missing check) so this is expected to repro the
  same way - not confirmed directly.

## Files here

- `repro_manual.c` - the recovered repro program (`usage: repro_manual <aesenc|aesdec> <key|iv>
  [hexlen]`).
- `asan_crash.log` - full ASAN/UBSAN output from `repro_manual aesenc key 400` against the
  existing `build-asan` build.

The `/tmp/bughunt-026` worktree itself has been removed (`git worktree remove`) after this capture.
