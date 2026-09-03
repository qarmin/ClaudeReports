# Bulk single-byte character-set decode/encode (bypass generic RawDecoder/RawEncoder dispatch)

`encoding/src/text.rs`'s `DefaultCharacterSetCodec` and every `decl_character_set!`-generated
codec (including the default `ISO_IR 6` and the common `ISO_IR 100`) go through the external
`encoding` crate's generic per-character virtual-dispatch `RawDecoder`/`RawEncoder` +
`StringWriter`/`ByteWriter` machinery - one call per input byte, each redoing UTF-8-length/
capacity-growth bookkeeping. Since a true single-byte character set (a `SingleByteEncoding` in the
`encoding` crate: ISO-8859-1..8, WINDOWS_874) is a trivial 1:1 byte<->codepoint mapping, a
hand-written bulk conversion should beat this by a wide margin. Carried over as a well-specified
lead from a prior session's notes (candidate 001 in
`~/.claude/skills/perf-optimize/projects/dicom-rs.md`), re-implemented and re-measured fresh here
(base commit `df2b9e4236298efa3fa37a1f360fc4bf284d586a`).

**This RESULT.md was substantially revised after an initial round of testing** (see "Revision
history" at the bottom) - the first round, against a pixel-data-heavy fixture, found no measurable
end-to-end effect and directly contradicted the prior session's own claimed regression. A second
round against a text-heavy synthetic fixture (built specifically to answer that round's own open
question) found the *opposite*: real end-to-end wins on `parse`/`roundtrip`, but also a large, real,
reproducible **regression specific to repeated `write_all` calls on an already-parsed object**
(`write-only` mode) - which does, after all, corroborate the prior session's own regression
finding, just under different conditions (a text-heavy file, `write-only` specifically) than this
session initially tested. Read the whole file, not just the top - the picture only became clear
after the second round.

## What was tried

Added `fast_singlebyte_decode`/`fast_singlebyte_encode` (`encoding/src/text.rs`), which call the
`encoding` crate's own `SingleByteEncoding`'s public `index_forward: fn(u8) -> u16` /
`index_backward: fn(u32) -> u8` function-pointer fields directly in a plain loop, replicating
`SingleByteDecoder::raw_feed`/`SingleByteEncoder::raw_feed` and the existing `decode_text_trap`
(octal-escape fallback for an unmapped byte) exactly. Added a `decl_singlebyte_character_set!`
macro (parallel to the existing `decl_character_set!`) that uses these fast functions, and
switched only the genuine `SingleByteEncoding` instantiations to it: `ISO_IR 100/101/109/110/126/
127/138/144/166` (ISO-8859-1..8, WINDOWS_874) and `DefaultCharacterSetCodec` (also ISO-8859-1).
`ISO_IR 13` (WINDOWS_31J), `ISO_IR 87` (ISO_2022_JP), `ISO_IR 149` (WINDOWS_949), `GB18030`, `GBK`
are each their own distinct multi-byte codec type in the `encoding` crate and were left on the
original generic path, since they have no `index_forward`/`index_backward` shortcut.

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s `text-decode`/`text-encode` isolated
microbenchmarks (repeated realistic Latin-1 text with accented characters), the existing
`parse`/`write-only`/`roundtrip` modes against `color3d_jpeg_baseline.dcm` (6.14MB,
pixel-data-heavy, confirmed via `dump`+`strings` to carry a real `ISO_IR 100` `SpecificCharacterSet`
element), and a new `gen-seq`-built synthetic fixture, `seq_10k.dcm` (800KB, one `SQ` element with
10,000 nested items, each holding 2 short `UI`-VR strings - 20,000 small text elements, no pixel
data at all, chosen specifically to be text-decode/encode-dominated rather than pixel-dominated).

| Variant | text-decode/-encode (isolated) | color3d parse/roundtrip | seq_10k parse/roundtrip | seq_10k **write-only** |
|---|---|---|---|---|
| baseline | 0.031/0.026 ms | 1.863/4.021 ms | 7.91-7.97/11.90 ms | 5.10 ms |
| patched | 0.011/0.015 ms | no change/no change | 6.88-6.91/10.49 ms | **9.87-9.89 ms** |

## Result

**Isolated microbenchmarks: real, large, reproducible wins** - decode -63.3%/-63.8%/-64.8% (3
rounds, 9/9 each), encode -31.2%/-42.9% (2 rounds, 9/9 each).

**color3d.dcm (pixel-data-heavy, ~40% of a callgrind `Ir` profile in this exact text-decode path
per a later session discovery - candidate 011/012's own profiling work - yet still): no consistent
wall-clock change anywhere** - `parse`: -3.3%, 7/9 (not consistent); `write-only`: -0.4%, 5/9 (not
consistent); `roundtrip`: -0.1%/-1.5% across two rounds (not consistent either time). Wall-clock
doesn't move even though a large fraction of *instructions* are spent here - this file's pixel
fragment memcpy is bandwidth-bound and dominates wall-clock far more than its instruction-count
share would suggest (a real instance of Ir-vs-wall-clock divergence, not a contradiction).

**seq_10k.dcm (text-heavy, no pixel data): real, large, reproducible wins on `parse` (-13.7%/-12.6%,
2 rounds, 9/9 each) and `roundtrip` (-11.8%, 9/9)** - this directly answers this candidate's own
original open question ("does a text-heavy fixture show the isolated win end-to-end?") with a clear
yes. **But `write-only` alone (parse once outside the loop, then repeat `write_all` many times on
the same already-decoded object) is a massive, reproducible regression: +93.4%/+93.9% across two
rounds, 0/9 both times** - nearly 2x slower, not a rounding-error-sized effect. Manually re-checked
outside the paired script at low iteration counts (`n=18`) to rule out a warmup/measurement
artifact: patched was consistently ~2x slower from the very first iterations, not a growing/leaking
effect. Bit-exact in every mode on both fixtures.

**This second round does, after all, corroborate the prior session's own claimed regression** -
the initial round's "no regression found, prior notes may have had a confound" conclusion was
itself a symptom of testing the wrong fixture shape (pixel-heavy, `roundtrip`-only) for what turns
out to be a `write-only`-and-text-heavy-specific effect. The magnitude found here (+93%) is far
larger than the prior notes' own remembered figure (~11-15%), consistent with this being a
different, more text-content-heavy fixture than whatever the prior session used.

## Does it change behavior/output?

- **Bit-exact?** Yes, in every mode on both fixtures tested. `text-decode`/`text-encode` checksums,
  and `parse`/`write-only`/`roundtrip` checksums on both `color3d_jpeg_baseline.dcm` and
  `seq_10k.dcm`, are identical between baseline and patched builds.
- **Is this a breaking change?** No public API/type changes - `SpecificCharacterSet`/`TextCodec`'s
  public interface is unchanged; `fast_singlebyte_decode`/`fast_singlebyte_encode` and
  `decl_singlebyte_character_set!` are private/internal to `encoding/src/text.rs`.
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures. All 15 existing
  `text::tests::*_baseline` unit tests (covering every character set, including the ones switched
  to the fast path) pass unchanged.

## Why the result looks the way it does

The isolated microbenchmark's win is real and matches the mechanism exactly as predicted: removing
per-byte virtual dispatch and UTF-8 bookkeeping in favor of a plain loop calling a function pointer
directly is a large win when text decode/encode is actually the bottleneck - confirmed at the whole
-file level on `seq_10k.dcm`'s `parse`/`roundtrip`. `color3d.dcm` shows no wall-clock change because
its cost is dominated by a large, bandwidth-bound pixel-fragment memcpy that swamps any
text-handling difference in wall-clock terms, even though text-decode is a large *instruction*
fraction there.

**The `write-only`-specific regression's root cause was not confirmed** (bounded time, per
SKILL.md's guidance on regressions) - it does not reproduce in the isolated `text-encode`
microbenchmark (which repeatedly encodes *one* ~5KB string) or in `roundtrip` (which also calls
`write_all` every iteration, just interleaved with a fresh `parse`), only in `write-only`'s specific
pattern of calling `write_all` many times in a row on the *same*, already-decoded object with
20,000 separate small strings. The leading, unconfirmed hypothesis (consistent with this
workspace's own candidate-001 "surprising finding" about allocator-arena effects not visible in
instruction counts) is an allocator interaction specific to `fast_singlebyte_encode`'s
`Vec::with_capacity(text.len())` - allocating one small `Vec` per string, 20,000 times per
`write_all` call, repeated many times on a *stable* set of same-sized strings - behaving worse
under glibc's allocator than whatever pattern the original generic `encoding` crate path produced
for the same strings. This is a hypothesis, not a confirmed mechanism; a `valgrind --tool=massif`
allocation-histogram comparison (available on this box, used successfully for a similar question in
prior session notes) would be the natural next step, not attempted this session.

## What's not done / open questions

- **The `write-only` regression's root cause is unconfirmed** - see "Why" above. This is the single
  most important open item for whoever picks this candidate up next: is it fixable (e.g. reusing a
  scratch buffer across calls instead of allocating one `Vec` per string), or is it inherent to the
  bulk approach for this specific repeated-small-allocation pattern?
- Given the size and reproducibility of the `write-only` regression, **taking this patch as-is would
  be a bad trade for any workload that writes the same in-memory object repeatedly without
  re-parsing** (e.g. a server holding a decoded object and sending it to multiple destinations) -
  stated as a fact for the reader to weigh, not a recommendation either way, per this skill's own
  "no verdict" discipline.
- `GB18030`/`GBK`/`WINDOWS_31J`/`ISO_2022_JP`/`WINDOWS_949` remain on the slow generic path, as
  designed - they are not `SingleByteEncoding` instances in the `encoding` crate and have no
  equivalent function-pointer shortcut available.

## Revision history

- **Round 1** (this session): tested only `color3d_jpeg_baseline.dcm` (pixel-heavy). Found no
  end-to-end change anywhere, concluded the prior session's claimed regression didn't reproduce.
  That conclusion is **superseded** by round 2 below - it was correct for that fixture, but
  incomplete as a general statement about this patch.
- **Round 2** (this session, later): built `seq_10k.dcm` (text-heavy, no pixel data) specifically to
  test this candidate's own "not tested: a text-heavy fixture" open question (originally logged in
  `IDEAS.md`). Found real end-to-end wins on `parse`/`roundtrip`, and separately, while checking
  `write-only` for completeness, found the large regression described above. This file was rewritten
  to present both rounds' findings together rather than leaving the superseded round-1-only
  conclusion in place - see `patch.diff` for the unchanged actual code (identical since round 1).

## Follow-up: the `write-only` regression is glibc-allocator-specific, not fundamental

Tested whether the `write-only` regression is inherent to this patch or an artifact of this box's
default allocator (glibc malloc) by rebuilding the harness with a `diag-mimalloc` feature (added
this session - swaps the global allocator to `mimalloc` via `#[global_allocator]`, see
`___OPTIMIZATIONS___/README.md`) and re-running the exact same `write-only` comparison on
`seq_10k.dcm`.

**With mimalloc instead of glibc's default allocator, the same patch shows a real, reproducible
WIN instead of a regression: -7.4%/-17.5%/-18.0% across three rounds, 9/9 consistent every time.**
This strongly confirms the leading hypothesis above (an allocator-arena interaction) and narrows it
considerably: the regression is not inherent to the code change itself, which is algorithmically
sound and a real win under a non-pathological allocator - it is specific to how glibc's malloc
(the default on this box, and the Linux default generally) handles this particular pattern of many
small, similarly-sized, repeated alloc/free cycles.

**Practical implication for whoever reads this**: if your application already uses (or is willing
to use) a non-default global allocator such as `mimalloc` or `jemalloc` - a common choice for
allocation-heavy Rust services - this patch is very likely a clean win with no caveat. If your
application uses the system default allocator (glibc malloc on Linux), the `write-only`-specific
regression characterized above still applies. This is not something `dicom-rs` itself can fix (a
library should not unilaterally override its dependents' global allocator) - it is information for
whoever decides whether to take this patch, per this skill's "no verdict, just facts" discipline.
