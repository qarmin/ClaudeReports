# `resolved_font_cache`'s miss path via `hashbrown::HashMap::raw_entry_mut()` instead of `Entry`

Backlog entry from `../IDEAS.md`: candidate 022 shrank, but did not eliminate, candidate 021's
worst-case `resolved_font_cache` regression (+0.452% -> +0.368% total `Ir`) by switching the miss
path from `get()`+`insert()` to `std`/`FxHashMap`'s `entry()`. `callgrind_annotate` attributed the
residual to `hashbrown`'s own `rustc_entry`/`reserve_rehash` bookkeeping (the layer `std`'s
`HashMap::entry()` goes through to implement the standard-library-compatible `Entry` enum), which
is bypassed if the map itself is a `hashbrown::HashMap` used directly via its own
`raw_entry_mut()` API instead of going through `std`'s wrapper at all. This candidate makes that
change and re-profiles with the same worst-case methodology.

`rustc 1.98.0 (88d9e12ae 2026-08-18)` / `cargo 1.98.0` (recorded per candidate 016's backlog
note).

## What was tried

Same `resolved_font_cache` feature as candidates 007/012/018/019/020/021/022 (a per-`interpret()`
cache from resource name to resolved font), but the map's type changes from `FxHashMap` (`std`'s
`HashMap` with an `FxHasher`) to `hashbrown::HashMap<Name<'a>, Option<TextStateFont<'a>>,
rustc_hash::FxBuildHasher>` - `hashbrown` added as a direct dependency of `hayro-interpret`
(`hashbrown = "0.17"`, matching the version already pulled in transitively via `std`). The miss
path becomes:

```rust
let font = resolved_font_cache
    .raw_entry_mut()
    .from_key(name)
    .or_insert_with(|| {
        let resolved = if let Some(font_dict) = font_dict_cache.get(name).cloned() {
            context.resolve_font(&font_dict)
        } else if let Some(font_dict) = resources.get_font(name) {
            font_dict_cache.insert(name.clone(), font_dict.clone());
            context.resolve_font(&font_dict)
        } else {
            Font::new_standard(StandardFont::Helvetica, &context.settings.font_resolver)
                .map(TextStateFont::Fallback)
        };
        (name.clone(), resolved)
    })
    .1
    .clone();
```

Reused candidate 021/022's exact bench approach (`bench/src/main.rs`) and testdata
(`worst_case_cache.pdf`, 326 `Tf` calls, every one a distinct never-reused resource name) against
the same working-tree commit (`5a5f0e24`, no upstream changes to this file since 021/022). During
the session the bench was built directly against the main project's own working tree (per the
skill's normal workflow, to reuse `target/`), not this folder's `./clone` - both give the
identical source at this pinned commit.

Both variants' `Ir` counts were confirmed exactly reproducible across two independent runs each
(callgrind is deterministic) before being compared. Wall-clock (`hyperfine`, 30 runs each after 3
warmups) was also measured, single-core (`taskset -c 2`) and multi-core (unpinned), two rounds
each, since the callgrind-only precedent set by 022 was for an even smaller effect than the one
actually found here (see below).

| Variant | total `Ir` (5 renders) | `Ir`/render | single-core mean (hyperfine, 2 rounds) | multi-core mean (hyperfine, 2 rounds) | pixmap hash |
|---|---|---|---|---|---|
| baseline (no `resolved_font_cache` at all) | 255,535,843 | 51,107,168.6 | 218.6ms / 218.6ms | 221.7ms / 219.9ms | `0x31cced99a6bfe199` |
| candidate 021 (`get()`+`insert()`, for reference from 021's own run) | 256,691,166 | 51,338,233 | - | - | `0x31cced99a6bfe199` |
| candidate 022 (`Entry` API, for reference from 022's own run) | 256,475,348 | 51,295,069.6 | - | - | `0x31cced99a6bfe199` |
| this candidate (`hashbrown::raw_entry_mut()`) | 256,759,952 | 51,351,990.4 | 221.7ms / 230.1ms | 223.6ms / 222.2ms | `0x31cced99a6bfe199` |

| Comparison | delta `Ir`/render | delta % vs. no-cache baseline |
|---|---|---|
| 021 (`get`+`insert`) vs. baseline | +231,055 | +0.452% |
| 022 (`Entry` API) vs. baseline | +187,866 | +0.368% |
| **this candidate (`raw_entry_mut`) vs. baseline** | **+244,821.8** | **+0.479%** |

This candidate's own baseline (255,535,843) differs slightly from 021's (255,535,890) and 022's
(255,536,020) recorded baselines by a couple hundred `Ir` over 5 renders - the same
absolute-path-length-embedded-as-string-constants artifact 022 already documented (this
candidate's own folder/build path has a different length than 021's or 022's). Immaterial: what
matters is each candidate's patched number vs. its *own* baseline, all built in the same location.

Wall-clock direction was consistent with the `Ir` result (patched slower) across all 4 runs (2
single-core, 2 multi-core), though single-core had much higher variance (±9.9ms / ±18.0ms on the
patched binary vs. ±2.3-2.5ms on baseline) than multi-core (±2.4-3.4ms both binaries) - consistent
with prior candidates' finding (018/019/021) that effects at this magnitude sit at or below this
sandbox's single-core noise floor; multi-core's tighter spread gave a cleaner, still-noisy ~1%
slower reading in both rounds.

## Result

`hashbrown::HashMap::raw_entry_mut()` does **not** recover any more of candidate 021's original
regression - it makes it slightly **worse** than both 021's naive `get()`+`insert()` and 022's
`Entry` API: +0.479% total `Ir` vs. the no-cache baseline, vs. 021's +0.452% and 022's +0.368%,
exactly reproducible across 2 runs. Of the three variants tried across candidates 021/022/023,
`Entry` (022) is the smallest regression, plain `get()`+`insert()` (021) is in the middle, and this
candidate's supposedly-lower-level `raw_entry_mut()` is the largest. Wall-clock (`hyperfine`,
multi-core) agrees in direction: ~1% slower than baseline in both of 2 rounds, though the effect is
small enough that single-core wall-clock is too noisy to resolve it cleanly (matching the
established pattern from candidates 018/019/021 that sub-1% `Ir` deltas in this family aren't
reliably visible at wall-clock granularity in this sandbox).

## Does it change behavior/output?

- **Bit-exact?** Yes. Baseline and all three variants (021, 022, this candidate) produce the
  identical pixmap hash (`0x31cced99a6bfe199`) for `worst_case_cache.pdf`.
- Not re-run against the project test suite in this candidate - font resolution semantics are
  unchanged from 007/012/018/019/020/021/022 (same cache, same key, same scoping), only the
  hashmap type/access API differs, which cannot affect what font gets resolved.
- **Is this a breaking change?** No - internal to `hayro-interpret::interpret()`. It does add a
  new direct dependency (`hashbrown`) to `hayro-interpret`'s `Cargo.toml`/`Cargo.lock` (pulling in
  `allocator-api2` and `equivalent` as transitive deps of `hashbrown`, both already present in the
  dependency graph transitively via `std`'s own vendored `hashbrown` copy, so no genuinely new
  crates are compiled that weren't already being built) - a maintainer decision point independent
  of the performance result, since it moves off `std`'s `HashMap`/`Entry` for one call site.

## Why the result looks the way it does

The backlog's framing - "safe `Entry` still has bookkeeping overhead that a lower-level
`hashbrown` API could avoid" - turned out backwards for this specific case. Two candidate
explanations, not fully distinguished from each other with the tools used this session:

1. **The reallocation-driven rehash isn't actually avoided.** `callgrind_annotate` on this
   candidate's patched binary shows `RawTable::reserve_rehash` and
   `raw_entry::RawVacantEntryMut::insert` as new named symbols (~90K-100K `Ir` combined across
   call sites, over 5 renders) - the same shape of cost 022 found under `Entry`'s `rustc_entry`.
   `worst_case_cache.pdf`'s map grows from empty to 326 entries every render regardless of which
   API inserts into it, so the table has to grow/rehash a similar number of times either way;
   `raw_entry_mut()` skips `std`'s `Entry`-enum compatibility wrapper, but doesn't skip the actual
   growth-triggered rehashing, which is where most of both variants' overhead already was.
2. **A second, separately-compiled `hashbrown` copy may codegen worse than `std`'s internal one.**
   `std`'s own `HashMap` is backed by a vendored copy of `hashbrown` built with
   `rustc-dep-of-std`/`nightly`-gated internal features (visible in this machine's toolchain
   sources at `~/.rustup/toolchains/*/lib/rustlib/src/rust/library/vendor/hashbrown-0.17.1`) that
   aren't available to an ordinary crates.io `hashbrown` dependency built as a normal external
   crate. This candidate never confirmed this by inspecting disassembly of the two `hashbrown`
   copies' inlined code side by side - flagged as unconfirmed, not shown.

Either way, the practical conclusion holds: reaching for `hashbrown` directly to "skip" `Entry`'s
overhead is not a reliable win here, and cost real effort (a new dependency, an unsafe-adjacent
lower-level API) for a result that is a regression relative to even the naive pre-022 code, let
alone 022's `Entry` rewrite.

## What's not done / open questions

- Disassembly-level comparison of `std`'s internal `hashbrown` copy vs. the external
  `hashbrown` crate's compiled code for the same operations, to distinguish explanation 1 vs. 2
  above, was not done.
- `from_key_hashed_nocheck`/computing the hash once via a separate `BuildHasher::hash_one` call
  and threading it through both the lookup and insert (the literal reading of the backlog's "compute
  the hash once" phrasing) was not tried as a separate variant - `from_key` already computes the
  hash exactly once internally before probing, so a hand-computed hash would only save the
  argument-passing overhead, not a second hash computation; unlikely to change the conclusion
  given the actual bottleneck (per above) looks like rehash/insert cost, not hashing itself.
- As with 021/022, this is the theoretical worst case (every `Tf` call a fresh, never-reused
  resource name), not a demonstrated real-world regression - no PDF in `hayro-tests`' 360-file
  corpus is known to have this shape.
- This closes out the backlog item that queued this candidate; no further "go lower than `Entry`"
  attempt is recommended based on this result, though a future session is of course free to
  revisit if a different data structure or a cache with a fixed known size to pre-`with_capacity`
  is considered (not attempted here - see IDEAS.md for a related pre-sizing idea if one is added).
