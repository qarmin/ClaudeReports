# mp4_ilst_duplicate_sibling_last_match_lost_write

Follow-up to candidate `023`'s open question: `023` fixed `atom_tree`'s "never matched" vs.
"matched at index 0" collision for the *no real `ilst` present* shape, but explicitly left the
*other* multi-`ilst`-sibling shape unchecked - more than one real `ilst` atom present as siblings
under a real `meta` atom. `atom_tree`'s loop resolves `found_idx` to the **last** child matching
fourcc `"ilst"` (`found_idx = i` is overwritten on every match, unconditionally), while the read
path (`lofty/src/mp4/moov.rs`) instead breaks on the **first** child matching `"ilst"`. This
mismatch was investigated by hand-constructing that exact shape (boundary-value construction, same
approach `023` used, no fuzzing needed).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code reading: compared `atom_tree`'s (`lofty/src/mp4/read/mod.rs`) "last match wins" selection against `moov.rs`'s meta-parsing loop's "first match wins, break immediately" selection | N/A (static reasoning) | Confirmed a real mismatch: whichever `ilst` sibling `save_to_existing` treats as "the" existing tag to overwrite is not necessarily the one any subsequent read of the file will ever see |
| Boundary-value construction: a real `udta > meta` (full, version/flags = `0`) with a real `hdlr` child followed by **two** real, distinct `ilst` atoms as siblings (`ilst` bytes for each produced by the crate's own writer, not hand-encoded, then spliced together) - real `Mp4File::read_from`/`set_ilst`/`save_to` public API, debug and release builds | Confirmed | `set_ilst("NEW")` + `save_to` reports success and does modify the file on disk, but re-reading the same file afterward (a fresh `Mp4File::read_from` call, simulating close+reopen) still returns the stale, pre-edit title from the *first* `ilst` sibling - the edit silently never becomes visible again; reproduced deterministically 3/3 debug + 6/6 release (`overflow-checks = true`) runs, both in the main project's working tree and in the standalone `repro/` against a pristine clone |
| Repeat-run confirmation via the standalone `repro/` against a pristine clone at `base_commit`, release profile with `overflow-checks = true` | 3 consecutive `cargo run --release` invocations, unpatched clone | 3/3 "REPRODUCED" (exit code 1) |
| Same repro, patched clone (`patch.diff` applied) | 3 consecutive `cargo run --release` invocations | 3/3 "did not reproduce" (exit code 0) |

## Status

**Confirmed.**

**Reproduction rate: 3/3 runs** (debug build against the main project's working tree), **3/3
release runs** (`overflow-checks = true`, same working tree), **and a further 3/3 runs** (release
build, `overflow-checks = true`, standalone `repro/` against a pristine clone at `base_commit`) -
fully deterministic, no randomness/fixed-seed dependency involved.

**Fully reproducible: Yes.** `repro/` calls the real public entry points a genuine caller uses to
open and re-tag an MP4/M4A file (`Mp4File::read_from` -> `set_ilst` -> `save_to` -> a fresh
`Mp4File::read_from` on the resulting bytes, simulating close+reopen) on a byte-for-byte real,
spec-shaped file built once in code, whose two `ilst` atoms are themselves real bytes produced by
the crate's own writer (not hand-encoded iTunes atom-data payloads) - no internal/private lofty API
and no hand-constructed `AtomInfo`/`ContextualAtom`/`atom_tree` state is fed directly to the
suspect function. This is the "malformed/adversarial file handed to a parser's real public entry
point" case from `CONFIRMATION.md` section 11 (a crafted MP4 file with duplicate `ilst` siblings on
disk **is** the genuine external input a real caller/attacker would supply); there is no second
participant to fake here, so no downgrade applies.

## Class

`logic` (a tag edit through the real public write API silently never becomes visible again on the
very next read - no crash/panic observed in either debug or `overflow-checks` release builds).

## Root cause

`lofty/src/mp4/ilst/write.rs::save_to_existing` locates the existing `ilst` atom (if any) among
`meta`'s children via `atom_tree` (`lofty/src/mp4/read/mod.rs`):

```rust
if let AtomIdent::Fourcc(ref fourcc) = atom.ident {
    i += 1;

    if fourcc == up_to {
        found_idx = i;
    }

    buf.push(atom);
}
```

Every child whose fourcc equals `up_to` (`"ilst"`) overwrites `found_idx`, so when `meta` has more
than one real `ilst` sibling, `found_idx` ends up pointing at the **last** one. `save_to_existing`
then treats `tree[ilst_idx]` (the last `ilst`) as "the" existing tag and overwrites/replaces
exactly that atom's byte range, leaving every earlier `ilst` sibling completely untouched on disk.

The read path disagrees about which sibling is authoritative. `lofty/src/mp4/moov.rs`'s
meta-parsing loop:

```rust
if atom.ident == AtomIdent::Fourcc(*b"ilst") {
    found_ilst = true;
    ilst_atom_size = atom.len;
    break;
}
```

breaks immediately on the **first** `"ilst"` child it finds and never looks further - so any
subsequent `Mp4File::read_from` call on the very same file always returns the *first* `ilst`
sibling's content, never the last one.

Net effect: on a file with duplicate `ilst` siblings, `save_to_existing` always writes to the
sibling that the read path will never look at again. The write itself succeeds without error and
does change the file's bytes on disk (so it is not silently a no-op from a "did the file change"
point of view, and the resulting file remains structurally well-formed), but the specific edit the
caller made (e.g. changing the title) is unobservable through the crate's own API on the very next
read - a caller who edits a tag, saves, and reopens the file to confirm the change would see their
old, stale tag data every time, indistinguishable from the edit having silently failed.

This is a distinct bug from candidate `023`'s (which was about the "no real `ilst` at all" shape
colliding with "matched at index 0"): here a real `ilst` atom is always what's found, matching
`023`'s original lower-suspicion assessment that this shape "wasn't ruled safe by construction" -
and in fact it was not safe.

## The fix

Change the match-selection rule in `atom_tree` to keep the **first** match instead of the last,
matching `moov.rs`'s own "first `ilst` wins" read-path semantics:

```rust
if fourcc == up_to && found_idx == 0 {
    found_idx = i;
}
```

This is a minimal, self-contained change to the existing `usize`-sentinel scheme already in this
file (candidate `023`'s separate `Option<usize>` fix for the "never matched" ambiguity is not
applied in this working tree and is orthogonal to this change - both could coexist, but this
candidate's fix stands alone against the current, unpatched source). With this change, `atom_tree`
returns the index of the *first* `ilst` sibling, `save_to_existing` overwrites that one, and a
subsequent read (which also picks the first sibling) now sees the caller's edit.

Verified: re-running `repro/` against the patched clone no longer reproduces (3/3 runs, see
`output.txt`) - `set_ilst("NEW")` + `save_to` followed by a fresh `Mp4File::read_from` now returns
`"NEW"` instead of the stale `"FIRST"`.

- Project test suite (`lofty` crate), run identically before and after the fix (main project
  working tree, not the standalone clone):
  - `cargo test -p lofty --lib`: **256 passed; 0 failed** (before and after, identical)
  - `cargo test -p lofty --test taglib`: **191 passed; 0 failed; 73 ignored** (before and after,
    identical)
  - No existing test covers a real `meta` atom with more than one `ilst` child, which is why this
    shipped unnoticed.

## What's not done / open questions

- This fix only changes *which* `ilst` sibling is treated as authoritative for writing; it does
  not deduplicate the file (the stale, now-orphaned second `ilst` sibling, or any additional extra
  siblings beyond the first, are left in place on disk after a write, still real but unreachable
  bytes inside `meta`, growing `meta`'s size unnecessarily). Whether the crate should instead strip
  every non-first `ilst` sibling on write is a design decision left to the project maintainers,
  not attempted here.
- Only the two-sibling shape was constructed by hand; three or more duplicate `ilst` siblings were
  not separately tried, though the same first/last mismatch mechanism applies identically
  regardless of count once more than one exists.
- Not independently re-checked whether `023`'s own fix (applied together with this one) changes
  this analysis - both patches touch the same function but different, non-overlapping lines
  (`023`'s changes the sentinel type from `usize` to `Option<usize>`; this one changes the
  match-selection condition), so they are expected to compose cleanly, but that combination was not
  built and tested together this session.
