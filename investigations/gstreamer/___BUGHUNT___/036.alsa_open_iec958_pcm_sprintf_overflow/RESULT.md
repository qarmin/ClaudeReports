# alsa_open_iec958_pcm_sprintf_overflow

`gst_alsa_open_iec958_pcm()` in `subprojects/gst-plugins-base/ext/alsa/gstalsa.c` (~line 650-687)
formats the element's `device` string into a fixed 256-byte stack buffer with plain `sprintf()`
and zero bounds checking. Flagged by `code-audit-structured`'s `findings.toml` as OVF_22.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Debug-assertions + clang ASAN/UBSAN build, real end-to-end trigger path | `--buildtype=debugoptimized -Db_sanitize=address,undefined`, scoped to core gstreamer + gst-plugins-base with `alsa=enabled`; real `alsasink` element driven via `g_object_set("device", <250-char string>)` + `GST_STATE_READY` + `gst_pad_query_caps()` | AddressSanitizer: stack-buffer-overflow in `vsprintf`/`sprintf`, inside `gst_alsa_open_iec958_pcm` at gstalsa.c:669, writing 293 bytes into the 256-byte `devstr` - reproduced 5/5 runs |

Only one approach was needed (tool 1 of the soft cap of 3) - the very first run under ASAN produced
a clean, deterministic, fully-attributed stack-buffer-overflow with no ambiguity, so no further
escalation (miri does not apply to C; a fuzz harness would have added nothing over the direct
boundary-value construction already used).

## Status

**Confirmed**

Reproduces 5/5 runs under a clang `-fsanitize=address,undefined` `debugoptimized` build (see
`output.txt` for one full captured run). The crash is a deterministic AddressSanitizer
stack-buffer-overflow, not layout/seed-dependent - every run with the same >212-character device
name hits it.

**Fully reproducible**: **Yes**.

`repro/repro.c` drives the real, public `alsasink` GObject "device" property via `g_object_set()`
(exactly as `g_object_set()` on an `alsasink`-family element from any real application would), then
performs a real `GST_STATE_NULL -> GST_STATE_READY` transition (running the real
`gst_alsasink_open()`, which really calls `snd_pcm_open()` with the attacker-controlled device
string) and a real `gst_pad_query_caps()` on the element's real sink pad - exactly what a real
pipeline does during caps negotiation. That is what actually invokes
`gst_alsasink_getcaps() -> gst_alsa_probe_supported_formats() -> gst_alsa_open_iec958_pcm()`,
reusing the very same device string, hitting the vulnerable `sprintf()`. No internal/private
gstalsa.c API is called directly.

The only thing not physically real is which ALSA PCM name `snd_pcm_open()` resolves to: the repro
points `ALSA_CONFIG_PATH` at a small config file it writes itself, defining a PCM of type `null`
under the long device name. This is the same real, standard ALSA mechanism any user's own
`~/.asoundrc` uses to define a named PCM alias (arbitrary name, arbitrary length) - it requires no
real audio hardware, matching the candidate description. This is a single-participant repro (one
local process, its own local ALSA config, its own real GObject property) - there is no second
peer/caller being impersonated, so CONFIRMATION.md section 11's two-participant bar does not apply
here; the "device" property string is the actual external input, supplied through the actual
GObject API a real caller would use.

## Class

memory-safety (stack-buffer-overflow)

## Root cause

`gst_alsa_open_iec958_pcm()` declares `char devstr[256]` and formats it with plain `sprintf()`:

```c
sprintf (devstr,
    "%s:{AES0 0x%02x AES1 0x%02x AES2 0x%02x AES3 0x%02x}",
    device, ...);
```

The format's own literal text (excluding `%s`) is ~43 bytes, so any `device` string longer than
about 212 characters overflows `devstr`. `device` is the alsasink/alsasrc `"device"` GObject
property (`gchar *`), user-settable via `g_object_set()` with no length restriction anywhere in the
property's `GParamSpec` or in `gst_alsasink_open()`/`gst_alsa_probe_supported_formats()`. The
function is reached from two real call sites, both passing the same unbounded string:
`gst_alsasink_prepare()` (`ext/alsa/gstalsasink.c:948`, when the negotiated caps are a passthrough
format like AC3/DTS/MPEG) and `gst_alsa_probe_supported_formats()` (`ext/alsa/gstalsa.c:767`,
called from both `gst_alsasink_getcaps()` and `gst_alsasrc_getcaps()` whenever the element's ALSA
handle is already open and downstream/upstream queries caps) - i.e. via ordinary caps negotiation,
not just the IEC958-specific code path.

## Introduced by

f0154849b0bfafa7bb8cd690389afc824be9e90e "ext/alsa/gstalsa.c: Probe for IEC958 pcm to detect if we
can do SPDIF output." (2008-02-29). This is the commit that added `gst_alsa_open_iec958_pcm()`
(then in the pre-monorepo gst-plugins-base tree) with the `sprintf()` into `devstr[256]` exactly as
it exists today - the bug has been present, unchanged, since this function's introduction.

## The fix

Replaced `sprintf()` with `g_snprintf(devstr, sizeof (devstr), ...)`, checked the returned length
against `sizeof (devstr)`, and made the function fail gracefully (`GST_WARNING_OBJECT` + `return
NULL`, the same "couldn't open IEC958 device" outcome the function already produces on any other
`snd_pcm_open()` failure) instead of writing past the buffer when the device name doesn't fit.

```c
-  sprintf (devstr,
+  len = g_snprintf (devstr, sizeof (devstr),
       "%s:{AES0 0x%02x AES1 0x%02x AES2 0x%02x AES3 0x%02x}",
       device, ...);
+  if (G_UNLIKELY (len < 0 || len >= (gint) sizeof (devstr))) {
+    GST_WARNING_OBJECT (obj, "device name too long, not opening IEC958 device");
+    return NULL;
+  }
```

Verified against the same `repro/repro.c` (250-char device name, the same input that originally
triggered the finding): no more ASAN report, 3/3 clean runs. Also verified against additional
lengths in the same bug class, per CONFIRMATION.md section 7 (not just the exact original input):
length 213 (the largest that still fits in `devstr[256]` - formats and opens fine, no behavior
change), length 214 (the smallest that would have overflowed by 1 byte under the old code - now
returns NULL cleanly instead of overflowing), and lengths 1000/3000 (far beyond the buffer) - all
4 additional lengths ran clean under ASAN with no crash.

Also re-confirmed, immediately before re-verifying the fix, that reverting only this hunk (`git
checkout` on gstalsa.c, rebuild, nothing else changed) makes the exact same repro crash again
(2/2 runs) - isolating that this specific hunk, not some other change, is what fixes it
(CONFIRMATION.md section 6).

- Project test suite: gst-plugins-base's 94 non-`validate.*` `meson test` unit/check binaries
  (the `validate.*` tests need `-Ddevtools=enabled`, out of scope for this minimal alsa-only build,
  and were not run) - **80 pass / 14 fail before, 80 pass / 14 fail after patch, identical failure
  set both times** (all 14 are GL-context tests: `elements_glfilter`, `elements_glimagesink`,
  `elements_glmixer`, `elements_glstereo`, `libs_gstglcolorconvert`, `libs_gstglcontext`,
  `libs_gstglheaders`, `libs_gstglmemory`, `libs_gstglquery`, `libs_gstglshader`,
  `libs_gstglslstage`, `libs_gstglupload`, `libs_video`, `pipelines_gl_launch_lines` - failing in
  both the pristine and patched builds because this sandbox has no GPU/display, unrelated to the
  patch). No ALSA-specific test exists in the suite at all (`alsasink`/`alsasrc` are explicitly
  listed in `GST_STATE_IGNORE_ELEMENTS` in `tests/check/meson.build`, and no test references
  `gst_alsa_open_iec958_pcm`/`iec958` anywhere under `tests/`), so this patch has zero prior test
  coverage either way - the before/after run only demonstrates no regression elsewhere, not
  regression coverage of the fixed function itself.

## What's not done / open questions

- `gst_alsa_probe_supported_formats()` (the caller at gstalsa.c:767) and
  `gst_alsasink_prepare()`'s direct call (gstalsasink.c:948) are the only two call sites of
  `gst_alsa_open_iec958_pcm()`; both were exercised (the repro goes through the
  `gst_alsa_probe_supported_formats()` path). The `gst_alsasink_prepare()` path (triggered by
  negotiating AC3/DTS/MPEG passthrough caps rather than by a caps query) was not separately
  repro'd, but it calls the exact same vulnerable function with the exact same `device` string, so
  the same fix covers it.
- No other `sprintf`/fixed-buffer pattern was searched for elsewhere in `gstalsa.c` or the rest of
  `ext/alsa/` this session - only the one function flagged by the audit (OVF_22) was investigated,
  per "Critical pitfalls" #7 in SKILL.md this is not a clean bill of health for the rest of the
  alsa plugin.
- No official upstream GStreamer fuzz target exists for the alsa plugin (it's excluded from
  `ci/fuzzing`'s scope, which makes sense - it needs a `device` property, not a byte-stream input),
  so a longer fuzzing campaign would not add anything here; this was a boundary-value/property-input
  bug, not a byte-stream-parsing one.
