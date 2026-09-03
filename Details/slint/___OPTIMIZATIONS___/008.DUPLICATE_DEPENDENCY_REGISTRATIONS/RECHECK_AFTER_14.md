# Is candidate 008 still worth it now that 14 pools the nodes?

Candidate 008 skips a *duplicate* dependency registration. Candidate 014 makes a registration cheap
by reusing a pooled node instead of allocating one. They are in the same file and they interact by
construction: 08's value is an allocation avoided, and 14 removes the allocation.

Re-measured with `tools/isolate.py`, which turns exactly one of them off. Both binaries built in the
same session, and each was checked to be genuinely different (allocation counts below) before its
row was read.

## Allocations per frame

| | `rects` 3 000 | `scroll` 100 000 |
| --- | --- | --- |
| both applied | 14 | 182 |
| 08 off, 14 on | 14 | 357 |
| 08 on, 14 off | 26 637 | 300 837 |
| both off | 26 727 | 701 187 |

08 alone (bottom two rows) is the **-57% on `scroll`** it was originally credited with. With 14
applied, the same comparison is 182 against 357 - still half, but of a number that no longer
matters: the duplicates now come from the pool rather than the allocator.

## Frame time, paired and interleaved

The unpaired medians disagreed with each other, so both were re-run as **interleaved A/B pairs**
(one run of each, alternating) to cancel machine drift:

| | 08 on | 08 off | 08 on won |
| --- | --- | --- | --- |
| `rects` 3 000, 12 pairs | 2.57 ms | 2.54 ms | **3 of 12** |
| `scroll` 100 000, 6 pairs | **116.87 ms** | 132.39 ms | **6 of 6** |

So, with 14 applied:

- On `scroll`, 08 is still worth **-12%**, unambiguously (every pair).
- On `rects` it is now a **~1% loss** (0.03 ms, losing 9 of 12 pairs) - the `front_is` check costs a
  dereference per registration and the allocation it avoids has become nearly free.

**Verdict: keep it.** -12% on the scene that needs it against ~1% on the scene that does not is a
clear trade. But its headline number is no longer -57% allocations; that figure belongs to a tree
without 14.

## Why this check was worth running

An earlier fix's value can be *consumed* by a later one, and nothing about the earlier fix changes
when that happens - its own write-up keeps quoting a number that is no longer reachable. The only
way to notice is to re-measure the earlier fix in isolation after the later one lands.

Two measurement notes from doing it:

- The first unpaired run showed `rects` at 3.04 ms with 08 on against 2.50 off, which looked like a
  20% pessimization. It was the **first run of the batch warming up** (3.24 ms, against 2.5-2.8 for
  every run after). Interleaving the two binaries made it 2.57 vs 2.54.
- `tools/isolate.py` restored two fixes in the *same file* in name order, which put back an
  intermediate state instead of the original and silently poisoned the tree for the next build. It
  now numbers the backups by application order and restores in reverse, with an md5 check after.
