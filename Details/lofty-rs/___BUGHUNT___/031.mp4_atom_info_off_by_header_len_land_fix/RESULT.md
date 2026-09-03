# mp4_atom_info_off_by_header_len_land_fix

`IDEAS.md` (2026-08-26 entry, noticed while working candidate `030`) asked to land candidate
`026`'s already-verified `lofty/src/mp4/atom_info.rs` fix (`(len - ATOM_HEADER_LEN) > reader_size`
-> `len > reader_size`) as a real candidate: both `026`'s and `030`'s sessions applied it only
temporarily during their own investigations and reverted it afterward, so it was never actually
landed as a `patch.diff` of its own. This session formalizes that fix into its own candidate
folder, re-confirms the bug fresh against the current mainline, and additionally verifies the
scope claim from `IDEAS.md`: that this bug independently provides its own slack at every
`AtomInfo::read` call site, not just the one crash `026` demonstrated.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Rebuilt candidate `026`'s exact crash reproduction (`crash.bin`, a real corpus-fuzzed `.m4a` whose top-level `moov` atom declares a size 8 bytes past the file's real length) against the current mainline (`5c6565e3`, unpatched) | Debug build, standalone `repro/` using the project's real public `Mp4File::read_from` -> `set_ilst` -> `save_to` API | Still panics identically: `range end index 3377 out of range for slice of length 3369` - confirms the bug is still present, unpatched, on today's mainline |
| Boundary-value construction: mutated a real fixture (`lofty/tests/taglib/data/has-tags.m4a`) by incrementing only the on-disk `mdia` size field by `ATOM_HEADER_LEN` (8), leaving every other byte (including the real `udta`/`meta`/`ilst` tag) untouched, and read it via the real public API | Debug build, temporary standalone program, real fixture | On fully unpatched mainline: tag silently lost (`read_from` returns `Ok`, `ilst()` is `None`) - reproduces candidate `030`'s own finding fresh |
| Isolating candidate `026`'s bug from candidate `030`'s separate `moov.rs` bug: temporarily applied `030`'s already-verified fix (`atom.len - 8` at `moov.rs` line 54, NOT part of this candidate's `patch.diff`, reverted afterward) on top of the still-unpatched `atom_info.rs`, then re-ran the same `+8` mutated fixture | Debug build, same standalone program, `atom_info.rs` left unpatched | Tag still silently lost - confirms candidate `026`'s bug alone (with candidate `030`'s bug fixed) is independently sufficient to reproduce the same silent-tag-loss symptom, not merely a restatement of `030`'s finding |

Three approaches, all producing a real signal (no clean negatives), so the soft cap was not a
factor - this candidate's job was landing an already-confirmed fix, not discovering a new one, so
the "dig" here is reconfirmation plus scope-verification rather than open-ended searching.

## Status

**Confirmed**

Reproduces deterministically: candidate `026`'s original crash reproduces 3/3 runs on unpatched
mainline (matching its own prior session) and is gone 3/3 runs after the fix (debug, standalone
`repro/`), plus again via the fully standalone `just test` clone-based reproduction (fresh `git
clone` of the real repo at `base_commit`, unmodified working tree, `--release`).

**Fully reproducible: Yes.** This is a single-participant finding (the library's own read/write
path mishandling a file it is handed) - no second participant/peer is involved, so section 11's
"real second participant" bar doesn't apply. `repro/`'s primary binary (`repro031`) runs the real,
public `Mp4File::read_from` -> `set_ilst` -> `save_to` entry point end-to-end against a real,
corpus-fuzzed crashing file, exactly as any real caller adding/editing MP4 metadata would. The
supplementary binary (`mdia_slack_scope`) runs the real public `Mp4File::read_from` entry point
end-to-end against a real fixture with one field mutated, everything else byte-for-byte untouched.

## Class

crash

## Root cause

Identical to candidate `026`'s already-documented root cause (unchanged, still present verbatim on
today's mainline): `lofty/src/mp4/atom_info.rs`'s `AtomInfo::read_inner` validates a newly-read
atom's declared size against the caller-supplied `reader_size` budget (the number of bytes the
caller believes remain, starting at and including this atom's own header) with:

```rust
// `len` includes itself and the identifier
if (len - ATOM_HEADER_LEN) > reader_size {
```

`len` is the atom's full declared size (header + content). Comparing only its *content-only*
portion (`len - ATOM_HEADER_LEN`, 8 bytes) against the *header-inclusive* `reader_size` budget is
off by exactly `ATOM_HEADER_LEN` on the two sides of the inequality, so any atom read via
`AtomInfo::read`/`AtomInfo::read_inner` - not just the one top-level `moov` case `026`'s crash
happened to hit - can declare a size up to 8 bytes larger than what's truly available and still be
accepted.

This session additionally re-verified the scope this implies, as `IDEAS.md` asked: because every
call site that budgets a child atom via `AtomInfo::read` shares this one check, the bug's 8 bytes
of slack is available independently at each of them - including `moov.rs`'s `trak` -> `mdia`
lookup (candidate `030`'s separate finding), even when `030`'s own, different, upstream bug (an
already-too-generous budget passed *into* that specific `find_child_atom` call) is fixed
separately. The two bugs are independent and additive: on the fully unpatched mainline they stack
(up to 16 bytes of combined slack for that one call site, per `030`'s own investigation); fixing
only this one (`026`'s bug, this candidate) removes exactly its own 8 bytes of contribution
everywhere `AtomInfo::read` is called, including at that call site, without needing to know
`030`'s bug exists or touching `moov.rs` at all.

## Introduced by

Same as sibling candidate 026 (this candidate's own Root cause states it's identical, unchanged, still present verbatim on mainline): 15c1aed9 "MP4: Allow invalid atom sizes with `ParsingMode::Relaxed`" (2024-07-22), see candidate 026's note for the same caveat about the check's asymmetry possibly predating this commit.

## The fix

Identical to candidate `026`'s already-verified fix, now landed as this candidate's `patch.diff`
against real source (previously only ever applied temporarily and reverted in both `026`'s and
`030`'s own sessions):

```rust
if len > reader_size {
```

Both sides now compare on the same (header-inclusive) basis. This is sufficient because
`reader_size` is always the number of bytes physically/logically remaining starting at (and
including) the current atom's own header at every call site (`ContextualAtom::read`,
`find_child_atom`, `atom_tree`, and the top-level `AtomWriter::new_from_file` loop all decrement
their running budget by the *full* `atom.len`, header included).

Verified against more than the one triggering input:

- Candidate `026`'s exact crashing input (`crash.bin`): panics before the fix, gracefully returns
  `FileParseError { ty: Some(Mp4), .. }` from `read_from` after the fix (3/3 runs each way, plus a
  clean `just test` run from a fresh standalone clone).
- The boundary-value `+16` mutation of `has-tags.m4a`'s `mdia` size field (exceeding what
  candidate `030`'s separate, still-*unpatched* `moov.rs` bug can provide on its own): silently
  loses the tag on the fully unpatched tree, but is correctly rejected (`Err`) once only this
  candidate's fix is applied - `moov.rs`'s own bug (not touched by this patch) can no longer
  contribute the extra headroom this fix used to leak on top of it.
- The boundary-value `+8` mutation of the same field: **still silently loses the tag even after
  this candidate's fix is applied**, because candidate `030`'s separate, still-unpatched `moov.rs`
  bug independently provides its own 8 bytes of slack at that specific call site, unrelated to
  this fix. This is the expected, documented scope limit of this fix - see "What's not done"
  below. `repro/`'s supplementary `mdia_slack_scope` binary demonstrates and asserts exactly this
  (see `output.txt`).
- Project test suite (`cargo test -p lofty --lib --tests -- --test-threads=1`, matching candidates
  `026`/`030`'s choice to avoid this checkout's pre-existing unrelated parallel-test flakiness):
  **646 passed, 0 failed, 73 ignored -> 646 passed, 0 failed, 73 ignored (no change, no new
  failures)**, both before and after the fix.

## What's not done / open questions

- This fix alone does **not** close candidate `030`'s own finding (`moov.rs`'s `trak` -> `mdia`
  `find_child_atom` call passing a header-inclusive budget) - that remains a separate, still-open,
  still-unpatched bug with its own already-verified fix, not landed by this candidate. A caller
  relying only on this candidate's fix should still expect the `+8`-scale silent-tag-loss symptom
  to be reachable via `030`'s bug alone until that one is also landed for real.
- Not re-checked whether this fix closes or narrows any instance of the bug class beyond the two
  already-documented call sites (`026`'s top-level `moov` crash, `030`'s `trak`/`mdia` lookup) -
  e.g. `properties.rs:489`'s `stbl`-within-`minf` `find_child_atom` call, flagged in `IDEAS.md` as
  sharing the same `find_child_atom` definition but not examined by either `026`, `029`, or `030`.
  This candidate's fix does remove this specific bug's own 8-byte contribution there too (it's the
  same shared check), but whether that call site has its own separate upstream budget-inflation
  bug analogous to `030`'s was not investigated this session.
- Candidate `026`'s own follow-up finding (a second, distinct crash - `update_offsets`'s `co64`
  `i64` overflow) is candidate `027`, already its own separate, already-landed-as-a-candidate
  finding; not revisited here.
- A longer fuzzing campaign against this patched tree (as `026`'s own "what's not done" section
  already suggested) was not re-run this session - the scope of this candidate was formalizing the
  already-verified fix, not further discovery.
