# ALAC_BUF_TRUNCATE_USES_LAST_ELEMENTS_SAMPLE_COUNT

`symphonia-codec-alac/src/lib.rs`, `AlacDecoder::decode_inner`. For multichannel ALAC (3-8
channels, coded as multiple SCE/CPE elements per packet), `num_frames` is overwritten by each
element's own return value rather than accumulated/validated, then `self.buf.truncate(num_frames)`
is applied to the whole (all-channel) buffer using only the LAST-decoded element's own sample
count. Flagged as a suspect because each element's `is_partial_frame`/`num_samples` fields are
read and trusted independently, with no cross-element consistency check before the shared,
single-`num_frames`-field `truncate` call at the end.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction | Hand-built a well-formed (per the decoder's own validation checks) 3-channel ALAC magic cookie + packet, with one SCE element decoding 8 samples (uncompressed path) followed by one CPE element decoding only 3 samples (a shorter partial frame) | Reproduced immediately - `decode_ref` returns `Ok` with `buf.frames() == 3`, silently discarding the SCE's own validly-decoded samples 103..107 (indices 3..8) |
| Repeat runs, debug + release build | Same input, `cargo run` and `cargo run --release`, 3 consecutive runs each | Identical, deterministic result every time (3/3 debug, 3/3 release) - no debug-assertion panic, no crash, purely a silent-data-loss logic bug |

No fuzzing/miri/ASAN was needed - the bug is a straightforward logic error reachable by hand
construction of two elements with differing declared sample counts, not memory-unsafety or
crash-class.

## Status

**Confirmed** - reproduces deterministically 3/3 runs under both `cargo run` (debug) and
`cargo run --release`, no fixed seed or layout dependency involved (pure logic bug, fully
deterministic bitstream input).

**Fully reproducible**: **Yes**. The repro is a single-participant case (see `CONFIRMATION.md`
section 11): the "other side" here is simply a malformed/adversarial ALAC packet, the same kind
of external artifact a real encoder, a fuzzer, or an attacker-controlled file could produce and
hand to `symphonia-codec-alac`'s real public entry points (`AlacDecoder::try_new` +
`AudioDecoder::decode_ref`) - there is no second participant being faked. `repro/src/main.rs`
drives those real public APIs end-to-end with no internal/private scaffolding.

## Class

logic

## Root cause

`decode_inner` loops over the packet's SCE/CPE/DSE/FIL/END elements. For each SCE or CPE element,
it calls `decode_sce_or_cpe(...)`, which independently reads that element's own
`is_partial_frame`/`num_samples` fields from the bitstream and returns however many samples it
actually decoded for that element's channel(s). The loop variable `num_frames` is *reassigned*
(not accumulated, not validated against any previous value) on every element:

```rust
let mut num_frames = 0;
...
ALAC_ELEM_TAG_SCE | ALAC_ELEM_TAG_LFE => {
    num_frames = decode_sce_or_cpe(...)?;   // overwrites, does not check against prior value
    ...
}
ALAC_ELEM_TAG_CPE => {
    num_frames = decode_sce_or_cpe(...)?;   // overwrites again
    ...
}
...
self.buf.truncate(num_frames);              // applies to ALL channel planes at once
```

`self.buf` is a single `AudioBuffer<i32>` with one shared `num_frames` counter across every
channel plane (see `symphonia-core/src/audio/buf.rs`'s `AudioBuffer::truncate`/`render_silence`).
Before the element loop, `self.buf.clear()` + `self.buf.render_silence(None)` sets every plane's
frame count to the buffer's full capacity (`config.frame_length`) and fills it with silence.
Each `decode_sce_or_cpe` call only writes its own channel(s)' plane(s) in range
`[0, num_samples)`; frames beyond that (for a partial-frame element) are left as the
render_silence-filled MID value.

Since only the *last-executed* element's return value survives into the final `truncate` call,
two failure modes exist whenever elements disagree on their own decoded sample count:

- If a non-last element decodes fewer samples than the last element, that non-last channel's
  trailing frames (still silence from `render_silence`) are exposed as if they were real decoded
  audio, because `truncate` doesn't cut them off.
- If the last element decodes fewer samples than an earlier element (reproduced here), the
  earlier element's own validly-decoded trailing samples are silently discarded, because
  `truncate` cuts every plane down to the last element's shorter count - with no error, no log,
  and no signal to the caller that anything was lost.

Single-channel (mono) and 2-channel (stereo) ALAC streams are unaffected because they only ever
contain exactly one element per packet, so `num_frames` is only ever assigned once.

## Introduced by

6e4653c "alac: Implement decoder." (2021-11-11) - `git blame` on the `self.buf.truncate(num_frames)`
line points directly at the ALAC decoder's original implementation commit; the overwrite-not-
accumulate pattern for `num_frames` has been present unchanged since this commit introduced the
whole `decode_inner` function.

## The fix

Changed `num_frames` from a bare `usize` overwritten per element to an `Option<usize>` that is set
on the *first* element and then checked for equality against every subsequent element's own
returned sample count; a mismatch now returns `decode_error("alac: elements in the same frame
decoded a different number of samples")` instead of silently truncating with an inconsistent
count. This is a conservative fix (reject the malformed/inconsistent packet) rather than trying to
guess which of several disagreeing element lengths is "the real" frame length - the ALAC element
format gives no way to know which of two mismatched counts (if either) is correct.

Verified: `repro/`'s SCE(8 samples)+CPE(3 samples) input, which previously returned `Ok` with 5
samples silently lost, now returns `Err(DecodeError("alac: elements in the same frame decoded a
different number of samples"))` - see `output.txt`.

Per CONFIRMATION.md section 7 (varying the input within the same bug class, not just the one
triggering input): also manually re-checked the reverse case (CPE decoding a longer/full frame
executed after a shorter SCE, and a mono/stereo single-element packet) against the patched
decoder logic by inspection - a single-element packet still assigns `num_frames` exactly once
(no `Some(nf) if nf != frames` branch is ever taken), so mono/stereo streams are provably
unaffected by the fix.

- Project test suite: `cargo test -p symphonia-codec-alac -p symphonia-core` -> 65 passed; 0
  failed, both before and after the patch (symphonia-codec-alac itself has 0 unit tests of its
  own; symphonia-core's 65 tests, none ALAC-specific, are unaffected either way).

## What's not done / open questions

- No existing regression test exercises multichannel ALAC decoding at all (there is no ALAC test
  audio/corpus anywhere in this repo - see `___BUGHUNT___/README.md`), so this fix has no
  project-level regression coverage beyond the standalone `repro/`. Adding a unit test with a
  hand-built multichannel packet (similar to `repro/src/main.rs`'s construction) directly in
  `symphonia-codec-alac` would be a reasonable follow-up but was left out of this session's bounded
  fix to keep the change minimal.
- Did not check whether any other codec in this workspace has a similar "one shared frame-count
  field truncated using only the last of several independently-decoded sub-elements" pattern; this
  is specific to ALAC's SCE/CPE/LFE element loop and no structurally similar loop was noticed
  elsewhere during this session.
- A longer fuzzing campaign (via `fuzz-bug-repro`, run separately) against `decode_alac` with a
  multichannel seed corpus might turn up further variants of element-count mismatches (e.g. DSE/FIL
  elements interacting with a later SCE/CPE) - not attempted here since manual construction already
  gave a clean, deterministic repro.
