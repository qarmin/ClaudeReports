# Text-codec multi-byte bulk decode/encode: profiling step (measurement-only, no patch)

Candidate 005 (`005.TEXT_CODEC_BULK_SINGLE_BYTE`) replaced the generic `encoding`-crate
per-character `RawDecoder`/`RawEncoder` dispatch with a hand-written bulk loop for every
`SingleByteEncoding` codec, and left the 5 genuinely multi-byte codecs still on
`decl_character_set!`'s original generic path: `ISO_IR 13` (WINDOWS_31J), `ISO_IR 87`
(ISO_2022_JP), `ISO_IR 149` (WINDOWS_949), `GB18030`, `GBK`. This candidate was scoped as
"profile first, before committing to the larger state-machine implementation" - this is that
profiling step, done for real with a new isolated microbenchmark and `perf record` against the
current (unpatched) code, not a re-implementation.

## What was tried

Extended `___OPTIMIZATIONS___/harness/bench_pipeline` with `text-decode-cs <code> <iters>` /
`text-encode-cs <code> <iters>` (real Japanese/Korean/Chinese sample text per codec, chosen so
each codec's actual multi-byte lead-byte path is exercised, not ASCII passthrough - see
`README.md`'s new row). Measured decode p50 twice (pinned single-core, `taskset -c 3`, through
`flock`) plus one unpinned run, then profiled each codec's `decode()` call with
`perf record -g` (`perf_event_paranoid=-1` on this box, no privilege change needed).

| Charset (`SpecificCharacterSet` code) | encoding-crate type | decode p50 round 1 | decode p50 round 2 | decode p50 unpinned | ns/input-byte |
|---|---|---|---|---|---|
| `ISO_IR 13` | WINDOWS_31J (Shift-JIS-shaped) | 4.499us | 4.468us | 4.478us | ~1.96 |
| `ISO_IR 87` | ISO_2022_JP (escape-sequence-shifted) | 4.118us | 3.817us | 3.707us | ~1.79 |
| `ISO_IR 149` | WINDOWS_949 (EUC-KR-shaped) | 3.496us | 3.437us | 3.416us | ~1.46 |
| `GB18030` | GB18030 (1/2/4-byte) | 3.747us | 3.737us | 3.667us | ~1.87 |
| `GBK` | GBK (shares GB18030's decoder) | 3.747us | 3.717us | 3.757us | ~1.87 |
| *reference, from candidate 005's own RESULT.md* | ISO_IR 100 / `SingleByteEncoding`, **before** that candidate's fix | 39.124us for 19800 bytes | - | - | 1.976 |

All 5 land in the same **~1.5-2.0 ns/input-byte** range as ISO_IR 100's own *pre-candidate-005*
cost (1.976 ns/byte) - despite each of these codecs decoding roughly 2 input bytes into 1 output
character (half as many `String::push` calls per input byte as a true single-byte codec, which
would naively predict a *lower* per-input-byte cost, not an equal one). Encode side (not tabulated
in full, see raw numbers in the harness run log) shows the same shape: 1.6-2.2 ns/byte, comparable
to ISO_IR 100's own pre-fix 1.539 ns/byte.

`perf record -g` on `text-decode-cs`'s inner `decode()` call, 400,000 iterations each (self-time
percentages include the benchmark harness's own per-iteration `fnv1a` checksum call, which runs
outside the timed `Instant` window but still executes on-CPU between iterations and so shows up in
`perf`'s system-wide sampling - only the *decode-specific* symbols below are meaningful, not their
percentage of total runtime):

| Charset | decode-specific self-time breakdown (as % of decode-only samples, `fnv1a` excluded) |
|---|---|
| `GB18030`/`GBK` (`gb18030::raw_feed`) | ~72% in `write_char`/`String::push` (UTF-8 encode + capacity-growth bookkeeping), ~18% in `raw_feed`'s own state machine, of which only ~12% is the actual `map_two_bytes`/`forward` table lookup |
| `ISO_IR 149` (`windows949`) | same shape (not fully re-broken-down; `write_char` dominates `raw_feed`'s self time in the same proportion as `GB18030`) |
| `ISO_IR 87` (`iso2022jp::raw_feed`) | ~74% `write_char`/`push`, remainder in the escape-sequence/shift-state dispatch |
| `ISO_IR 13` (`windows31j::raw_feed`) | more evenly split: ~40% `write_char`/`push`, ~48% `raw_feed`'s own dispatch (of which ~39% is the real `map_two_0208_bytes`/`forward` two-level JIS X 0208 table lookup - a bigger legitimate-work share than the other 4 codecs, since Shift-JIS's index needs two chained lookups, not one) |

## Result

**Confirmed: the same class of hot spot candidate 005 found and fixed for single-byte codecs is
real and present in all 5 remaining multi-byte codecs.** Per-input-byte decode/encode cost is the
same order of magnitude as ISO_IR 100's own *unfixed* cost (not fixed-and-faster), and for 4 of
the 5 codecs (`GB18030`, `GBK`, `ISO_IR 149`, `ISO_IR 87`), `perf` attributes roughly 70-75% of
decode-specific self-time to the same `String`-as-`StringWriter`'s `write_char`/`push`
(per-character UTF-8-length/capacity-growth bookkeeping) mechanism candidate 005's bulk rewrite
bypasses entirely - not to the codec's own actual byte-to-codepoint mapping logic, which is a
small minority (5-12%) of the cost for those 4. `ISO_IR 13` (Shift-JIS) is the one exception with
a materially different balance (~40%/48%): its two-level JIS X 0208 index lookup is real,
nontrivial work, not just dispatch overhead, so a bulk rewrite for this one codec specifically
would need to keep that lookup rather than assume it's free like the others.

This is a **profiling/measurement result only - no code change to `encoding/src/text.rs` or any
other project source was made or measured.** No `patch.diff` for this candidate, by design (see
"Does it change behavior/output?" below) - only the harness (`bench_pipeline/src/main.rs`,
untracked by the main project's own git, per this workspace's own convention) was extended, and
that extension has been folded into the canonical harness copy at
`___OPTIMIZATIONS___/harness/bench_pipeline/` for reuse by whichever future candidate implements
the actual fix.

## Does it change behavior/output?

**N/A - measured/quantified only, no patch applied to real source.** This candidate is a pure
investigation: it confirms the premise ("does the same hot spot exist here") before committing to
the state-machine implementation the candidate description itself flagged as the larger,
not-yet-attempted follow-up work. No `patch.diff` exists for `encoding/` (or any other crate) and
none should be fabricated - see `SKILL.md`'s "patch.diff is mandatory" section, which explicitly
carves out "a pure measurement/investigation" as a case where the absence of a patch is correct,
not incomplete.
- Project test suite: **N/A - no patch applied, test suite not run for this candidate** (per
  `SKILL.md`'s "Correctness" section: the before/after test-suite requirement is explicitly waived
  "for a pure investigation with no patch").

## Why the result looks the way it does

The `encoding` crate implements every one of these 5 codecs' decoders as a `stateful_decoder!`
macro-generated state machine (`encoding/src/codec/{japanese,korean,simpchinese}.rs` in the
vendored `encoding-0.2.33` crate) that still funnels every decoded character through the same
generic `StringWriter` trait `String::write_char` implementation candidate 005 found and bypassed
for single-byte codecs - each decoded character goes through `char`-boxing, UTF-8-length
computation (`len_utf8`), and a capacity-growth check (`reserve`/`needs_to_grow`) individually,
regardless of whether the codec itself needed 1, 2, or 4 input bytes to produce that character.
The per-input-byte cost lands in the same range as ISO_IR 100's own *unfixed* cost specifically
*because* half as many produced characters (2-byte-per-char codecs) roughly cancels out against
each character's own per-*character* overhead being fixed regardless of how many input bytes fed
it - the mechanism (not just the magnitude) matches candidate 005's own finding exactly. `GB18030`
and `GBK` are confirmed (by reading `simpchinese.rs`) to share one decoder (`GB18030Decoder`) -
the `initial_gbk_flag` distinction between them only affects the *encoder's* fallback behavior for
codepoints outside the legacy GBK range, not decode - so a bulk decode fix for one covers both.

## What's not done / open questions

- **No implementation was attempted this candidate**, by design - see "Result" above. This
  profiling step's job was to confirm the premise, not build the fix.
- **The 5 codecs are not equally tractable as a single follow-up candidate**, discovered by
  reading `encoding-0.2.33`'s source while investigating the profile, worth recording so a future
  session doesn't re-derive it:
  - `ISO_IR 13` (WINDOWS_31J/Shift-JIS-shaped) and `ISO_IR 149` (WINDOWS_949/EUC-KR-shaped) are
    structurally the same shape as candidate 005's single-byte codecs, just with one extra
    lead-byte branch before a single table lookup (`map_two_0208_bytes`/`map_two_bytes`) - the
    most direct extension of candidate 005's existing mechanism.
  - `GB18030`/`GBK` share one decoder needing a 4-state (not 2-state) machine (1/2/4-byte
    sequences, two separate lookup tables - `gb18030::forward` and `gb18030_ranges::forward`) -
    still a deterministic, non-shift-based FSM, but with more states than the DBCS pair above.
    Their *encoders* genuinely differ from each other (`initial_gbk_flag`-gated fallback), so a
    bulk encode fix would need two variants even though bulk decode needs only one.
  - `ISO_IR 87` (ISO_2022_JP) is qualitatively different from all 4 of the above: it is not
    lead-byte-driven at all, but escape-sequence-driven (`ESC $ B` / `ESC ( B` etc. shift between
    ASCII/JIS-Roman/JIS X 0208 sub-modes) - a genuinely different, more complex state machine, not
    a variation on the same DBCS pattern. This is the one codec here that most closely matches the
    original candidate description's phrase "shift sequences", and the one most likely to need
    its own dedicated candidate rather than being bundled with the other four.
  - Given this, a future follow-up implementing "the larger state-machine implementation" is more
    honestly scoped as *up to three* separate candidates (DBCS pair; GB18030/GBK pair; ISO_2022_JP
    alone) than one, if this workspace's "one fix per candidate" discipline is to be kept - bundling
    all 5 into one patch would mix at least three distinct, independently-reviewable mechanisms.
  - `ISO_IR 13`'s own profile (~48% real dispatch/table-lookup work, not just `write_char`
    overhead) suggests its *relative* win from a bulk rewrite would likely be smaller than the
    other 4's (candidate 005 found bulk rewrites help most when the removed overhead - not the
    codec's own real work - dominates); not measured directly, since no implementation exists yet.
  - No real DICOM fixture with one of these `SpecificCharacterSet` values was found in
    `dicom-test-files`' already-known file list (only checked the names already referenced
    elsewhere in this workspace, e.g. `object/tests/integration_test.rs`,
    `pixeldata/tests/image_reader.rs` - no exhaustive search of the upstream data set was done) -
    an end-to-end `parse`/`roundtrip`/`write-only` measurement (the kind candidate 005 itself found
    necessary to catch its own `write-only`-specific regression) would need either finding such a
    fixture or extending `gen-seq` to emit non-ASCII multi-byte content under a declared
    `SpecificCharacterSet`, neither of which was attempted this candidate - the isolated
    microbenchmark and `perf` profile above establish the *premise* (the hot spot exists) but not
    the *end-to-end magnitude*, which candidate 005's own history shows can differ sharply by
    fixture and by mode (`write-only` vs `roundtrip` vs `parse`).
