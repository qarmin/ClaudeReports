# libav_dsd_planar_div_by_zero

`subprojects/gst-libav/ext/libav/gstavdemux.c`'s `gst_ffmpegdemux_loop()` (~line 1555-1577), for
`AV_CODEC_ID_DSD_LSBF_PLANAR`/`AV_CODEC_ID_DSD_MSBF_PLANAR` streams, computes
`int num_bytes_per_channel = pkt.size / num_channels;` where `num_channels` comes straight from
`avstream->codecpar->ch_layout.nb_channels`, with no local check that it's non-zero - looked
suspicious as an attacker-controlled integer division-by-zero (SIGFPE) reachable by demuxing a
malicious container that declares a DSD planar stream with 0 channels (source: `findings.toml`
PANIC_9).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Grep the entire bundled FFmpeg source (`subprojects/FFmpeg`, meson-port pin `cde7209`, 2025-09-16, libavutil 59.39.100) for every place that assigns `AV_CODEC_ID_DSD_LSBF_PLANAR`/`AV_CODEC_ID_DSD_MSBF_PLANAR` to a stream's `codecpar->codec_id` | `grep -rln` over `libavformat/*.c` `libavcodec/*.c` | Exactly one producer in the whole codebase: `libavformat/dsfdec.c` (the DSF/"DSD Stream File" demuxer, wrapped by GStreamer's `avdemux_dsf` element). The two other files that reference DSD codec IDs at all (`libavformat/iff.c` for DSDIFF, `libavformat/wsddec.c` for WavPack-DSD-ish WSD) both only ever assign the non-planar `AV_CODEC_ID_DSD_MSBF`, never the `_PLANAR` variants - so they can never reach this switch case in `gstavdemux.c` at all. |
| Read `dsf_read_header()` in `libavformat/dsfdec.c` line by line, tracing how `st->codecpar->ch_layout.nb_channels` is derived from the file's `fmt ` chunk (`channel_type` field looked up in a fixed `dsf_channel_layout[]` table, falling back to the file's raw `channels` field only when `channel_type` maps to `AV_CHANNEL_ORDER_UNSPEC`, i.e. `nb_channels == 0` from the table) | Source reading only | Found an unconditional guard a few lines after that assignment, still inside `dsf_read_header()` and strictly *before* the `switch` that assigns `AV_CODEC_ID_DSD_LSBF_PLANAR`/`AV_CODEC_ID_DSD_MSBF_PLANAR`:<br>`if (st->codecpar->ch_layout.nb_channels <= 0) return AVERROR_INVALIDDATA;`<br>So by the time `dsf_read_header()` could ever set one of the two `_PLANAR` codec IDs, `nb_channels` is already provably `> 0` - the file is rejected outright (`avformat_open_input()` fails) before FFmpeg would ever produce the state the finding assumes. |
| Real end-to-end repro: built gst-libav (clang, `--buildtype=debug`, bundled FFmpeg subproject, core+base+libav only) in an isolated worktree, crafted a malicious `.dsf` file by hand (`make_dsf.py`, copied into this folder) with `channel_type=0` (`AV_CHANNEL_ORDER_UNSPEC`) and `channels=0` in the `fmt ` chunk - the only way to get `nb_channels` down to 0 per the trace above - and ran it through the real `avdemux_dsf` GStreamer element via `gst-launch-1.0 filesrc location=malicious.dsf ! avdemux_dsf ! fakesink`, 3 repeat runs | `gst-launch-1.0`, `ulimit -c 0`, full pipeline (real filesrc + real element, not a hand-built internal-state call) | 3/3 runs: clean `GST_FLOW_ERROR`/`Unhandled error code received` from `gst_ffmpegdemux_open()` (`gstavdemux.c:1400`), pipeline fails to preroll, process exits with code 1 - no SIGFPE, no crash, no core dump. `gst_ffmpegdemux_loop()` (where the division lives) is never reached at all, because `avformat_open_input()` itself already failed inside `dsf_read_header()`. Full output: `output.txt`. |

## Status

**Not reproducible.** The only producer of `AV_CODEC_ID_DSD_LSBF_PLANAR`/`AV_CODEC_ID_DSD_MSBF_PLANAR`
anywhere in the bundled FFmpeg is `libavformat/dsfdec.c`'s DSF demuxer, and it already validates
`ch_layout.nb_channels > 0` unconditionally, before ever assigning either of those two codec IDs -
so `gst_ffmpegdemux_loop()`'s `num_channels` can never be 0 for a stream carrying one of these
codec IDs, via the real end-to-end path (a crafted file demuxed through the real `avdemux_dsf`
element). Covered: manual FFmpeg source read (exhaustive grep across `libavformat`/`libavcodec` for
every assignment site of these two codec IDs) plus a real crafted-file repro run 3/3 times through
the actual GStreamer element with `ulimit -c 0`, no crash observed. This is a clean negative for
the real trigger path this candidate names; see "What's not done" below for the residual, narrower
concern this leaves open.

**Fully reproducible**: N/A - not a bug via the real end-to-end path, this column doesn't apply.

## Class

N/A (Not reproducible).

## Root cause

Not a reachable bug via the real trigger path described in the finding. `gstavdemux.c`'s own
division genuinely has no local zero-check (the finding's description of the code itself is
accurate), but the *only* caller that can ever hand it a stream with codec ID
`AV_CODEC_ID_DSD_LSBF_PLANAR`/`AV_CODEC_ID_DSD_MSBF_PLANAR` is FFmpeg's own `dsf_read_header()`
(`libavformat/dsfdec.c`), which independently enforces `nb_channels > 0` a few lines before setting
either of those two codec IDs:

```c
st->codecpar->sample_rate  = avio_rl32(pb) / 8;

if (st->codecpar->ch_layout.nb_channels <= 0)
  return AVERROR_INVALIDDATA;

switch(avio_rl32(pb)) {
case 1: st->codecpar->codec_id = AV_CODEC_ID_DSD_LSBF_PLANAR; break;
case 8: st->codecpar->codec_id = AV_CODEC_ID_DSD_MSBF_PLANAR; break;
...
```

so `avformat_open_input()` itself fails for any DSF file that would otherwise produce
`num_channels == 0`, and GStreamer's `gst_ffmpegdemux_open()` turns that failure into a graceful
`GST_FLOW_ERROR` long before `gst_ffmpegdemux_loop()` (where the unguarded division lives) ever
runs. `gstavdemux.c`'s missing local check is real as a defense-in-depth gap (it silently relies on
every upstream libavformat demuxer that could ever produce one of these codec IDs to have already
validated the channel count, which happens to hold today but isn't documented or enforced at the
`gstavdemux.c` call site itself), but it is not, on the current bundled FFmpeg
(meson-port pin `cde7209117ec2be9da087b9e5f96d48450fdecd2`, 2025-09-16, libavutil 59.39.100), an
attacker-reachable division-by-zero.

## Introduced by

N/A for the (non-reproducing) division-by-zero bug itself. For reference, the surrounding
DSD-planar-meta code in `gstavdemux.c` was added by commit
34238e251d "libav: Integrate FFmpeg's DSD support with GstDsd caps" (`git log` on
`subprojects/gst-libav/ext/libav/gstavdemux.c`).

## The fix

No fix - nothing reproduced via the real trigger path, so no `patch.diff` is included. A defensive
`if (num_channels <= 0) break;` guard directly in `gstavdemux.c` (as `findings.toml` PANIC_9's own
suggested fix shows) would still be a reasonable, low-risk hardening change even though the crash
isn't currently reachable - it removes an implicit, undocumented dependency on every possible
upstream libavformat demuxer's own validation - but was not applied here since there is no
reproducing negative case to verify it against, and "add a defensive check for an unreached path"
is a different, lower-value kind of change than a verified bug fix.

- Project test suite: N/A - no fix applied, no before/after to report.

## What's not done / open questions

- Did not check every third-party/system libavformat build gst-libav could conceivably be linked
  against outside this workspace (only the project's own pinned/bundled FFmpeg subproject,
  `meson-ports/ffmpeg.git` revision `meson-7.1.1`, was tested) - if gst-libav is ever built against
  a different, older, or patched libavformat that lacks `dsfdec.c`'s `nb_channels <= 0` guard (or a
  future FFmpeg version regresses/removes that guard while retaining the DSD-planar codec IDs),
  `gstavdemux.c`'s own unguarded division would become reachable again through the exact same code
  path checked here. This wasn't pursued further this session (out of scope: this workspace only
  has the one pinned FFmpeg source available, and bisecting FFmpeg's own upstream history for this
  guard's introduction date was judged low value given the current, pinned version is what
  gst-libav actually ships against).
- Did not attempt a fuzz burst against `avdemux_dsf`/other `avdemux_*` elements specifically (no
  `ci/fuzzing/*` target covers `gst-libav` in this workspace - see `___BUGHUNT___/README.md`'s
  toolchain table, `libav` is disabled in the scoped ASAN/AFL build configs) - this candidate's
  question was narrow enough (a single named division site with a single named attacker-controlled
  input) that manual boundary-value construction plus a real end-to-end run was judged sufficient
  and cheaper than standing up a new fuzz harness for one function.
