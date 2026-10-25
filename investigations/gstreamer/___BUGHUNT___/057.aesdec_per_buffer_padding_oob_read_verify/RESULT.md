# aesdec_per_buffer_padding_oob_read_verify

`gst-plugins-bad/ext/aes/gstaesdec.c` `gst_aes_dec_transform()`'s `per_buffer_padding` path reads
`padding = plaintext[plaintext_len - 1]` without checking `plaintext_len != 0` first (OpenSSL can
legitimately produce 0 output bytes for a partial block when its own padding is disabled), and the
subsequent unpad loop reads up to `padding - 1` further bytes before that, with no check that
`padding <= plaintext_len`. Flagged by `findings.toml` (`OVF_11`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end repro: `GstHarness("aesdec")` with `key`/`iv` set, `serialize-iv=FALSE`, `per-buffer-padding=TRUE`, pushing a real 5-byte ciphertext buffer (not a multiple of the 16-byte AES block size) through the real, unmodified element via `gst_harness_push()` | Scoped meson build (core+base+bad/aes only), plain debug, run under `valgrind` | The vulnerable code path is genuinely reached: `GST_DEBUG` output confirms `EVP_CipherUpdate()` produced `plaintext_len=0` for the 5-byte input and the code read `padding = plaintext[-1]`, which happened to evaluate to `0`, triggering the "Illegal PKCS7 padding value 0" rejection. `valgrind` reported 0 errors for this read. |
| Same harness, rebuilt with a scoped ASAN+UBSAN build (`clang -fsanitize=address,undefined`, same plugin scope) | Same input | AddressSanitizer **also** reported no heap-buffer-underflow for the same `plaintext[-1]` read (only unrelated harness-teardown leak reports, expected noise for a partial `GstHarness` cleanup). Read `_sysmem_new_block()`'s allocation layout (`gstallocator.c`) directly to understand why: the default `GstAllocator` always places a `GstMemorySystem` header (`GstMemory mem; guint8 *data; gpointer user_data; GDestroyNotify notify;` - comfortably more than `GST_AES_BLOCK_SIZE - 1 = 15` bytes) in the **same malloc'd block**, immediately before the `data` pointer that `gst_buffer_map()` hands back as `outmap.data`/`plaintext`. Any read of up to 15 bytes before `plaintext` therefore lands inside that header, still within the real heap allocation - never crossing into an ASAN/valgrind-detectable redzone or unmapped page. |

## Status

**Not reproducible** (as a memory-safety, allocation-crossing OOB read). The code path is real and
genuinely reads `plaintext[plaintext_len - 1 - k]` for `k` up to `padding - 1` when `plaintext_len`
is smaller than that - which is undefined behavior relative to the *logical* bounds of the
`plaintext` array the code itself declares (`plaintext_len` bytes) - but it can never actually
escape the real underlying heap allocation via the entry point exercised here (the real
`gst_aes_dec_prepare_output_buffer()` -> `gst_buffer_new_allocate(NULL, ...)` -> default sysmem
allocator path), because that allocator unconditionally reserves a `GstMemorySystem`-sized header
(well over the maximum possible 15-byte read distance) immediately before the data pointer, in the
same allocation.

## Root cause

Not a bug in the memory-safety sense reachable through the real element - see "What was tried"
above for the specific reason: GStreamer's default buffer memory layout always keeps a large
allocator-private header immediately before the data pointer any `GstMapInfo`/`plaintext` view
exposes, which happens to fully absorb this specific class of small (`< GST_AES_BLOCK_SIZE`)
before-the-pointer read. The missing `plaintext_len != 0` / `padding <= plaintext_len` checks are
still a real logic wart (reading allocator-internal header bytes and treating them as "padding
data" is nonsensical, even if harmless from a memory-safety standpoint) and could misbehave if a
future refactor ever changed how the output buffer is allocated (e.g. a caller supplying a
tightly-packed externally-allocated memory with no such header, via a custom `GstAllocator` or
`gst_buffer_new_wrapped()`), so it's still worth fixing defensively - just not something this
session could demonstrate as an actual crash/OOB through the real, current call path.

## What's not done / open questions

- Not checked whether any *other* caller could hand `gst_aes_dec_transform()` an `outbuf` backed
  by a non-default, header-less allocator (the real element always uses its own
  `prepare_output_buffer()`, which always calls `gst_buffer_new_allocate(NULL, ...)` - the default
  allocator - so this doesn't appear reachable through the real element as currently written, but
  wasn't independently verified for every possible `GstBaseTransform` configuration).
- A defensive fix (checking `plaintext_len != 0` and `padding <= plaintext_len` before the reads,
  matching `findings.toml`'s suggested shape) was not applied this session, since there's no
  verified reproduction to confirm a fix against - would be speculative hardening rather than a
  verified bug fix. Worth doing anyway given the low cost, but left to a session that wants to do
  defensive hardening independent of a confirmed reproduction.
