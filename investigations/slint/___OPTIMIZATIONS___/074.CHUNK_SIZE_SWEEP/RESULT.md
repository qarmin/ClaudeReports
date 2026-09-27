# 74 - Sweep `chunked_arena::ChunkedArena`'s `CHUNK_SIZE` constant (candidate 073's arena)

IDEAS.md's backlog (added while closing out candidate 073) asked whether candidate 073's
`layout` static win (-2.9%/-3.3%) being smaller than candidate 071's diagnostic-only figure for
the same scene (-5.5%/-5.7%) is affected by `CHUNK_SIZE` (fixed at 256, not swept in 073): a
smaller value means more chunks (more `Box` indirections at the outer `Vec` level, worse
locality across the whole scene) but a possibly cache-friendlier per-chunk array; a larger value
means fewer, bigger chunk allocations. This candidate builds four more binaries, identical to
073's except for that one constant, and pairs each against 073's own `CHUNK_SIZE = 256` binary.

`chunked_arena::ChunkedArena` does not exist on `master` - candidate 073's own patch was reverted
per this workspace's rule ("the main project's working tree is always left clean"), so
`patch.diff` here is 073's full patch, unmodified (`CHUNK_SIZE = 256`), applied first as the
prerequisite baseline every swept value is compared against. The four other values tested (32,
64, 512, 1024, 4096 - five, in the end) are each a single-line edit to
`chunked_arena::CHUNK_SIZE` in that already-applied patch; none of them are shipped as a separate
patch.diff since they're one-line variations of the same file `patch.diff` already touches - see
the exact lines below.

## What was tried

Binaries built once per `CHUNK_SIZE` value (`cargo build --bin perf_test --profile rdebug`,
`internal/core/partial_renderer.rs`'s `const CHUNK_SIZE: usize = <value>;`), compared pairwise
against the `256` binary via `tools/paired.sh` (`RUNS=9 FRAMES=20`), same protocol as
candidate 073. Scene: `scroll --count 100000 --row-style layout --scroll-step 0` (the scene
IDEAS.md's entry named), unless noted.

Each row's median pair is (`CHUNK_SIZE = 256` A-side of that specific `paired.sh` invocation) ->
(the swept value's B-side of the same invocation); the 256 side is re-measured per pairing rather
than reused from a single run, per this workspace's paired-comparison discipline.

| Variant | scenario | single-core (`taskset -c 2`) | multi-core | notes |
|---|---|---|---|---|
| `CHUNK_SIZE = 64` | `layout` static | median 147.02 -> 152.43ms (**+3.7%**, 0/9 paired) | median 144.88 -> 151.79ms (**+4.8%**, 0/9 paired) | consistent regression vs. 256, both cores |
| `CHUNK_SIZE = 512` | `layout` static | median 146.25 -> 147.58ms (+0.9%, 2/9); rerun: 146.95 -> 147.43ms (+0.3%, 3/9) | not measured | NOT consistent - small lean toward 256, inside noise |
| `CHUNK_SIZE = 1024` | `layout` static | median 147.26 -> 145.34ms (-1.3%, 7/9); rerun: 147.12 -> 144.21ms (-2.0%, 7/9); rerun 2: 146.46 -> 144.93ms (-1.0%, 6/9) | median 146.13 -> 146.93ms (+0.5%, 4/9) | NOT consistent on either core count - repeatable small negative lean single-core, no lean at all multi-core |
| `CHUNK_SIZE = 4096` | `layout` static | median 148.34 -> 149.02ms (**+0.5%, 8/9 paired**) | median 145.68 -> 146.82ms (+0.8%, 3/9) | consistent small regression single-core; NOT consistent multi-core |
| `CHUNK_SIZE = 64` | `bare` scrolling | median 21.94 -> 25.20ms (+14.9%, 3/9); rerun: 21.88 -> 22.15ms (+1.2%, 4/9) | not measured | NOT consistent either run - `bare` is the bimodal/lottery-mode scene `paired.sh`'s own header warns about; no usable signal from it here |

Correctness only, not timed: `CHUNK_SIZE = 32` (`cargo test -p i-slint-core --lib`: 307/307;
`cargo test -p slint --test partial_renderer`: 24/24 - see "Does it change behavior/output?").

## Result

No value tested beats `CHUNK_SIZE = 256` on the `layout` static scene the way IDEAS.md's entry
was hoping for. `64` is a clear, consistent regression on both core counts (+3.7%/+4.8%, 9/9 and
9/9 paired) - a plausible outcome of nearly 4x as many chunk allocations for the same 100 000-row
scene, each one an extra `Box` indirection layer at the outer `Vec` level. `4096` is a small but
paired-consistent regression single-core (+0.5%, 8/9) and an inconsistent (3/9) wash multi-core.
`512` and `1024` both land inside the noise floor on repeated runs - `1024` leans negative
(faster) single-core across three repeated pairings (-1.3%/-2.0%/-1.0%) but never reaches the
8-or-9-of-9 threshold this workspace's tooling requires to call a result real, and shows no lean
at all multi-core (4/9), so it is reported as a wash, not a win. `bare` scrolling - the scene
where 073 saw its largest win (-7.4%/-15.2%) - could not produce a usable signal for `CHUNK_SIZE`
at all in two attempts; it is the specific bimodal scene `tools/paired.sh`'s own header warns
about ("two performance modes... surviving taskset pinning"), and neither run reached a
consistent verdict in either direction.

Net: 256 is not provably suboptimal by this sweep. The one clean finding is that going
*smaller* (64) is a real regression; going *larger* (1024, 4096) is at worst a small single-core
regression (4096) and at best a same-noise-floor wash (1024) - not the recovery of candidate
071's diagnostic gap that motivated this candidate.

## Does it change behavior/output?

- **Bit-exact.** `tools/pixelcheck.sh` (`RUNS=6`), `256` vs. `64`: 0 px difference outside the
  measured noise floor (244 px noise floor, 206 px raw difference, 0 px outside it) on
  `scroll --row-style layout --scroll-step 0`, and again (181 px noise floor, 156 px raw, 0 px
  outside it) on `scroll --row-style layout --mutate 50 --mutate-mode insert-front` (exercises
  the arena's slot-reuse/free-list path directly, the path most likely to be sensitive to a
  chunk-size change since free-list indices wrap at chunk boundaries).
- `cargo test -p i-slint-core --lib` at `CHUNK_SIZE = 32` (the smallest value tried, to bias
  toward catching an off-by-one at a chunk boundary): 307/307 passed, unchanged from candidate
  073's own report.
- `cargo test -p slint --test partial_renderer` at `CHUNK_SIZE = 32`: 24/24 passed, unchanged.
- **Is this a breaking change?** No - `CHUNK_SIZE` is a private `const` inside a private module
  (`chunked_arena`, itself private to `partial_renderer.rs`), not part of candidate 073's public
  surface (which is unchanged, i.e. still none - see 073's own `RESULT.md`).

## Why the result looks the way it does

Candidate 073 already named the two suspected (not isolated) causes for its own gap versus
candidate 071's diagnostic figure: (a) `get_mut`/`slot_mut`'s division/modulo indexing plus one
extra `Box` indirection per access, and (b) less predictable inlining through the now-generic
`Option<PropertyTracker>` accesses. This sweep speaks only to (a), and only partially confirms
it: `CHUNK_SIZE` is a compile-time constant and, at every value tried here (32, 64, 256, 512,
1024, 4096 - all powers of two), `index / CHUNK_SIZE` and `index % CHUNK_SIZE` compile to a
shift and a mask, not an actual division instruction, so the "division/modulo" framing in 073's
own writeup somewhat overstates that part of the cost for any power-of-two chunk size (this
sweep did not test a non-power-of-two value, which would pay a real division). What chunk size
*does* change is how many chunks a 100 000-row scene needs (measured cache-hit ratio for the
`chunks` outer `Vec` element access) and how large each chunk's own `Box::new(core::array::from_fn(...))`
allocation is - `64` needs ~1560 chunks for 100 000 rows (mostly resident, so the outer `Vec`
itself grows to non-trivial size and its own elements may not stay resident in cache), while
`4096` needs only ~25 chunks (fewer, larger allocations, worse for the allocator, but that cost
is paid once at startup/scroll-extension, not per-frame the way `get_mut`'s indexing is). The
direction of the `64` regression (more, smaller chunks hurt) is consistent with an outer-`Vec`
cache-locality cost dominating over any per-access indexing cost; the `4096` result (also a
regression, just far smaller) suggests the fewer-chunks direction is not free either, plausibly
because each `Box<[Slot<T>; 4096]>` chunk is now ~145 KB (at this candidate's `Slot<T>` size),
large enough that touching a chunk for the first time after `grow()` brings in far more of it
than a single frame's dirty-row set needs. Neither of these was isolated with a profiler
(`perf_event_open` remains blocked in this environment, same limitation 073 itself noted) - this
is reasoned from the shape of the results, not measured allocator/cache behavior directly.

## What's not done / open questions

- No non-power-of-two value was tested (e.g. 200, 300) - would isolate whether a real
  division/modulo instruction (rather than the shift/mask a power-of-two constant compiles to)
  changes the per-access cost 073's writeup attributed to "division/modulo", since every value
  actually tried here sidesteps that cost by construction.
- `layout`/`flat` scrolling (073's own wash scenes) and `rects`/`list` scenes were not swept -
  only `layout` static (the scene IDEAS.md's entry named) and one `bare` scrolling attempt (which
  produced no usable signal) were measured.
- No `perf`/allocation-profiler attribution was attempted for any swept value, same limitation as
  073 - the cache-locality/allocation-size reasoning above is inferred from the pattern of
  results, not confirmed by a trace.
- Only `taskset -c 2` (single-core) and unpinned multi-core wall-clock were measured, on one
  24-core machine; no attempt to correlate with L1/L2/LLC size to explain the `64`-chunk
  regression's magnitude more precisely.
- Because none of the swept values showed a decisive win, this candidate does not produce a
  "recommended" `CHUNK_SIZE` different from 073's original 256 - see "Result" above for why 256
  is not provably suboptimal, only 64 provably worse.
