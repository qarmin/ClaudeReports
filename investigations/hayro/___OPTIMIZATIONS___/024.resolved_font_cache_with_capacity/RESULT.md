# `resolved_font_cache` pre-sized with `with_capacity_and_hasher` instead of growing from empty

Backlog item (candidate 024): candidate 023's `callgrind_annotate` output showed
`RawTable::reserve_rehash` as a real, non-trivial cost (~90-100K `Ir` over 5 renders) for both the
`Entry`-API (022) and `raw_entry_mut` (023) variants of `resolved_font_cache` on
`worst_case_cache.pdf` (326 distinct resource names, the map grows from empty on every
`interpret()` call). This candidate tests whether pre-sizing the map once, up front, removes that
cost directly - by taking candidate 022's `Entry`-API implementation unchanged and only adding
`FxHashMap::with_capacity_and_hasher(resources.fonts.len(), Default::default())` in place of
`FxHashMap::default()`.

`rustc 1.98.0 (88d9e12ae 2026-08-18)` / `cargo 1.98.0` (recorded per candidate 016's backlog note).

## What was tried

Same `resolved_font_cache` feature as candidates 007/012/018/019/020/021/022/023 (a per-`interpret()`
cache from resource name to resolved font), reusing candidate 022's exact `Entry`-API miss path,
with one change: the map is created with an exact-capacity constructor instead of `::default()`.
`resources.fonts` (the page's own `/Font` resource sub-dictionary, already a struct field on the
`Resources` passed into `interpret()`) exposes a cheap `.len()` (backed by a dictionary's own
lazily-built, cached offset table - not a fresh scan of the content stream), so no extra parsing
work is added to obtain the count:

```rust
let mut resolved_font_cache = FxHashMap::<Name<'a>, Option<TextStateFont<'a>>>::with_capacity_and_hasher(
    resources.fonts.len(),
    Default::default(),
);
```

The miss path itself (`.entry(name.clone()).or_insert_with(...)`) is byte-identical to candidate
022 - reused unchanged so the only variable between the two candidates is presence/absence of the
`with_capacity_and_hasher` pre-sizing.

Reused candidate 021/022/023's exact bench approach (`bench/src/main.rs`) and testdata
(`worst_case_cache.pdf`, 326 `Tf` calls, every one a distinct never-reused resource name - so
`resources.fonts.len()` is exactly 326, an exact match, not an estimate, for this worst-case input)
against the same working-tree commit (`5a5f0e24`, no upstream changes to this file since 021-023).
Built and measured directly against a dedicated git worktree of the main project pinned to that
commit (not this folder's own `./clone`, which is the just-in-case reproducibility path) - both
give identical source at this pinned commit.

Both variants' `Ir` counts were confirmed exactly reproducible across two independent runs each
(callgrind is deterministic) before being compared. Wall-clock (`hyperfine`, 30 runs after 3
warmups, `N=50` renders per invocation) was also measured, single-core (`taskset -c 2`) and
multi-core (unpinned), two rounds each, consistent with 023's precedent given this candidate's
effect size is again small.

| Variant | total `Ir` (5 renders) | `Ir`/render | single-core mean (hyperfine, 2 rounds) | multi-core mean (hyperfine, 2 rounds) | pixmap hash |
|---|---|---|---|---|---|
| baseline (no `resolved_font_cache` at all) | 255,414,628 | 51,082,925.6 | 221.7ms / 214.7ms | 215.5ms / 215.8ms | `0x31cced99a6bfe199` |
| this candidate (`Entry` API + `with_capacity_and_hasher(326, ..)`) | 256,064,988 | 51,212,997.6 | 224.7ms / 214.7ms | 215.3ms / 219.9ms | `0x31cced99a6bfe199` |

For reference, from each candidate's own prior run (different absolute build path -> slightly
different baseline `Ir` in the low hundreds, see "immaterial" note below - only each candidate's
own delta against its own baseline is meaningful):

| Comparison | delta `Ir`/render | delta % vs. own baseline |
|---|---|---|
| 021 (`get`+`insert`, no pre-sizing) vs. its own baseline | +231,055 | +0.452% |
| 022 (`Entry` API, no pre-sizing) vs. its own baseline | +187,866 | +0.368% |
| 023 (`raw_entry_mut`, no pre-sizing) vs. its own baseline | +244,821.8 | +0.479% |
| **024, this candidate (`Entry` API + `with_capacity_and_hasher`) vs. its own baseline** | **+130,072.0** | **+0.255%** |

This candidate's own baseline (255,414,628 total `Ir` over 5 renders) differs slightly from
021/022/023's recorded baselines by low hundreds of `Ir` - the same absolute-build-path-length-
embedded-as-string-constants artifact those candidates already documented (each candidate's own
worktree/folder path has a different length). Immaterial: what matters is each candidate's patched
number vs. its *own* baseline, both built in the same location for this candidate.

Wall-clock across both single-core rounds and one multi-core round showed no consistent direction
(patched sometimes very slightly faster, sometimes slower, well within each round's own spread -
e.g. single-core round 2: baseline 214.7ms ± 4.4ms vs. patched 214.7ms ± 5.0ms, indistinguishable);
one multi-core round showed patched ~2% slower (219.9ms ± 9.8ms vs. baseline 215.8ms ± 1.8ms) but
with a spread wide enough to overlap the other round's near-identical reading. Consistent with
018/019/021/022/023's established finding that a delta of this magnitude (~0.25% `Ir`) sits at or
below this sandbox's wall-clock noise floor.

## Result

Pre-sizing `resolved_font_cache` with `with_capacity_and_hasher(resources.fonts.len(), ..)` reduces
candidate 022's `Entry`-API regression from +187,866 `Ir`/render (+0.368%) to +130,072 `Ir`/render
(+0.255%) vs. the no-cache baseline - a **30.8% reduction of 022's own regression**
((187,866-130,072)/187,866), exactly reproducible across 2 runs each for baseline and patched. It
is also the smallest regression of all four `resolved_font_cache` implementations measured so far
(021/022/023/024), though still a net regression relative to no cache at all on this specific
worst-case input (326 `Tf` calls, no name ever reused).

`callgrind_annotate --auto=yes` against `resolved_font_cache`'s own type
(`HashMap<Name, Option<TextStateFont>, FxBuildHasher>`) shows **no `reserve_rehash` entry at all**
for this map in the patched build (searched explicitly; none found), confirming growth-driven
rehashing was fully eliminated by the pre-sizing, exactly as the candidate's own hypothesis
predicted. What remains, attributed directly to `resolved_font_cache` in the annotated output:
`rustc_entry` bookkeeping (~58,885 `Ir` combined across its two call sites in the annotated output,
over the 5 renders the profiled process performs) and `Option<TextStateFont>` drop glue when the
map itself is dropped at the end of each `interpret()` call (~39,090 `Ir`, same basis) - together
~97,975 `Ir`, only about **15%** of the 650,360 `Ir` total delta (over the same 5 renders) between
this candidate and baseline. The remaining ~85% was not cleanly attributed to a single named symbol
in the annotated output (`RawTableInner::fallible_with_capacity` - the map's one-time up-front
allocation - appears in the output but not attributed unambiguously enough, by caller, to
`resolved_font_cache` specifically versus other call sites in the same binary to give it a number
here); the likeliest remaining contributors are the `Name` key hashing itself on every lookup/insert
and, notably, the `.clone()` on `or_insert_with`'s return value that both 022 and this candidate pay
on every `Tf` call (326 of them, 100% miss rate on this worst-case input) to turn the map's `&mut
Option<TextStateFont>` into an owned value - a clone the no-cache baseline never needs, since it
already produces (and directly moves) an owned value with no map in between. In short: this
candidate confirms rehashing was a real, addressable component of 022's regression (removing it cut
022's regression by 30.8%), and of what's left after that removal, only about 15% is cleanly
attributed to the two named symbols above (`rustc_entry`, drop glue) - the bulk of this candidate's
own residual regression is not conclusively attributed by this candidate's profiling depth. A
possible follow-up (not investigated here) would be checking whether the per-lookup `.clone()`
described above is the dominant remaining cost.

Wall-clock (`hyperfine`, both single- and multi-core, 2 rounds each) is not able to resolve this
delta from noise - consistent with the same finding for 021/022/023 at similar or larger magnitudes.

## Does it change behavior/output?

- **Bit-exact?** Yes. Baseline and this candidate produce the identical pixmap hash
  (`0x31cced99a6bfe199`) for `worst_case_cache.pdf`, matching every prior candidate in this family
  (007/012/018/019/020/021/022/023).
- **Project test suite** (`cargo test --release -p hayro-interpret -p hayro-tests`, run through the
  shared build lock, both before and after applying this candidate's patch, in the same worktree):
  baseline 430 passed / 1277 failed; patched 430 passed / 1277 failed - **identical pass/fail
  counts, and the failing-test-name *sets* are exactly identical** (`diff` of the two sorted
  `FAILED` name lists produced no output). All 1277 failures are a pre-existing environment issue
  in this sandbox worktree unrelated to this patch: `hayro-tests` loads PDF fixtures from a
  `WORKSPACE_PATH`-relative corpus directory that isn't checked out in a plain `git worktree`
  (confirmed directly - a single failing test's panic message is
  `Os { code: 2, kind: NotFound, message: "No such file or directory" }` from `load_pdf`'s
  `std::fs::read(&path).unwrap()`), not a real assertion failure. This matches
  021/022/023's own documented precedent for this same pre-existing condition; this candidate
  re-ran the suite directly (rather than relying on precedent alone) and confirmed both counts and
  the exact failing-name set are unaffected by this patch.
- **Is this a breaking change?** No - internal to `hayro-interpret::interpret()`, no public API
  change, same as the rest of this candidate family (021/022/023).

## Why the result looks the way it does

`resolved_font_cache`'s original (007/012/018/019/020/021) miss path did two hashmap touches per
first-seen name (`get` + `insert`); 022's `Entry` API collapsed that to one logical touch but still
grew the map from an empty `FxHashMap::default()` on every `interpret()` call, paying `hashbrown`'s
own doubling-growth rehash cost as the map crosses each capacity boundary on the way to 326 entries
in this worst-case input. This candidate's hypothesis - that `RawTable::reserve_rehash`, visible in
022/023's own `callgrind_annotate` output, was directly addressable by pre-sizing rather than
requiring a different hashmap API - holds: `reserve_rehash` disappears entirely from
`resolved_font_cache`'s own annotated cost once the map is created with the right capacity up
front, and the total regression against baseline drops by close to a third. It does not disappear
altogether because a hash map - any hash map, any API, correctly pre-sized or not - still pays a
per-entry cost for hashing the key, storing the entry, and (at scope exit) dropping every stored
value; only the *growth* component of that cost was ever addressable by this specific change, and
this measurement puts a number on how much of 022's regression that growth component actually was
(30.8%) versus how much was unavoidable per-entry overhead (69.2%).

`resources.fonts.len()` being available for free (a struct field already parsed as part of building
`Resources`, with `Dict::len()` backed by an already-built/cached offset table rather than a fresh
scan) is what makes this pre-sizing essentially free to obtain - the candidate's own alternative of
"a small fixed guess like 8" was not tried, since an exact count was available directly and is
strictly at least as good (no under-sized guess ever triggers a rehash; no over-sized guess ever
wastes memory) for any input where `/Font` resource-dict size and distinct-`Tf`-name count roughly
track each other, which is the common case this cache targets.

## What's not done / open questions

- Not tried against a realistic (non-worst-case) PDF corpus: as with 021/022/023, no PDF in
  `hayro-tests`' corpus is known to have this "every `Tf` call a fresh, never-reused resource name"
  shape, so this candidate's +0.255% (down from 022's +0.368%) is a bound on a synthetic worst case,
  not a demonstrated real-world measurement. On a typical PDF where `resolved_font_cache` mostly
  gets hits after the first few insertions (few distinct fonts, reused across many `Tf` calls), both
  the original regression and this candidate's reduction of it would be far smaller in absolute
  terms, though the *relative* removal of the growth component should still apply proportionally.
- `resources.fonts.len()` counts *all* entries in the `/Font` resource dict, including any that
  might never actually be referenced by a `Tf` operator in this particular content stream (e.g. a
  shared `Resources` dict reused across multiple content streams, each only using a subset) - this
  makes the pre-sized capacity an upper bound on what the cache will actually hold for this call,
  not necessarily exact, though for the tested worst-case input, and for any content stream that
  does use every listed font name in `Tf` calls, it is exact. This is a deliberate, low-risk
  over-provisioning: worst case wastes a few unused hashmap slots per `interpret()` call, never
  causes a rehash a correctly-sized-in-advance map wouldn't otherwise need.
- The residual per-entry overhead identified above (`rustc_entry`, `Option<TextStateFont>` drop
  glue) was not itself targeted by this candidate - a follow-up idea (not tried here) would be
  testing whether that drop-glue cost is reducible, e.g. by not storing `None` misses in the cache
  at all if they're expected to be rare, trading a possible re-lookup for a smaller live cache.
