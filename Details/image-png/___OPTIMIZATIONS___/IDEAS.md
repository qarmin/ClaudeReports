<!--
Copy into ___OPTIMIZATIONS___/IDEAS.md. This file holds UNTESTED ideas only - the backlog of
places to look next. Once something is actually tried (a candidate folder exists with a
patch.diff/measurement), it belongs in SUMMARY.md, not here - SUMMARY.md is the single source of
truth for what's been tried, this file is only what hasn't. When a backlog item gets tried, remove
its row from here rather than moving it to an "already tried" list in this file.

Append to the backlog continuously while working a candidate, not just during a dedicated
discovery round - anything noticed in passing goes here immediately so it isn't lost.

The watermark line below persists across sessions - see SKILL.md's "GitHub issue/PR inspiration"
section. Update it to `#<MAX_NUMBER_SEEN> (<REPO>)` every time a raw sync runs (both values printed
by the sync command), even if nothing relevant was found; older unreviewed numbers below it are
intentionally never revisited. If the sync's printed `REPO` doesn't match the repo already recorded
here, treat the old number as stale (reset to 0 before applying `--since`) instead of silently
applying it to a different repo's own issue/PR numbering - issue/PR numbers are only unique within
one repo.
-->

# Ideas

Untested ideas only - see `SUMMARY.md` for what's already been tried.

Last GitHub issue/PR checked for inspiration: **#716** (repo: image-rs/image-png)

This workspace was created on 2026-09-02 by a discovery-only session. No candidate has been tried
yet and `SUMMARY.md` is empty, but `README.md` and `harness/encoder_bench/` now exist (`tools/` and
the root `justfile` still do not - the first real session should copy those in). Base commit
`05ccdb9c`. The `issues_raw/` cache here is a copy of the one the `bug-hunt` workspace synced the
same day (same repo, same watermark), not a second sync.

**Harnesses - reuse, do not rebuild.** See `README.md` for the full table. In short: the project's
own `benches/decoder.rs` (default features) covers decoding, `benches/unfilter.rs` /
`benches/adam7.rs` / `benches/expand_paletted.rs` cover three decoder hot spots but **need
`--features benchmarks`**, and `harness/encoder_bench/` (written by this workspace on 2026-09-02,
since the repo has no encoder bench at all) covers the encoder with modes for filters, compression
settings, color types, APNG, `StreamWriter` buffer sizes, APNG decoding, and re-encoding a whole
corpus directory. It verifies every iteration (round trip + output determinism) and reports
allocation/reallocation counts alongside timings.

**Read `README.md`'s "Baseline numbers" before picking anything below.** The one fact that re-ranks
this whole backlog: the encoder's default is `Compression::Balanced` -> `DeflateCompression::Level(6)`,
measured at **69 ms** on a 768x512 RGB photo versus **1.88 ms** for `FdeflateUltraFast` (~37x) for
5% smaller output - so on the default path deflate is ~97% of encode time and everything else is
rounding error. Encoder rows below are therefore about the **fast** paths
(`Fastest`/`Fast`/`FdeflateUltraFast`), where adaptive filtering alone is ~20% of the time; say
which compression setting every measurement used.

**Measure memory too, not just time.** Several rows below are allocation-shaped, and the harness
already quantifies them: a single buffered encode does 19-31 reallocations (32.25 MiB of allocation
churn to produce a 9.96 MB output on the largest corpus file), and an 8-frame APNG encode does 120.
A candidate that cuts allocation count or peak RSS with no wall-clock change is still a real result
here. On the decode side, issue #567's profiling puts `unfilter_curr_row` at 43-56% of decode time,
so decoder rows that are not about unfiltering are competing for a much smaller slice.

## Backlog (not yet tried)

Consumed top-to-bottom by default when a session isn't given a specific target - see `SKILL.md`.

| Added | Idea | Why it looks promising | Source |
|---|---|---|---|
| 2026-09-03 | Vendor `fdeflate`, add `Decompressor::reset(&mut self)` reusing its two boxed lookup tables in place instead of `Decompressor::new()` reallocating them | Candidate 007 confirmed precisely: `Decompressor::new()` costs exactly 2 allocations/call (~145-175 ns), and real APNG decode allocation growth is exactly +2/frame across 1-32 frames - a 1:1 match. `fdeflate` 0.3.7 exposes no public reset method. `build_tables` always fully rebuilds both tables from each block's Huffman header before reading them, so stale contents are provably not a correctness concern - only the allocations need to survive a reset. Needs its own `___OPTIMIZATIONS___/` in a cloned `fdeflate` checkout, not a `png`-side change | `007.FDEFLATE_DECOMPRESSOR_RESET_COST/RESULT.md` |
| 2026-09-02 | Multi-threaded encoding | Issue #479 asks for an mtpng-style parallel encoder. Architectural and almost certainly out of scope for a single candidate, and it would change the crate's dependency/API surface - recorded here so it is not rediscovered as if it were new. If anything, measure first what fraction of encode time is deflate vs filtering, which the fused-cost candidate above needs anyway | issue #479 |
| 2026-09-02 | `Filter::NoFilter` is *slower* end-to-end than every real filter in the fast path | Measured on `kodim02.png` at `FdeflateUltraFast`: NoFilter 1.869 ms vs Sub/Up/Avg 1.19-1.21 ms, because unfiltered rows compress far worse (1,180,318 B vs ~700 KB) and every extra output byte costs deflate work and another realloc. Two consequences worth writing down: it is a documentable gotcha for users picking `NoFilter` "for speed", and it means any measurement that varies the filter also varies the compression work - a candidate that wants *pure* filter cost must hold the output size constant (e.g. `--pattern flat`) or subtract the deflate component | `harness/encoder_bench` baseline |
| 2026-09-03 | **[Reserved as candidate 023, do not reassign this number]** Investigate build-to-build wall-clock non-reproducibility on this machine before trusting any candidate's timing to better than a few percent | Candidate 001 measured the same patch with the same paired/`taskset -c 0` protocol on two independently-built (`cargo build --release`) binary pairs from identical source and got opposite paired-run consistency verdicts (NOT consistent 3/13 vs consistent 21/21 for the same file); re-testing the original pair immediately after reproduced the original read, ruling out simple machine-load drift. Try `CARGO_INCREMENTAL=0` + `-C codegen-units=1` across several independent build+measure cycles to determine whether this is real codegen variance (parallel codegen-unit scheduling) or something else | `001.ENCODER_OUTPUT_VEC_CAPACITY/RESULT.md` |
| 2026-09-03 | **[Reserved as candidate 024, do not reassign this number]** Design a correct fix to stop doubling the chunk buffer from 128 bytes for large ancillary chunks (`src/decoder/stream.rs:801-824`) | `ChunkState::raw_bytes` starts at `CHUNK_BUFFER_SIZE=128` and doubles via `reserve_exact(capacity.min(limits.bytes))` as bytes arrive, so a 500 KB iCCP profile is reallocated/copied ~12 times; a single `reserve(min(remaining, limits))` up front would avoid that, since `remaining` already holds the chunk's declared length and a `min()` against the limit already guards against a bogus length. This was investigated for candidate 003 but not attempted: `parse_captured` (~line 1815) already does its own unconditional `limits.reserve_bytes()` charge for every captured chunk's bytes, independent of the buffer-growth reserve calls, so naively adding an upfront reserve in `start_chunk` would double-charge captured chunks against `Limits::bytes` and regress legitimate large captured chunks to roughly half their intended budget. A correct fix needs `parse_captured`'s charge to be aware an upfront reserve already happened for the same chunk (or a restructure of where the limit gets charged) before measuring the allocation-count win. Related: issue #621 wants ignored/oversized chunks skipped without buffering at all, which would remove both the copy and the limit charge | code read + issue #621, prior investigation notes in candidate 003's backlog entry |
