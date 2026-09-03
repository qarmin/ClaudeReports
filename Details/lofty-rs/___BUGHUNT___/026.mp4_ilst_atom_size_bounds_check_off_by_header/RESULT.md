# mp4_ilst_atom_size_bounds_check_off_by_header

`IDEAS.md` asked for a real corpus-driven `fuzz-bug-repro`-style campaign (not hand-picked/
boundary-value construction) against `lofty/src/mp4/ilst/write.rs`'s write path, since candidates
015/023/025 had each only used manual tracing or boundary-value construction there. This session
wrote a new one-off `cargo fuzz` harness exercising that write path end-to-end (real
`Mp4File::read_from` -> mutate `ilst` -> `save_to` -> re-`read_from`), seeded it with the repo's
own real `.m4a`/`.mp4` test fixtures, and ran it for real.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| New `cargo fuzz` harness `mp4_ilst_write_roundtrip` (temporarily added to `fuzz/fuzz_targets/`, removed again after this session), corpus-driven, persistent seed corpus of 23 real `.m4a`/`.mp4` fixtures copied from `lofty/tests/**` | `cargo +nightly fuzz run --sanitizer address`, `RLIMIT_NOFILE` lowered to `65536` first (candidate 024's ASAN-hang workaround), `-max_total_time=300` | Crash found at 85,599 executions (~12s in): `attempt to add with overflow`-free panic - `range end index 3377 out of range for slice of length 3369` inside `Vec::splice`/`Vec::drain`, called from `mp4/ilst/write.rs:367` (`save_to_existing`) |
| Standalone (non-fuzz) reproduction of the saved crash artifact, using the project's own public API (`Mp4File::read_from`/`set_ilst`/`save_to`) | Debug build, 3 repeat runs | Deterministic panic, identical message, 3/3 runs |
| Manual root-cause tracing with temporary `eprintln!` instrumentation (reverted before this candidate was written up) through `write_to`/`save_to_existing`/`AtomInfo::read_inner` | Debug build against the exact crashing input | Traced to `mp4/atom_info.rs`'s `AtomInfo::read_inner` bounds check being off by `ATOM_HEADER_LEN` (8 bytes) - see Root cause below |
| Same fuzz harness re-run for 120s after applying the fix, same seed corpus | `cargo +nightly fuzz run --sanitizer address`, `-max_total_time=120` | Fix confirmed for this bug class (no repeat of this crash); found a second, distinct crash (`attempt to add with overflow` at `write.rs:491`, in `update_offsets`'s `co64` handling) - **not investigated further this session**, added to `IDEAS.md` as a new backlog item instead, since this candidate covers one bug per the project's own convention |

## Status

**Confirmed**

Reproduces deterministically 3/3 runs (debug build, standalone `repro/` against the project's
public API) and 3/3 runs again via the fully standalone `just test` clone-based reproduction
(fresh `git clone` of the real repo at `base_commit`, unmodified working tree).

**Fully reproducible: Yes.** This is a single-participant finding (the library's own write path
mishandling a file it itself already accepted at read time) - no second participant/peer is
involved, so section 11's "real second participant" bar doesn't apply here; `repro/` runs the
real, public `Mp4File::read_from` -> `set_ilst` -> `save_to` entry point end-to-end against a real
crafted file, exactly as any real caller adding/editing MP4 metadata would.

## Class

crash

## Root cause

`lofty/src/mp4/atom_info.rs`'s `AtomInfo::read_inner` validates a newly-read atom's declared size
against the caller-supplied `reader_size` budget (the number of bytes the caller believes remain,
starting at and including this atom's own header) with:

```rust
// `len` includes itself and the identifier
if (len - ATOM_HEADER_LEN) > reader_size {
```

`len` is the atom's full declared size (header + content). Comparing only its *content-only*
portion (`len - ATOM_HEADER_LEN`, i.e. minus the 8-byte header already consumed from the reader)
against the *header-inclusive* `reader_size` budget is off by exactly `ATOM_HEADER_LEN` (8 bytes)
on the two sides of the inequality - it should be `len > reader_size` (both header-inclusive), or
equivalently `(len - ATOM_HEADER_LEN) > (reader_size - ATOM_HEADER_LEN)`. As written, any atom
(starting at the very top level - `moov` itself, in this crash) can declare a total size up to 8
bytes larger than what's truly physically present in the file/buffer without tripping
`SizeMismatchError`.

In the crashing input, the top-level `moov` atom (`moov_start = 32`, declared `moov_len = 3345`,
not extended) is accepted even though `32 + 3345 = 3377` is 8 bytes past the file's real length
(3369 bytes) - exactly the `ATOM_HEADER_LEN` slack this check leaks. `Mp4File::read_from` still
succeeds (the read path only ever uses these lengths as best-effort skip/consume budgets, which
degrade gracefully via `saturating_sub`/`Option` handling), so a real caller can read this file
without any error or warning surfacing as a hard failure.

The corruption becomes fatal specifically in `mp4/ilst/write.rs::save_to_existing`'s "insert a new
`ilst` atom, since `meta` has none" path (`tree.is_empty()` branch): it computes the insertion
offset as `meta_end = meta.start + meta.len` and splices the new `ilst` bytes in at that raw
offset with no check that it's within the actual write buffer. Since `meta`'s own bounds were
derived (through `udta`, itself derived from `moov`) from the already-8-bytes-too-generous `moov`
extent, `meta_end` (3377) ends up 8 bytes past the real buffer length (3369), and
`AtomWriterCompanion::splice` -> `Vec::drain(range)` panics with "range end index 3377 out of
range for slice of length 3369".

This is a different, more upstream root cause than any of candidates 015/023/025's findings (all
of which were in `mp4/ilst/write.rs`/`mp4/read/mod.rs`'s `atom_tree` bookkeeping itself, not in the
shared atom-size validation both of those functions rely on) - a corpus-driven campaign starting
from real files was needed to reach it, since it requires a *specific, self-consistent* 8-byte
size lie that a hand-picked boundary sweep of a single field wouldn't obviously think to construct
across a whole nested atom chain.

## Introduced by

15c1aed9 "MP4: Allow invalid atom sizes with `ParsingMode::Relaxed`" (2024-07-22) - `git show` confirms this commit is where the `(len - ATOM_HEADER_LEN) > reader_size` comparison was last touched (adding the `ParsingMode::Relaxed` branch around the existing check); the off-by-`ATOM_HEADER_LEN` shape of the comparison itself predates this commit (present since at least 39ec33c9 "MP4: Make heavier use of named constants", 2023-03-13, per `git log -S`), so this is the most recent commit that touched the check without fixing the asymmetry, not necessarily the one that first introduced it.

## The fix

Changed the check to compare both sides on the same (header-inclusive) basis:

```rust
if len > reader_size {
```

This is sufficient because `reader_size` is always the number of bytes physically/logically
remaining starting at (and including) the current atom's own header at every call site
(`ContextualAtom::read`, `find_child_atom`, `atom_tree`, and the top-level `AtomWriter::new_from_file`
loop all decrement their running budget by the *full* `atom.len`, header included, confirming this
is the intended semantics `reader_size` was always meant to have).

- Project test suite (`cargo test -p lofty --lib --tests -- --test-threads=1`, chosen to avoid an
  unrelated pre-existing parallel-test flakiness in `probe::tests::probe_{aac,vorbis,wav}` that
  reproduces on this checkout with the default multi-threaded runner regardless of this patch):
  **646 passed, 0 failed, 73 ignored -> 646 passed, 0 failed, 73 ignored (no change, no new
  failures)**, both before and after the fix.
- `repro/` (the exact crashing input): panics before the fix, gracefully returns
  `FileParseError { ty: Some(Mp4), .. }` from `read_from` after the fix (3/3 runs each way, plus a
  clean `just test` run from a fresh standalone clone).

## What's not done / open questions

- A second, distinct crash (`attempt to add with overflow` at `mp4/ilst/write.rs:491`, in
  `update_offsets`'s `co64`/64-bit-offset handling) turned up in the post-fix validation burst.
  Not investigated as part of this candidate (one bug per candidate, per the project's own
  convention) - added to `IDEAS.md`'s backlog for a follow-up candidate.
- This fix only addresses `atom_info.rs`'s bounds check. It was not additionally checked whether
  `mp4/ilst/write.rs`'s own `find_child_atom` calls (lines ~80-86 for `udta` within `moov`, and
  ~109-115 for `meta` within `udta`) have an *independent* off-by-`ATOM_HEADER_LEN` of their own
  (they pass the parent atom's full declared size, e.g. `moov_len`, as the child-search budget,
  rather than `moov_len - moov.info.header_size()`, even though the reader has already been
  seeked past the parent's own header at that point) - given the fix above already closes the one
  concretely demonstrated crash, and the corpus-driven campaign found no further crash of this
  specific class after the fix (in the 120s it ran), this was left unconfirmed either way rather
  than fixed speculatively. Worth a maintainer's look, or a longer follow-up fuzzing session.
- Not checked whether this same `atom_info.rs` off-by-8 is independently reachable (crash or
  silent data corruption, not just a benign accepted-with-slack read) via any of the *read-only*
  entry points (`Mp4File::read_from` itself, without ever writing) - this session only observed it
  surface as a crash through the write path's `meta_end` arithmetic.
- The 120s post-fix fuzz burst is short by this project's own standards (candidate 013's targeted
  JPEG campaign ran 300s/126.8M executions); a longer follow-up campaign against the same harness
  (kept in `___BUGHUNT___/_reference/026_corpus/` as 23 seed files, not committed to the project's
  own `fuzz/`) would very likely turn up more, starting with the `update_offsets` overflow noted
  above.
