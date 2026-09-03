# Profile candidate 007/012/018's `resolved_font_cache` fix on `cache_defeating.pdf` with callgrind

Backlog entry from `../IDEAS.md`: candidate 018 measured a small (~1-3%), noise-comparable wall-
clock *increase* on `cache_defeating.pdf` (326 `Tf` calls, described as guaranteeing a cache miss
on every single call) for the identical `resolved_font_cache` patch that won -16% to -18% on
`img.pdf`. `perf` remains unavailable in this sandbox (not installed), but `valgrind`/`callgrind`
**is** available here (a change since the README's "Instrumentation added" section was written -
worth updating) and gives deterministic per-run instruction counts (`Ir`), sidestepping the
wall-clock noise floor entirely. `patch.diff` in this folder is byte-for-byte identical to
candidates 007/012/018.

## What was tried

No new source code was written for the fix itself. A new standalone probe binary
(`bench/src/main.rs`) renders `cache_defeating.pdf`'s first page 5 times (no warmup needed -
callgrind's `Ir` count is exactly reproducible run to run for the same binary/input, unlike wall
time) and is run under `valgrind --tool=callgrind` for the unpatched (baseline) and patched
`hayro-interpret`, built against the real working tree at commit `5a5f0e24` (this candidate's
`base_commit`). `callgrind_annotate --auto=yes` against `hayro-interpret/src/interpret/mod.rs`
then isolates the exact instruction cost of the `TypedInstruction::TextFont` match arm and its
callees (`Context::resolve_font`, `Resources::get_font`) for each build.

**Sandbox gotcha found while setting this up**: `valgrind --tool=callgrind` fails outright
(`Private file creation failed`) under this container's default `ulimit -n`
(2,147,483,584 - functionally unlimited). Lowering it (`ulimit -n 4096`) in the same shell before
invoking `valgrind` fixes it; not previously documented anywhere in this workspace or
`COMMANDS.md` since `perf` was the only profiler considered available until now.

| Variant | total `Ir` (5 renders) | `Ir`/render | `resolve_font`+`get_font` calls (5 renders) | that subtree's `Ir` (5 renders) | pixmap hash |
|---|---|---|---|---|---|
| baseline | 248,307,327 | 49,661,465 | 1,630 (every `Tf` call) | 1,066,763 | `0x31cced99a6bfe199` |
| patched | 247,128,891 | 49,425,778 | 40 (once per distinct resource name) | 404,393 | `0x31cced99a6bfe199` |
| **delta** | **-1,178,436 (-0.47%)** | **-235,687/render** | **-1,590 calls** | **-662,370 (-62.1%)** | identical |

Raw callgrind output (`callgrind.baseline.out`, `callgrind.patched.out`) and the full
`callgrind_annotate --auto=yes` dumps (`interpret_mod_annotated.{baseline,patched}.txt`) are kept
in this folder's `callgrind/` subdirectory for inspection.

## Result

The patch is **not** a regression on `cache_defeating.pdf` under callgrind's deterministic
instruction count - it is a small, unambiguous **win**: -0.47% total instructions, -235,687
instructions/render. The mechanism is exactly what candidate 007 originally intended: baseline
calls `Context::resolve_font()` on **every single** `Tf` operator (1,630 calls across 5 renders,
one per `Tf` op, regardless of whether `font_dict_cache` already has the `Dict` cached -
`font_dict_cache` only ever caches the `Dict` lookup, not the resolved `TextStateFont`), costing
1,066,763 instructions total for that subtree. The patch's `resolved_font_cache` cuts this to 40
calls total (once per *distinct* resource name actually seen), costing only 404,393 instructions -
a 62.1% reduction in the font-resolution subtree alone, which fully explains (and then some) the
small net win in total `Ir`.

This directly overturns candidate 018's characterization of `cache_defeating.pdf` as this cache's
worst case. See "Why the result looks the way it does" below for the specific reasoning error.

## Does it change behavior/output?

- **Bit-exact?** Yes. Both variants' 5-iteration runs (and the earlier 1-iteration smoke check
  before profiling) produced the identical pixmap hash, matching candidate 018's own recorded
  hash for this same file (`0x31cced99a6bfe199`).
- Not re-run against the project test suite in this candidate - the patch is byte-identical to
  007/012/018/019, all of which already confirmed an identical failing-test-name set before/after
  (all failures are pre-existing missing-test-asset environment issues, unrelated to this patch).
  No reason to expect that to differ here since no source changed beyond what those candidates
  already covered.
- **Is this a breaking change?** No - same patch as 007/012/018/019, internal to
  `hayro-interpret::interpret()`, no public API change.

## Why the result looks the way it does

Candidate 018's synthetic PDF generator (`testdata/gen_cache_defeating_pdf.py`, reused verbatim
here) round-robins through 8 distinct standard fonts by **reusing 8 fixed resource names**
(`F0`..`F7`, `f"F{i % 8}"`) across all 326 `Tf` calls. `resolved_font_cache` (like the
pre-existing `font_dict_cache`) is keyed by **resource name**, and persists for the whole content
stream (not just across one `Tf` call to the next) - so despite no two *consecutive* calls
sharing a font, every resource name is seen 40+ times over the page (326/8 ≈ 40.75), and
`resolved_font_cache` hits on all but the first occurrence of each of the 8 names. Confirmed
directly: the callgrind trace shows exactly 40 real `Context::resolve_font()`/`Resources::get_font()`
calls across 5 renders (8 per render - one per distinct name), not 1,630 (one per `Tf` call).

Candidate 018's description - "no two consecutive `Tf` calls ever reference the same font... the
worst case for a resource-name-keyed cache, which therefore misses on every single call" -
conflated "no immediate repeat" with "always a cache miss." Those are only equivalent for a
cache with capacity 1 (an LRU-of-the-single-previous-call), not for a `HashMap` that persists and
accumulates every distinct key seen across the whole content stream. `cache_defeating.pdf`, as
actually built, is close to a **best** case for this cache (8 misses, 318 hits out of 326 calls),
not a worst case - which also explains why candidate 018's own wall-clock bench saw no clear
regression: there wasn't one to find, and if anything the small `Ir`-level win (-0.47%, ~236k
instructions out of ~49.6M/render) is well below what a ~3.1-3.3ms wall-clock render could resolve
above its own ~1-2% noise floor, consistent with 018's own "not distinguishable from noise"
framing (it just had the reason backwards).

The genuinely worst case for this cache - a page where every `Tf` call references a **distinct**
resource name that's never reused for the entire content stream (e.g. hundreds of individually-
named font resources, all pointing at valid dictionaries so the miss path's full `resolve_font()`
cost is paid every time) - was not built or tested here; see "What's not done" below.

## What's not done / open questions

- A genuinely cache-hostile PDF (every `Tf` call uses a resource name never reused anywhere else
  on the page, all resolving successfully so the full `resolve_font()` path runs every time,
  matching the *worst case* the patch's own doc comment in `interpret/mod.rs` anticipates: a
  nested Form XObject with its own fresh `Resources`/cache) was not built. That would be the
  actual worst case for `resolved_font_cache`'s extra lookup-then-insert (paid on every call, with
  the `Option<TextStateFont>` clone on the hit path never actually exercised) - added to the
  backlog below.
- Only `Ir` (instruction count) was profiled, not cache-miss/branch-misprediction event counters
  (`--cache-sim=yes`/`--branch-sim=yes`) - would add wall-clock-relevant detail (e.g. whether the
  extra hashmap ever causes a cold cache line) but `Ir` alone already answers the specific
  question this candidate was queued to answer (is there a real regression here at all).
- The `ulimit -n` fix for `valgrind` in this sandbox is only recorded in this `RESULT.md` and the
  candidate's own `justfile` - worth promoting to `COMMANDS.md`/`README.md` if `valgrind` gets
  used again in a future session (see `IDEAS.md` note added).
