# ADPCM_MS_EXPAND_NIBBLE_DELTA_MUL_OVERFLOW

Follow-through on the candidate description's requested action: apply bug-hunt candidate 012's
patch (`symphonia/fuzz/src/lib.rs::fuzz_audio_decoder` always constructed
`Packet::new(0, Timestamp::ZERO, Duration::ZERO, packet_data)`, so `symphonia-codec-adpcm`'s
block-decode loop - the only decoder among the fuzz targets that derives its per-packet work from
`Packet::block_dur()` - never actually ran, no matter what `packet_data` the fuzzer generated), then
run the `decode_adpcm` cargo-fuzz target for real. Doing so found a genuine, previously-untested
integer-overflow panic in `symphonia-codec-adpcm`'s MS ADPCM decoder.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Applied candidate 012's `patch.diff` to `symphonia/fuzz/src/lib.rs` in an isolated git worktree, built the `decode_adpcm` cargo-fuzz target (`cargo +nightly fuzz build decode_adpcm`) | release profile w/ debug-assertions+overflow-checks (cargo-fuzz defaults) | Builds cleanly. |
| Ran `decode_adpcm` under libFuzzer (`cargo +nightly fuzz run decode_adpcm ... -max_total_time=200`) | First attempt: default flags. Process pegged at 99.9% CPU, single thread, no child processes, for the entire budget with only 2 executions ever completing (`INITED` then stuck mid-`NEW_FUNC` coverage-symbolization print) | Initially looked like a possible hang bug in the newly-reachable block-decode loop itself - investigated with `ps`/`/proc/<pid>/status` (single thread, state R, no forked symbolizer subprocess) since `strace`/`gdb -p` were unavailable (no ptrace permission in this sandbox) to attach directly. Root-caused as libFuzzer's own built-in coverage-symbolizer being extremely slow against this large statically-linked binary (60+ crates via the `all` feature), not a bug in the decoder - confirmed by re-running with `-print_funcs=0` (disables that print), which immediately unblocked normal fuzzing speed (~18,000 execs in 200s). Noted in `projects/Symphonia.md` for future sessions fuzzing this repo. |
| Re-ran with `-print_funcs=0 -max_total_time=200`, no corpus seed | release, ~18,000 executions in ~180s before hitting the crash | Found: `thread '<unnamed>' panicked at symphonia-codec-adpcm/src/codec_ms.rs:96:22: attempt to multiply with overflow` - the ADPCM MS block-decode loop, now actually reachable by the fuzzer for the first time (confirming the candidate's premise), immediately turned up a real bug. |
| Reduced the fuzzer-found crash to a minimal, hand-crafted 19-byte MS ADPCM stereo packet calling the real public `AdpcmDecoder` API directly (no fuzz harness involved) | debug build (`cargo run`, no `--release`) | Deterministic 3/3 reproduction, identical panic location and message every time (see `output.txt`). |
| Boundary sweep: same repro harness varied across 5 different `delta` seed values (including 0x0000, 0x7FFF, 0x8000, 0xFFFF, an arbitrary mid value) x 6 different payload fill bytes x a full 1022-byte block, against the *patched* tree | debug build | All 30 combinations returned `Ok(())` with no panic - confirms the fix isn't narrowly tailored to the one triggering input (CONFIRMATION.md section 7). |

Three tool escalations were used (build+fuzz, hang investigation via `/proc`, minimized direct-API
repro) - the fuzz run itself supplied a definitive, reproducible crash on the first successful
attempt, so no further escalation (miri/ASAN) was needed.

## Status

**Confirmed**, 3/3 runs, debug build (`cargo run`, dev profile - default `overflow-checks = true`),
deterministic, bit-for-bit identical panic location and backtrace every run.

**Fully reproducible**: **Yes**. `repro/main.rs` calls the real, unmodified, public
`AdpcmDecoder::decode()` (via `RegisterableAudioDecoder::try_registry_new` + `AudioDecoder::decode`)
- the exact same entry point `symphonia::default`'s codec registry and every real format reader in
this workspace (e.g. the WAV demuxer) goes through - with a hand-crafted MS ADPCM packet and a
correct, non-zero `Duration` (i.e. what a real demuxer would actually produce for this packet, not
the fuzz harness's `Duration::ZERO` bug from candidate 012 - that harness bug is irrelevant to this
finding once you bypass the harness and call the decoder directly, which is exactly what any real
consumer of a malicious/malformed MS ADPCM WAV file would do). There is no second participant to
fake here: this is a single caller (a format reader, or this repro standing in for one) driving a
real public decoder API with real, attacker-controlled packet bytes - the case CONFIRMATION.md
section 11 says stays a plain single-process repro, consistent with candidates 002/003/029/030's
own repros of the same decoder family.

## Class

crash (also logic: on a plain release build with `overflow-checks` off, the same input silently
wraps `delta` to a nonsense value instead of panicking - see "Root cause").

## Root cause

`AdpcmMsBlockStatus::expand_nibble` (`symphonia-codec-adpcm/src/codec_ms.rs:96`):

```rust
self.delta = (MS_ADAPTATION_TABLE[nibble as usize] * self.delta) / 256;
```

`self.delta` is seeded directly from a 16-bit value in the block preamble
(`u16_to_i32!(stream.read_u16()?)`, which is `stream.read_u16()? as i16 as i32` - sign-extended, so
its effective range is -32768..=32767) and is **never bounded above** afterwards; only
`self.delta = self.delta.max(DELTA_MIN)` clamps it from *below*. Every adaptation-table entry is
positive and the largest, `MS_ADAPTATION_TABLE[8] == 768`, multiplies `delta` by exactly 3 each time
that nibble value (`0x8`) is decoded (`768 / 256 == 3`). Since the encoded bitstream fully controls
which nibble appears in every byte, repeating nibble value 8 a handful of times in a row grows
`delta` geometrically with no upper bound: starting from the maximum seed value 32767, the 6th
repetition makes `MS_ADAPTATION_TABLE[8] * self.delta` (`768 * 7,962,381`) exceed `i32::MAX`
(2,147,483,647) - the multiplication itself overflows before the `/ 256` ever runs, panicking in
any debug-assertions/overflow-checks build (the default for `cargo fuzz build`, and for any
consumer of this crate building in debug mode). In a plain release build (`overflow-checks` off,
Symphonia's own default release profile), the same input doesn't panic but silently wraps `delta`
to an arbitrary value via two's-complement wraparound - a real, silent logic bug (corrupted decoded
samples and a corrupted `delta` state carried into every subsequent nibble in the block) even where
it doesn't crash.

This is unreachable via the pre-candidate-012 fuzz harness (candidate 012's own finding: the
harness always builds packets with `Duration::ZERO`, so `AdpcmDecoder::decode_inner`'s
`block_count = packet.block_dur().get() as usize / frames_per_block` is always `0` and the
block-decode loop containing `expand_nibble` never runs) - but it *is* reachable by any real
consumer of this crate decoding a real (malformed/adversarial) MS ADPCM WAV file, since a real
demuxer sets a correct non-zero packet duration. Candidate 012's fix was needed only to make the
*fuzz target* exercise this code, not to make the underlying decoder path exploitable in
production.

## Introduced by

4c508b2 "adpcm: Add ADPCM Decoder (#160)" (2022-11-08) - `git log -S "MS_ADAPTATION_TABLE[nibble as
usize] * self.delta"` on `symphonia-codec-adpcm/src/codec_ms.rs` shows this line (in this exact
form) has existed unchanged since this commit, which added the ADPCM decoder crate from scratch.
Same introducing commit as candidate 002's `frames_per_block` bug in the same file.

## The fix

Changed `AdpcmMsBlockStatus::expand_nibble` to use `wrapping_mul`/`wrapping_add` throughout
(matching the precedent already established in `symphonia-codec-alac`'s
`decorrelate_mid_side`/`predict_fir` fixes, candidates 029/030, for the same class of
attacker-controlled-magnitude overflow): both the `delta`-update multiplication
(`MS_ADAPTATION_TABLE[...] * self.delta`) and the predictor computation that reads the *previous*
`delta` (`signed_nibble * self.delta`, plus the `sample1*coeff1 + sample2*coeff2` term feeding into
the same expression) now wrap on overflow instead of panicking - fixing only the update line and
leaving the predictor line's multiplication untouched would just move the panic to the *next*
nibble decoded with an extreme wrapped `delta` value, so both needed the same treatment. This
matches how the reference (Microsoft's) codec's C `int` arithmetic behaves in practice on real
hardware, and keeps behavior on well-formed streams (where `delta` never approaches these
magnitudes) completely unchanged.

Verified against more than the exact triggering input (CONFIRMATION.md section 7): re-ran the
patched tree with a 30-combination boundary sweep (5 different `delta` seeds x 6 different payload
fill bytes, each a full 1022-byte block) - no panics, all returned `Ok(())` (see "What was tried").

- Re-running `repro/` against the patched tree: no longer panics; `decode()` returns
  `Err("buffer underrun")` (the crafted packet is deliberately short, past the point the overflow
  used to occur) - 3/3 runs, debug build, deterministic.
- Isolation check (Critical pitfall #6): reverted only this fix (`git stash` on
  `symphonia-codec-adpcm/src/codec_ms.rs` alone, `symphonia/fuzz/src/lib.rs`'s unrelated candidate
  012 patch left applied) and re-ran `repro/` - the panic came back, confirming this specific hunk
  (and not some other coincident state) is what fixes it.
- Project test suite (`cargo test --workspace`, main Symphonia workspace - `symphonia/fuzz` is its
  own separate `[workspace]` and is never part of this): **106 passed, 0 failed** before the fix,
  **106 passed, 0 failed** after (identical pass count both times; all doc-tests across every crate
  also 0 passed/0 failed both times, i.e. unaffected).

## What's not done / open questions

- The sibling IMA ADPCM decoders (`codec_ima_wav.rs`, `codec_ima_qt.rs`, sharing
  `common_ima::AdpcmImaBlockStatus::expand_nibble`) were checked for the same overflow class and do
  **not** have it: their `step` value is read from a fixed 89-entry table (`IMA_STEP_TABLE`, max
  32767) indexed by a `step_index` that is explicitly `.clamp(0, 88)`'d every update, so `step`
  itself can never grow - only MS ADPCM's `delta` has this unbounded-growth property. No further
  fix needed there.
- This session's fuzz run was a single ~200s burst after the initial hang investigation; a longer,
  dedicated campaign (`fuzz-bug-repro`, run separately) against `decode_adpcm` - now that
  candidate 012's harness fix makes the block-decode loop actually reachable - would likely be
  worth running, given this bug turned up within the first ~18,000 executions with no seed corpus.
- Candidate 012's own patch (`fuzz_audio_decoder`'s `Duration::ZERO` -> `max_frames_per_packet`
  fix) was applied only inside this candidate's isolated worktree to make the fuzz target reachable
  for this investigation; per the workspace's standing rule, the main project tree was reverted and
  candidate 012's fix itself is not re-applied to the real source by this candidate - it remains a
  separate, already-recorded candidate with its own `patch.diff`.
