# avidemux: heap-buffer-overflow reading a Nikon CreationDate tag with a short declared length

`gst_avi_demux_parse_ncdt()` (Nikon-specific metadata parsing in the AVI demuxer) reads a
fixed-offset byte (`ptr_dup[4]`/`ptr_dup[7]`) out of a heap buffer that was allocated to hold only
`sub_size` bytes - if a crafted AVI file declares a Nikon `nctg`/CreationDate (`sub_tag=0x13`)
entry with `sub_size < 8`, the read lands past the end of the allocation. Found by re-verifying
one of a cluster of recent avidemux bounds-checking commits (candidate #008's own lead) - in this
case the specific commit **introduced** the bug while fixing a different one.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `gst_avi_demux_parse_ncdt()`, `case 0x13` (`subprojects/gst-plugins-good/gst/avi/gstavidemux.c` ~line 3983-4000) | `if (left > 7 && ptr[0] != '\0') { guint8 *ptr_dup = g_memdup2(ptr, sub_size); if (ptr_dup[4]==':') ...; if (ptr_dup[7]==':') ...; }` - `ptr_dup` is allocated to hold exactly `sub_size` bytes (attacker-controlled, can be as small as 0), but indices 4 and 7 are accessed unconditionally regardless of `sub_size`. `left > 7` only bounds the *source* buffer, not the size of the *copy* | Looked like a real, reachable heap-buffer-overflow |
| Code reading: `git blame`/`git show` on this exact code to understand why the copy exists at all | The direct predecessor of this code (before `ed6dcb3322` "avidemux: Don't modify read-only mapped buffer data") wrote straight into the original mapped buffer (`ptr[4]`/`ptr[7]`, no copy) - which was safe *for that code*, since `left > 7` genuinely guarantees the real source buffer has at least 8 valid bytes at that point, regardless of what `sub_size` claims. Fixing the "don't mutate a read-only mapped buffer" problem by copying into `ptr_dup` reintroduced the same fixed-offset access, but now against a buffer sized by the untrusted `sub_size` instead of the real, always-≥8 source buffer | Confirms the exact mechanism and pinpoints the regression to a specific commit |
| Built a real, standalone reproduction: a genuine `gst-launch-1.0 filesrc location=evil.avi ! avidemux ! fakesink` pipeline (the real public entry point, not a hand-built stand-in) against a crafted `.avi` file - a real base fixture (`base_avi_h264_pcm.avi`, from this session's ffmpeg-generated corpus at `___BUGHUNT___/__corpus/`) with a hand-spliced malicious `LIST 'ncdt'` chunk inserted into its `hdrl` LIST (`repro/craft_avi.py` does the splice: a `nctg` sub-chunk containing one `sub_tag=0x13` entry with `sub_size=1`), run against the ASAN+UBSAN build (`build-asan-fuzz/`, `gst-plugins-good:avi=enabled`) | **Reproduced immediately**: real `AddressSanitizer: heap-buffer-overflow`, `READ of size 1 ... 3 bytes after 1-byte region`, at `gstavidemux.c:3990`, through the real call chain `gst_avi_demux_loop` -> `gst_avi_demux_stream_header_pull` -> `gst_avi_demux_parse_ncdt` - see `output.txt` |

Two code-reading passes plus a successful runtime reproduction - well within the soft cap of 3, no
further escalation needed.

## Status

**Confirmed.** Deterministic, 3/3 runs (before the fix: 3/3 crash with the identical ASAN report;
after the fix: 3/3 clean runs to EOS - see `output.txt` for one before/after pair).

**Fully reproducible**: **Yes.** The repro drives the exact real public entry point
(`gst-launch-1.0` building a real pipeline: `filesrc ! avidemux ! fakesink`) with a byte-for-byte
crafted file - not a hand-constructed internal call into `gst_avi_demux_parse_ncdt()` bypassing
the real demuxing/header-parsing path. This is a file-parsing finding, not a peer-to-peer
protocol one, so `CONFIRMATION.md` section 11's "real second participant" requirement doesn't add
anything beyond what's already here (the "second participant" is the untrusted file itself,
fully present and byte-exact in `repro/evil.avi`).

## Class

`memory-safety`

## Root cause

`gst_avi_demux_parse_ncdt()`'s Nikon `CreationDate` (`sub_tag=0x13`) handling allocates
`ptr_dup = g_memdup2(ptr, sub_size)` where `sub_size` is a fully attacker-controlled 16-bit value
read a few lines earlier from the file (`GST_READ_UINT16_LE(ptr+2)`, only checked against the
*outer* tag's remaining budget, `sub_size > tsize`, never against a minimum of 8). The code then
unconditionally reads/writes `ptr_dup[4]` and `ptr_dup[7]`, assuming the copy is at least 8 bytes
long - true only when `sub_size >= 8`. A `sub_size` of, e.g., 1 produces a 1-byte allocation, and
`ptr_dup[4]`/`ptr_dup[7]` land 3 and 6 bytes past its end.

## Introduced by

ed6dcb332204c5054107b7c0aed24decd8985564 "avidemux: Don't modify read-only mapped buffer data"
(2026-07-14). The pre-existing code (before this commit) wrote directly into the original mapped
buffer at `ptr[4]`/`ptr[7]`, which was memory-safe on its own terms (guarded by `left > 7`, a bound
on the *real* source buffer, not on `sub_size`) but violated the `GST_MAP_READ` read-only
contract. This commit fixed that contract violation by copying into a new buffer first - but sized
the copy by the untrusted `sub_size` instead of by the same `left`-based guarantee the original
code relied on, turning a policy violation into a memory-safety bug.

## The fix

One-line change in `subprojects/gst-plugins-good/gst/avi/gstavidemux.c`: require
`sub_size >= 8` (in addition to the existing `left > 7`) before entering the block that indexes
`ptr_dup[4]`/`ptr_dup[7]`, so the copy is only made - and only indexed at those fixed offsets -
when it's actually long enough to contain them. A `CreationDate` value shorter than 8 bytes can't
contain the expected `YYYY:MM:DD` prefix being colon-fixed-up here anyway, so this also skips
meaningless work on truncated/malformed entries rather than just relocating the same bug.

Verified against the standalone repro (`output.txt`): heap-buffer-overflow before, clean run to
EOS after, 3/3 both ways.

- Project test suite: **N/A** - no `avidemux`-specific test exists in this codebase
  (`subprojects/gst-plugins-good/tests/check/elements/` has `avisubtitle.c` and `avimux.c` but no
  `avidemux.c`), and this session's scoped ASAN build has no `avidemux` test binary to run either
  way.

## What's not done / open questions

- Only the `0x13`/`CreationDate` sub-tag was checked for this exact pattern; the sibling sub-tags
  in the same `switch` (`0x03`/Make, `0x04`/Model, `0x06`/Software - see the surrounding code,
  ~line 3972-3983) just set a `type` string and let `parse_tag_value()` handle the value generically
  without a fixed-offset copy, so they don't share this specific bug - not re-verified in depth
  beyond confirming they don't do the same `ptr_dup[N]` pattern.
- Did not check whether `gst_matroska_parse_flac_stream_headers`-style "read enough data" fixes
  elsewhere in this same MR (`!12231`) have a sibling case of "fixed the letter of the bounds check
  but not a related copy-size assumption" - candidate #008/#009 checked two other hunks from the
  same commit series and found them correct, but this specific "copy sized by the wrong variable"
  shape wasn't the thing being checked for there.
- The crafted `evil.avi` splices into a real `base_avi_h264_pcm.avi` fixture generated by this
  session's own fuzzing setup (`___BUGHUNT___/__corpus/`, ffmpeg-generated, not part of the GStreamer
  repo itself) - `repro/craft_avi.py` documents the exact byte layout inserted and would work
  against any structurally-similar real AVI file with an `hdrl` LIST.
