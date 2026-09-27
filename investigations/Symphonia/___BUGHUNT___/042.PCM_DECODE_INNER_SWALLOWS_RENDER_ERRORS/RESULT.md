# PCM_DECODE_INNER_SWALLOWS_RENDER_ERRORS

`symphonia-codec-pcm/src/lib.rs` `PcmDecoder::decode_inner` (lines ~318-330 at base_commit)
computes `num_frames` from `packet.data.len() / bytes_per_coded_frame`, uses it only to
`grow_capacity` (which never shrinks), then discards the `Result` of the `render_with(None, ...)`
call via `let _ = match self.params.codec { ... }` and unconditionally returns `Ok(())`. The
candidate's concern: a short/truncated packet, or any non-EOF `io::Error` bubbling out of the
packet's `BufReader`, would be silently reported to the caller as a fully successful decode, with
the committed frame count possibly decided by "wherever reads happened to stop" rather than by the
computed `num_frames` - i.e. a silently wrong frame count.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `GenericAudioBuffer::render_with` (`symphonia-core/src/audio/buf.rs`) and `Packet`/`PacketRef::as_buf_reader` (`symphonia-core/src/packet.rs`) to understand the actual commit/error semantics before writing any repro | N/A (static read) | `render_with` commits a frame (`self.num_frames += 1`) only *after* the per-frame closure returns `Ok`; a failing read inside the closure aborts the loop before that frame's commit, so partially-read frames are never counted. `Packet`/`PacketRef::data` is always an in-memory `Box<[u8]>`/`&[u8]` slice, so `as_buf_reader()`'s only possible read failure is an "unexpected EOF"-style short read when the packet's own bytes run out - there is no other I/O error source (no real file/socket underneath) for this decoder's inputs. |
| Manual boundary-value construction driving the real public API (`PcmDecoder::try_new` + `AudioDecoder::decode_ref`, no internals faked) with a dedicated example program (`repro/main.rs`), `cargo run -p symphonia-codec-pcm --example repro042`, dev build (debug-assertions + overflow-checks on) | Targeted cases per codec/channel-count: grow the decoder's buffer to a large frame count with one packet, then feed a packet 1, 2, 3, or 5+ bytes short of a whole number of frames (a truncated/corrupted trailing packet), an exact-but-smaller packet, a 1-byte packet, and a 0-byte packet, across PCM_S16LE (mono+stereo), PCM_S24LE (stereo), PCM_F32LE (mono), PCM_U8 (stereo), PCM_ALAW (mono) | Every case's `decode_ref()` returned `Ok`, and the resulting buffer's `frames()` exactly equalled `floor(packet.data.len() / bytes_per_coded_frame)` in every single case - i.e. exactly the value the allegedly-unused `num_frames` variable computes. No wrong frame count, no error ever surfaced. |
| Same driver, 2000 pseudo-random packet sizes (xorshift64, range 0..4000 bytes) fed in sequence to one PCM_S16LE mono decoder instance, repeatedly growing and shrinking the buffer's peak capacity | Same dev build, then repeated again under `--release` (optimizations on, debug-assertions/overflow-checks off) | All 2000 cases matched `floor(len/frame_size)` with `Ok` in both builds - see `output.txt` for the full captured output of both runs (2042 total `decode_ref()` calls, zero mismatches, zero `Err` results). |

## Status

**Not reproducible.**

Covered: 3 escalating rounds within the single "manual boundary-value construction against the
real public API" tool category appropriate for this logic-bug class (per `HEURISTICS.md`) - (1)
static reading of the exact commit/error semantics of `render_with` and the in-memory nature of
`PacketRef`'s reader, (2) targeted boundary-value cases (truncated-by-1/2/3/5+ bytes, exact
smaller packet, 0-byte, 1-byte) across 6 codec/channel-count combinations, both in a dev
(debug-assertions on) and a release build, and (3) 2000 pseudo-random packet sizes fed to the same
decoder instance to stress the "grow big then shrink" sequence the candidate specifically worried
about. All three rounds produced the same clean negative, with round 1's code reading explaining
*why* rounds 2 and 3 could not turn up a divergence (see Root cause) - stopped without exhausting
the soft cap of 3 approaches because every round already agreed and no partial signal ever
appeared to justify escalating to a different tool (e.g. a fuzz harness, which would not add
information beyond the 2000-case random sweep already run for this same reachable-input space).

**Fully reproducible**: N/A (not Confirmed - no repro/ folder; this is a Not reproducible result,
`CONFIRMATION.md` section 11 does not apply).

## Class

N/A (not Confirmed).

## Root cause

The suspect turned out to be safe because of how `AudioBuffer::render_with` commits frames
(`symphonia-core/src/audio/buf.rs:318-346`): the per-frame render closure `f(self.num_frames,
&mut planes)?` is called first, and `self.num_frames += 1` (the actual commit) only happens *after*
that closure returns `Ok`. If any field read inside the closure fails (via the macros' `reader.read_*()?`),
the `?` propagates immediately, `render_with` returns `Err` *before* incrementing `self.num_frames`
for that frame - so a partially-read trailing frame is never counted, regardless of what `decode_inner`
does with the returned `Result`.

Separately, `PacketRef`/`Packet::as_buf_reader()` (`symphonia-core/src/packet.rs:102`,`198`) always
wraps a plain in-memory byte slice (`&self.data`, a `Box<[u8]>` or borrowed `&[u8]`) - there is no
file, socket, or other I/O source underneath for this decoder's packets, so the only possible read
failure is a short read once the packet's own bytes are exhausted. That short read always occurs
at exactly the byte offset implied by `floor(packet.data.len() / bytes_per_coded_frame)` complete
frames, because each frame consumes exactly `bytes_per_coded_frame` bytes and the read for frame
`floor_frames` (the first incomplete one) is guaranteed to fail partway through (or immediately, if
`data.len()` is an exact multiple) precisely because fewer than `bytes_per_coded_frame` bytes
remain.

Consequently, `self.buf.num_frames` after `render_with` returns (`Ok` or swallowed `Err`) always
equals `floor(packet.data.len() / bytes_per_coded_frame)` - exactly the value the local `num_frames`
variable at line 320 computes. Its only actual effect (beyond an unused-after-`grow_capacity`
binding) is to size `grow_capacity`, which is a pure preallocation optimization: `grow_capacity`
never needing to shrink is irrelevant to correctness here because `render_with(None, ...)`'s
`num_new_frames = self.capacity - self.num_frames` only ever *bounds how far the loop is allowed to
run*, not how far it actually gets - the loop is always halted earlier, by the reader running out of
packet bytes at the correct frame boundary, whenever the current packet is smaller than the
buffer's peak capacity. The `let _ = match { ... }` swallow therefore never discards a case where
the true (post-error) frame count differs from what an un-swallowed `Err` path would have reported
either - in both cases the exposed buffer holds exactly the frames that were fully, successfully
decoded from real packet data.

This does mean `decode_inner` can never actually return `Err` from a `render_with` failure for any
input reachable through the public API (the swallow is *effectively* dead code, not a
correctness-affecting one) - which is arguably worth a maintainer's attention as a code-quality/
dead-code observation on its own, but it is not the "silently wrong frame count" correctness bug
the candidate description hypothesized.

## Introduced by

N/A - not Confirmed, no bug to trace the introduction of.

## The fix

Not applicable - no bug was confirmed, so no fix was attempted. (A maintainer might still choose to
stop swallowing the `Result` and/or use the local `num_frames` for something other than
pre-allocation sizing purely for code clarity / defense-in-depth against a future change to
`render_with`'s semantics, but that would be a robustness/clarity improvement, not a bug fix - no
`patch.diff` is included.)

- Project test suite: N/A, no fix was made (before/after comparison does not apply).

## What's not done / open questions

- This investigation covered only the PCM decoder's own packet-driven decode path
  (`decode_ref` -> `decode_inner`) via its real public API. It did not check whether any *caller*
  of `AudioDecoder::decode_ref` elsewhere in the codebase (a format reader, the top-level decode
  loop) might itself misinterpret a `Result::Ok` here as "no truncation occurred" for some purpose
  beyond frame count (e.g. logging, statistics, seek-related accounting) - that's a different,
  broader question about `AudioDecoder`'s error-reporting contract across all decoders, not
  specific to this file, and was out of scope for this candidate.
  - Correctness was checked (2042 packet cases; the value returned by `decode_ref` was always the
    same as `floor(len/frame_size)` with no divergence in the swallowed-error case).
- The `let _ = match { ... }` swallow itself is real - if `render_with`'s implementation ever
  changes to commit partial-frame writes, or a future PCM-family codec branch introduces a
  genuinely-fallible operation unrelated to short reads (e.g. a checked arithmetic call), this
  swallow would then start hiding a real error. This is a latent code-quality risk worth flagging
  to maintainers even though it is not exploitable against the current code.
