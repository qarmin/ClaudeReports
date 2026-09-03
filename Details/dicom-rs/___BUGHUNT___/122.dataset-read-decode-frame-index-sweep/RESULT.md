# dataset-read-decode-frame-index-sweep

Follow-on sweep explicitly requested at the end of candidate 107's "What's not done" section:
"a longer/fuzzing-based sweep of `parser/src/dataset/read.rs` and `parser/src/stateful/decode.rs`
specifically was not attempted [by 107] (out of scope for that session's manual/boundary-value-
focused approach)." This candidate targets exactly that gap: the "count check without a
corresponding index bound" idiom (`if X.len() > 1 { X[frame] } else { X[0] }`-shaped code, no
bound on the actual index variable) that candidates 107/112/113/114 found repeatedly in
`pixeldata/src/lib.rs`, specifically in these two dataset/stateful-decoding files.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code read against the correct base commit (df2b9e42, matching this project's justfile convention) for the specific idiom: grep `parser/src/dataset/read.rs` and `parser/src/stateful/decode.rs` for `.len()` equality/threshold comparisons (`==`/`!=`/`<`/`>`/`<=`/`>=`), then a second sweep for any variable-indexed slice/array access (`\[[a-zA-Z_][a-zA-Z0-9_]*\]`, i.e. `foo[bar]` shaped, as opposed to `foo[..]`/`foo[0u8; N]` literals), plus targeted checks of `.first()`/`.last()`/`.get(0)`/`unwrap()`/`.expect()` call sites | Static code reading, no build needed. (Note: an initial pass was mistakenly run against a stale worktree HEAD, commit 6dc8782e, ~1361 diff lines behind the intended df2b9e42 base - re-done in full against the correct commit once the discrepancy was noticed via `git diff 6dc8782e df2b9e42 -- <files>`.) | Zero occurrences of the `.len()`-check-without-matching-index-bound idiom in either file. Both files' only container-index-adjacent code is: (a) `Vec<u8>`/`SmallVec` value buffers built via `resize`/`resize_with` immediately before being filled via a matching `&mut vec[..]` or `&mut vec[base..]` slice (`base = vec.len()` captured *before* the resize, so the slice bound and the fill length are always in lockstep - `read_u32` at decode.rs:768-779 is the closest-looking candidate and is safe by this construction); (b) a `Vec<SeqToken>` stack (`seq_delimiters`) only ever accessed via `.last()`/`.pop()`, both `Option`-returning and already handled with explicit `None` arms (in `read.rs`, e.g. lines 378-388, 683-713) - no `.unwrap()`/`.expect()`/direct indexing on it anywhere in the current file |
| Cross-checked against the file's own git history to see whether this idiom class ever existed here and was already fixed, which would explain why 107's sweep found nothing today | `git diff 6dc8782e df2b9e42 -- parser/src/dataset/read.rs parser/src/stateful/decode.rs` (full diff read) | Confirms the class *did* exist in this project's history and was independently fixed before df2b9e42: `decode.rs` had `self.signed_pixeldata = Some(vec[0] != 0)` (direct index on a `vec` whose length is a declared-but-attacker-controlled DICOM element length, no bound at all) - now `vec.first().map(|rep| *rep != 0)`. `read.rs` had `self.seq_delimiters.last().expect("item header should be read only inside an existing sequence")` - an assumption-as-panic on an empty stack reachable by a stray item header outside any open sequence - now returns `UnexpectedItemHeaderSnafu` instead of panicking. Both fixes are already present as of df2b9e42, so re-confirming them as new candidate 122 findings would be a duplicate of already-merged work, not a new bug |
| Short fuzz burst reusing the project's own tracked `object/fuzz/fuzz_targets/open_file.rs` target (exercises the real `DataSetReader`/`StatefulDecoder` machinery end-to-end via `dicom_object::OpenFileOptions::from_reader`) | `DEBUGINFOD_URLS=` (empty, per this project's sandbox note) `cargo +nightly fuzz run open_file -- -max_total_time=150`, empty corpus, release+debuginfo build; then a second round seeded with the 13 real-world `.dcm` files from this repo's own `dcms/` reference corpus (`-max_total_time=100`) for deeper coverage into sequence/item/pixel-fragment code paths | Empty-corpus round: 15,943,512 executions in 151s, zero crashes (expected low depth without a seed corpus - stated explicitly per `CONFIRMATION.md` section 10). Seeded round: found one crash almost immediately, but it is the assert-line panic (`open_file.rs:33`, `assert_eq!(obj, obj2)`) that is a harness-inherent limitation already documented in this project's own `~/.claude/skills/bug-hunt/projects/dicom-rs.md` "2026-08-29 session" note (`decode_text_trap`'s lossy-substitution character can never round-trip through a strict re-encode, independent of which placeholder is used) - not a panic inside `read.rs`/`decode.rs`, and not a new finding. No panic was observed at any point inside either target file across both rounds |

## Status

**Not reproducible** (ruled out for the specific idiom in scope). No instance of the "count check
without a corresponding index bound" pattern exists in `parser/src/dataset/read.rs` or
`parser/src/stateful/decode.rs` as of df2b9e42 - covered by a full grep-based structural sweep of
both files (not just a sample) plus ~16M fuzz executions (150s, empty corpus) and a further ~100s
seeded with 13 real-world DICOM files, with zero panics originating in either target file. The one
crash observed during fuzzing is a pre-existing, already-documented, unrelated harness limitation
(the tracked `open_file` fuzz target's own `assert_eq!` invariant, not a bug in the two files under
review).

**Fully reproducible**: N/A (no bug to reproduce - see Status).

## Class

N/A (not Confirmed).

## Root cause

Not applicable - no bug found. The two files' actual design already avoids the shape being hunted
for: every buffer that is resized-then-filled captures its bound (`vec.len()`/`base`) from the same
`resize` call that's about to fill it, so the fill-length and the slice-bound can never drift apart;
the one stack-like container (`seq_delimiters: Vec<SeqToken>`) is only ever read via
`Option`-returning `.last()`/`.pop()`, both of which are explicitly matched with an error-returning
`None` arm rather than unwrapped. The general class this candidate was looking for (a `.len()`
threshold check that doesn't actually bound a *different* index variable used later, the way
`pixeldata/src/lib.rs`'s `if X.len() > 1 { X[frame] } else { X[0] }` idiom does for per-frame
metadata arrays) requires a per-frame/per-index random-access lookup into an array whose length
comes from a source independent of the index being used - these two files are streaming
sequential decoders (they consume the wire byte-by-byte via a `Read` source and either push onto a
growing `Vec`/`SmallVec` or read into a freshly-sized buffer), so this specific access shape simply
doesn't arise here; it's specific to `pixeldata`'s per-frame metadata-array accessors, which take an
externally supplied `frame` argument disconnected from the array's own length.

Two adjacent instances of a related but distinct bug shape (unconditional `.expect()`/direct
indexing without any bound check at all, as opposed to a bound check that's merely insufficient)
did exist in these exact files in this project's own history and are already fixed as of the df2b9e42
base commit used by this bug-hunt session (see "What was tried" row 2) - re-confirming them here
would be a duplicate of already-merged work, not a new candidate.

## Introduced by

N/A - no bug found this session to attribute.

## The fix

N/A - no bug found, no patch produced.

- Project test suite: not run (no change made to the tree; `git status` confirms no tracked files
  were modified by this session's investigation).

## What's not done / open questions

- This sweep covered exactly the two files named in the task (`parser/src/dataset/read.rs`,
  `parser/src/stateful/decode.rs`) for exactly the named idiom. `parser/src/dataset/lazy_read.rs`
  (the third, independent dataset-reading implementation referenced repeatedly in
  `~/.claude/skills/bug-hunt/projects/dicom-rs.md`'s risk map, e.g. candidates 006/007/049/050/051)
  was not swept this session - a future session doing the same idiom-specific sweep should include
  it for completeness, since this project's risk map explicitly notes that fixes/bugs in
  `read.rs` and `lazy_read.rs` have repeatedly paired up historically.
- The fuzz burst was short (150s empty-corpus + 100s seeded, ~16M total execs) and single-target
  (`open_file`, which exercises `dicom-object`'s full parse path including `read.rs`/`decode.rs`
  but not either file in complete isolation). It corroborates the manual sweep but is not proof of
  absence - a dedicated harness feeding raw bytes directly into `DataSetReader`/`StatefulDecoder`
  (bypassing `dicom-object`'s wrapping/validation) combined with a longer `fuzz-bug-repro` campaign
  (run separately, per this skill's convention) would cover more ground, particularly around
  encapsulated-pixel-data/offset-table paths that need a specific transfer syntax and structural
  shape to reach and that this seed corpus's 13 files may not fully represent.
- Whether the same idiom recurs in `object/src/mem.rs` beyond the already-confirmed-safe
  `apply`/`apply_leaf` (per the prior session's manual grep sweep this candidate's task description
  references) was not re-checked here - out of scope for this candidate, which was scoped
  specifically to `parser/src/dataset/read.rs` and `parser/src/stateful/decode.rs`.
