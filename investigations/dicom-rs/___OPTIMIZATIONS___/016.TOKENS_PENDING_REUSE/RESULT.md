# InMemObjectTokens: reuse tokens_pending's VecDeque allocation instead of collecting a fresh one per element

A fresh discovery this session, found while re-reading a callgrind profile of `write-only` on
`seq_10k.dcm` (candidate 005's text-heavy synthetic fixture) for unrelated reasons.
`InMemObjectTokens::next()` (`object/src/tokens.rs`) does
`self.tokens_pending = elem.into_tokens().collect();` every time it moves to a new element -
dropping the (now-empty, since it was just drained via `pop_front`) previous `VecDeque` and
`collect()`-ing a brand new one, once per element in the whole object. For an object with many
elements (10,000 sequence items in this fixture), that's 10,000+ fresh small-`VecDeque`
allocations per `write_all` call.

## What was tried

Changed `self.tokens_pending = ....collect();` to `self.tokens_pending.extend(...)` (after
confirming via a `debug_assert!` that it's already empty at that point, which it always is - the
only way to reach that branch is after `pop_front()` returned `None`) - reusing the existing
`VecDeque`'s allocated capacity instead of dropping it and allocating a new one, since consecutive
elements typically produce a similar, small number of tokens (2, usually: header + value).

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s `write-only`/`roundtrip`/`parse` modes
against `seq_10k.dcm` (10,000-element object, chosen specifically because this pattern's benefit
scales with element count) and `color3d_jpeg_baseline.dcm` (far fewer, larger elements, as a
"should show little/no effect" check).

## Result

**Mixed and, on balance, not a clean improvement - flagged here rather than shipped.**
`write-only` on `seq_10k.dcm`: round 1 showed a real-looking win (-3.1%, 8/9, consistent), but
round 2 was NOT consistent (7/9, -8.5% median but not reliable per this workspace's own pairing
discipline) - already a yellow flag on its own. Far more seriously, **`roundtrip` (parse then
write, every iteration) showed a large, reproducible REGRESSION: +33.7%/+36.5% across two
rounds, 0/9 both times** - `parse` alone is unaffected (-0.9%, technically "consistent" per the
`wins<=1` rule but small enough to be noise, as expected since this fix touches only the write
path). `write-only` on `color3d_jpeg_baseline.dcm` (few large elements): no consistent change
(-0.1%, 4/9), as expected. Bit-exact in all cases tested.

This is the **third** instance this session of a real, reproducible effect that only shows up in a
*combined* parse-then-write loop and not in an isolated write-repeated-on-a-held-object loop (the
first two: candidate 001's original allocator-arena puzzle, and candidate 005's `write-only`-
specific text-codec regression) - unlike those two, here the anomaly runs the *other* direction
(`roundtrip` is worse, not `write-only`), which is itself a notable, if unexplained, data point.

## Does it change behavior/output?

- **Bit-exact?** Yes, in every mode/fixture tested - `sha256sum`/checksums identical between
  baseline and patched builds.
- **Is this a breaking change?** No public API/type changes - `InMemObjectTokens::next` is a
  private implementation detail.
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures.

## Why the result looks the way it does

Not root-caused (bounded time, per SKILL.md's guidance on regressions - this is the third such
allocator-adjacent mystery encountered this session, and each individual one already consumed a
real, bounded investigation slot; a fourth deep-dive was not attempted given diminishing returns).
The change is algorithmically sound and should, in isolation, only ever reduce allocation count
(reusing capacity is never worse than reallocating fresh, for a same-or-similar-sized refill) - the
regression must come from a second-order effect (allocator arena/locality interaction with the
*specific* combination of repeated parse+write cycles) rather than the change's own direct
algorithmic cost, but this is inference from the pattern, not a confirmed mechanism.

## Peak memory follow-up (2026-08-26)

Requested as a backlog item ("measure peak RSS for candidates 001/008/016, only ever timed
before"), using the same `write-only-peak-mem` (`/proc/self/status` `VmHWM`) technique as
candidates 001/008's own follow-ups. **No meaningful difference**: baseline ~37.65 MB avg (3 runs)
vs. patched ~37.92 MB avg (3 runs) on `seq_10k.dcm`, well within run-to-run noise. This is the
expected result, not a surprise: this fix's mechanism is allocation *count* (reusing one
`VecDeque`'s capacity instead of allocating 10,000 fresh tiny ones), not allocated *byte volume* -
each individual `tokens_pending` buffer holds only 2-3 `DataToken`s at a time regardless of
whether it's freshly allocated or reused, so peak RSS was never going to move here. Allocation
*count* itself (which would be the metric that actually reflects this fix's mechanism) was not
measured in either the original candidate or this follow-up - would need a custom allocator
counter, not just `VmHWM`, left as a further follow-up if wanted. Bit-exact (checksum identical to
baseline), consistent with the original candidate's own correctness check.

## Allocation-count follow-up (2026-08-26)

Closes the open item directly above: built a `diag-alloccount` cargo feature for
`bench_pipeline` (a `#[global_allocator]` wrapping `System`, counting every `alloc`/`dealloc` call
via atomics - see `README.md`'s "Instrumentation added" section) and a new `write-dataset-
alloc-count <file> <iters>` mode (same call as `write-dataset-only`, but counts allocations across
all iterations instead of timing them). Measured this candidate's own patch directly:

| Fixture | baseline allocs/iter | patched allocs/iter | reduction |
|---|---|---|---|
| `seq_10k.dcm` (10,000 SQ items) | 90,020 | 80,020 | -10,000 (-11.1%) |
| `liver.dcm` (small, few top-level elements) | 560 | 423 | -137 (-24.5%) |

**The reduction on `seq_10k.dcm` is exactly 10,000 - one avoided `VecDeque` allocation per SQ
item, precisely matching this candidate's own original description of its mechanism ("10,000
fresh tiny `VecDeque`s -> 1 reused one") with no slack.** This directly confirms, for the first
time with a real count rather than an inference from reading the code, that the fix does exactly
what it was designed to do at the allocation-count level - consistent with, and now quantifying,
both the "algorithmically sound, should only reduce allocation count" reasoning in the main
regression discussion above and the peak-RSS follow-up's prediction that this fix's effect is
about *count*, not byte volume (RSS stayed flat because each avoided allocation was tiny; the
*count* of avoided allocations was real and large). Checksum identical between baseline and patched
on both fixtures, consistent with every other correctness check in this candidate.

## What's not done / open questions

- **Root cause of the `roundtrip`-specific regression is unconfirmed** - the single most important
  open item. A `valgrind --tool=massif` allocation-histogram comparison between baseline and
  patched, run specifically under `roundtrip`'s exact call pattern, would be the natural next step
  (not attempted this candidate, given time already spent on two similar mysteries earlier in this
  session - see candidates 001 and 005).
- Given the large, reproducible `roundtrip` regression clearly outweighs the small and
  inconsistent `write-only` signal, **this patch is not a good trade as currently written** - stated
  as a fact for the reader to weigh, not a recommendation, per this skill's "no verdict" discipline.
- Whether a more conservative version (e.g. only reuse the allocation if the previous element's
  token count was within some small bound of what's needed next, falling back to a fresh
  `VecDeque` otherwise) would avoid the regression while keeping any real benefit was not
  attempted.

## Follow-up: the `roundtrip` regression is glibc-allocator-specific, not fundamental

Same investigation as candidate 005's own follow-up (see that file) - rebuilt the harness with the
`diag-mimalloc` feature (swaps the global allocator to `mimalloc`) and re-ran the exact same
`roundtrip` comparison on `seq_10k.dcm`.

**With mimalloc, the `roundtrip` regression disappears entirely**: -1.5% (9/9, consistent), -1.1%
(7/9, not consistent), -2.2% (8/9, consistent) across three rounds - never a regression, sometimes
a small real win, sometimes noise-level. This confirms the same conclusion as candidate 005: the
change is algorithmically sound (it can only ever reduce allocation count, never increase it), and
the large, reproducible regression measured under glibc's default allocator is an allocator-specific
pathology with this exact repeated-small-VecDeque-reuse pattern, not a flaw in the optimization
itself.

**Revised bottom line**: under glibc's default allocator (this box, and the Linux default
generally), this patch is a bad trade for `roundtrip`-shaped workloads, as originally found. Under
a non-default allocator such as `mimalloc`/`jemalloc`, it is neutral-to-slightly-positive with no
regression in any mode tested. Same "not something dicom-rs itself can fix, information for the
reader" framing as candidate 005 - stated as fact, not a recommendation either way.
