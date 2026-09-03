# CORE_AUDIOBUFFER_NEW_ZERO_CHANNEL_DIV_BY_ZERO

`AudioBuffer::new` (`symphonia-core/src/audio/buf.rs:83-86`) guards its allocation with
`assert!(capacity <= (usize::MAX / (std::mem::size_of::<S>() * num_channels)), "capacity too
large")`, where `num_channels` comes straight from `spec.channels().count()`. The suspicion
(arithmetic on an unvalidated size, per `HEURISTICS.md`): when the audio specification has zero
channels the divisor is `0`, so instead of the intended "capacity too large" assertion the
constructor panics with `attempt to divide by zero` - and integer division by zero panics in
release builds too, not only under debug assertions/overflow checks.

A zero-channel `AudioSpec` is not an illegal state elsewhere in this API: `AudioBuffer::is_unused`
(`buf.rs:99-101`) explicitly documents "no channels" as a legitimate condition, and
`Channels::count()` (`symphonia-core/src/audio/channels.rs:310-318`) returns `0` for
`Channels::None`, `Channels::Discrete(0)` and an empty `Channels::Custom`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction against the affected public entry point (`AudioBuffer::<f32>::new` with `Channels::None`, `Channels::Discrete(0)` and `Channels::Custom([])`) | debug build (debug assertions + overflow checks) and release build, `capacity = 1024` | Panics in both builds at `symphonia-core/src/audio/buf.rs:84:25` with `attempt to divide by zero`, for all three zero-channel `Channels` variants. Deterministic. |
| Code read to find whether any in-tree format/codec pair actually produces a zero-channel `AudioSpec` from real file input (grep over every `Channels::*` construction site in `symphonia-format-*`/`symphonia-bundle-*`/`symphonia-codec-*`, plus each decoder's own channel validation) | n/a (static) | Found a real, reachable path: `symphonia-format-caf`'s `ChannelLayout::channels()` (`chunks.rs:343-397`) maps a `chan` chunk with `mChannelLayoutTag = kAudioChannelLayoutTag_UseChannelDescriptions` (0) and `mNumberChannelDescriptions = 0` to `Channels::Custom([])` and unconditionally overwrites `codec_params.channels` with it (`demuxer.rs:485-490`); `symphonia-codec-adpcm`'s `try_new` only rejects `channels.count() > 2` (`lib.rs:97-100`), never `== 0`, and then calls `AudioBuffer::new` (`lib.rs:118`). Every other decoder checked rejects 0 channels first (pcm `lib.rs:232`, vorbis `lib.rs:434`, alac, aac) or derives the count from its own header (flac, mp3). |
| End-to-end repro: crafted 126-byte CAF file driven through symphonia's real public `get_probe().probe()` -> `default_track()` -> `get_codecs().make_audio_decoder()` pipeline (the same calls `symphonia-play` makes) | release build, 3/3 consecutive runs; debug build, 3/3 consecutive runs | Panics at `symphonia-core/src/audio/buf.rs:84:25`, `attempt to divide by zero`, in both builds, byte-identical output every run. Real backtrace: `AudioBuffer<i32>::new` <- `AdpcmDecoder::try_new` <- `try_registry_new` <- `CodecRegistry::make_audio_decoder` <- caller. A baseline CAF file identical except for the omitted `chan` chunk decodes a packet successfully in the same process, same run. |
| Boundary-value sweep of the patched assert (`repro/src/bin/bounds.rs`) to check the fix doesn't weaken the over-large-capacity check it replaces | release build, 12 cases: `usize::MAX`, `usize::MAX / size_of::<S>() + 1` and ordinary capacities across `f32`/`i32`/`f64`, 1/2/24 channels, plus the three zero-channel variants | Before the patch: every non-zero-channel over-large case panics `capacity too large`, every zero-channel case panics `attempt to divide by zero`. After: the over-large cases still panic `capacity too large` unchanged, the ordinary cases still succeed unchanged, and the zero-channel cases now construct a 0-plane buffer with `is_unused() == true`. |

## Status

**Confirmed.** Reproduces deterministically: 3/3 consecutive runs in a release build and 3/3 in a
debug build, against a pristine clone of `ee35874`, byte-identical output each run, for both the
direct public-API call and the end-to-end crafted-file path. Not layout-, seed- or timing-dependent
- it is an unconditional `x / 0` on a value that is `0` by construction.

**Fully reproducible**: **Yes**. `repro/` runs the real affected entry point through public APIs
only, two ways: (Part A) `symphonia::core::audio::AudioBuffer::new` called exactly as a
third-party format/codec crate built on `symphonia-core` calls it, and (Part B) the real
`get_probe().probe()` / `make_audio_decoder()` pipeline fed a crafted CAF *file*. There is no
second participant being impersonated in either part - the CAF bytes are the genuine external
artifact (the same bytes an attacker or a broken encoder would supply to `probe`), and every
`Channels`/`AudioSpec` value involved in Part B is produced by symphonia's own demuxer from those
bytes, not hand-built.

## Class

crash

## Root cause

`AudioBuffer::new` expresses its "the allocation must not exceed `usize::MAX` bytes" precondition
as a division by the byte size of one audio frame:

```rust
let num_channels = spec.channels().count();
assert!(capacity <= (usize::MAX / (std::mem::size_of::<S>() * num_channels)), "capacity too large");
```

`std::mem::size_of::<S>()` is always at least 1, so the divisor is zero exactly when
`num_channels == 0`. Rust's integer division panics on a zero divisor unconditionally - this is not
a debug-assertion/overflow-check panic, it is present in release builds as well - so the
constructor aborts with `attempt to divide by zero` before it ever gets to the `planes.resize_with`
that would simply have produced a legitimate empty (0-plane) buffer. The assertion's own message
never appears, and the caller has no way to distinguish "you asked for too much memory" from "you
passed a zero-channel spec".

The zero-channel case is not a caller error this constructor is entitled to assume away: the same
type's `is_unused()` documents "no channels" as a supported state, `AudioBuffer::default()` (derived
`Default`) produces exactly that state, and `Channels::None` / `Channels::Discrete(0)` / an empty
`Channels::Custom` are all publicly constructible values whose `count()` is 0.

The end-to-end reachability inside this repository comes from two further, separate gaps (each a
distinct defect, see "What's not done"): `symphonia-format-caf` converts a `chan` chunk carrying
zero channel descriptions into `Channels::Custom([])` and overwrites the (validated, non-zero)
count from the `desc` chunk with it, and `symphonia-codec-adpcm::AdpcmDecoder::try_new` validates
only the upper bound on the channel count (`> 2`), never the lower bound, before handing the spec
to `AudioBuffer::new`.

## Introduced by

The divide-by-a-channel-count pattern was introduced by 7769309 "core: Assert limits when creating
Audio/SampleBuffers." (2021-07-11), which added
`assert!(duration <= u64::MAX / spec.channels.count() as u64, "duration too large")` to the then
`symphonia-core/src/audio.rs`. The exact line this candidate reproduces against was carried over
into the rewritten audio module by f1722b6 "core (audio): New audio module." (2024-03-10), which is
also the only commit `git log -S "usize::MAX / (std::mem::size_of"` reports for the current form.

## The fix

`patch.diff` (+6/-2, `symphonia-core/src/audio/buf.rs`) replaces the division with the equivalent
checked multiplication:

```rust
assert!(
    capacity
        .checked_mul(std::mem::size_of::<S>())
        .and_then(|len| len.checked_mul(num_channels))
        .is_some(),
    "capacity too large"
);
```

For every non-zero channel count this is exactly the original predicate: `capacity <=
floor(usize::MAX / d)` is equivalent to `capacity * d <= usize::MAX`, i.e. to the multiplication
not overflowing. For `num_channels == 0` it no longer divides by zero; the per-plane size
(`capacity * size_of::<S>()`) is still required to be representable, so an absurd capacity is still
rejected with the intended `capacity too large` message rather than being silently accepted - only
the divide-by-zero disappears. `planes.resize_with(0, ..)` then yields the empty, `is_unused()`
buffer that the surrounding code already documents as valid.

Verified per `CONFIRMATION.md` sections 6 and 7 against more than the one triggering input: the
12-case boundary sweep in `repro/src/bin/bounds.rs` (see the last row of "What was tried" and the
tail of `output.txt`) shows the over-large-capacity rejections and the ordinary successes are
byte-for-byte unchanged, and all three zero-channel `Channels` variants now succeed.

Re-running `repro/` against the patched tree: Part A's three zero-channel constructions all
succeed (`num_planes = 0, capacity = 1024, is_unused = true`), Part B's `make_audio_decoder` now
returns a decoder instead of panicking, and the process exits 0 instead of 1. The full before/after
is in `output.txt`.

- Project test suite (`cargo test --workspace --all-features`): 106 passed / 0 failed before the
  patch -> 106 passed / 0 failed after the patch. No new failures, no tests skipped or removed.

## What's not done / open questions

- **Part B still panics after this patch, at a different place, for a different reason.** With the
  divide-by-zero fixed, `symphonia-codec-adpcm` happily builds a decoder around a 0-channel spec
  and then hits `_ => unreachable!()` in its `decode_inner` channel-count `match`
  (`symphonia-codec-adpcm/src/lib.rs:164`) on the first packet. That is a distinct defect - the
  decoder's `try_new` (`lib.rs:97-100`) checks only `channels.count() > 2`, never `== 0`, unlike
  `symphonia-codec-pcm` which explicitly rejects 0 (`symphonia-codec-pcm/src/lib.rs:232-234`) - and
  it is not fixed here, since one candidate folder is one bug. The repro labels that panic
  explicitly and does not count it as this bug. Recorded as a new suspect.
- **`symphonia-format-caf` overwriting a valid channel count with an empty one** is a third,
  separate defect on the same path: `ChannelLayout::channels()` returns `Some(Channels::Custom([]))`
  for a `chan` chunk with `mNumberChannelDescriptions = 0`, and `demuxer.rs:485-490` assigns it over
  the count already derived (and validated as non-zero) from the `desc` chunk. Every other
  "unsupported layout" case in that function returns `None`, which the demuxer correctly treats as
  "keep what `desc` said". Not fixed here. Recorded as a new suspect.
- Only `AudioBuffer::new` was audited for this pattern. A grep for the sibling sites the
  introducing commit 7769309 mentioned (`SampleBuffer`, `RawSampleBuffer`) finds no remaining
  division-by-channel-count anywhere under `symphonia-core/src/audio/` - the only other
  channel-count division in the workspace is `symphonia-bundle-mp3/src/layer2/mod.rs:140`, where
  the count is structurally 1 or 2. `GenericAudioBuffer::new`
  (`symphonia-core/src/audio/generic.rs:80-92`) reaches the same constructor and is therefore
  covered by the same fix.
- No fuzzing was run for this candidate; the input space that matters here is a single integer
  (the channel count), so exhaustive boundary-value construction covers it and a fuzz campaign
  would add nothing. A campaign pointed at `symphonia-format-caf`'s chunk parsing generally
  (via `fuzz-bug-repro`, separately) would likely surface the two related defects above and
  possibly more in the same file.
- Whether the maintainers prefer this fix or an explicit `num_channels > 0` precondition (an
  `assert!`/`debug_assert!`, or making `new` fallible) is a design decision: this patch chooses to
  keep the zero-channel buffer legal, because `is_unused()`/`Default` already treat it as such. If
  they would rather forbid it, the panic should at least say so instead of surfacing as
  `attempt to divide by zero`.
