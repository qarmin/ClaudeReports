# aesdec_iv_only_buffer_assert

`gst_aes_dec_prepare_output_buffer()` in `subprojects/gst-plugins-bad/ext/aes/gstaesdec.c`
(`aesdec` with `serialize-iv=true`) contains `g_assert (in_size > GST_AES_BLOCK_SIZE)` guarding
the case where the IV is stripped from the first buffer. The comparison is strict `>`, so the
very first buffer on the pad being exactly 16 bytes (a complete serialized IV, zero ciphertext
bytes) aborts the whole process, even though `gst_aes_dec_transform()`'s own check three lines of
logic earlier (`ciphertext_len < GST_AES_BLOCK_SIZE`) treats exactly 16 bytes as valid. Flagged by
static review (findings.toml PANIC_10) and escalated here to a real reproduction; same plugin as
candidate 026 (`aes` infra already built in this workspace).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + a real GStreamer pipeline (`appsrc ! aesdec ! fakesink`, gcc debugoptimized build, no sanitizer needed - this is a plain `g_assert` abort, not a memory-safety bug) | `gst-plugins-bad:aes=enabled` only; a real `appsrc` pushes a real 16-byte `GstBuffer` (IV only, zero ciphertext) through `push-buffer`/the real pad-push/chain path into a real `aesdec` (`serialize-iv=true`) | `SIGABRT` (g_assert failure) reproduced deterministically on the first attempt |

Only one approach was needed - the static-analysis lead already pointed at the exact off-by-one
comparison, and driving the real element through a real pipeline with the boundary value (exactly
16 bytes) reproduced it immediately, so no further escalation (ASAN/fuzzing) was warranted per the
soft cap in `SKILL.md` - this class of bug (a `g_assert` on an externally-controlled length) needs
no sanitizer to observe.

## Status

**Confirmed.** Reproduces 4/4 runs (this session: an initial repro plus 3 explicit repeat runs),
deterministic - `g_assert` aborts identically every time `in_size == GST_AES_BLOCK_SIZE` on the
first buffer with `serialize-iv=true`, no flakiness, no memory-layout dependence (a plain integer
comparison, not UB).

**Fully reproducible**: **Yes**. `repro/repro.c` builds a real `GstPipeline`
(`appsrc ! aesdec ! fakesink`), with the real `aesdec` element instantiated via
`gst_element_factory_make` (loaded through the real plugin-scanner/registry mechanism) and
configured with `serialize-iv=true` - the documented mode where aesdec expects the IV serialized
into the first 16 bytes of the first buffer, exactly as a real upstream element (a network source,
or a file/socket source that happened to do a short first read) would deliver it. A real 16-byte
`GstBuffer` is pushed into `appsrc` via its public `push-buffer` signal, which drives the buffer
through GStreamer's real `gst_pad_push` -> `gst_pad_chain_data_unchecked` ->
`gst_base_transform_chain` -> `default_generate_output` -> `bclass->prepare_output_buffer` call
path (confirmed via the captured gdb backtrace during a related follow-up, see below) - not a
hand-built stand-in for that path. `CONFIRMATION.md` section 11 is satisfied: appsrc genuinely
produces and pushes the buffer through the real GStreamer buffer-passing machinery; nothing about
the crash depends on an assumption about what a real upstream element "would" send that wasn't
actually exercised.

## Class

crash (an internal `g_assert` aborting the whole process on ordinary, attacker/peer-controlled
input length - not memory-unsafe, but a remotely-triggerable denial of service)

## Root cause

`gst_aes_dec_prepare_output_buffer()` runs *before* `gst_aes_dec_transform()` in
`GstBaseTransform`'s own pipeline (`default_generate_output()` calls `prepare_output_buffer` first,
and only calls `transform()` if that returns `GST_FLOW_OK` with a non-NULL `*outbuf`). When
`serialize_iv` is set and this is the first buffer, `prepare_output_buffer()` needs to shrink the
output-buffer size estimate by `GST_AES_BLOCK_SIZE` (16 bytes) to account for the IV that
`transform()` will strip later - and guards this with `g_assert (in_size > GST_AES_BLOCK_SIZE)`.
But `in_size` is `gst_buffer_get_size(inbuf)`, i.e. the size of the buffer as delivered by
whatever upstream element/peer produced it - entirely external, attacker/peer-controlled input,
not an internal invariant a `g_assert` is meant to protect. `gst_aes_dec_transform()`'s own check
on the same boundary (`if (ciphertext_len < GST_AES_BLOCK_SIZE) { GST_ELEMENT_ERROR(...); goto
cleanup; }`, i.e. it *accepts* `ciphertext_len == GST_AES_BLOCK_SIZE`) never gets a chance to run
for `in_size == GST_AES_BLOCK_SIZE`, because `prepare_output_buffer()`'s stricter `>`-only assert
aborts the process first. Any real upstream element that ever hands aesdec a first buffer of
exactly 16 bytes (a short first read from a socket/file, a deliberately malicious peer, or simply
a stream whose ciphertext happens to be empty after the IV) crashes the whole pipeline/process,
not just this element.

A related second crash was found and fixed as part of verifying this one (see "The fix" below):
once the assert boundary is corrected to accept `in_size == GST_AES_BLOCK_SIZE`, the same 16-byte
input reaches `gst_aes_dec_transform()`, where `ciphertext_len` becomes 0 after the IV is
stripped, `EVP_CipherUpdate` produces `plaintext_len == 0`, and (with the default
`per-buffer-padding=TRUE`) the line `padding = plaintext[plaintext_len - 1];` reads
`plaintext[-1]` - an out-of-bounds read that segfaults. This is the same underlying design flaw
(the "first buffer is exactly the IV, zero ciphertext" case was never actually exercised anywhere
in this code, because `prepare_output_buffer()`'s assert always aborted first) manifesting a
second time once the first crash is removed - both had to be fixed together to actually eliminate
the crash for this input, not just change its shape (see `CONFIRMATION.md` section 6).

## Introduced by

5cf4dc2b82f10cbe2dc603fd49ab080ff8d32e56 "aes: add aes encryption and decryption elements"
(2021-07-06). This commit added the entire `ext/aes` plugin in one shot, including
`gst_aes_dec_prepare_output_buffer()` with the original strict-`>` assert
(`g_assert (gst_buffer_get_size (inbuf) > GST_AES_BLOCK_SIZE);`, confirmed present verbatim in
this commit's own version of the file). e00b052b779e8e45a1949ebb9428ab8c727ce87f "aesdec: use
gsize for buffer sizes and fix log format" (2025-12-17) later refactored the surrounding code
(introduced the `in_size` local, changed types/log format) but did not change the comparison
operator - `git blame` attributes the literal line to that later commit only because of the
refactor, not because the bug was introduced then. The bug has been present, unmodified in
substance, since the plugin's initial commit.

## The fix

Two changes to `subprojects/gst-plugins-bad/ext/aes/gstaesdec.c`, both required to actually
eliminate the crash for this input class (not just change its shape):

1. In `gst_aes_dec_prepare_output_buffer()`: replaced the `g_assert (in_size > GST_AES_BLOCK_SIZE)`
   with a real, graceful bounds check (`if (in_size < GST_AES_BLOCK_SIZE) { ... return
   GST_FLOW_ERROR; }`), matching the exact boundary `gst_aes_dec_transform()` itself already
   accepts (`==` is valid: IV only, zero ciphertext). Returning `GST_FLOW_ERROR` with `*outbuf =
   NULL` is the base class's own documented contract for a failed `prepare_output_buffer`
   (`gstbasetransform.c`'s `default_generate_output()`: `if (ret != GST_FLOW_OK || *outbuf == NULL)
   goto no_buffer;` - it stops cleanly without ever calling `transform()`), so this is not a new
   error path invented for this fix, just using the one the class already provides instead of
   aborting.
2. In `gst_aes_dec_transform()`: guarded the per-buffer-padding sanity-check block with
   `plaintext_len > 0` (previously unconditional whenever `per_buffer_padding` was set) - closes
   the `plaintext[-1]` out-of-bounds read described above, which fix 1 alone would have newly
   exposed (it was already latently reachable in principle for any input that decrypts to zero
   plaintext bytes, but the assert removed in fix 1 was the only way to actually reach it with
   `serialize_iv=true`, so fixing 1 without 2 would have turned a `SIGABRT` into a `SIGSEGV` for
   the exact same input rather than a clean fix).

Verified against more than the one triggering input (per `CONFIRMATION.md` section 7):
- The exact original repro (16-byte first buffer, `serialize-iv=true`): before the fix, aborts
  4/4 runs (`g_assert (in_size > GST_AES_BLOCK_SIZE)` failing, `gstaesdec.c:548`); after the fix,
  succeeds 5/5 runs (`GST_FLOW_OK`, `EOS` posted, zero-length output, no crash).
- A 15-byte first buffer (one byte *shorter*, the case `gst_aes_dec_transform()` was always
  supposed to reject): before the fix, also aborts via the same assert (never reaches
  `transform()`'s own graceful check at all); after the fix, correctly produces a
  `GST_ELEMENT_ERROR` ("Cipher text too short") and a clean pipeline error, no crash.
- A full `aesenc | aesdec` round trip on ordinary, non-block-aligned plaintext (57 bytes, real
  encryption then real decryption, `serialize-iv=true`, default `per-buffer-padding=TRUE`): after
  the fix, decrypts back to the exact original plaintext byte-for-byte - confirms the
  `plaintext_len > 0` guard added in fix 2 doesn't change behavior for the normal, non-empty-output
  case.

All three checks are in `repro/boundary.c`, run via `repro/run.sh boundary`; the original crash
repro is `repro/repro.c`, run via `repro/run.sh repro`. Full captured output (before and after) is
in `output.txt`.

- Project test suite (`gst-plugins-bad:elements_aesenc`, `gst-plugins-bad:elements_aesdec`, the
  only tests covering this plugin): 12/12 passing before the fix (6/6 each suite), 12/12 passing
  after (6/6 each suite, unchanged) - the existing suite doesn't exercise the exactly-16-byte or
  15-byte first-buffer boundary at all (its own serialize-iv test cases use 16-byte "text16" and
  17-byte "text17" *plaintext* payloads, which after AES padding overhead are always well above
  the `GST_AES_BLOCK_SIZE` boundary once the IV is prepended), so it neither caught the original
  bug nor is affected by the fix.

## What's not done / open questions

- `gstaesenc.c` (the encoder side of the same plugin) was not audited for an equivalent
  boundary-value issue - it doesn't have a `serialize_iv`-stripping code path in
  `prepare_output_buffer` (the encoder is the one that *generates* the serialized IV, not consumes
  it), so this specific bug class doesn't obviously apply there, but wasn't independently checked
  line-by-line this session.
- No fuzzing campaign was run against this plugin (already noted as not run in candidate 026's own
  `RESULT.md`); the boundary-value approach was sufficient to find and fully characterize both
  crashes here.
- The `plaintext[-1]` read described in "Root cause" (fixed as part of this candidate's fix 2) was
  found only as a consequence of fixing bug 1, not through independent investigation - it's
  plausible a similar "decrypts to zero bytes" condition is reachable through some other input
  shape with `serialize_iv=FALSE` (e.g. an empty non-first buffer); this wasn't checked, since
  `awaiting_first_buffer` is only relevant to `serialize_iv=TRUE` and the guard added in fix 2 is
  unconditional on `serialize_iv`, so it should already cover that case too, but no dedicated
  input was constructed to verify a non-serialize-iv empty-plaintext buffer specifically.
