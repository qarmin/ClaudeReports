# dictionary-std tag lookup: is it a linear scan, sorted binary search, or a hash map?

An open question carried over from a prior session's notes (never profiled/confirmed there,
described only as "check before assuming either way") - `dictionary-std`'s ~4,600-entry generated
tag table (`dictionary-std/src/tags.rs`'s `ENTRIES` array) could plausibly have been indexed for
lookup via a linear scan, a sorted array + binary search, or a hash map, with very different
performance implications for a many-small-elements-heavy parse workload.

## What was tried

Read `dictionary-std/src/data_element.rs` directly rather than guessing from behavior:
`StandardDataDictionaryRegistry` (built once, lazily, via `once_cell::sync::Lazy`) stores
`by_tag: HashMap<Tag, &'static DataDictionaryEntryRef<'static>>`, populated once from the static
`ENTRIES` array in `init_dictionary()`. `StandardDataDictionary::by_tag()` (the actual per-element
lookup called during parsing) does a single `HashMap::get`, falling back to two more `HashMap`
lookups only for the rare repeating-tag-group/repeating-element cases (`Group100`/`Element100`
patterns, e.g. overlay/curve data groups), and only reaching a linear "private creator"/"group
length" special case after all three hash lookups miss. **This is already a hash map, not a linear
scan or binary search** - the backlog item's own cautious phrasing ("likely fine ... but check")
turns out correct.

Bench: added a `dictlookup` mode to `___OPTIMIZATIONS___/harness/bench_pipeline` (10,000
`StandardDataDictionary::by_tag()` calls per iteration, a realistic mix of 9 common tags plus 1
deliberately-absent tag to also exercise the miss path) as a quantification, not a comparison (no
patch exists to compare against).

| Metric | Value |
|---|---|
| Throughput | ~121us / 10,000 lookups = ~12ns/lookup |

## Result

**N/A - measurement/investigation only, no patch applied to real source.** ~12ns/lookup is
consistent with a single `HashMap::get` call (typical Rust `std::collections::HashMap` lookups are
in the 5-30ns range depending on hash cost and cache locality) and many orders of magnitude faster
than a linear scan over ~4,600 entries would be (which would need to compare against a meaningful
fraction of the table on average). No further optimization is indicated here - the dictionary
lookup is already appropriately structured for its access pattern.

## Does it change behavior/output?

N/A - no patch was written or applied; this candidate is a pure investigation with a "no fix
needed" conclusion, per SKILL.md's guidance that a null result / "already optimal" finding is
still worth recording so a future session doesn't re-ask the same question. Project test suite was
not re-run since no source was changed.

## Why the result looks the way it does

The prior session's notes' own reasoning was correct: whoever wrote `dictionary-std` reached for
`HashMap` for exactly this kind of large-fixed-key-set lookup, which is the standard, appropriate
choice - there is no indication this needed profiling to justify improving, only to confirm.

## What's not done / open questions

- Not measured: relative cost of dictionary lookup vs. the rest of per-element parsing overhead in
  a real end-to-end parse (i.e., whether ~12ns/lookup is negligible or non-negligible next to
  everything else `stateful::decode` does per element). This candidate only answers "is the lookup
  structure itself reasonable," not "does dictionary lookup show up in a real profile" - the
  session 1 callgrind profile (per prior notes) did not flag dictionary lookup as hot for either
  workload tried there, which is a second, independent (though not re-verified this session) signal
  pointing the same direction.
- A perfect hash (e.g. `phf`) could theoretically be marginally faster than `std::HashMap` for a
  fixed, compile-time-known key set (no hashing-function DoS resistance needed, potentially better
  cache behavior), but the current ~12ns/lookup is already fast enough that this is unlikely to be
  worth the added build-time code-generation complexity - not benchmarked, since there's no
  indication the current lookup is a bottleneck anywhere in this workspace's profiling so far.
