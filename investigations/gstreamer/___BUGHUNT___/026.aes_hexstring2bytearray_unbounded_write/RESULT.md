# aes_hexstring2bytearray_unbounded_write

`gst_aes_hexstring2bytearray()` in `subprojects/gst-plugins-bad/ext/aes/gstaeshelper.c` converts a
caller-supplied hex string into a byte array with no bound on the destination buffer. It is called
from `aesenc`/`aesdec`'s `set_property()` for the "key" and "iv" GObject string properties, writing
directly into the fixed-size `filter->key` (`guchar key[EVP_MAX_KEY_LENGTH]`, 64 bytes) and
`filter->iv` (`guchar iv[GST_AES_BLOCK_SIZE]`, 16 bytes) struct fields - length is checked only
*after* the call, via the returned byte count, by which point any overflow has already happened.
Flagged by static review (findings.toml OVF_2) and escalated here to a real reproduction.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + real GObject property API | clang ASAN+UBSAN build (`-Db_sanitize=address,undefined -fno-sanitize=function`), `gst-plugins-bad:aes=enabled`; `g_object_set(elem, "key"/"iv", <N-char hex string>, NULL)` for `elem` in `{aesenc, aesdec}`, N swept from 32 (valid) up to 400 | Heap-buffer-overflow WRITE confirmed for all 4 combinations (aesenc/aesdec x key/iv) at N=400; clean boundary at N=128 for key (exactly fills the 64-byte `EVP_MAX_KEY_LENGTH` buffer) and N=32 for iv (exactly fills the 16-byte `GST_AES_BLOCK_SIZE` buffer); N=130 (one byte past the key buffer) already overflows |

Only one approach was needed - the static-analysis lead already pointed at the exact missing bound
check, and a real property-setter call through the public API reproduced it deterministically on
the first attempt, so no further escalation (fuzzing, miri - not applicable to C) was warranted per
the soft cap in `SKILL.md`.

## Status

**Confirmed.** Reproduces 100% of the time (this session: an initial repro plus 3 explicit repeat
runs of `aesenc key 400`, all 4/4 crashing identically at the same source line with the same ASAN
signature; also reproduced once each for `aesenc iv 400`, `aesdec key 400`, `aesdec iv 400`) -
deterministic, no flakiness, no memory-layout dependence: the write walks straight past a
fixed-size `calloc`'d struct field with no bound check at all, so every run with an over-length hex
string overflows identically.

**Fully reproducible**: **Yes**. `repro/repro.c` calls the real public GStreamer API end-to-end:
`gst_element_factory_make("aesenc"/"aesdec", NULL)` to instantiate the real element (loaded through
the real plugin-scanner/registry mechanism, not a hand-built stand-in), then
`g_object_set(elem, "key"/"iv", <hex string>, NULL)` - exactly what any application embedding
GStreamer does when taking a key/iv value from a config file, command line, or pipeline description
(e.g. `gst-launch-1.0 aesenc key=<hex> ...`). No second participant/peer is involved in this
finding (it's a single-process API-misuse-shaped overflow, not a wire-protocol one), so
`CONFIRMATION.md` section 11's "real second participant" requirement does not apply here.

## Class

memory-safety (heap-buffer-overflow, out-of-bounds write)

## Root cause

`gst_aes_hexstring2bytearray()` (`gstaeshelper.c`, originally lines ~140-187) loops over the input
hex string two characters at a time, writing one decoded byte to `*out` and incrementing `out` each
iteration, until it hits a NUL terminator or an invalid hex character. It never compares how many
bytes it has written against any notion of the destination buffer's capacity - the function
signature doesn't even take a size for `out`. Both call sites
(`gst_aes_enc_set_property`/`gst_aes_dec_set_property` in `gstaesenc.c`/`gstaesdec.c`, `PROP_KEY`
and `PROP_IV` cases) pass the address of a fixed-size struct field (`filter->key`,
`EVP_MAX_KEY_LENGTH` = 64 bytes; `filter->iv`, `GST_AES_BLOCK_SIZE` = 16 bytes) and only look at the
returned `hex_count` *after* the call returns, to validate it equals the expected length - by then,
if the input hex string decoded to more bytes than the destination holds, the overflow has already
happened during the call itself. A hex string is fully attacker/user controlled via the "key"/"iv"
GObject properties (settable via `g_object_set()`, `gst-launch-1.0 aesenc key=... `, a serialized
pipeline description, etc.), so any caller that forwards an oversized or malformed value into these
properties (e.g. from a config file, IPC, or a parsed pipeline string) triggers a real heap
out-of-bounds write with no validation anywhere on the path.

## Introduced by

5cf4dc2b82f10cbe2dc603fd49ab080ff8d32e56 "aes: add aes encryption and decryption elements"
(2021-07-06). This commit added the entire `ext/aes` plugin, including
`gst_aes_hexstring2bytearray()` with its current (missing) bound check, in one shot. The only other
commit ever touching this file is 019971a3c7 ("Move files from gst-plugins-bad into the
"subprojects/gst-plugins-bad/" subdir"), a pure path-rename as part of the gstreamer monorepo
merge - it carried no functional change. The bug has been present unmodified since introduction.

## The fix

Added an `out_size` parameter (`gsize`) to `gst_aes_hexstring2bytearray()` and a bound check at the
top of the write loop (`if (hex_count >= out_size) { GST_ERROR_OBJECT (...); return 0; }`), then
updated all 4 call sites (`gstaesenc.c` PROP_KEY/PROP_IV, `gstaesdec.c` PROP_KEY/PROP_IV) to pass
`sizeof (filter->key)` / `sizeof (filter->iv)` respectively. This is sufficient because the
destination is always one of exactly these two fixed-size struct fields at every call site in the
tree (`grep` confirms no other caller exists) - the size is a compile-time constant available at
every call site, so passing it costs nothing and closes the write for every value that could
previously overflow, not just the one 400-hex-char input that happened to trigger the crash.
Checked against multiple input sizes, not just the original 400-char trigger: too-long key/iv now
fails cleanly (logged `GST_ERROR`, property rejected, `g_object_set` returns without crashing) for
all 4 call sites at N=400; a key exactly filling the 64-byte buffer (128 hex chars) and an iv
exactly filling the 16-byte buffer (32 hex chars) still succeed correctly (verifies the fix doesn't
introduce an off-by-one that would reject valid maximum-length values); one hex char past a valid
key length (130 hex chars) is correctly rejected.

- Project test suite (`gst-plugins-bad:elements_aesenc`, `gst-plugins-bad:elements_aesdec`, the
  only tests covering this plugin): 12/12 passing before the fix, 12/12 passing after (6/6 each
  suite, both states) - the existing suite doesn't exercise the oversized-input path at all, so it
  neither caught the original bug nor is affected by the fix. (Run with
  `UBSAN_OPTIONS=halt_on_error=0` to work around this sandbox's pre-existing, unrelated UBSAN
  misaligned-load/store findings inside `gst-plugin-scanner`'s own IPC protocol
  (`gstpluginloader.c`) during registry building - present identically before and after this
  patch, unrelated to the aes plugin, and already a known environment quirk for this workspace; see
  `___BUGHUNT___/README.md`.)

## What's not done / open questions

- `gst_aes_bytearray2hexstring()` (the inverse direction, byte array -> hex string, in the same
  file) was not audited here - it's used internally with compile-time-fixed lengths at every call
  site found (`GST_AES_BLOCK_SIZE`), not with user-controlled input, so it looked out of scope for
  this candidate, but wasn't independently exercised.
- No fuzzing campaign was run against this plugin; the bound-value approach was sufficient to find
  and fully characterize this bug, but a short campaign via `fuzz-bug-repro` could still be worth
  pointing at the aes plugin's property-setting code generally if a deeper audit of this plugin is
  ever wanted.
