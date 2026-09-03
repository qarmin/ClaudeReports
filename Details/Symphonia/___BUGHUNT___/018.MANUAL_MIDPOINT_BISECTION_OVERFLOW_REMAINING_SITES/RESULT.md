# Remaining `clippy::manual_midpoint` bisection sites (util.rs / OGG demuxer.rs / OGG physical.rs)

`cargo clippy --pedantic` flags four `(a + b) / 2` bisection-midpoint sites in total.
`symphonia-bundle-flac/src/demuxer.rs:322` was already confirmed exploitable (candidate 016 -
FLAC's `SeekPoint::byte_offset` is a raw, unvalidated `u64` read directly off attacker-controlled
SEEKTABLE bytes). This candidate checks the 3 siblings candidate 016 flagged as "not exploited this
session" - `symphonia-core/src/formats/util.rs:120` (`usize` seek-index binary search),
`symphonia-format-ogg/src/demuxer.rs:192`, and `symphonia-format-ogg/src/physical.rs:130` (both
`u64` byte-offset bisection) - to determine, concretely, whether any of them is reachable with
realistic attacker-controlled input the way FLAC's was.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Provenance tracing: for each of the 3 sites, traced every operand back to its ultimate source | Code reading (`symphonia-core/src/formats/util.rs`, `symphonia-format-ogg/src/{demuxer,physical}.rs`, `symphonia-core/src/io/{mod,media_source_stream}.rs`, `symphonia-metadata/src/embedded/flac.rs`) | `util.rs:120`'s `lower`/`upper` are `usize` indices into `SeekIndex.points: Vec<SeekPoint>`; the only public way to grow that `Vec` is `SeekIndex::insert()`, and the only caller of `insert()` anywhere in the workspace is FLAC's `read_flac_seektable_block` (`symphonia-metadata/src/embedded/flac.rs`) - itself capped at ~932,067 points by a FLAC metadata block's 24-bit length field (16,777,215 bytes / 18 bytes-per-point). `demuxer.rs:192`'s `start_byte_pos`/`end_byte_pos` and `physical.rs:130`'s `start`/`end` all trace back to `self.reader.pos()` (a genuine absolute stream position tracked by `MediaSourceStream`) and `self.reader.byte_len()`, which for every `MediaSource` impl in this codebase (`std::fs::File`, `io::Cursor<T>`) is the real, filesystem-/allocation-backed length (`std::fs::File::metadata().len()` / `Vec::len()`) - never a field read out of the file's own bytes the way FLAC's `SeekPoint::byte_offset` is. `ReadOnlySource` (unseekable) returns `byte_len() -> None`, and both OGG bisection sites are only reached when `is_seekable()` is true, so that path is excluded entirely. |
| Arithmetic feasibility calculation (manual boundary-value reasoning, no code run) | N/A | For `util.rs:120` to overflow, `self.points.len()` would need to exceed roughly `usize::MAX/2` (~9.22e18 on 64-bit, ~2.1e9 on 32-bit) - but `Vec<T>` can never hold more than `isize::MAX` *bytes* regardless of platform, and `SeekPoint` is >= 20 bytes, so `Vec::push` panics with its own "capacity overflow" (or the allocator returns `Err`/aborts on OOM) enormously before `len()` gets anywhere near that threshold, on every platform, independent of the ~932,067-point real-world FLAC ceiling already established above. For the two OGG sites to overflow, `start + end` (both real byte offsets bounded by actual file size / `Vec` length) would need to exceed `u64::MAX`, i.e. a real file (or in-memory buffer) of roughly 9.2 exabytes - not a plausible on-disk or in-memory artifact on any real system today. |
| Debug-assertions build (cargo's own `dev` profile default, `overflow-checks=true`) + a real, standalone `repro/` exercising all 3 sites end-to-end through the real public API: `SeekIndex::insert`/`search` called 2,000,000 times (>2x the real FLAC ceiling) directly; 3 genuine, ffmpeg-encoded OGG Vorbis files (a small 2-physical-stream chained file, a larger 96KB chained file sized to force the "media source stream is chained, bisecting end of physical stream" branch, and a 231KB single-physical-stream file sized to force `demuxer.rs`'s own `while end_byte_pos - start_byte_pos > 2*OGG_PAGE_MAX_SIZE` bisection loop) opened/seeked through `symphonia::default::get_probe()` + `FormatReader::seek` | Debug build (`cargo run`) and release build (`cargo run --release`), `RUST_LOG=debug` to confirm via the crate's own log lines which branch actually executed; 3 consecutive debug runs | All 3 sites' surrounding code paths were confirmed to actually execute (not just reasoned about): `RUST_LOG=debug` shows `"media source stream is chained, bisecting end of physical stream"` (confirms `physical.rs:130`'s bisection loop ran) and `"seek: bisect step: ... byte_range=[3935, 117867, 231799]"` (confirms `demuxer.rs:192`'s loop ran, with `mid_byte_pos = (3935+231799)/2 = 117867` exactly, both operands genuine file-position values). `util.rs:120`'s midpoint ran on every one of 14,599 `search()` calls across 2,000,000 real `SeekPoint`s. No panic, no crash, in any of the 3 cases, debug or release, 3/3 repeat runs identical. |

## Status

**Not reproducible.** Covered: full provenance trace of every operand feeding all 3 sites (not a
fuzz campaign - this is a "can the value ever be large enough" question, better answered by tracing
every real producer of these values than by fuzzing for an overflow the tracing already shows
cannot occur at any realistic scale), an arithmetic feasibility calculation for the actual
threshold each would need to hit, and a real, end-to-end `repro/` (3 debug runs + 1 release run,
deterministic, no crash) that drives the real surrounding code paths at the largest scale
realistically constructible in this session (2,000,000 SeekPoints; a 231KB real OGG file large
enough to force both OGG-side bisection loops to actually execute, confirmed via debug logging, not
just assumed).

**Fully reproducible**: N/A (no bug found to report a reproduction rate for; the negative result
itself reproduced 4/4 runs across debug and release builds, and the 3 code paths under
investigation were confirmed via `RUST_LOG=debug` to have actually executed in every run, not
silently skipped).

## Class

N/A (no bug found).

## Root cause

Not applicable (no bug found) - but *why* these 3 sites turn out safe, unlike FLAC's, is itself the
useful finding, and it is the same distinction candidate 016 already drew, now verified end-to-end
rather than by code reading alone:

- **`util.rs:120`** (`SeekIndex::search`'s `(lower + upper) / 2`): `lower`/`upper` are `usize`
  indices into a `Vec<SeekPoint>` that is only ever grown, workspace-wide, by FLAC's own
  `read_flac_seektable_block`, which is itself hard-capped at ~932,067 points by FLAC's own 24-bit
  metadata-block-length field. Even setting that ceiling aside entirely, `Vec<T>`'s own
  `isize::MAX`-byte allocation limit means no real process on any platform can ever grow this `Vec`
  anywhere close to the ~`usize::MAX/2` length that would be needed to overflow the addition -
  this is a structural guarantee of `Vec`'s own allocator discipline, not something specific to how
  FLAC happens to populate it today.
- **`demuxer.rs:192`/`physical.rs:130`** (`(start + end) / 2` on `u64` byte offsets): unlike FLAC's
  `SeekPoint::byte_offset` (a raw `u64` read directly off attacker-controlled file bytes, with zero
  validation against real stream length), every operand feeding these two sites is either
  `self.reader.pos()` (an absolute position `MediaSourceStream` derives purely from how many bytes
  it has actually read/sought through) or `self.reader.byte_len()` (which, for every `MediaSource`
  implementation that exists in this codebase - `std::fs::File`, `io::Cursor<T>` - reflects the
  real, OS-/allocation-backed length of the underlying data, not a value the file's own bytes can
  independently claim to be anything else). Reaching `u64::MAX`/2 in either operand would require
  an actual ~9.2-exabyte file or in-memory buffer - not a value any adversarial byte sequence
  embedded *within* a realistically-sized file can manufacture, which is exactly the property that
  made FLAC's case exploitable and these two not.

One theoretical narrowing worth naming explicitly (not pursued further, see below): `byte_len()`
is a method any *caller-supplied* `MediaSource` implementation gets to define arbitrarily (e.g. an
application wrapping a network stream that misreports `Content-Length`). If such an implementation
lied about its own length while `is_seekable()` also returned `true`, `total_len` could in
principle be an arbitrary, non-file-backed `u64`. This is a different trust boundary than "bytes
inside the file" though - it requires the *host application* to plug in a dishonest `MediaSource`,
not an attacker to craft a malicious OGG file read through a normal, honest source (`File`/
`Cursor`) - and is therefore out of scope for "the same class of bug as FLAC's SEEKTABLE issue",
which is specifically about trusting a value that came from inside the untrusted file itself.

## Introduced by

N/A - no bug found to attribute to a commit.

## The fix

N/A - no bug found, so no fix was made or needed. The `clippy::manual_midpoint` lint itself remains
technically "correct" at all 3 of these sites in the sense that `lower + (upper - lower) / 2` /
`start + (end - start) / 2` is strictly more defensive than `(a + b) / 2` with no downside, so a
maintainer could still choose to apply the lint's suggestion purely as defense-in-depth / to silence
the lint - but this candidate found no reachable input that actually needs that rewrite to avoid a
real panic, unlike candidate 016's FLAC fix.

Project test suite: not run for this candidate (no source change was made to verify against - see
`SKILL.md` step 7, which only mandates a before/after test-suite run for a *verified fix*; this
candidate found nothing to fix).

## What's not done / open questions

- The "dishonest caller-supplied `MediaSource`" scenario named above (a `MediaSource` impl that
  reports `is_seekable() == true` and an arbitrary, non-data-backed `byte_len()`) was reasoned
  about but not built and run - it's a different threat model (a buggy/malicious *host
  application-supplied* trait implementation, not a malicious *file*) from what this candidate and
  candidate 016 were both scoped to, and this session judged it out of scope rather than spending
  budget on it; a future candidate could check what MediaSourceStream/OggReader actually do if
  handed such an implementation, if that trust boundary is ever considered in scope.
- This candidate did not re-check whether any *other* format reader in the workspace (beyond FLAC)
  populates a `SeekIndex` via `insert()` with a value that isn't bounded the way FLAC's is - as of
  this codebase, FLAC is the only caller, so the question is currently moot, but would need
  re-checking if a future format reader starts using `SeekIndex` with a less-bounded input.
- Not a fuzzing campaign in the `fuzz-bug-repro` sense - the provenance trace already rules out any
  input-dependent path to the required magnitudes, so a longer fuzz run against these 3 sites
  specifically would not be expected to find anything beyond what a few minutes of targeted
  boundary-value construction (this session) already showed.
