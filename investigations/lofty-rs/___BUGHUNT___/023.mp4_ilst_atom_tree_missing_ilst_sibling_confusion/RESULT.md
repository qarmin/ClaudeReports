# mp4_ilst_atom_tree_missing_ilst_sibling_confusion

Follow-up to the `IDEAS.md` lead left by candidate `015`: "a longer, corpus-driven campaign
against `mp4/ilst/write.rs`'s write path... especially around multi-sibling atom layouts inside
`udta`/`meta` that weren't tried". Rather than starting with a fuzz harness, reading
`save_to_existing`'s helper `atom_tree` (`lofty/src/mp4/read/mod.rs`) against exactly that
"multi-sibling `meta` children, `ilst` missing/not-first" shape surfaced a deterministic bug via
manual boundary-value construction on the first approach tried (arithmetic/index-math class per
`HEURISTICS.md`'s tool-selection guidance), so a fuzz harness was never needed for this candidate.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code reading of `atom_tree` (`lofty/src/mp4/read/mod.rs`) against every multi-sibling `meta`-children shape not covered by candidate `015`'s 33 size-mutation sweep (which only varied declared-size fields on a file that already had a real `ilst`) | N/A (static reasoning) | `found_idx` is tracked as a 1-based counter (`found_idx = i` on a match), then unconditionally `saturating_sub(1)`'d at the end to convert to a 0-based index - but this collides "matched at buf-index 0" with "never matched at all" (both start/settle at the sentinel `0`), and the caller (`write.rs`) only checks `tree.is_empty()`, not "was `up_to` actually found" |
| Boundary-value construction: a real, valid `udta > meta` (full, version/flags = `0`) containing real children (`hdlr` + `free`) but **no `ilst` atom among them at all** - a legitimate "metadata container present, no iTunes tag written yet" MP4 layout, real `Mp4File::read_from`/`set_ilst`/`save_to` public API, debug build | Confirmed | The real `hdlr` atom (`tree[0]`, since `found_idx` incorrectly resolves to `0` in the "never matched" case) is wrongly treated as the existing `ilst` atom and gets clobbered/replaced by the newly written `ilst` atom's bytes instead of the new atom being appended after `hdlr`/`free` as intended; reproduced deterministically 3/3 runs (debug and release, `overflow-checks = true`, no randomness) |
| Repeat-run confirmation via the standalone `repro/` against a pristine clone at `base_commit`, release profile with `overflow-checks = true` | 3 consecutive `cargo run --release` invocations, unpatched clone | 3/3 "REPRODUCED" (exit code 1) |
| Same repro, patched clone (`patch.diff` applied) | 3 consecutive `cargo run --release` invocations | 3/3 "did not reproduce" (exit code 0) |

## Status

**Confirmed.**

**Reproduction rate: 3/3 runs** (debug build against the main project's working tree) **and a
further 3/3 runs** (release build, `overflow-checks = true`, standalone clone via `repro/`) -
fully deterministic, no randomness/fixed-seed dependency involved.

**Fully reproducible: Yes.** `repro/` calls the real public entry points a genuine caller uses to
open and re-tag an MP4/M4A file (`Mp4File::read_from` -> `set_ilst` -> `save_to`) on a
byte-for-byte real, spec-shaped file built once in code - no internal/private lofty API and no
hand-constructed `AtomInfo`/`ContextualAtom`/`atom_tree` state is fed directly to the suspect
function. This is the "malformed/adversarial file handed to a parser's real public entry point"
case from `CONFIRMATION.md` section 11 (a crafted MP4 file on disk **is** the genuine external
input a real caller/attacker would supply); there is no second participant to fake here, so no
downgrade applies.

## Class

`logic` (silent, deterministic destruction of a real, existing MP4 atom on every write - no
crash/panic observed in either debug or `overflow-checks` release builds).

## Root cause

`lofty/src/mp4/ilst/write.rs::save_to_existing` locates the existing `ilst` atom (if any) among
`meta`'s children via `atom_tree` (`lofty/src/mp4/read/mod.rs`):

```rust
let mut found_idx: usize = 0;
...
while len > 8 {
    ...
    if let AtomIdent::Fourcc(ref fourcc) = atom.ident {
        i += 1;
        if fourcc == up_to {
            found_idx = i;
        }
        buf.push(atom);
    }
}
found_idx = found_idx.saturating_sub(1);
Ok((found_idx, buf))
```

`found_idx` is meant to convert `i`'s 1-based "position of the last atom whose fourcc matched
`up_to`" into a 0-based index into `buf`. But it starts at `0` and is only ever assigned when a
match occurs - so if `up_to` (`"ilst"`) never matches any child, `found_idx` stays `0` for the
entire loop, and the final `saturating_sub(1)` on an already-`0` value is a no-op, leaving it at
`0` again. This is **indistinguishable** from the genuinely valid case where the very first child
atom actually is the `ilst` atom being searched for (`i` becomes `1` on the first match,
`saturating_sub(1)` also yields `0`).

The caller only guards against the *other* degenerate case:

```rust
if tree.is_empty() {
    // Nothing to do / append new ilst at meta_end
} else {
    let existing_ilst = &tree[ilst_idx];   // ilst_idx may be 0 with NO real match!
    ...
}
```

`tree.is_empty()` is false whenever `meta` has *any* children at all (e.g. just a `hdlr` atom, or
a `hdlr` + `free` atom pair - an entirely ordinary, spec-valid MP4 layout for a file whose
metadata container exists but has never had an iTunes-style `ilst` tag written into it, or one
using a different metadata scheme under `meta`). In that case `existing_ilst = &tree[0]` silently
picks whatever real atom happens to be first (here, `hdlr`) and treats its byte range as if it
were the (nonexistent) `ilst` atom to overwrite/replace/pad around - the real atom's on-disk bytes
are subsequently spliced out and replaced with the newly built `ilst` atom's bytes, and the
`meta`/`udta`/`moov` parent atom sizes are all updated consistently around this wrong byte range
(so the resulting file remains structurally well-formed and does not fail an integrity check on
read-back), permanently and silently destroying the real, distinct atom that used to be there
(here, `hdlr` - `mdirappl`) with every save.

## The fix

Change `atom_tree`'s return type from `(usize, Vec<AtomInfo>)` to `(Option<usize>, Vec<AtomInfo>)`
so "found at index 0" and "never found" can no longer collide: track the index directly as
`buf.len()` at the moment of a match (`found_idx = Some(buf.len())`, right before `buf.push`)
instead of a separate 1-based counter needing an error-prone post-hoc `saturating_sub(1)`
adjustment. Update the sole caller (`save_to_existing`) to branch on `Option::is_some()` instead of
`tree.is_empty()` - which also folds the previous `tree.is_empty()` special case into the same
`None` arm for free, since an empty tree can now only ever produce `found_idx == None` too (see
`patch.diff`).

Verified: re-running `repro/` against the patched tree no longer reproduces (3/3 runs, see
`output.txt`) - the real `hdlr` atom's `mdirappl` payload survives the write, and the new `ilst`
atom is correctly appended after `meta`'s existing children instead of overwriting one of them.

- Project test suite (`lofty` crate), run identically before and after the fix:
  - `cargo test -p lofty --lib`: **256 passed; 0 failed** (before and after, identical)
  - `cargo test -p lofty --test taglib`: **191 passed; 0 failed; 73 ignored** (before and after,
    identical)
  - No existing test covers a real `meta` atom with children but no `ilst` among them, which is
    why this shipped unnoticed - every existing MP4 fixture/test that writes a tag either has no
    `udta`/`meta` at all, or already has a real `ilst` as (one of) `meta`'s children.

## What's not done / open questions

- Only the specific "no `ilst` present among real `meta` children" shape was constructed by hand.
  The same off-by-one could theoretically also matter for "multiple `ilst` atoms present"
  (malformed duplicate `ilst` siblings) - `found_idx` there resolves to the *last* match, which is
  at least a real `ilst` atom (not an unrelated one), so it wasn't pursued further as a distinct
  bug this session, but wasn't independently fuzzed either.
- This closes out the `IDEAS.md` "corpus-driven `mp4/ilst/write.rs` campaign" lead's stated
  motivation (a multi-sibling `udta`/`meta` layout candidate `015`'s hand-picked 33 boundary
  mutations never tried, since none of them removed the existing `ilst` atom while keeping other
  real siblings) without needing an actual fuzz harness - the bug was reachable directly via
  boundary-value construction once the right shape was tried by hand. A real corpus-driven
  campaign against this same write path might still be worth running once for its own sake (e.g.
  via `fuzz-bug-repro`), but is no longer specifically motivated by this exact gap.
