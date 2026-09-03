# 53 - `IdLookup::for_each_entry` full-tree walk on every plain-property identifier

From the 2026-08-23 compiler discovery sweep (`IDEAS.md`): `IdLookup` (`internal/compiler/lookup.rs`,
originally lines 343-385, wired in at the front of `global_lookup()` right after `SpecialIdLookup`
and before the HashMap-backed `InScopeLookup`) resolves every identifier lookup by recursively
walking the whole current component's element subtree looking for a matching `id`, with a
`// TODO: hash based lookup` comment already flagging it. For an ordinary property reference (not
an element id - the overwhelmingly common case), this walk fails end-to-end and only then falls
through to `InScopeLookup`. That's an O(elements) cost paid on *every* identifier occurrence in
every binding, i.e. O(n^2)-shaped in element count for a property-dense component.

## What was tried

Memoized `IdLookup::lookup` (the single-name resolution path used by real compilation, as opposed
to `for_each_entry`'s full enumeration used only by LSP completion) with a `HashMap<SmolStr,
ElementWeak>` built once per distinct scope root (the component's own root element, or a `repeated`
`for`/`if` element acting as a nested id scope - matching the original algorithm's scoping exactly)
and cached on that root's owning `Component` (`id_lookup_cache`, keyed by the root's address via
`by_address::ByAddress` since one `Component` can have several such roots). `for_each_entry` itself
is untouched (still does the full recursive walk) since LSP completion needs to enumerate, not
just check one name. Defensive cache invalidation added at the only 3 places in the compiler that
mutate an element's `id` after it may have been cached (`unique_id::assign_unique_id_in_component`,
`unique_id::rename_globals`, `windows::ensure_window`) - none of these currently run before
`resolve_expressions` (the only caller of `IdLookup::lookup`) has already finished for a given
document, so this is future-proofing rather than something the current benchmark exercises.

Bench: a Python fixture generator (`testdata/gen_propdense.py`) produces a flat `.slint` file with
N sibling `Rectangle`s, each declaring `property <length> p0..p3` where `p1..p3` each reference the
previous one by plain identifier (`x: p3;` etc) - 4 in-scope identifier lookups per element, no
element ids anywhere, so every single one of those N*4 lookups pays for a full O(N)-element failed
`IdLookup` walk on the unpatched compiler. Timed with `hyperfine` against `tools/compiler`
(`slint-compiler -f rust <fixture> -o /tmp/out.rs`).

| elements | baseline mean (unpinned) | patched mean (unpinned) | speedup | baseline mean (`taskset -c 0`) | patched mean (`taskset -c 0`) | speedup |
|---|---|---|---|---|---|---|
| 200 | 19.5 ms | 17.8 ms | 1.09x | - | - | - |
| 500 | 41.1 ms | 39.3 ms | 1.05x | - | - | - |
| 1 000 | 86.9 ms | 73.6 ms | 1.18x | - | - | - |
| 2 000 | 204.5 ms | 167.1 ms | 1.22x | 198.6 ms | 160.2 ms | 1.24x |
| 4 000 | 570.4 ms | 375.8 ms | 1.52x | 572.3 ms | 373.6 ms | 1.53x |
| 8 000 | 1.713 s | 835.7 ms | 2.05x | 1.702 s | 847.8 ms | 2.01x |

10+ runs per cell (`hyperfine --warmup 2 --min-runs 10`), `uptime` load average 2.4/24 cores at
measurement time (no reference re-baseline needed - load well under core count). Single-core
(`taskset -c 0`) and unpinned multi-core numbers agree within noise, as expected: `slint-compiler`
is a single-threaded CLI tool for this workload, so there is no parallelism to lose.

## Result

Real, and grows with element count exactly as the O(n^2)-vs-O(n) hypothesis predicts: 1.05-1.09x
at 200-500 elements (fixed compiler overhead dominates), climbing to 1.5x at 4 000 and 2.05x at
8 000 elements. The baseline's own scaling from 1000->2000->4000->8000 elements (86.9ms -> 204.5ms
-> 570.4ms -> 1713ms, i.e. 2.35x/2.79x/3.00x per doubling) is clearly superlinear; the patched
build's scaling over the same steps (73.6ms -> 167.1ms -> 375.8ms -> 835.7ms, i.e. 2.27x/2.25x/
2.22x per doubling) is close to linear, consistent with the walk's O(elements) cost being removed
and only genuinely O(n) or O(n log n) compiler work remaining.

## Does it change behavior/output?

- **Bit-exact?** Yes. `slint-compiler -f rust` output (stdout/generated file) is byte-identical
  between baseline and patched builds on all six fixture sizes (200/500/1000/2000/4000/8000
  elements), verified with `cmp`.
- **Is this a breaking change?** No.
- Project test suite: `cargo test -p i-slint-compiler --release` (unit tests + the compiler's own
  `syntax_tests` corpus, which exercises id lookup, `for`/`if` scoping, and duplicate-id
  diagnostics extensively) - same pass count before and after (all passing, 0 failures either way).
  Also ran the broader interpreter-based corpus, `tests/driver/interpreter` (`tests/cases/**`,
  858 test cases covering widgets, models, layouts, repeaters, globals, etc.): 858 passed / 0
  failed with the patch applied.

## Why the result looks the way it does

The fix removes the O(elements) part of the cost (the failed tree scan) and replaces it with an
O(1) HashMap lookup after the first, one-time O(elements) map build per scope root - so the
remaining cost is whatever else scales with element count in this fixture (parsing, binding
resolution bookkeeping, per-element codegen, etc.), which is why the smaller sizes (200-1000
elements) show a modest 5-18% win: at that size the O(9ms) fixed cost measured in candidate 048
and the genuinely-O(n) compiler work still dominate over the now-fixed-up id-lookup walk. The win
compounds as element count grows because the removed term was quadratic and everything else is
roughly linear - which is exactly the shape the discovery note predicted.

## What's not done / open questions

- Only `IdLookup::lookup` (the single-name path used by real compilation) was memoized;
  `for_each_entry` (full enumeration, used by LSP completion for listing candidates) still does
  the original O(elements) walk every time - completion is interactive/human-paced and wasn't the
  reported hot path, so it wasn't touched.
- The cache is invalidated defensively at all 3 known `.id` mutation sites, but relies on the
  invariant (verified by reading `internal/compiler/passes.rs`'s pass ordering) that
  `IdLookup::lookup` is only ever called from `resolve_expressions`/`resolve_two_way_binding`
  during `run_import_passes`, which completes for a given document before any of those 3 mutation
  sites run in the later whole-program `run_passes`. If a future refactor calls `global_lookup()`
  again after ids are mutated without going through one of those 3 sites, the cache would need a
  new invalidation point added there too - nothing currently enforces this at compile time (it's
  a documented invariant, not a type-level guarantee).
- Not measured: real-world `.slint` files with heavy `for`/`if` nesting (many distinct repeated
  scope roots caching independently) - the property-dense flat fixture isolates the exact
  mechanism described in `IDEAS.md`, but doesn't exercise the multi-root cache-key path under
  realistic nesting depth/shape. The interpreter test suite's repeater-heavy cases (radiogroups,
  tableview, etc.) passed, which is evidence of correctness there, not a performance measurement
  of that specific path.
