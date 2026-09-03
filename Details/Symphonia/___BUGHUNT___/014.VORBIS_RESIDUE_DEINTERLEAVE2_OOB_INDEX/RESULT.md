# VORBIS_RESIDUE_DEINTERLEAVE2_OOB_INDEX

`Residue::deinterleave_2` in `symphonia-codec-vorbis/src/residue.rs` handles Vorbis type-2
(interleaved) residue decoding for a submap that owns exactly 2 channels. It was flagged in
`IDEAS.md` as a suspect after code reading: the channel-index arithmetic looked wrong for any
2-channel submap whose first channel index isn't 0. This candidate carried a long resume history
(five claim attempts across two autonomous-workflow sessions and this direct session) because
building a genuine triggering file requires a hand-crafted Vorbis setup header - no real encoder
output was ever found to produce the required channel-mapping shape.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real multichannel encoder output (prior sessions) | 8 real `.ogg` files from ffmpeg/libvorbis covering mono, stereo, quad, 5.0/5.1/6.1/7.0/7.1 layouts | No real encoder ever produces a type-2 residue submap with exactly 2 channels that aren't `{0,1}` - libvorbis's own channel-mapping templates either use 1 submap per stream, or split off single-channel submaps (e.g. LFE alone). Ruled out as a source of a real-world trigger. |
| Manual code tracing | `residue.rs` (`deinterleave_2`, `read_residue`, `read_residue_inner_type_2`), `lib.rs` (`decode_inner`, `read_mapping_type0`), `floor.rs` (`Floor1::read_channel`/`read_setup`) | Established the exact minimal legal Vorbis setup-header shape needed to reach the buggy code path without needing any real residue-codebook decoding to succeed (see Root cause) |
| Hand-crafted Ogg/Vorbis file via the real public `symphonia::default::get_probe()` + `AudioDecoder::decode()` API | Debug build, 4-channel stream, `num_submaps=2`, `multiplex=[0,0,1,1]` (submap 1 owns channels `{2,3}`) | **Reproduces deterministically, 3/3 runs** - panics at `residue.rs:189:63` with `mid > len` |
| Same repro, patched tree | Same input, `ch1 - ch0` fix applied | No panic, decode completes |
| Second independent variant (6 channels, submap owning `{2,4}`) | Debug build | Unpatched: panics (`index out of bounds: the len is 0 but the index is 0`, line 191). Patched: completes without panicking. Confirms the fix isn't specific to the first triggering input. |

## Status

**Confirmed** - reproduces deterministically, 3/3 runs, debug build, no seed/timing dependence
(a plain slice-bounds panic driven entirely by the crafted file's static structure). Also
independently confirmed on a second, structurally different channel-mapping variant (6 channels,
non-adjacent submap `{2,4}`), which panics differently pre-fix (`index out of bounds`, not `mid >
len`) but is fixed by the same one-line change.

**Fully reproducible**: **Yes**. The repro is a single external artifact - a malformed-but-format-
legal `.ogg` file - fed to Symphonia's real, public, end-to-end API (`probe()` then
`AudioDecoder::decode()`), exactly the same path a real caller (`symphonia-play`, `symphonia-check`,
or any downstream user of the `symphonia` crate) uses. Per `CONFIRMATION.md` section 11 this needs
no second participant to fake: there's no client/server or caller/callee boundary being
impersonated, just a bitstream a real attacker could equally construct and hand to any Symphonia
consumer.

## Class

crash (a Rust panic reachable via ordinary safe-Rust slice-bounds checks - `split_at_mut`'s
`assert(mid <= len)` and, in other channel-count combinations, plain out-of-bounds indexing - not
memory-unsafety/UB, since Rust's own bounds checks catch it before anything unsafe would happen)

## Root cause

`deinterleave_2`'s 2-channel branch does:

```rust
let (a, b) = channels.split_at_mut(ch0).1.split_at_mut(ch1);
(&mut a[0], &mut b[0])
```

`ch0` and `ch1` are the two (ascending, absolute) channel indices that this submap owns, read from
`residue_channels: &BitSet256` - itself built directly from the untrusted Vorbis setup header's
`mapping.multiplex` array whenever `num_submaps > 1` (`lib.rs:230-236`; multiplex values are only
range-checked against `num_submaps` at `lib.rs:690-701`, never checked against forming a
`{0,1}`-relative pair). `channels.split_at_mut(ch0)` produces `.1 = channels[ch0..]`, a slice
**already shifted by `ch0`** - so splitting *that* slice at absolute index `ch1` is wrong; the
correct split point relative to the shifted slice is `ch1 - ch0`. The bug is invisible for the
extremely common case where a 2-channel submap happens to own exactly channels `{0,1}` (`ch0 == 0`,
so the shift is a no-op) - which is every real-world encoder output checked this session (plain
stereo, and the one 2-channel submap seen in any tested 5.1/6.1/7.x layout is always `{0,1}`-shaped
or a different channel count entirely). It only misbehaves when a 2-channel submap's first channel
index is `> 0`, a shape that is fully legal per the Vorbis I bitstream format but that no tested
real encoder happens to produce.

Depending on the exact indices this manifests two ways, both confirmed this session:
- `ch0 + ch1 > channels.len()` (e.g. `{2,3}` in a 4-channel stream: `channels[2..4]` has length 2,
  `.split_at_mut(3)` panics with `mid > len`).
- `ch0 + ch1 <= channels.len()` but `ch1 != correct` (e.g. `{2,4}` in a 6-channel stream:
  `channels[2..6].split_at_mut(4)` succeeds structurally but yields `b = channels[6..6]` (empty),
  so `b[0]` panics with `index out of bounds`) - in general this shape can also silently decode the
  wrong channel's residue into the wrong buffer without panicking at all, for indices where the
  arithmetic doesn't happen to hit an empty/out-of-range sub-slice; that variant was not
  additionally constructed this session (the two panicking shapes above were sufficient to confirm
  and fix the underlying arithmetic error).

Getting to this code path does **not** require any real residue-codebook (Huffman) decoding to
succeed: `read_residue_inner_type_2` unconditionally calls `deinterleave_2` (via `read_residue`'s
outer wrapper, `residue.rs:169-171`) after ignoring an end-of-bitstream error as "legal" -
including the trivial case where `has_channel_to_decode` is false or `parts_to_read == 0`, in which
case the inner function returns `Ok(())` having read zero bits. Combined with `Floor1::read_channel`
short-circuiting on a single `is_used = false` bit (`floor.rs:663-669`, no codebook needed), the
entire audio-packet payload needed to reach the panic is 5 bits (1 packet-type bit + 4 floor-unused
bits, one per channel) - the repro's crafted `.ogg` file needs a real (if minimal) setup header, but
essentially no real audio payload at all.

## Introduced by

06b7cb0 "vorbis: Optimize type 2 stereo deinterleave" (2023-01-31). Confirmed via `git log -S
"split_at_mut(ch1)" -- symphonia-codec-vorbis/src/residue.rs`: this is the commit that introduced
the `split_at_mut`-based 2-channel fast path (replacing a presumably-correct generic per-channel
loop for this case) and, with it, this arithmetic error.

## The fix

Change the second split point from the absolute channel index `ch1` to the offset relative to the
already-shifted slice, `ch1 - ch0`:

```diff
-                    let (a, b) = channels.split_at_mut(ch0).1.split_at_mut(ch1);
+                    let (a, b) = channels.split_at_mut(ch0).1.split_at_mut(ch1 - ch0);
```

This is sufficient because `iter.next()` yields `ch0`/`ch1` in ascending order from a `BitSet256`
iterator (so `ch1 > ch0` always holds when `count() == 2`), making `ch1 - ch0` a valid, always-
positive, in-bounds relative offset into the `ch0`-shifted slice - matching exactly what the
generic (`stride != 2`) branch below it already does correctly via direct `channels[ch]` indexing
by absolute index, with no such shift to account for.

Verified against both confirmed triggering shapes (`{2,3}` in 4 channels, `{2,4}` in 6 channels):
neither panics after the fix, and the `{0,1}`-shaped path (plain stereo, the common case) is
untouched (`ch1 - ch0` reduces to `ch1` exactly when `ch0 == 0`).

- Project test suite (`cargo test -p symphonia-codec-vorbis -p symphonia-format-ogg -p symphonia`):
  6 passed, 0 failed before the fix; 6 passed, 0 failed after (all 6 pre-existing tests are in
  `symphonia-codec-vorbis`'s `codebook`/`common` unit tests, unrelated to this code path - no
  existing test covered `deinterleave_2` at all).

## What's not done / open questions

- The generic (`stride != 2`, i.e. `residue_channels.count() != 2`) branch of `deinterleave_2` was
  read and confirmed correct (plain `channels[ch]` absolute indexing, no shifted-slice arithmetic)
  - this bug class does not recur there.
  - `Residue`'s type-0/type-1 decode path (`read_residue_inner_type_0_1`) does not call
  `deinterleave_2` at all (that function is specific to type-2/interleaved residue) and was not
  otherwise re-examined for a similar channel-index bug, since it doesn't share this code.
- The "silently decodes into the wrong channel without panicking" variant of this bug (channel
  index combinations where `ch0 + ch1 <= channels.len()` and `b` is non-empty) was reasoned about
  but not separately constructed/confirmed this session - the two panicking shapes were sufficient
  to confirm the root cause and verify the fix; a maintainer wanting an exhaustive channel-count
  sweep could extend `repro/src/main.rs`'s `build_setup_packet` to iterate more `(ch0, ch1,
  n_channels)` combinations.
- This is a fully synthetic (hand-crafted) input, not observed from any real encoder - see "What
  was tried" above. It's a legitimate Vorbis-I-spec-legal bitstream shape, just one no tested
  real-world encoder happens to produce, so this is a defensive-parsing gap rather than something
  likely to be hit by ordinary (non-adversarial) files in practice.
