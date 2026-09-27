# Gradient ramp cache: `compact_luts` prefix-sum underflow (`lut_start -= prefix_sum[pos]`)

`sparse_strips/vello_hybrid/src/gradient_cache.rs`'s `GradientRampCache::compact_luts` (line 238)
adjusts each surviving cache entry's `lut_start` (a `u32` texel offset into the packed gradient
LUT texture) with plain subtraction: `ramp.lut_start -= prefix_sum[pos]`, no `checked_sub`/assert.
This looked suspicious because it relies on an invariant (every surviving entry's `lut_start` is at
least as large as the cumulative width of every *removed* entry that precedes it in the packed
layout) that isn't checked anywhere, combined with `remove_lru_entries`'s use of
`select_nth_unstable_by_key` on `last_used` timestamps, whose docs explicitly call out unspecified
ordering among ties - the suspicion was that a churny/adversarial eviction pattern with tied
timestamps could pick an LRU set inconsistent with the packed-layout invariant and underflow the
subtraction (a debug-build panic; a garbage `lut_start`/wrong texture offset in release).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading / invariant proof | Traced every writer of `last_used` (`get_or_create_ramp`, the only mutator) and every writer of `lut_start` (append-only at `self.luts.len()`, and `compact_luts` itself) | `self.epoch` is incremented exactly once per `get_or_create_ramp` call (hit or miss) and the resulting value is assigned to at most one entry's `last_used` per call - so **no two entries can ever have equal `last_used`** through the real API; the "unspecified tie-breaking" concern cannot actually manifest here (`select_nth_unstable_by_key` still picks an unspecified *order* among ties, but there are structurally no ties to have). Separately, `lut_start` values are unique, strictly increasing with insertion order (new entries always start at the current `self.luts.len()`), and `determine_lut_size` (`vello_common/src/encode.rs`) never returns a LUT width of 0 (minimum 256 texels for any non-empty gradient), so entries can never share a `lut_start`. Given these two facts, the packed-layout invariant `compact_luts` depends on (surviving+removed entries' `[lut_start, lut_start+width)` ranges exactly and non-overlappingly partition `[0, luts.len()/4)` in `lut_start` order) holds by induction across every insert/evict cycle, which makes `prefix_sum[pos] <= ramp.lut_start` always true (a subset-sum of preceding removed widths can't exceed the total preceding width, which by the invariant equals `ramp.lut_start` exactly). |
| Randomized property-based stress test (`fuzz_compact_luts_invariants`, `repro/harness.patch`) | Debug build (`cargo test`, `debug_assertions` on by default), dependency-free xorshift64* PRNG (no `rand` crate added), 40 seeds x 300 insert steps each (12,000 insertions total) with `retained_count` randomized in `1..=6`, gradient stop count randomized in `2..=6`, and 1-in-4 insertions using a deliberately tight stop spacing (offset `0.0005`) to push `determine_lut_size` up to much larger LUT widths (up to `MAX_GRADIENT_LUT_SIZE = 4096` texels instead of the default 256/512/1024), `maintain()` called on ~1/3 of steps; after every `maintain()` call, asserted the packed-layout invariant directly (no gaps/overlaps, total bytes match `luts.len()`) | 0 panics, 0 invariant violations across all 40 seeds. |
| Steady-churn boundary-value stress test (`fuzz_compact_luts_steady_churn`, same patch) | Same debug build; `retained_count` fixed at the tightest possible boundary values (`0`, `1`, `2`) x 10 seeds x 300 steps, with `maintain()` called after **every single insertion** (so `evict(1)` - or a full evict for `retained_count = 0` - fires on essentially every step once the cache reaches its cap), same invariant check after every `maintain()` | 0 panics, 0 invariant violations across all 3 x 10 = 30 runs (9,000 insert+evict cycles at the tightest boundary). |
| `cargo +nightly miri test -p vello_hybrid --lib gradient_cache` | Attempted as a third tool per the escalation cap; `timeout 300` | Did not complete within 300s - miri was still building `vello_hybrid`'s full dependency graph (this crate pulls in `wgpu`/`wgpu-hal`/`naga` etc. even for a lib-only test filter) when the timeout fired; never reached running the actual test. Not pursued further: this bug class (a plain `u32` arithmetic underflow) is exactly the case `HEURISTICS.md` says a debug-assertions build + boundary values already covers for free, and the two runs above already exercise that build/config extensively; miri's own strength (real UB in `unsafe`/FFI) isn't what this candidate is about. |

## Status

**Not reproducible.** Covered: the invariant proof above (which structurally rules out the ties
concern that motivated this candidate), plus 12,000 randomized insert/evict cycles across varied
`retained_count` (1-6) and LUT widths (256-4096 texels), plus 9,000 steady-churn cycles at the
tightest boundary `retained_count` values (0, 1, 2) with `maintain()` on every step - all under a
debug build with `debug_assertions` on, which would turn any actual `u32` underflow into an
immediate panic. Not covered: a fixed-seed adversarial search (e.g. a dedicated `cargo-fuzz`
target) beyond the 40+30=70 pseudo-random seeds tried by hand above; see "What's not done" below.

**Fully reproducible**: N/A - no bug was found by this candidate to reproduce; see Status.

## Class

N/A (not Confirmed - see Status).

## Root cause

Not a bug in the sense originally suspected. The reason the suspected underflow can't occur:
`last_used` values are provably unique per cache instance (see the invariant-proof row above), so
`select_nth_unstable_by_key`'s "unspecified order among ties" concern never actually applies to
this cache's real usage - there are never any ties to break. Independently, the packed-LUT layout
invariant `compact_luts` relies on (`prefix_sum[pos] <= ramp.lut_start` for every surviving entry)
is maintained correctly by construction: new entries are always appended at the current end of
`self.luts` (so `lut_start` order matches insertion order with no possible duplicates, since
`determine_lut_size` never returns 0), and `compact_luts` recomputes `lut_start` for every
survivor using a prefix sum over the *sorted* set of removed ramps' widths, which by that same
invariant can never sum to more than the survivor's own offset.

## Introduced by

N/A - not applicable, since Status is Not reproducible (no bug found to trace to an introducing
commit). For context: the current `compact_luts`/`select_nth_unstable_by_key`-based implementation
was introduced by baae4b9bdfe74ec0d481b33e13cbc26c06bca519 "vello_hybrid: Fix slow scenes with many
changing gradients (#1496)" (2026-03-16), which replaced an earlier O(n^2) `sort_by_key`-based
compaction with the current prefix-sum approach - that commit is where the specific code pattern
this candidate suspected originates, even though no bug was found in it.

## The fix

No fix attempted - no bug was found to fix (see Status/Root cause). No source files in the main
project were modified during this investigation; the two stress tests were only ever added inside
an isolated git worktree (`/tmp/vello-058-worktree`, discarded after use) and in `repro/`'s own
patch, never in the main project's working tree.

- Project test suite: `cargo test -p vello_hybrid --lib` (with the two stress tests added) ->
  124 passed, 0 failed, 0 ignored (122 pre-existing tests + the 2 added here). No change was made
  to the pristine tree, so there is no meaningful "before" run distinct from this.

## What's not done / open questions

- A dedicated, longer `cargo-fuzz` campaign against `compact_luts`/`remove_lru_entries` directly
  (rather than this session's ~70 pseudo-random seeds run by hand) is out of scope for this skill
  (see `fuzz-bug-repro` for that) but would give more confidence than a bounded property test,
  especially if the invariant proof above turns out to have a hole this session didn't spot.
- The invariant proof depends on `EncodedGradient::u8_lut`/`determine_lut_size` never returning a
  LUT width of 0; this was checked by reading `vello_common/src/encode.rs`'s current
  implementation, not proven exhaustively for every possible future change to gradient encoding -
  if that function's minimum-width guarantee is ever relaxed, this invariant (and this candidate's
  "Not reproducible" conclusion) would need re-checking.
- `miri` was not actually run to completion (see the table above) - if a future session has more
  time budget or narrows the crate's dependency surface (e.g. a `--no-default-features` build
  without `wgpu`) enough for `cargo +nightly miri test` to finish, it would be worth letting it run
  to completion, though it's not expected to add much for this specific bug class (plain integer
  arithmetic, no `unsafe`/UB involved in the suspected code path itself - the module's only
  `unsafe` is the unrelated `reuse_vec` lifetime-reinterpretation helper).
