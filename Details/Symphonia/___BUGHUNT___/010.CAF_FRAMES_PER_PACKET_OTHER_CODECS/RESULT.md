# CAF `frames_per_packet` trusted for other codecs (AAC, ALAC, FLAC, Opus, MPEG layers) - checked for the same root cause as candidate 003

Follow-on from candidate 003 (CAF -> ADPCM IMA QT index-out-of-bounds via attacker-controlled
`frames_per_packet`), whose own "What's not done" section explicitly flagged this as unchecked:
`symphonia-format-caf`'s `read_audio_description_chunk` passes the CAF file's own
`desc.frames_per_packet` field straight through as both `AudioCodecParameters::max_frames_per_packet`
and `::frames_per_block` for *every* codec it selects, not just `ima4`. Checked whether AAC, ALAC,
FLAC, Opus, and the MPEG audio layers (MP1/MP2/MP3) share the same exploitable pattern: a decoder
that blindly indexes a fixed-size buffer using a container-supplied count without cross-checking it
against the buffer's real length.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `symphonia-format-caf/src/demuxer.rs::read_audio_description_chunk` | manual | Confirmed the untrusted-passthrough happens for every codec, not just ima4 (lines ~420-432): `with_max_frames_per_packet`/`with_frames_per_block` are set from `desc.frames_per_packet` whenever it isn't exactly `1`, regardless of `desc.codec_id()`. |
| `grep -rn "frames_per_block\|max_frames_per_packet"` across every codec crate (`symphonia-codec-aac`, `symphonia-codec-alac`, `symphonia-bundle-flac`, `symphonia-codec-opus`, `symphonia-bundle-mp3`) plus `symphonia-core`/`symphonia-codec-pcm` for comparison | manual, ripgrep | AAC, ALAC (read side), Opus, and MP1/2/3 **never reference these fields at all** - only `symphonia-codec-adpcm` (both fields, unconditionally, is the candidate-003 bug) and `symphonia-codec-pcm` (as a `Vec` pre-allocation capacity hint only, not a fixed indexing bound - buffer still grows dynamically if the hint is wrong) read them back. FLAC's decoder *writes* `max_frames_per_packet` (from its own `STREAMINFO`), it never reads what CAF set. |
| Read each decoder's actual buffer-sizing call site (`render_uninit`/`render_silence`/`AudioBuffer::new` in `symphonia-codec-aac`, `symphonia-codec-alac`, `symphonia-bundle-flac`, `symphonia-bundle-mp3`) | manual | AAC: `render_uninit(None)` (dynamic). MP1/2/3: hardcoded per-spec constants (384/1152/576 samples), independent of any codec_params field. ALAC: buffer size driven by `cookie.frame_length`, parsed from the ALAC magic cookie the decoder reads for itself (`MagicCookie::read`), completely independent of CAF's `desc.frames_per_packet`. FLAC: `FlacDecoder::try_new` clones `params` and immediately **overwrites** `max_frames_per_packet` with `info.block_len_max` from its own `STREAMINFO`, and buffer sizing (`render_uninit(Some(header.block_num_samples))`) uses a value parsed from each individual FLAC frame header, not from `codec_params` at all. |
| Checked whether Opus is even reachable via the real `symphonia` crate | manual (`grep opus symphonia/Cargo.toml symphonia/src/lib.rs`) | No `opus` feature exists in `symphonia/Cargo.toml`, and `register_enabled_codecs`/`register_enabled_formats` in `symphonia/src/lib.rs` never registers an Opus decoder. `symphonia-codec-opus` exists as a crate but is not wired into the facade crate at all - a CAF file selecting `CODEC_ID_OPUS` cannot get a decoder via the real public API today, regardless of this bug pattern. |
| End-to-end manual boundary-value construction: 5 minimal, real CAF files (one per codec family - AAC, ALAC, FLAC, MP3, Opus), each declaring `frames_per_packet = 0xFFFFFFF0` (an absurd, attacker-chosen value analogous to candidate 003's `frames_per_packet = 2`), fed through the real `symphonia::default::get_probe().probe(...) -> format.next_packet() -> decoder.decode(...)` pipeline (`repro/main.rs`) | debug build (`cargo run`), 3 runs; then release build (`cargo run --release`) via `just test` from a fresh clone, 1 more run | All 5 cases completed without a panic, every run, byte-for-byte identical output. `codec_params.max_frames_per_packet`/`frames_per_block` are confirmed to carry the fully attacker-controlled value (`4294967280`) through to the decoder-construction call for AAC and MP3 (the "Fixed audio packet" path), same as ima4 does - but neither decoder's own `decode()` ever reads that field, so nothing indexes with it. ALAC and FLAC decoders (given a valid, separately-supplied magic cookie/`STREAMINFO`, as any real file needs) actually ran `decode()` on the packet and still didn't panic, because their own extra-data-derived frame length is what governs buffer size, not `codec_params`. Opus never got a decoder at all (`unsupported audio codec`, since it isn't registered), consistent with the previous finding. |

## Status

**Not reproducible.** No approach found a code path where AAC, ALAC, FLAC, Opus, or MP1/2/3
consume the CAF-supplied `frames_per_packet`-derived fields (`max_frames_per_packet`/
`frames_per_block`) for buffer indexing the way `symphonia-codec-adpcm`'s IMA QT decoder does.
Covered: full source read of every reachable decoder's buffer-allocation/indexing call sites for
these 5 codec families (not a fuzz campaign - this is a structural "is the tainted value ever
read back" question, better answered by reading every consumer than by fuzzing for a crash that
the code shows cannot occur), plus a real end-to-end boundary-value repro exercising all 5 through
the actual public API with an absurd `frames_per_packet`, 4 runs total (3 debug + 1 release),
deterministic, no crash.

**Fully reproducible**: N/A (no bug reproduced to report a reproduction rate for; the negative
result itself was reproduced 4/4 runs across debug and release builds).

## Root cause

Not applicable (no bug found) - but the reason the suspect turned out safe is itself the useful
finding: `AudioCodecParameters::max_frames_per_packet`/`frames_per_block` are plain, optional
struct fields (`symphonia-core/src/codecs/audio.rs`) with no inherent safety contract - they are
only dangerous to a specific decoder if that decoder chooses to read them back and trust them for
fixed-size buffer indexing without validating against the packet/buffer's actual length. Auditing
every codec CAF can select today shows only one decoder does that: `symphonia-codec-adpcm`'s IMA
QT/MS variants (candidate 003's finding, and MS ADPCM's own analogous gap from candidate 002,
though MS ADPCM isn't reachable via CAF). The other five codec families avoid the pattern for
different, uncoordinated reasons - not because of any deliberate cross-check in
`symphonia-format-caf` itself:
- AAC, MP1/2/3: never look at these fields; frame/buffer sizing is either fully dynamic or a
  hardcoded per-spec constant.
- ALAC, FLAC: do read a "frames per block"-like value, but from their own codec-specific extra
  data (ALAC magic cookie's `frame_length`, FLAC's `STREAMINFO.block_len_max`/each frame's own
  header), which the decoder parses and validates itself, independently of (and, for FLAC,
  explicitly overwriting) whatever `symphonia-format-caf` set from the container header.
- Opus: not reachable at all via the real `symphonia` crate's default codec registry - a separate,
  pre-existing fact unrelated to this bug pattern.

So `symphonia-format-caf` still passes an unvalidated container field through indiscriminately
(the demuxer-side half of candidate 003's two-part root cause is unchanged and still applies to
every codec, not just ima4), but that half alone is inert for these five codec families because
nothing downstream consumes it unsafely - the vulnerability requires *both* halves (an untrusted
source **and** a consumer that trusts it for indexing) to be present, and only ADPCM currently has
the second half.

## Introduced by

N/A - no bug found to attribute to a commit.

## The fix

N/A - no bug found, so no fix was made or needed for this candidate. (The demuxer-side passthrough
itself remains exactly as candidate 003 described it and is unchanged by this candidate; candidate
003's `RESULT.md` already suggests a defense-in-depth option - CAF rejecting/clamping an implausible
`frames_per_packet` for codecs it knows have fixed block sizes - as something a maintainer could
still choose to do, independent of whether any currently-registered decoder is exploitable through
it today.)

Project test suite: not run for this candidate (no source change was made to verify against - see
`SKILL.md` step 7, which only mandates a before/after test-suite run for a *verified fix*; this
candidate found nothing to fix).

## What's not done / open questions

- `symphonia-codec-vorbis` (used by CAF? no - CAF's `codec_id()` has no Vorbis mapping at all) was
  not in scope and wasn't checked; confirmed via `symphonia-format-caf/src/chunks.rs::codec_id()`
  that Vorbis isn't one of the codecs CAF can select in the first place.
- This candidate only checked the specific "trusted count used for buffer indexing" pattern from
  candidate 003. It did not re-audit AAC/ALAC/FLAC/MP3/Opus decoders for unrelated bug classes,
  nor did it fuzz them - the "not reproducible" verdict here is scoped strictly to this one root
  cause, not a general clean bill of health for these decoders.
- If a future PR wires up an `opus` feature in the `symphonia` facade crate (registering
  `symphonia-codec-opus`'s decoder), this exact question (does the Opus decoder trust
  `frames_per_block`/`max_frames_per_packet`?) should be re-checked at that point - a quick look at
  `symphonia-codec-opus/src/*.rs` today shows no reference to either field, but that crate isn't
  currently reachable end-to-end to verify runtime behavior via the real public API, so this is a
  static-only observation for Opus specifically.
- The demuxer-side gap itself (CAF setting these fields from an unvalidated container field for
  every codec, not just the ones that turn out to be safe) was left exactly as-is, per candidate
  003's own decision to fix the decoder side instead - still worth a maintainer's defense-in-depth
  consideration, but not itself a reproducible bug on the current decoder set.
