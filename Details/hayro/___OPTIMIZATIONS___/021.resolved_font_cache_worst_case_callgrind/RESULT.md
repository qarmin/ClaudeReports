# Profile candidate 007/012/018/020's `resolved_font_cache` fix on a genuinely cache-hostile PDF

Backlog entry from `../IDEAS.md`: candidate 020 found candidate 018's `cache_defeating.pdf` (326
`Tf` calls cycling **8 reused** resource names `F0`-`F7`) is actually close to a **best** case for
`resolved_font_cache` (a `HashMap` keyed by resource name, persisted per content stream) - 8
misses, 318 hits - not the worst case 018 assumed. This candidate builds the actual worst case: a
PDF where every `Tf` call uses a **distinct** resource name, never reused anywhere else on the
page, so the cache can never hit even once. Same patch as 007/012/018/019/020, byte-identical
`patch.diff`.

`rustc 1.98.0 (88d9e12ae 2026-08-18)` / `cargo 1.98.0` (recorded per candidate 016's backlog note).

## What was tried

A new synthetic PDF, `testdata/gen_worst_case_pdf.py` (adapted from candidate 020's
`gen_cache_defeating_pdf.py`), generates `worst_case_cache.pdf`: 326 `Tf` calls, each with its own
resource name `F0`..`F325` (never repeated), each pointing at one of only 8 underlying Type1 font
objects (cycled, just to keep them valid fonts - the underlying object doesn't matter, only the
resource *name* uniqueness does, since both `font_dict_cache` and `resolved_font_cache` are keyed
by name). This guarantees every single `Tf` call takes the full miss path on both caches.

Reused candidate 020's exact methodology: a standalone probe (`bench/src/main.rs`, unchanged
from 020 except for the input file) renders the page 5 times under `valgrind --tool=callgrind`
(`ulimit -n 4096` workaround applied), for the unpatched (baseline) and patched
`hayro-interpret`, built against the real working tree at commit `5a5f0e24` (this candidate's
`base_commit`). Both variants' `Ir` counts were confirmed **exactly reproducible** across two
independent runs each (not just assumed, per candidate 020's own claim about callgrind
determinism) before being compared.

| Variant | total `Ir` (5 renders, run 1) | total `Ir` (5 renders, run 2) | `Ir`/render | `get_font`+`resolve_font` calls (5 renders) | that subtree's `Ir` (5 renders) | pixmap hash |
|---|---|---|---|---|---|---|
| baseline | 255,535,890 | 255,535,890 | 51,107,178 | 1,630 + 1,630 (every `Tf` call) | 6,302,857 + 926,145 = 7,229,002 | `0x31cced99a6bfe199` |
| patched | 256,691,166 | 256,691,166 | 51,338,233 | 1,630 + 1,630 (every `Tf` call - cache never hits) | 6,307,592 + 926,123 = 7,233,715 | `0x31cced99a6bfe199` |
| **delta** | **+1,155,276 (+0.452%)** | **+1,155,276 (+0.452%)** | **+231,055/render** | **0 (both miss on every call, as designed)** | **+4,713 (+0.065%, noise-level)** | identical |

Raw callgrind output (`callgrind.baseline.out`, `callgrind.patched.out`) and the full
`callgrind_annotate --auto=yes` dumps (`interpret_mod_annotated.{baseline,patched}.txt`) are kept
in this folder's `callgrind/` subdirectory.

## Result

On the genuine worst case - every `Tf` call a guaranteed miss on `resolved_font_cache` - the patch
is a small, unambiguous, **exactly reproducible** regression: **+0.452%** total instructions
(+1,155,276 `Ir` over 5 renders, +231,055/render), confirmed identical across two independent
callgrind runs of each variant. This is the mirror image of candidate 020's result on
`cache_defeating.pdf` (-0.47% there, because that file's 8 reused names let the cache hit 318/326
times).

Critically, the `get_font`/`resolve_font` subtree itself is **not** where the regression comes
from: both baseline and patched call `get_font`+`resolve_font` exactly 1,630 times each (every
single `Tf` call, since neither `font_dict_cache` nor `resolved_font_cache` ever hits when every
resource name is unique), and that subtree's own cost differs by only +4,713 `Ir` (+0.065%,
consistent with measurement noise, not a real effect). The +231,055/render regression is entirely
the **wrapper overhead** the patch adds around the unchanged miss path: an extra failed
`resolved_font_cache.get(name)` lookup before falling into the `else` branch, plus a
`resolved_font_cache.insert(name.clone(), resolved.clone())` afterward (cloning the resolved
`Option<TextStateFont>` to store it, on top of the clone already needed to return it) and the
corresponding extra drop of that clone - exactly the "extra hashmap lookup-then-insert paid on
every single call, with the hit path never exercised" cost this candidate was queued to isolate.

## Does it change behavior/output?

- **Bit-exact?** Yes. Both variants produced the identical pixmap hash
  (`0x31cced99a6bfe199`) across both runs of each, and it matches candidates 018/020's hash for
  the *other* synthetic file too (expected: same font substitution order, same text, only the
  resource-name spelling differs, which isn't rendered).
- Not re-run against the project test suite in this candidate - the patch is byte-identical to
  007/012/018/019/020, all of which already confirmed an identical failing-test-name set
  before/after (pre-existing missing-test-asset environment issues, unrelated to this patch). No
  reason to expect that to differ here since no source changed beyond what those candidates
  already covered.
- **Is this a breaking change?** No - same patch as 007/012/018/019/020, internal to
  `hayro-interpret::interpret()`, no public API change.

## Why the result looks the way it does

`resolved_font_cache` trades a per-call cost (one extra hashmap lookup, and on a miss one extra
hashmap insert plus an extra `Option<TextStateFont>` clone) for skipping the *entire*
`resolve_font()`/`get_font()` subtree on a cache hit. That trade is a clear net win whenever hits
are common (candidate 020: 318/326 hits, -0.47%) and a clear net loss whenever hits are
impossible by construction (this candidate: 0/326 hits, +0.452%) - the added lookup/insert
overhead is paid unconditionally on every call, but the savings it's meant to buy are only
realized on a hit. Both directions are small in absolute terms (sub-1% of total `Ir`) because the
font-resolution subtree itself is a small fraction (~2.8% of total `Ir` here) of the whole render,
even when it's on the guaranteed-miss path every single time.

This also confirms candidate 020's diagnosis was correct in the other direction: `Tf`-resolution
caching only pays off when a real page's font-name reuse rate is high enough (candidate 019 found
even 147 real-world `Tf` calls/page with only 7 distinct fonts - a good reuse rate - was already a
coin-flip, not a clear win at wall-clock granularity), and this candidate shows the downside case
is real too, just similarly small in absolute magnitude.

## What's not done / open questions

- Only `Ir` (instruction count) was profiled, not cache-miss/branch-misprediction event counters
  (`--cache-sim=yes`/`--branch-sim=yes`) - see candidate 020's identical caveat.
- No PDF anywhere in `hayro-tests`' 360-file corpus is known to have this "every `Tf` call a fresh,
  never-reused resource name" shape (it would be an unusual, resource-inefficient way to author a
  real PDF - hundreds of distinct named font resources all pointing at valid dictionaries). This
  candidate's finding is about the theoretical worst case the patch's own doc comment anticipates
  (a nested Form XObject with its own fresh `Resources`/cache on every call), not a demonstrated
  real-world regression - worth keeping in mind before treating +0.452% as something end users
  would actually observe.
- Wall-clock (not just `Ir`) was not re-measured for this specific file - given +0.452% `Ir` is
  already below what candidate 018/019 found resolvable above this sandbox's wall-clock noise
  floor (see their `paired.sh` results), a wall-clock run would very likely just reproduce "not
  distinguishable from noise," which is why it wasn't run; flagged here rather than silently
  skipped.
