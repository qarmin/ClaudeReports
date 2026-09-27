# MKV nested EBML master elements: does the demuxer bound recursion depth?

Follow-up to candidate 001 (ID3v2 CHAP/CTOC unbounded recursion, a real stack-overflow crash),
whose open-questions section explicitly flagged `symphonia-format-mkv`'s own nested master
elements (Chapters/`ChapterAtom`, Tags/`SimpleTag`, Cluster/`BlockGroup`) as the same suspected
bug class and not yet checked. `symphonia-format-mkv/src/segment.rs` recurses natively
(`ChapterAtomElement::read` calls `it.read_master_element::<ChapterAtomElement>()` on its own
child `ChapterAtom` elements, and `SimpleTagElement::read` does the same for nested `SimpleTag`),
which is the same recursive shape that made candidate 001 exploitable.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read code against candidate 001's exact heuristic (recursive `EbmlElement::read` calls, checked for a depth counter) | manual read of `symphonia-format-mkv/src/ebml.rs` and `segment.rs` | Found `EbmlIterator::push_element` unconditionally checks `self.stack.len() >= S::MAX_DEPTH` (schema-wide constant, `MkvSchema::MAX_DEPTH = 16` in `schema.rs`) and returns `Err(EbmlError::MaximumDepthReached)` *before* any recursive `E::read` call runs. Every master-element read in this crate (`ChaptersElement`, `EditionEntryElement`, `ChapterAtomElement`, `TagsElement`, `TagElement`, `SimpleTagElement`, `Cluster`/`BlockGroup`, etc.) goes exclusively through `EbmlIterator::read_master_element`, which calls `push_element()?` first - `grep`ing for every call site of `E::read`/`read_master_element` in the crate (`ebml.rs`, `demuxer.rs`, `segment.rs`) found none that bypass this check. `SimpleTagElement` additionally enforces its own tighter local cap (`if hdr.depth() < 6`) on top of the schema-wide one. |
| Standalone repro calling the real public `MkvReader::try_new` API directly, boundary-value nesting depths (4, 10, 15, 16, 17, 20, 50, 500, 5000, 60000 nested `ChapterAtom` levels) | debug and release builds, 3 repeat runs (release) + 1 run (debug), deterministic (no randomness) | No crash at any depth in either build. Depths that stay within the schema's depth budget (4, 10) parse successfully with the full chapter tree; every depth from 15 upward (i.e. as soon as the `Segment > Chapters > EditionEntry > ChapterAtom...` chain would exceed `MAX_DEPTH = 16`) fails fast with `DecodeError("mkv (ebml): maximum ebml document depth reached")` - the whole file is rejected (parsing does not silently drop the offending subtree and continue, unlike candidate 001's ID3v2 fix), but there is no recursion runaway, no stack growth beyond the depth cap, and file size scales linearly with nesting (a 60,000-level file is ~2.6 MB and parses/rejects in well under a second). |
| Existing `demux_mkv` libFuzzer target (`symphonia/fuzz/fuzz_targets/demux_mkv.rs`), seeded with this candidate's nested-`ChapterAtom` files at several depths around/past the cap | release, `-max_total_time=90`, `-rss_limit_mb=4096`, then re-run with `-timeout=5` | Found no crash related to nesting depth. A single fuzzer-mutated input caused one execution to hang well past the requested time budget (had to be killed externally); every seed file, including all of this candidate's crafted nested-chapter files, individually replays in well under 5s, so the hang is unrelated to nested-element recursion depth. Not chased down further in this candidate (out of scope for the recursion-depth hypothesis) - filed to `IDEAS.md` as a new, separate suspect. |

Note: an early version of the boundary-value test harness itself used *recursive* Rust code to
*build* the crafted 60,000-level input (not to parse it), and that harness code overflowed its
own stack in a debug build - a false alarm in the test tooling, not in `symphonia-format-mkv`.
The harness was rewritten to build the nested structure iteratively before drawing any
conclusion; see `repro/src/main.rs`'s doc comment on `nested_chapter_atom` and `output.txt`.

## Status

**Not reproducible.** `symphonia-format-mkv`'s `EbmlIterator` enforces a schema-wide maximum
nesting depth (16) uniformly for every recursive master-element read in the crate, checked before
each recursive descent, so native Rust call-stack recursion for nested `ChapterAtom`/`SimpleTag`
elements can never exceed ~16 stack frames regardless of how deeply an adversarial file claims to
nest them. Verified up to 60,000 claimed nesting levels (a ~2.6 MB crafted file) with no crash, in
both debug and release builds, 3/3 repeat runs (release) + 1 debug run, all identical.

**Fully reproducible**: N/A (not applicable to a Not-reproducible result - no bug to reproduce).

## Class

N/A (Not reproducible).

## Root cause

Not applicable - the suspected code path turned out to already be safe. The reason: unlike
candidate 001's ID3v2 `CHAP`/`CTOC` frame readers (which had no depth tracking anywhere),
`symphonia-format-mkv`'s EBML layer (`ebml.rs`) was designed with a single, centrally-enforced
depth limit (`EbmlIterator::push_element`, gated on `EbmlSchema::MAX_DEPTH`) that every master
element - recursive (`ChapterAtom`, `SimpleTag`) or not - must pass through via
`read_master_element`. This makes the recursion-depth bug class structurally impossible here by
construction, rather than prevented by an ad hoc check in each recursive reader.

## Introduced by

N/A (Not reproducible - no bug to attribute). For context (not a bug fix, just when the relevant
protection was put in place): `MkvSchema::MAX_DEPTH = 16` and the `push_element` depth check are
both present at the tip of `main` checked in this session (ee35874); not bisected further since
there is no regression to locate.

## The fix

Not applicable - no bug found, no patch produced.

## What's not done / open questions

- The Cluster/`BlockGroup` nesting path (`demuxer.rs`'s `next_element` state machine) is driven
  iteratively by an external loop rather than native recursion, and also passes through
  `push_element`/`read_master_element` for its one level of nesting - not a recursion-depth risk
  by construction, not specifically fuzzed beyond what the existing `demux_mkv` corpus already
  covers.
- A separate, unrelated signal turned up while fuzzing: one fuzzer-mutated input made a single
  `demux_mkv` execution hang well past the fuzzer's own time budget and had to be killed
  externally. Not investigated further here (out of scope for this candidate's recursion-depth
  hypothesis) - added to `IDEAS.md` as a new backlog suspect for a dedicated future candidate
  (possible hang/resource-exhaustion bug somewhere in MKV parsing, cause not yet isolated).
- Only `symphonia-format-mkv` was checked this session, matching this candidate's scope; other
  hierarchical formats in the workspace (besides ID3v2, already covered by candidate 001) were not
  re-examined here.
