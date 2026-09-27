# mold_h_get_fragment_negative_index

`MergeableSection<E>::get_fragment()` (`src/mold.h`) resolves an offset into a mergeable/string-
merge section's fragment table via `upper_bound`. If the offset compares less than the first
recorded fragment's offset, `upper_bound` returns `frag_offsets.begin()`, so the computed index
(`it - 1 - frag_offsets.begin()`) becomes `-1` - a negative index into two `std::vector`s
(`fragments`/`frag_offsets`). Flagged by a prior static-audit pass, not yet built/verified. (This
candidate's own dispatched agent was killed mid-investigation by an unrelated infrastructure
safety-filter error after already producing a complete, working repro generator and run script but
before finishing the build/verify/fix steps - the orchestrator completed the investigation directly
in the surviving worktree rather than losing that work.)

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `MergeableSection<E>::get_fragment()` (`src/mold.h:3449-3455`) and its two callers - `InputSection<E>::get_fragment()` (relocation path, `src/mold.h:3374-3391`) and `ObjectFile<E>::reattach_section_pieces()` (`src/input-files.cc:1025-1046`, the plain symbol-value path, no relocation needed) | static read | Confirmed the exact off-by-one/negative-index mechanism the audit described, and that the simpler trigger is `reattach_section_pieces()`: a symbol defined directly inside a mergeable section with an out-of-range `st_value`, no relocation required at all |
| Crafted x86_64 `ET_REL` object: one `SHF_ALLOC\|SHF_MERGE\|SHF_STRINGS` section (`.rodata.str1.1`) with one real string fragment (`"hello\0"`), a `.symtab` with one `STT_OBJECT` symbol pointing into that section with `st_value = 0xFFFFFFFFFFFFFFFF` (i.e. `-1` once reinterpreted as the signed `i64` `get_fragment()` actually takes) | plain Debug build (`build-plain`, `-DCMAKE_BUILD_TYPE=Debug`, g++ 15, `_GLIBCXX_ASSERTIONS` active by default), `mold -o out.exe evil.o` | **Aborted**: `std::vector<SectionFragment<E>*>::operator[]` assertion failure (`__n < this->size()`), exit 134 (SIGABRT). Reproduced 5/5 identical runs. |
| Confounded-fix check: reverted the patch, rebuilt, re-ran the exact same repro | same plain build, unpatched | Crash returns identically (exit 134, same assertion) - confirms the fix (not something else) is what makes the crash go away |
| Patched build re-verify (after re-applying the fix and rebuilding) | same plain build, patched | No crash: `mold: fatal: <file>: bad symbol value: 18446744073709551615`, exit 1, 3/3 runs |

All runs use the real, unmodified `mold` binary via its actual CLI entry point on a hand-crafted-
but-otherwise-standard ELF relocatable object file - a malformed object file handed to the linker's
real parsing entry point is the genuine external input (CONFIRMATION.md section 11's two-
participant requirement doesn't apply; single-participant finding, same as every other candidate in
this project).

## Status

**Confirmed.** Reproduces deterministically, 5/5 runs on the unpatched plain Debug build (SIGABRT,
exit 134), and the crash was independently confirmed to return after reverting the fix and to stay
gone after re-applying it (confounded-fix check, `SKILL.md` pitfall #6).

**Fully reproducible**: **Yes.** `repro/run.sh`/`repro/gen_evil_elf.py` runs the real `mold` binary
end-to-end (`mold -o <out> <crafted.o>`) against a single, standalone, hand-crafted ELF object
file - a single-participant case (a malformed input file handed to the real linker's real CLI entry
point is the genuine external artifact a real attacker/build system would supply; no second
participant to fake).

## Class

memory-safety (negative/out-of-bounds index into two small `std::vector`-backed containers;
degrades to a raw OOB heap read/wild-pointer propagation in a build without libstdc++'s
`_GLIBCXX_ASSERTIONS` hardening, and to a clear SIGABRT with it - same hardening-dependent behavior
already observed for candidate 016's `get_shndx()` bug).

## Root cause

```cpp
template <typename E>
std::pair<SectionFragment<E> *, i64>
MergeableSection<E>::get_fragment(i64 offset) {
  auto it = ranges::upper_bound(frag_offsets, offset);
  i64 idx = it - 1 - frag_offsets.begin();
  return {fragments[idx], offset - frag_offsets[idx]};
}
```

`offset` is ultimately derived from an untrusted ELF symbol's `st_value` (a `u64` field, reinterpreted
as a signed `i64` by every caller) or a relocation's signed addend added to it. If `offset` compares
less than `frag_offsets[0]` (the first recorded fragment's own offset - normally 0, but the check is
missing regardless), `ranges::upper_bound` returns `frag_offsets.begin()`, making `idx == -1`.
`fragments[-1]` and `frag_offsets[-1]` are then negative-index reads on `std::vector` - undefined
behavior that either aborts (with libstdc++'s `_GLIBCXX_ASSERTIONS` hardening active, as in this
project's default Debug build) or silently returns adjacent heap memory reinterpreted as a
`SectionFragment<E>*`/offset pair. The caller's only guard, `if (!frag) Fatal(...)`, does **not**
catch a garbage *non-null* pointer - so without the assertion hardening, a wild pointer could be
stored directly onto the symbol (`sym.set_frag(frag)`) and dereferenced later during output
writing, a potentially more severe issue than the immediate abort seen here.

The simplest trigger (used by this repro) needs no relocation at all: `ObjectFile<E>::reattach_
section_pieces()` (`src/input-files.cc:1025-1046`) calls `m->get_fragment(esym.st_value)` directly
for any symbol defined inside a mergeable section, with `esym.st_value` read straight from the
file. `InputSection<E>::get_fragment()` (the relocation path, `src/mold.h:3374-3391`) shares the
identical unguarded call and is a second, independent way to reach the same bug via a negative
relocation addend - not independently repro'd this session (see "What's not done").

## Introduced by

Not determined. `git log -S'MergeableSection<E>::get_fragment'`/`-S'i64 idx = it - 1'` on
`src/mold.h` bottoms out at 5b859497 "Reorganize directory structure" (2024-08-14), a pure
file-move/rename commit (confirmed via `git show --stat`) that carried this function's body over
unchanged from its prior location - the trail continues further back through several more
"temporary"/"Refactor" bulk-squash commits without landing on a single clear introducing change,
the same pattern already seen for candidates 005/010/012 in this project. Not worth a full `git
bisect` this session given how far back the trail already goes with no bisectable signal to anchor
on.

## The fix

Reject an out-of-range `offset` before computing the index, returning the same `{nullptr, 0}`
sentinel the function could already legitimately return in principle (both existing callers already
handle a null `frag` via `Fatal(ctx) << *this << ": bad symbol value: " << esym.st_value`, so no
caller-side change was needed):

```cpp
template <typename E>
std::pair<SectionFragment<E> *, i64>
MergeableSection<E>::get_fragment(i64 offset) {
  if (frag_offsets.empty() || offset < frag_offsets[0])
    return {nullptr, 0};
  auto it = ranges::upper_bound(frag_offsets, offset);
  i64 idx = it - 1 - frag_offsets.begin();
  return {fragments[idx], offset - frag_offsets[idx]};
}
```

Verified:
- Re-ran `repro/run.sh` against the patched build: no longer crashes, exits 1 with `mold: fatal:
  <file>: bad symbol value: 18446744073709551615` instead of aborting - confirmed 3/3 runs.
- Confounded-fix check (`SKILL.md` pitfall #6): reverted just this hunk, rebuilt, re-ran the
  repro - crash returned identically (5/5), then re-applied the fix and reconfirmed it stays fixed.
- Project test suite (`ctest --test-dir build-plain -j4`, plain Debug build): **476/476 pass before
  the patch, 476/476 pass after** - no existing test exercised this exact crash path, and the fix
  doesn't regress legitimate mergeable-section/string-merging use (a common case, e.g. string
  literal deduplication).

## What's not done / open questions

- Only the `reattach_section_pieces()` trigger path (a symbol's raw `st_value` directly out of
  range) was independently built and confirmed this session. The second path noted in "Root cause" -
  `InputSection<E>::get_fragment()`'s relocation-addend route (a negative signed addend on a
  relocation against a mergeable-section symbol) - shares the identical unguarded call and is fixed
  by the same patch (the check is centralized inside `get_fragment()` itself, not per-caller), but
  wasn't independently driven through a dedicated crafted relocation this session; a future session
  could construct one for full completeness (per `CONFIRMATION.md` section 7's "checked more than
  just the one input" bar, already partially satisfied here by the confounded-fix revert/re-apply
  cycle serving as a second independent confirmation of the same trigger, but not a second distinct
  trigger *path*).
- `MergeableSection<E>::get_contents(i64 i)` (`src/mold.h:3457-3463`, a few lines below
  `get_fragment()`) does the same unguarded `frag_offsets[i]` indexing with an `i` that in principle
  could come from a similarly corrupted table state - not investigated this session as its own
  finding; worth a future look if this file's mergeable-section code is revisited.
