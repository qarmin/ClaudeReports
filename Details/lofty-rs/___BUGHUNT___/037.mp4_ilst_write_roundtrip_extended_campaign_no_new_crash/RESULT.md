# mp4_ilst_write_roundtrip_extended_campaign_no_new_crash

`IDEAS.md` (2026-08-26 entry, noticed while working candidate `026`) asked for a longer
(multi-minute-to-hour) corpus-driven fuzzing campaign against candidate `026`'s
`mp4_ilst_write_roundtrip` harness (`___BUGHUNT___/_reference/026_fuzz_harness/`, seed corpus at
`___BUGHUNT___/_reference/026_corpus/`), on top of the patched tree, since `026`'s own session only
ran 300s + 120s bursts - short by this project's own standards (candidate `013`'s targeted campaign
ran 300s/126.8M executions). This session built the "patched tree" by applying every currently
verified `patch.diff` in this same MP4 write path (candidates `026`, `027`, `030`, `033`, `036`,
which together close every previously-confirmed crash/logic bug reachable from this harness) to the
real source, and re-ran the harness for real, much longer, against it.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Applied all 5 currently-verified patches for this write path (`026.../patch.diff` `atom_info.rs`, `027.../patch.diff` `update_offsets` co64/tfhd overflow, `030.../patch.diff` `moov.rs` mdia slack, `033.../patch.diff` `properties.rs` stbl slack, `036.../patch.diff` `update_offsets` stco truncating-cast wrap) together to `lofty/src/` | `git apply --check` on each individually, then all 5 applied together in one working tree | All 5 apply cleanly with zero conflicts (also closes the `027`/`036` "never verified together" gap noted in `IDEAS.md`, since both touch `update_offsets` on non-overlapping lines) - confirms this is a real usable "patched tree" baseline, not just individually-verified diffs |
| Re-added `mp4_ilst_write_roundtrip` as a temporary `cargo fuzz` target (from `_reference/026_fuzz_harness/`), seeded with `_reference/026_corpus/`'s same 23 real `.m4a`/`.mp4` fixtures, single-threaded, `RLIMIT_NOFILE` lowered first (candidate `024`'s ASAN-hang workaround) | `cargo +nightly fuzz run --sanitizer address -- -max_total_time=1800`, patched tree | Completed cleanly: `Done 16731440 runs in 1801 second(s)`, exit code 0, corpus grew from 23 to 2249 files, no crash artifact, no ASAN report, no hang - roughly 130x the raw execution count of candidate `026`'s original 300s burst that found a crash within 85,599 executions |
| Same harness/corpus/patched-tree, re-run with libFuzzer's own parallel fuzzing (`-jobs=8 -workers=8`, 8 independent mutation streams/seeds instead of one) to explore a different slice of state space in bounded wall-clock time | `cargo +nightly fuzz run --sanitizer address -- -max_total_time=600 -jobs=8 -workers=8`, patched tree, same seed corpus | Completed cleanly: `Done 2834147 runs in 601 second(s)` (master log; 8 workers ran in parallel for the same 601s), exit code 0, no crash artifact, no ASAN report, no hang |

Two approaches, both clean negatives with real signal absent (no crash, no hang, no ASAN report,
no flaky/partial symptom) - per `SKILL.md`'s soft cap, no signal to chase further, and the seed
corpus already covers every real `.m4a`/`.mp4` fixture that exists in the repo (`find
lofty/tests -iname '*.m4a' -o -iname '*.mp4'` returns the same 23 files used as the original seed),
so a third "broader seed" escalation would not add anything a corpus-driven mutation campaign
hasn't already explored across ~19.5M total executions. Stopped at 2/3 approaches.

## Status

**Not reproducible**

No new crash of any kind was found in ~19.5M cumulative executions (16,731,440 single-threaded +
2,834,147 reported by the 8-worker parallel run's master process, itself aggregating 8 concurrent
workers over the same 601s) against the patched tree, using the exact harness and seed corpus named
in the `IDEAS.md` request. This does not prove the write path is bug-free (see "What's not done"
below) - it specifically rules out finding a *new*, cheaply-fuzzable crash in this harness's
reachable surface within this session's campaign, on top of the 5 bugs already found and fixed by
prior candidates.

## Class

N/A

## Root cause

N/A - nothing new reproduced. The write path's previously-confirmed bug classes (header-inclusive
vs. content-only atom-size off-by-8 checks in `atom_info.rs`/`moov.rs`/`properties.rs`, and
unchecked 64-bit/32-bit offset-rewrite arithmetic in `ilst/write.rs::update_offsets`) are already
covered by candidates `026`/`027`/`030`/`033`/`036`; this campaign's job was to look for a *sixth*,
not-yet-found instance in the same reachable surface, and found none.

## The fix

Not applicable - no bug found this session, so nothing to fix. The 5 patches applied to build the
"patched tree" for this campaign are each already documented and verified in their own candidate
folders (`026`, `027`, `030`, `033`, `036`); this candidate only re-confirms they apply and build
together cleanly, it does not introduce a new fix of its own.

- Project test suite: not run this session - no source change was retained (the 5 patches were
  applied only for the duration of the fuzzing campaign and reverted immediately afterward per
  `SKILL.md` step 8; see `IDEAS.md`'s already-open item asking whether all of the project's
  currently-drafted MP4 patches compose and pass the suite together, which this candidate narrows
  but does not itself close since it never ran `cargo test`).

## What's not done / open questions

- This campaign only exercises `mp4_ilst_write_roundtrip`'s own reachable surface (parse real file
  -> mutate `ilst` -> `save_to` -> re-parse). It does not cover MP4 write paths this harness never
  touches (e.g. writing brand-new atoms/chunks not derived from an existing real file's `ilst`,
  or the `Atom`/`AtomData` variants the harness's own mutation logic doesn't construct - see the
  harness source for exactly which fourccs/`DataType`s it generates).
- ASAN was used for both runs (with candidate `024`'s `RLIMIT_NOFILE` workaround); UBSAN and `miri`
  were not tried against this harness this session - `miri` in particular cannot run a real
  `cargo-fuzz`/libFuzzer harness directly (no libFuzzer support under `miri`), so a `miri`-based
  campaign against this same code would need a different (non-fuzz) driver.
- Corpus minimization/coverage plateaued visibly by the end of the single-threaded run (`cov: 2781`
  barely moving across the final several hundred thousand executions, `exec/s` dropping from
  ~24k to ~10k as `rss` grew to ~709Mb) - a much longer (multi-hour, `fuzz-bug-repro`-style
  persistent-corpus) campaign was still not attempted, and per `IDEAS.md`'s own earlier note (from
  candidate `009`'s session) a dedicated `fuzz-bug-repro` campaign across MP4 write entry points
  together remains a distinct, not-yet-done escalation beyond what this skill's own bounded
  campaigns are designed to do.
