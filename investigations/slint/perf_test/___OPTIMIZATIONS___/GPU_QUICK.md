
## Run 2026-08-01 07:17

| scene | frame time | allocations/frame | window |
| --- | --- | --- | --- |
| `rects 3000` | 46.95ms | 35320 | 1280x860 physical, scale factor 1 |
| `rects 3000 --gradient` | 49.05ms | 36431 | 1280x860 physical, scale factor 1 |
| `list 100000` | 4.85ms | 2052 | 1280x860 physical, scale factor 1 |
| `scroll 100000` | 51.67ms | 301974 | 1280x860 physical, scale factor 1 |
| `scroll 100000 --row-style bare` | 12.64ms | 201018 | 1280x860 physical, scale factor 1 |

`scroll` vs `scroll --row-style bare` is the prize still on the table: pruning the
children of culled items (candidate 12). Everything else in this table is the current
state of the tree - compare against the previous run above.

## Run 2026-08-01 09:30

| scene | frame time | allocations/frame | window |
| --- | --- | --- | --- |
| `rects 3000` | 5.49ms | 35164 | 1280x860 physical, scale factor 1 |
| `rects 3000 --gradient` | 7.44ms | 36267 | 1280x860 physical, scale factor 1 |
| `list 100000` | 1.46ms | 2039 | 1280x860 physical, scale factor 1 |
| `scroll 100000` | 50.38ms | 301961 | 1280x860 physical, scale factor 1 |
| `scroll 100000 --row-style bare` | 10.70ms | 201014 | 1280x860 physical, scale factor 1 |

`scroll` vs `scroll --row-style bare` is the prize still on the table: pruning the
children of culled items (candidate 12). Everything else in this table is the current
state of the tree - compare against the previous run above.

## Reference baseline (user's GPU, window pinned)

The run above is the first one with the window pinned, so it is the baseline future runs compare
against. All five rows report `1280x860 physical, scale factor 1`, which is what makes them
comparable at all.

| scene | frame time | allocations/frame |
| --- | --- | --- |
| `rects` 3 000 | 5.49 ms | 35 164 |
| `rects` 3 000 `--gradient` | 7.44 ms | 36 267 |
| `list` 100 000 | 1.46 ms | 2 039 |
| `scroll` 100 000 | 50.38 ms | 301 961 |
| `scroll` 100 000 `--row-style bare` | **10.70 ms** | 201 014 |

`scroll` vs `bare` = **4.7x** still on the table (candidate 12: prune the children of culled
items). The other four rows are the current tree; a future run that moves any of them by more than
a few percent means something changed.

## Run 2026-08-01 12:08

| scene | frame time | allocations/frame | window |
| --- | --- | --- | --- |
| `rects 3000` | 5.67ms | 32973 | 1280x860 physical, scale factor 1 |
| `rects 3000 --gradient` | 6.46ms | 34075 | 1280x860 physical, scale factor 1 |
| `list 100000` | 1.70ms | 2082 | 1280x860 physical, scale factor 1 |
| `scroll 100000` | 51.24ms | 302009 | 1280x860 physical, scale factor 1 |
| `scroll 100000 --row-style bare` | 12.27ms | 200993 | 1280x860 physical, scale factor 1 |

`scroll` vs `scroll --row-style bare` is the prize still on the table: pruning the
children of culled items (candidate 12). Everything else in this table is the current
state of the tree - compare against the previous run above.

## Run 2026-08-01 12:08

| scene | frame time | allocations/frame | window |
| --- | --- | --- | --- |
| `rects 3000` | 5.58ms | 32973 | 1280x860 physical, scale factor 1 |
| `rects 3000 --gradient` | 6.16ms | 34075 | 1280x860 physical, scale factor 1 |
| `list 100000` | 1.59ms | 2082 | 1280x860 physical, scale factor 1 |
| `scroll 100000` | 48.19ms | 302009 | 1280x860 physical, scale factor 1 |
| `scroll 100000 --row-style bare` | 12.89ms | 200993 | 1280x860 physical, scale factor 1 |

`scroll` vs `scroll --row-style bare` is the prize still on the table: pruning the
children of culled items (candidate 12). Everything else in this table is the current
state of the tree - compare against the previous run above.

## Open question from the 2026-08-01 12:08 runs

The two 12:08 runs are the first with candidate 03 applied, and they are byte-identical to each
other in the allocations column, so the tree is deterministic. Against the reference baseline above
they show:

| scene | reference | 12:08 | delta |
| --- | --- | --- | --- |
| `rects 3000` | 35 164 | 32 973 | **-2 191** |
| `rects 3000 --gradient` | 36 267 | 34 075 | **-2 192** |
| `list 100000` | 2 039 | 2 082 | +43 |
| `scroll 100000` | 301 961 | 302 009 | +48 |
| `scroll --row-style bare` | 201 014 | 200 993 | -21 |

Allocation counts are deterministic and hardware independent, so -2 191 on a scene with one text
item is a real code difference, not noise - and candidate 03 does not obviously explain it. On the
software renderer `rects 3000` is 18 414 allocations per frame both before and after the change, so
whatever moved is on the FemtoVG path. The small `+43`/`+48` are consistent with the extra `Box`
per shaped paragraph; the `-2 191` is not.

**Answered, and the answer is that the question was wrong.** `just gpu-03` ran both binaries in
one session with a same-binary control row: `rects 3000` is 36 164 (pristine) against 36 171
(session), **+7**, and the control reproduces 36 171 exactly. Candidate 03 does not move that
column.

The same session binary produced 29 972 in one invocation and 36 171 in another, while being
exactly reproducible *within* each. So allocation counts on this harness are deterministic within
a run and not across runs - which makes every cross-run comparison in the tables above, including
against the "reference baseline", meaningless for attribution. Only a pristine-vs-session pair
measured in one invocation says anything. See `GPU_03.md`.
