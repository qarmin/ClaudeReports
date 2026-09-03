# Second-allocator (jemalloc) check for candidates 005/016's glibc-specific regressions

Candidates 005 (bulk single-byte text codec) and 016 (`InMemObjectTokens::tokens_pending`
`VecDeque` reuse) each measured a real, reproducible write-path regression on `seq_10k.dcm`
(`write-only` for 005, `roundtrip` for 016) that **disappeared or reversed under `mimalloc`**
(`--features diag-mimalloc`) - confirming both were allocator-specific pathologies with glibc's
default malloc for these particular repeated-small-allocation patterns, not flaws in the
optimizations themselves. `IDEAS.md` flagged one remaining open question: only `mimalloc` was
tried; whether `jemalloc` shows the same resolution was never checked, and the specific glibc
mechanism (arena count, `M_MMAP_THRESHOLD`, thread-local cache behavior) remains uncharacterized
either way.

## What was tried

Added a new harness feature, `diag-jemalloc` (harness-only, mirrors the existing `diag-mimalloc` -
swaps the global allocator to `tikv-jemallocator` instead of the system default). Re-applied each
candidate's own `patch.diff` unchanged, one at a time (not combined - each tested in isolation
against the same jemalloc-enabled baseline), using the same fixture/mode each original candidate
found its regression on.

| Comparison | mode | jemalloc round 1 | jemalloc round 2 (re-check) | jemalloc round 3 (fresh `just test` clone) | original (glibc default) | original (mimalloc) |
|---|---|---|---|---|---|---|
| candidate 005 | `write-only`, `seq_10k.dcm` | -13.6%, 9/9 consistent | -16.9%, 9/9 consistent | -15.1%, 9/9 consistent | +93-100% (regression) | -7 to -18% (win) |
| candidate 016 | `roundtrip`, `seq_10k.dcm` | +1.5%, 2/9 not consistent | -1.0%, 7/9 not consistent | +0.9%, 5/9 not consistent | +31-37% (regression) | -1 to -2% (neutral-to-slightly-positive) |

## Result

**Both regressions are confirmed non-fundamental under jemalloc too, using a second, independent
alternative allocator - not just mimalloc:**

- Candidate 005's `write-only` regression: **fully reverses** under jemalloc, into a real,
  consistent **win** (-13.6%/-16.9%, both rounds 9/9) - same direction and similar rough magnitude
  as the mimalloc result (-7 to -18%).
- Candidate 016's `roundtrip` regression: **disappears** under jemalloc (neither round clears the
  consistency bar, matching mimalloc's own "neutral-to-slightly-positive" finding almost exactly).

This strengthens the original conclusion considerably: two independent, unrelated allocator
implementations (mimalloc's page-based design, jemalloc's arena-based design) both resolve the
same two regressions the same way glibc's ptmalloc doesn't - strong evidence this is a real
ptmalloc-specific pathology for these access patterns, not a coincidence of one particular
alternative allocator's behavior.

## Does it change behavior/output?

- **Bit-exact?** Yes - `checksum=0x6884f8e2551ec289` identical across baseline/patched for both
  candidate 005 (`write-only`) and candidate 016 (`roundtrip`) under jemalloc.
- **Is this a breaking change?** No - measurement-only investigation, no new library-source change
  (both patches are candidates 005/016's own, applied unchanged, one at a time). Copies of both are
  saved here as `patch_005.diff`/`patch_016.diff` (identical to `005.TEXT_CODEC_BULK_SINGLE_BYTE/patch.diff`
  and `016.TOKENS_PENDING_REUSE/patch.diff` respectively) so this folder is self-contained; each
  applies clean against this workspace's `base_commit` on its own (`git apply --check`, verified).
- Project test suite: `cargo test --release -p dicom-encoding` (candidate 005's patch applied):
  **40 + 4 = 44 passed / 0 failed**, matching candidate 005's own recorded count.
  `cargo test --release -p dicom-object` (candidate 016's patch applied): **75 + 9 + 22 = 106
  passed / 0 failed**, matching candidate 016's own recorded count. Both reverted-tree (baseline)
  counts are identical.

## Why the result looks the way it does

Not newly explained here beyond what candidates 005/016's own mimalloc follow-ups already
established - this candidate's contribution is corroboration via a second, architecturally
different allocator, not a new mechanism. The specific *glibc* mechanism (why ptmalloc specifically
mishandles these two repeated-small-allocation patterns) remains uncharacterized - `strace`/`ltrace`
are still unavailable on this box, and no attempt was made to inspect glibc's own arena/heap
internals directly (e.g. via `malloc_stats`/`mallinfo2`).

## What's not done / open questions

- The exact glibc mechanism is still not root-caused - this closes the "try jemalloc too" half of
  the original `IDEAS.md` item, but the "root-cause the exact glibc mechanism" half remains open
  (arena count, `M_MMAP_THRESHOLD`, thread-local cache behavior are all still just named
  possibilities, none investigated directly).
- Only `seq_10k.dcm`'s two known regression cases were re-checked - no attempt to find a third,
  independent case to further generalize the "glibc-specific" conclusion.
