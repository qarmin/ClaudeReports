# 87 - Why is `flat` scrolling's `CHUNK_SIZE = 1024` win multi-core-consistent but single-core-inconsistent?

IDEAS.md's backlog (added while closing out candidate 078) asked for profiling, or at least an
allocation/scheduling counter, to explain why candidate 078 found `flat` scrolling's chunked-arena
`CHUNK_SIZE = 1024` a consistent multi-core win (-4.9%/-4.6%, 9/9 both reruns) but only a leaning,
inconsistent single-core one (-2.1%/-2.2%, 5-6/9 both reruns) on the same scene - the first
paired-consistent `CHUNK_SIZE` win anywhere in this sweep (073/074/075/078). `perf` is blocked in
this environment (same limitation every candidate in this sweep has hit), so this candidate adds
a scheduling counter to `perf_test` instead and measures with it, plus two other direct
measurements (chunk count, chunk byte size) that needed no counter at all.

Same prerequisite as 074/078/080/083: `chunked_arena` does not exist on `master`, so candidate
073's patch (`CHUNK_SIZE = 256`) is applied first; `patch.diff` here is that same 073 patch,
unmodified. `CHUNK_SIZE = 1024` (the value under investigation) is a one-line edit on top of it,
not a separate patch, same as every other candidate in this sweep.

## What was tried

1. **A new scheduling counter in `perf_test`** (`perf_test/src/alloc_stats.rs`'s
   `scheduling_stats()`, printed by `bench.rs` after the existing `resident memory` line): reads
   `/proc/self/status` for voluntary/involuntary context-switch counts and `/proc/self/stat` for
   minor/major page faults, once at the end of a run (not per frame, so it doesn't perturb the
   timing it explains). This is a **permanent addition to the live `perf_test/` checkout**, not
   part of `patch.diff` - see `README.md`'s "Instrumentation added" table for the same treatment
   every other `perf_test` extension in this workspace gets. Full source is in that table's row
   and reproduced below since `perf_test` has no git history of its own:

   ```rust
   // perf_test/src/alloc_stats.rs
   pub struct SchedulingStats {
       pub voluntary_ctxt_switches: u64,
       pub nonvoluntary_ctxt_switches: u64,
       pub minor_faults: u64,
       pub major_faults: u64,
   }

   pub fn scheduling_stats() -> Option<SchedulingStats> {
       #[cfg(target_os = "linux")]
       {
           let status = std::fs::read_to_string("/proc/self/status").ok()?;
           let mut voluntary_ctxt_switches = None;
           let mut nonvoluntary_ctxt_switches = None;
           for line in status.lines() {
               if let Some(rest) = line.strip_prefix("voluntary_ctxt_switches:") {
                   voluntary_ctxt_switches = rest.trim().parse().ok();
               } else if let Some(rest) = line.strip_prefix("nonvoluntary_ctxt_switches:") {
                   nonvoluntary_ctxt_switches = rest.trim().parse().ok();
               }
           }
           let stat = std::fs::read_to_string("/proc/self/stat").ok()?;
           let after_comm = stat.rsplit_once(')')?.1;
           let fields: Vec<&str> = after_comm.split_whitespace().collect();
           let minor_faults = fields.get(7)?.parse().ok()?;
           let major_faults = fields.get(9)?.parse().ok()?;
           Some(SchedulingStats {
               voluntary_ctxt_switches: voluntary_ctxt_switches?,
               nonvoluntary_ctxt_switches: nonvoluntary_ctxt_switches?,
               minor_faults,
               major_faults,
           })
       }
       #[cfg(not(target_os = "linux"))]
       None
   }
   ```

   `bench.rs`'s `report()` prints it right after the resident-memory line:
   `scheduling     ctxt switches {v} voluntary / {nv} involuntary, page faults {min} minor / {maj} major`.

2. **A temporary debug counter** (added, measured, then removed before finishing - not part of
   any saved patch) in `chunked_arena::grow()`, printing `size_of::<[Slot<T>; CHUNK_SIZE]>()` and
   the per-`grow()` count, to get the arena's actual chunk count and byte size instead of the
   arithmetic 078 used.

3. **Repeated `tools/paired.sh` runs** (`RUNS=9 FRAMES=20`, same protocol as 073/074/075/078/080)
   on `scroll --count 100000 --row-style flat`, both `taskset -c 2` (single-core) and unpinned
   (multi-core), attempting to reproduce 078's specific result this session.

4. **`tools/pixelcheck.sh`** (`RUNS=12`) and the project's `cargo test` suite, both before/after,
   per this workspace's standard correctness check.

## Result

Four measured facts, plus a failed reproduction attempt:

1. **078's chunk-count arithmetic was off by ~4x**, measured directly: `flat` row style has ~4
   arena entries per row (matches `README.md`'s per-row-style item-count table: `flat` = 4 items),
   not 1, so `--count 100000` produces **~400,000** arena entries, not 100,000. The debug counter
   confirmed **1563 `grow()` calls at `CHUNK_SIZE = 256`** and **391 at `CHUNK_SIZE = 1024`** -
   each exactly 4x what 078's "Why the result looks the way it does" section stated for those same
   two values (078 said "391 for 256" and "about 98 chunks for 1024", i.e. it computed against
   100,000 rows instead of ~400,000 items). The qualitative "1024 makes fewer, larger chunks"
   direction 078 reasoned from is unaffected by this, but the specific numbers in 078's writeup are
   wrong by a factor of 4 - see `CORRECTION.md`-equivalent note below (078's `RESULT.md` conclusion
   is not invalidated, only that one paragraph's arithmetic).
2. **Chunk byte size is small either way**: `size_of::<Slot<PartialRenderingCachedData>>() = 96`
   bytes, so a chunk is 24 576 B (`256`) or 98 304 B (`1024`) - both far under glibc's default
   ~128 KB `mmap` threshold. Both `CHUNK_SIZE` values therefore allocate chunks through the same
   `brk`/main-arena path; there is no "one value mmaps, the other doesn't" explanation available
   here.
3. **The arena performs zero inserts/removals during measured steady-state frames**, confirmed
   (not just inferred) via `perf_test`'s existing allocations-per-frame counter: both `CHUNK_SIZE`
   values report identically **301 283 allocations in every "later frame"** on this scene. `scroll`
   is non-virtualized (every row exists in the tree from fill time on), so nothing grows or frees
   the arena once scrolling starts; whatever effect `CHUNK_SIZE` has on frame time has to come from
   the static memory layout already in place (cache/TLB locality of already-resident chunks), not
   from allocation activity during scrolling - this confirms rather than merely reasons out the
   premise of 078's own speculation.
4. **The new scheduling counter shows a strong, `CHUNK_SIZE`-independent correlation between
   involuntary context switches and frame time**, even `taskset -c 2` pinned. Six pinned runs each,
   same command, same core, alternating build:

   | binary | involuntary ctxt switches | later-frames median |
   |---|---|---|
   | 256 | 4 | 131.55 ms |
   | 256 | 6 | 129.04 ms |
   | 1024 | 6 | 128.27 ms |
   | 1024 | 7 | 123.84 ms |
   | 1024 | 7 | 124.94 ms |
   | 256 | 424 | 230.33 ms |
   | 1024 | 542 | 212.00 ms |
   | 1024 | 844 | 226.72 ms |
   | 256 | 916 | 272.40 ms |
   | 256 | 932 | 270.18 ms |
   | 256 | 941 | 269.27 ms |
   | 1024 | 973 | 282.42 ms |

   Sorted by involuntary switches, the split is essentially binary and matches both binaries
   equally: single digits -> ~124-132 ms, hundreds -> ~212-282 ms. Minor page faults stayed flat
   (~89 700-89 820) across every run regardless of pinning or `CHUNK_SIZE` - no signal there.
5. **This session could not reproduce 078's specific paired result.** `tools/paired.sh`
   (`RUNS=9 FRAMES=20`), same scene, same protocol:

   | mode | run | median A->B | pairs |
   |---|---|---|---|
   | single-core (`taskset -c 2`) | 1st | 128.33 -> 126.53 ms (-1.4%) | 6/9 |
   | single-core (`taskset -c 2`) | 2nd | 129.15 -> 125.75 ms (-2.6%) | 6/9 |
   | multi-core (unpinned) | 1st | 121.96 -> 121.12 ms (-0.7%) | 5/9 |
   | multi-core (unpinned) | 2nd | 134.58 -> 137.19 ms (+1.9%) | 6/9 |

   None of the four reruns reached this workspace's 8-9/9 consistency bar, in **either** core
   count - including the multi-core case 078 reported as 9/9 twice. `uptime`'s load average
   fluctuated between ~4 and ~12 across this session's testing window (rising over the course of
   the runs above; `nproc` = 24, so still under the "load > core count" re-baseline trigger, but
   clearly noisier than whatever state 078's session measured in).

## Does it change behavior/output?

- **Bit-exact.** `tools/pixelcheck.sh` (`RUNS=12`), `CHUNK_SIZE = 256` vs. `1024` on `flat`
  scrolling: 247 px same-binary noise floor, 159 px raw difference between binaries, **0 px
  outside the noise floor** - pixel-identical, matching 078's finding on the same scene (which
  used `RUNS=6` and got an inconclusive 8 px outside a 232 px floor at `RUNS=6`; `RUNS=12` here
  resolves it cleanly to 0).
- **Correctness suite, run fresh this candidate** (not just cited from 074/078):
  `cargo test -p i-slint-core --lib`: **307 passed, 0 failed** both before (`master`, no
  `chunked_arena`) and after (patched, `CHUNK_SIZE = 1024`).
  `cargo test -p slint --test partial_renderer`: **24 passed, 0 failed** both before and after.
  No new failures either direction.
- **Is this a breaking change?** No - `CHUNK_SIZE` is a private `const`, same as every other
  candidate in this sweep. The `perf_test` scheduling-counter addition is harness-only (see
  `README.md`), not a change to Slint.

## Why the result looks the way it does

Findings 1-3 rule out three candidate explanations directly (chunk-count-driven allocation-path
difference, an `mmap`-threshold crossing, and per-frame arena allocation activity), narrowing what
could distinguish the two `CHUNK_SIZE` values on this scene to something about the *existing*
memory layout (cache-line/TLB locality across ~400 000 already-resident items spread over either
1563 or 391 heap-allocated chunks) rather than anything actively happening during scrolling. That
part of 078's reasoning stands, corrected for the 4x chunk-count arithmetic error.

Findings 4 and 5 point the other way, though: a context-switch-correlated bimodal frame-time
pattern (essentially the same "two modes with nothing in between" behavior `README.md` already
documented for *any* paired comparison on this machine) shows up **identically for both
`CHUNK_SIZE` values**, even pinned to a single core - meaning at least part of what makes a single
paired-comparison run look "consistent" or not is which side of that host-level scheduling coin
flip each binary happens to land on during its 9 alternating runs, not something intrinsic to
`CHUNK_SIZE`. Combined with this session's inability to reproduce 078's specific 9/9 multi-core
result even once (getting 5/9 and 6/9 instead, on both core counts), the most defensible
conclusion is: **the mechanism 078 speculated (fewer/larger chunks trading allocation cost for
multi-core cache locality) remains unconfirmed - not disproven, but not supported by anything
measured here either**, and the core-count split 078 reported may be partly or wholly attributable
to this session's (and possibly 078's) exposure to the same host-level scheduling noise that
produces the wider "two modes, nothing in between" behavior this whole workspace's `paired.sh`
exists to defend against. Neither `perf` nor a lower-noise host was available to settle which of
these it is.

## What's not done / open questions

- **No cache-miss/TLB-miss counter was obtained** - `perf_event_open` remains blocked in this
  environment (same limitation every candidate in this sweep notes); the scheduling counter added
  here measures OS-level rescheduling/paging, not CPU cache behavior, so it cannot directly confirm
  or refute the "cache locality" mechanism 078 proposed, only rule out the three explanations in
  "Result" items 1-3 and surface the context-switch confound in item 4.
- **078's result was not re-attempted enough times to average out the noise** - this candidate ran
  2 single-core and 2 multi-core `paired.sh` repeats (4 total, vs. 078's 2 multi-core repeats that
  both landed 9/9); a much larger number of repeats (e.g. 10+ at each core count) might still show
  078's multi-core-consistent pattern re-emerge on average even with individual runs landing
  5-6/9 - not attempted here due to budget, and each `RUNS=9 FRAMES=20` repeat already takes
  several minutes.
- **A cleaner test of the noise hypothesis** would isolate the pinned core from other schedulable
  work entirely (`isolcpus`/a dedicated cgroup) rather than relying on `taskset` alone, then see
  whether `paired.sh` becomes decisive (either way) for both `CHUNK_SIZE` values under those
  conditions - not attempted here (would need host-level configuration this environment likely
  disallows, per this workspace's existing note on not escalating privileges unilaterally).
- **078's `RESULT.md` was not edited** - per this skill's rule, a later candidate correcting an
  earlier one's numbers gets a note in the later candidate's own folder (this file) plus a
  `SUMMARY.md` pointer, not a silent edit to 078's original text. The chunk-count arithmetic
  correction (finding 1) belongs to 078's "Why the result looks the way it does" section
  specifically; its "Result" section's percentages and consistency counts are unaffected.
- Per this workspace's standing rule, no `CHUNK_SIZE` value different from 073's original 256 is
  recommended here either - this candidate is a mechanism investigation, not a re-measurement of
  the win/loss itself.
