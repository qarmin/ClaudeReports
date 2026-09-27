# mp4_ilst_write_find_child_atom_off_by_header

`IDEAS.md` (2026-08-26 entry, left open by candidate `026`) asked whether `lofty/src/mp4/ilst/write.rs`'s
own `find_child_atom` calls in `write_to` (searching for `udta` within `moov` using `moov_len`,
and `meta` within `udta` using `udta.len` - both header-inclusive budgets, even though the reader
is already seeked past the parent's own header at that point) have an *independent* off-by-`ATOM_HEADER_LEN`
bug of their own, separate from candidate `026`'s already-fixed `atom_info.rs` check.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code-reading / control-flow tracing of `write_to`, `find_child_atom`, `AtomWriter::new_from_file`, `ContextualAtom::read`, and `AtomReader` | N/A | Found that `write_to` (`lofty/src/mp4/ilst/write.rs`) calls `AtomWriter::new_from_file` (line ~52) *before* either `find_child_atom` call (lines 80-86, 109-115). `new_from_file` recursively re-parses the whole atom tree via `ContextualAtom::read`, which - because both `udta` and `moov` are in `IMPORTANT_CONTAINERS` - always passes the *correct*, header-excluded, running-decremented budget (`info.len - info.header_size()`) to `AtomInfo::read` for every descendant down to and including `meta` (a validated leaf, since `meta` itself isn't a container `ContextualAtom` recurses into). Any atom whose declared size doesn't fit that tight budget is rejected there, propagating as a hard `Err` out of `write_to` well before either of the two later `find_child_atom` calls (which read the *same* physical bytes at the *same* starting offsets) can ever run. This is a structural proof that the two `find_child_atom` calls' looser budgets can never accept anything `new_from_file` wouldn't already have accepted first, i.e. the suspected extra slack is dead/unreachable code in the write path. |
| Boundary-value construction: patched `atom_info.rs` with candidate `026`'s already-verified fix (`len > reader_size`, isolating this candidate's issue from `026`'s), took a real fixture (`lofty/tests/taglib/data/has-tags.m4a`, `udta` is `moov`'s last child, ending exactly at both `moov`'s and the real file's end), and bumped the on-disk `udta` size field by exactly `ATOM_HEADER_LEN` (8) - the exact slack window this candidate's suspected bug would extend acceptance by, given `find_child_atom`'s budget for that call is `moov_len` instead of the correct `moov_len - ATOM_HEADER_LEN` | Debug build, temporary instrumented `#[test]` (`lofty/tests/probe_026_scratch_test.rs`, not committed) calling the real public `Mp4File::read_from` -> `set_ilst` -> `save_to` | `read_from` succeeded (the read path, `mp4/moov.rs`, silently drops the malformed `udta` via its own `while let Ok(Some(atom)) = reader.next()` pattern, so `ilst_tag` ends up `None`). `save_to` (which re-invokes `write_to`) returned a graceful `Err`, not a panic - confirmed via temporary `eprintln!` instrumentation that the error originates from `AtomWriter::new_from_file` itself (i.e. exactly where the code-reading proof predicted), never reaching either `find_child_atom` call at all. |
| Short `cargo fuzz` burst against a reconstructed version of candidate `026`'s one-off `mp4_ilst_write_roundtrip` harness (real `Mp4File::read_from` -> mutate `ilst` -> `save_to` -> re-`read_from`), with candidate `026`'s fix *and* candidate `027`'s already-verified `update_offsets` overflow fix both applied (to remove the two already-known, unrelated crash signatures from the search space and isolate this candidate's specific class), seeded with the same 23-file real `.m4a`/`.mp4` corpus | `cargo +nightly fuzz run --sanitizer address`, `RLIMIT_NOFILE` lowered to `65536` first (candidate `024`'s ASAN-hang workaround), `-max_total_time=90` | 1,803,993 executions in 91s, 0 crashes, 0 artifacts written |

## Status

**Not reproducible.**

Both the boundary-value construction (a hand-crafted input specifically designed to land in the
exact slack window this bug would need) and a 1.8M-execution fuzz burst (with the two other known
bugs in this file already patched to remove noise) found nothing, consistent with the structural
proof: any file malformed enough to matter to `find_child_atom`'s looser budget is already rejected
earlier in the very same function, by `AtomWriter::new_from_file`'s own correctly-budgeted
recursive parse of the identical bytes.

## Class

N/A

## Root cause (why this turned out to be safe)

`write_to` in `lofty/src/mp4/ilst/write.rs` performs two independent, redundant traversals of the
same on-disk `moov` subtree, in this order:

1. `AtomWriter::new_from_file` (line ~52), which recursively parses `moov` -> `udta` -> (leaf)
   `meta` via `ContextualAtom::read` (`lofty/src/mp4/write.rs`). Because `udta` (and `moov`) are
   listed in `IMPORTANT_CONTAINERS`, this recursion always computes each container's children
   budget as `info.len - info.header_size()` (correctly header-exclusive, and correctly adjusted
   for the 8-byte extended-size field via `header_size()`), and decrements it exactly per real
   child consumed. It additionally requires the leftover budget to hit exactly `0` at the end of
   each container (`if len != 0 { return Err(...) }`), i.e. zero tolerance for any inconsistency
   between a container's declared size and the sum of its real children's declared sizes. `meta`,
   though not itself a recursed-into container, is still read as a plain child via the same
   correctly-budgeted `AtomInfo::read` call, so its declared size is validated too.
2. Only *after* this succeeds does `write_to` seek back to `moov`'s data start and call
   `find_child_atom` (lines 80-86) for `udta`, then (lines 109-115) for `meta` within `udta` -
   both using the parent's *full*, header-inclusive declared size (`moov_len`, `udta.len`) as the
   search budget, instead of `moov_len - moov.info.header_size()` / `udta.len - ATOM_HEADER_LEN`.
   In isolation this is indeed off by `ATOM_HEADER_LEN` (matching the `IDEAS.md` suspicion,
   confirmed by inspection of `find_child_atom`'s use of that budget in `AtomInfo::read`'s
   now-fixed `len > reader_size` check).

Because step 1 reads the identical physical bytes at the identical starting offsets as step 2,
and step 1's budget is always tighter than or equal to step 2's, anything step 2 could accept
that step 1 wouldn't have already validated first simply never gets the chance to matter: step 1
either already rejects the file outright (propagating a hard `Err` out of `write_to`, as observed
in the boundary-value test) before step 2 runs at all, or step 1 already accepted the exact same
atom with the exact same declared length that step 2 will independently re-derive from the same
bytes. `find_child_atom`'s looser budget is real, off-by-header code, but it is unreachable dead
code from the `write_to` entry point in current lofty - it would only matter if this file's
`find_child_atom`/`atom_tree` primitives (`lofty/src/mp4/read/mod.rs`) were ever called from a
context that does *not* first go through an equivalently-tight-budgeted parse of the same bytes,
which is not the case for `save_to_existing`'s two calls today.

## The fix

None attempted - there is nothing to fix; the code is safe by construction of its call order, not
merely by luck on the inputs tried. Tightening `find_child_atom`'s budget in `write_to` to
`moov_len - moov.info.header_size()` / `udta.len - ATOM_HEADER_LEN` would still be reasonable
defensive-hygiene (removing genuinely-incorrect-looking code, and removing a latent trap for any
future refactor that changes the call order or reuses `find_child_atom` from a less-guarded
context), but is not required to close an actual reachable bug, so no patch was applied this
session.

- Project test suite: not run (no fix applied, nothing to verify - N/A).

## What's not done / open questions

- This reasoning is specific to today's `write_to` call order (`AtomWriter::new_from_file` always
  runs first). If a future refactor moves `find_child_atom`'s two calls in `write_to` to run
  *before* `AtomWriter::new_from_file`, or reuses them from a new call site that doesn't first
  re-validate the same bytes with a tight budget, this dead code becomes live again - worth a
  one-line comment or a defensive tightening at the two call sites regardless, even though no bug
  is reachable today. Left as a suggestion, not applied, per "no bounded fix needed" above.
- Did not check whether `moov.rs`'s *read-path* `find_child_atom` call (`ilst_from_udta`'s
  `mdia`-search via `find_child_atom(reader, atom.len, *b"mdia", ...)`, line 54 of `moov.rs`) has
  the same header-inclusive-budget shape; that one runs against `AtomReader`'s own live
  `remaining_size` bound (not a fixed `atom.len` snapshot) and was out of scope for this
  `write.rs`-focused candidate - added to `IDEAS.md` as a new suspect.
