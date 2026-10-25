# gsthlsdemux_finish_fragment_pkcs7_negative_resize

`gst_hls_demux_finish_fragment()` in `subprojects/gst-plugins-bad/ext/hls/gsthlsdemux.c`
(~line 1076-1089) performs manual PKCS7 unpadding on a decrypted AES-128 HLS fragment:
`unpadded_size = info.size - info.data[info.size - 1];` where the last decrypted byte is fully
controlled by whoever serves the HLS media segment/key, then passes `unpadded_size` (a `gssize`)
straight into `gst_buffer_resize()`. Flagged by static review (findings.toml OVF_18), same plugin
as previously-confirmed candidates #027/#028.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + real end-to-end pipeline | debugoptimized build (`gst-plugins-bad:hls=enabled`, `hls-crypto=openssl`, `gst-plugins-good:soup=enabled`); real local HTTP server (`repro/serve.py`) serving a crafted AES-128-CBC media playlist/key/16-byte "segment" whose decrypted last byte is 0xFF; real `souphttpsrc ! hlsdemux ! fakesink` pipeline via the project's own `gst-launch-1.0`, `G_DEBUG=fatal-warnings` | `GStreamer-CRITICAL **: gst_buffer_resize_range: assertion 'size >= -1' failed`, process `SIGABRT` (exit 134) - reproduced immediately on the first attempt |

Only one approach was needed: this is a plain out-of-range value fed to a `g_return_val_if_fail`
guarded function, not a memory-safety UB class that would need a sanitizer/fuzzer to surface, and
the static-analysis lead already pinned the exact vulnerable line. A second, different boundary
value (last byte 0x80, i.e. `unpadded_size = 16 - 128 = -112`, vs. the primary case's
`16 - 255 = -239`) was also constructed and confirmed to hit the same crash, per
`CONFIRMATION.md` section 7 (checking the fix generalizes, not just the one triggering byte).

## Status

**Confirmed.** Reproduces 5/5 runs (an initial run plus 4 explicit repeat runs across fresh
processes/fresh ephemeral HTTP-server ports), deterministic - `unpadded_size` is a pure function
of the crafted ciphertext's last decrypted byte and the AES key/IV, both fully controlled by this
repro, no timing/memory-layout dependence. Both the primary (0xFF) and boundary (0x80) values
reproduce identically, each 1/1 in the pre-fix build (see "The fix" below for their post-fix
behavior).

**Fully reproducible**: **Yes**. `repro/run.sh` starts a real local HTTP server
(`repro/serve.py`, `http.server.ThreadingHTTPServer`) serving the pre-baked crafted playlist/key/
segment files in `repro/www/`, then drives the real, unmodified `hlsdemux` and `souphttpsrc`
elements (loaded through the real GStreamer plugin registry, invoked via the project's own public
`gst-launch-1.0` tool - not a hand-built stand-in for the pipeline) with
`souphttpsrc location=http://127.0.0.1:<port>/media.m3u8 ! hlsdemux ! fakesink`. `souphttpsrc`
genuinely fetches the media playlist, the AES key, and the encrypted segment over a real HTTP
connection to that server; `hlsdemux` genuinely decrypts the fetched segment via its real OpenSSL
AES-128-CBC decrypt path and runs the real, unmodified `gst_hls_demux_finish_fragment()`. This
satisfies `CONFIRMATION.md` section 11: the local HTTP server is a real second participant
(standing in for a malicious/compromised HLS origin, exactly the real-world threat model - "the
HLS server fully controls both the AES key file and the ciphertext" from the original finding),
not hand-constructed internal state fed directly to the vulnerable function.

## Class

crash (a `g_return_val_if_fail`-guarded internal assertion reachable with fully attacker/server-
controlled input, aborting the whole process under `G_DEBUG=fatal-warnings`/`--gst-fatal-warnings`
- common in CI/dev builds; on a build where GLib warnings are non-fatal, `gst_buffer_resize()`
instead silently fails and leaves the fragment buffer un-unpadded, corrupting the demuxed stream -
see "Root cause" below)

## Root cause

`gst_hls_demux_finish_fragment()` unconditionally trusts the value of the final decrypted byte of
a fragment as a PKCS7 padding length:

```c
unpadded_size = info.size - info.data[info.size - 1];
...
gst_buffer_resize (hls_stream->pending_decrypted_buffer, 0, unpadded_size);
```

`info.data[info.size - 1]` is the last byte of the AES-128-CBC-decrypted fragment content. Both
the AES key (fetched from a URI the playlist itself specifies) and the ciphertext (the fragment
body) are supplied by whatever serves the HLS playlist/segments, so this byte is entirely
attacker/server-controlled. A byte value greater than `info.size` (a real, valid PKCS7 padding
byte is normally in `1..=16` for AES-128, but nothing here checks that) makes `unpadded_size`
negative - e.g. a single-AES-block (16-byte) fragment with a final decrypted byte of `0xFF` (255)
gives `unpadded_size = 16 - 255 = -239`.

`gst_buffer_resize()` forwards straight to `gst_buffer_resize_range()`
(`subprojects/gstreamer/gst/gstbuffer.c`), which only guards against `size < -1`:

```c
g_return_val_if_fail (size >= -1, FALSE);
```

`-239 < -1`, so this `g_return_val_if_fail` fails, emitting a `GLib`-level `g_critical()`
("`GStreamer-CRITICAL **: gst_buffer_resize_range: assertion 'size >= -1' failed`"). Under
`G_DEBUG=fatal-warnings` (or the project's own `--gst-fatal-warnings`, routinely used in CI/test
runs), `g_critical()` is fatal and aborts the whole process (`SIGABRT`). Without a fatal-warnings
setting, `gst_buffer_resize_range()` returns `FALSE` and `gst_buffer_resize()`'s return value is
discarded (the call site never checks it), so the buffer is silently left at its original,
still-padded size and handed on to the rest of the pipeline as if it had been correctly unpadded -
stream corruption instead of a crash.

## Introduced by

b9124cad88 "hlsdemux: Implement pkcs7 unpadding" (2013-07-23, Sebastian Dröge) - the commit that
first added this exact unchecked `unpadded_size = info.size - info.data[info.size - 1]` /
`gst_buffer_resize()` pair to hlsdemux; the code has moved around within the file (and across the
2018 gst-plugins-bad-into-monorepo merge, commit 019971a3c7) since, but the vulnerable computation
itself is unchanged from its original form.

## The fix

Validate `unpadded_size` against the actual buffer size before resizing, rejecting the fragment
with a clean stream error instead of passing an out-of-range value to `gst_buffer_resize()`:

```c
        if (unpadded_size < 0 || (gsize) unpadded_size > info.size) {
          GST_ELEMENT_ERROR (demux, STREAM, DECODE,
              ("Invalid PKCS7 padding in decrypted fragment"), (NULL));
          gst_buffer_replace (&hls_stream->pending_decrypted_buffer, NULL);
          return GST_FLOW_ERROR;
        }
```

(applied directly before the existing `gst_buffer_resize()` call). This matches the fix suggested
in findings.toml OVF_18.

Verified per `CONFIRMATION.md` section 7 (checked against more than the one triggering input):
`repro/run.sh primary` (last byte 0xFF) and `repro/run.sh boundary` (last byte 0x80, a different
value in the same "invalid PKCS7 padding length" class) both re-run against the patched tree - see
`output.txt` for the full captured before/after output of both. Before the fix, both abort the
process (`GStreamer-CRITICAL **: gst_buffer_resize_range: assertion 'size >= -1' failed`,
exit 134/SIGABRT). After the fix, both instead produce a clean
`ERROR: from element .../GstHLSDemux:hlsdemux0: Invalid PKCS7 padding in decrypted fragment`,
`GST_FLOW_ERROR` propagates up, the pipeline fails to preroll gracefully, and `gst-launch-1.0`
exits with status 1 - no assertion, no crash.

- Project test suite: `meson test --suite gst-plugins-bad` (this session's scoped build:
  `gst-plugins-bad:hls=enabled`/`hls-crypto=openssl` + `gst-plugins-good:soup=enabled`, everything
  else at this workspace's usual scope) - **103 pass / 1 fail / 5 skip before -> 103 pass / 1 fail
  / 5 skip after** (`elements_vmaf` fails identically before and after, an environment issue
  unrelated to HLS - confirmed pre-existing/unrelated in prior candidates in this workspace, e.g.
  #027). One additional test, `libs_vkwindow`, failed on the very first "before" run (a Vulkan
  windowing test that times out at 20s) but passed on an immediate re-run with no code changes at
  all - confirmed as pre-existing flakiness in this sandboxed/headless environment, unrelated to
  `gsthlsdemux.c` (this patch touches nothing outside `ext/hls/gsthlsdemux.c`). No test anywhere
  in the suite exercises `hlsdemux`'s encryption/PKCS7-unpadding path (the project's own
  `elements_hls_demux` check test - see #027's `RESULT.md` "What's not done" - never gets
  configured in this build at all, an independent, already-noted issue), so no test's pass/fail
  status was expected to change as a direct result of this fix, and none did.

## What's not done / open questions

- The two other cipher backends compiled by the same `#if defined(HAVE_OPENSSL)` /
  `#elif defined(HAVE_NETTLE)` / `#else` (libgcrypt) ladder around `decrypt_fragment()` share the
  exact same call site and unpadding code in `gst_hls_demux_finish_fragment()` - the vulnerable
  line itself is backend-independent (it operates on the already-decrypted plaintext, not on
  anything backend-specific), so the same fix covers all three crypto backends without needing a
  backend-specific repro; only the OpenSSL backend was actually built/exercised this session
  (`hls-crypto=openssl`).
- No project unit test covers hlsdemux's AES-128/PKCS7-unpadding path at all (see "Project test
  suite" above) - a longer fuzzing campaign or a new unit test targeting `m3u8.c`'s
  `EXT-X-KEY`/`decrypt_fragment()`/`gst_hls_demux_finish_fragment()` trio (via
  `fuzz-bug-repro`, run separately) would be worth pointing at this code given it had zero prior
  coverage and a real, confirmed crash was found on the first attempt.
