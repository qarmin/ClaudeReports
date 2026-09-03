# `std`'s internal vendored `hashbrown` vs. an external crates.io `hashbrown` dependency - disassembly/codegen comparison

Backlog item from candidate 023's own "What's not done" section: candidate 023 found that
`resolved_font_cache` implemented with the external `hashbrown` crate's `raw_entry_mut()` was a
slightly *larger* regression (+0.479% total `Ir`) than candidate 022's `std`/`FxHashMap`
`entry()` (+0.368%), the opposite of what "skip `std`'s `Entry` wrapper" predicted, and flagged
"maybe the external `hashbrown` crate codegens worse than `std`'s internal, nightly-feature-gated
copy" as an unconfirmed hypothesis. This candidate investigates that hypothesis directly: source
diff between the two copies, a same-API-shape controlled variant (external `hashbrown` via
`.entry()`, matching 022's access pattern exactly instead of 023's `raw_entry_mut()`), and a
side-by-side disassembly/branch-simulation comparison of the two crate copies' compiled code for
structurally analogous map operations that coexist in the *same* binary.

`rustc 1.98.0 (88d9e12ae 2026-08-18)` / `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

**1. Source-level diff.** `std`'s internal vendored copy
(`~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/vendor/hashbrown-0.17.1`,
requires the `rust-src` rustup component) vs. the crates.io-published `hashbrown-0.17.1` pulled by
this candidate's `Cargo.lock` (pinned to the same `0.17.1` via `cargo update -p hashbrown
--precise 0.17.1`, matching the version already present transitively via `std`). `diff -rq` on
`src/` and `Cargo.toml`: **zero differences** - the source is byte-identical between the two
copies. `hashbrown-0.17.1`'s own `Cargo.toml` defines a `rustc-dep-of-std` feature (`= ["nightly",
"core", "alloc", "rustc-internal-api"]`) that only `std`'s own build of the crate enables; an
ordinary external dependency (this candidate's, and 023's) never gets it. Concretely, in
`src/util.rs`, `nightly` gates whether `likely`/`unlikely` are the real, unstable
`core::intrinsics::{likely, unlikely}` compiler hints (`std`'s copy) or a `#[cold]`-function-call
fallback approximation (external copy) - used throughout `raw.rs`'s hot lookup/insert/rehash
paths. This confirms the *mechanism* 023 flagged as unconfirmed is real and present in this
`hashbrown` release.

**2. A controlled variant isolating crate provenance.** Candidate 022's `resolved_font_cache`
(`std`/`FxHashMap`, accessed via `.entry(name.clone()).or_insert_with(...)`) re-implemented with
the *external* `hashbrown` crate's own `HashMap`, using the exact same `.entry()`/`or_insert_with`
access pattern (not 023's `raw_entry_mut()`) - so the only variable that changes vs. 022 is which
crate's `RawTable`/`HashMap` code is linked in, not the API shape:

```rust
let mut resolved_font_cache =
    hashbrown::HashMap::<Name<'a>, Option<TextStateFont<'a>>, rustc_hash::FxBuildHasher>::default();
// ...
let font = resolved_font_cache
    .entry(name.clone())
    .or_insert_with(|| { /* same body as 022 */ })
    .clone();
```

Same `worst_case_cache.pdf` (326 `Tf` calls, every one a distinct never-reused resource name) and
`callgrind_probe` bench reused unchanged from candidates 021/022/023 (`bench/src/main.rs`,
`testdata/worst_case_cache.pdf`).

| Variant | total `Ir` (5 renders, this session's own baseline) | `Ir`/render | delta vs. this session's baseline | pixmap hash |
|---|---|---|---|---|
| baseline (no `resolved_font_cache`) | 255,403,430 (2/2 runs identical) | 51,080,686.0 | - | `0x31cced99a6bfe199` |
| this candidate (external `hashbrown::HashMap`, `.entry()` API) | 256,712,319 (2/2 runs identical) | 51,342,463.8 | +261,777.8 (**+0.512%**) | `0x31cced99a6bfe199` |

| Candidate | API | Map crate | `Ir`/render delta vs. its own baseline |
|---|---|---|---|
| 022 | `.entry()` | `std` (internal, nightly-gated `hashbrown`) | +187,866 (+0.368%) |
| 023 | `.raw_entry_mut()` | external `hashbrown` | +244,821.8 (+0.479%) |
| **025 (this candidate)** | `.entry()` | external `hashbrown` | **+261,777.8 (+0.512%)** |

Holding the API pattern fixed at 022's `.entry()` shape and changing only the crate, the external
crate is the larger regression by +0.144 percentage points (+261,777.8 vs. +187,866 `Ir`/render,
~39% relatively larger) - a real, reproducible (2/2 runs each) difference at the whole-program
level, consistent with the direction 023's hypothesis predicted.

Absolute-value note: this session's own baseline (255,403,430) differs slightly from
021/022/023's recorded baselines (255,535,843 / 255,535,890 / 255,536,020) - the same
build-path-length-embedded-as-string-constants artifact those candidates already documented
(different candidate folder path length). Only within-session comparisons are meaningful; the
table above compares each candidate's patched number to its own session's baseline, not across
sessions.

**3. Same-binary disassembly comparison of the actual `RawTable` operations.** Because
`font_dict_cache` (an existing, untouched cache in `interpret()`, `FxHashMap` = `std`'s `HashMap`)
and this candidate's `resolved_font_cache` (external `hashbrown::HashMap`) both grow from empty to
a comparable number of `Name`-keyed entries in the *same* render, both crate copies' compiled
`RawTable` code exist side by side in the one patched binary, distinguishable by their embedded
debug-info source paths (`/rust/deps/hashbrown-0.17.1/...` for `std`'s internal copy vs.
`/home/rafal/.cargo/registry/.../hashbrown-0.17.1/...` for the external one). `objdump -d
--demangle` on `<hashbrown::raw::RawTable<(Name, Dict)>>::reserve_rehash` (`std`-backed,
`font_dict_cache`) vs. `<hashbrown::raw::RawTable<(Name, Option<TextStateFont>)>>::reserve_rehash`
(external-backed, `resolved_font_cache`):

| | `std`-backed (`Name`, `Dict`) | external-backed (`Name`, `Option<TextStateFont>`) |
|---|---|---|
| disassembled instruction count | 441 | 489 (+48, +11%) |
| `movdqu`/`movdqa`/`movups` (SIMD group load) | 12 / 12 | 12 / 12 (as `movups`) |
| `pmovmskb` (SIMD group-match mask) | 8 | 8 |
| `callgrind` `Ir` for this symbol (5 renders) | 47,460 | 47,420 |
| `callgrind --branch-sim=yes` `Bc`/`Bcm` for this symbol's own line | 2,380 / 84 (3.5%) | 2,380 / 74 (3.1%) |

The core SIMD group-scanning shape (`movdqu`/`pmovmskb`, hashbrown's `Group::match_byte`) is
identical in count between the two copies - not surprising given the source is byte-identical.
The external version's function is ~11% larger in raw instruction count, but this is **not**
crate-provenance evidence: `reserve_rehash` is monomorphized per value type, and `Dict` and
`Option<TextStateFont>` are different sizes/shapes, so more code to move/relocate table entries on
growth is an artifact of comparing two different generic instantiations, not of which crate
compiled it. Summing **all** lines callgrind attributes to either monomorphization (`reserve_rehash`
+ `insert` + their inlined-into-`core`/`alloc` fragments) gives 165,330 `Ir` (`std`-backed) vs.
166,165 `Ir` (external-backed) - a 0.5% relative difference, i.e. no meaningfully different total
cost for this specific hot path once the value-type-size confound is accounted for by summing the
whole operation rather than eyeballing one function's raw size.

Branch misprediction counts (via `callgrind --branch-sim=yes`, since a static `Ir` instruction
count cannot see whether `core::intrinsics::likely`/`unlikely`'s real LLVM branch-weight metadata
actually reduces mispredicts vs. the `#[cold]`-function fallback - both hint mechanisms only
affect code layout/branch-prediction, not which instructions execute, so `Ir` is structurally
blind to this) were, if anything, **lower** for the external-crate version at this specific
symbol (74 vs. 84 mispredicts) - the opposite of what "std's real branch hints reduce
mispredicts" would predict, though at a sample size (74 vs. 84 events) too small to treat as a
real effect either way.

Whole-program branch-sim totals (baseline vs. this candidate's patched binary): `Bcm` (conditional
mispredicts) 1,059,050 (3.1%) baseline -> 1,147,862 (3.3%) patched; `Bim` (indirect mispredicts)
140,240 (20.1%) -> 154,574 (21.5%). Both move in the same direction as `Ir` (more total branches
and mispredicts because there's simply more map work happening, an extra map that didn't exist in
the baseline at all) - not isolated to the `std`-vs-external comparison specifically.

## Result

- **Source**: `std`'s vendored `hashbrown-0.17.1` and the crates.io-published `hashbrown-0.17.1`
  are byte-identical in `src/` and `Cargo.toml`. The mechanism 023 flagged as an unconfirmed
  hypothesis - `std`'s copy is compiled with nightly-only internal feature gates unavailable to an
  external dependency - is real and confirmed: `rustc-dep-of-std` implies `nightly`, which
  upgrades `likely`/`unlikely` from a `#[cold]`-function-call approximation to real
  `core::intrinsics::likely`/`unlikely` compiler hints throughout `raw.rs`.
- **Whole-program effect**: holding the access API fixed at 022's `.entry()` pattern and changing
  only the map's crate provenance (std-internal vs. external) reproduces a real, 2/2-reproducible
  regression gap of +0.144 percentage points (022's +0.368% vs. this candidate's +0.512% total
  `Ir` vs. each's own baseline) - consistent in *direction* with 023's hypothesis.
- **Operation-level effect**: a direct, same-binary, same-render disassembly and branch-simulation
  comparison of the two crate copies' compiled `RawTable` operations for structurally comparable
  `Name`-keyed maps shows **no measurable per-operation cost difference** - `Ir` totals differ by
  ~0.5% (165,330 vs. 166,165, likely noise/attribution granularity) and branch mispredicts are, if
  anything, slightly lower for the external copy at the one directly-compared symbol. The `Ir`
  metric is structurally incapable of detecting a pure branch-hint/code-layout effect (same
  instructions execute regardless of which basic block is laid out as the likely path), and
  callgrind's built-in branch simulator did not show one either at this sample size.
- These two findings aren't contradictory: the whole-program gap (022 vs. this candidate) is real
  but its source isn't pinned down by this candidate's operation-level tooling - it may sit in
  code this candidate didn't examine (the `Entry`/closure/drop-glue layer sitting above
  `RawTable`, table-growth-curve differences, or codegen decisions elsewhere in the two
  `HashMap`/`Entry` implementations, which do differ in source between `std`'s own `HashMap`
  wrapper and `hashbrown::HashMap` even though the underlying vendored `hashbrown-0.17.1`
  `RawTable` source is identical either way).

## Does it change behavior/output?

**A real fix to the tested project's own source** (adds `hashbrown` as a direct dependency of
`hayro-interpret`, changes `resolved_font_cache`'s map type) - `patch.diff` in this folder,
checked against `base_commit` below.

- **Bit-exact?** Yes. Baseline and this candidate produce the identical pixmap hash
  (`0x31cced99a6bfe199`) for `worst_case_cache.pdf`, matching candidates 021/022/023's own
  baseline/patched hashes.
- **Project test suite**: `cargo test -p hayro-interpret --release`: 48 passed / 0 failed, both
  baseline and patched (deterministic, identical both runs). `cargo test -p hayro-tests
  --release`: baseline 430 passed / 1,277 failed, patched 430 passed / 1,277 failed - **identical
  failing-test-name sets** (diffed explicitly, not just counts). All 1,277 failures are a
  pre-existing environment limitation unrelated to this patch: only 391 of this suite's ~1,707
  test PDFs are present in this sandbox (`hayro-tests/pdfs/{custom,load,other}`, checked into
  git); the `corpus`/`pdfbox`/`pdfium`/`pdfjs`-named tests need PDFs fetched at test-prep time by
  `hayro-tests/sync.py` (network access), unavailable/not run in this sandbox, and fail with `Os
  { code: 2, kind: NotFound }` regardless of any code change. Confirmed this also happens on a
  completely clean, unpatched checkout - not introduced by this candidate.
  - Caveat on measurement discipline: an *initial* unlocked test run (not wrapped in the shared
    `.locks/build.lock`) read 117 passed / 1,590 failed - inconsistent with every subsequent
    properly-`flock`-wrapped run (430/1,277, reproducible 3/3 times including a
    `--test-threads=1` run). Likely CPU/file-I/O contention from a concurrently-running sibling
    candidate during that one unlocked invocation. The 430/1,277 numbers, all captured through the
    lock, are the ones treated as authoritative here; the 117/1,590 reading is noted, not silently
    dropped, but not used as evidence of anything.
- **Is this a breaking change?** No - internal to `hayro_interpret::interpret()`. Adds `hashbrown`
  as a direct dependency (already present transitively via `std`, so no genuinely new crate
  compiled), same as candidate 023.

## Why the result looks the way it does

The source-identical-but-differently-compiled explanation from 023's open question is now
confirmed to exist as a real mechanism (nightly-gated `likely`/`unlikely` intrinsics via
`rustc-dep-of-std`), and a controlled, API-matched variant shows the whole-program direction 023's
hypothesis predicted (external crate worse than `std`, by a similar order of magnitude to 023's
own `raw_entry_mut()` finding). But the direct, same-binary operation-level comparison this
candidate was specifically asked to do doesn't localize that gap to `RawTable`'s core
insert/rehash machinery - the compiled code for that specific hot path is nearly identical in
both instruction count and simulated branch mispredicts between the two crate copies. The
practical conclusion updates 023's "unconfirmed, not shown" to "confirmed to exist as a compiler
feature-gate mechanism, but not the dominant contributor to the measured regression at the specific
operation this candidate could directly compare" - the whole-program gap is real but its precise
source within the `HashMap`/`Entry` call stack remains open.

## What's not done / open questions

- The precise source of the whole-program +0.144-percentage-point gap (022 `.entry()`/`std` vs.
  this candidate's `.entry()`/external) was not localized beyond ruling out `RawTable::reserve_rehash`/`insert`
  themselves as the dominant contributor - the `Entry` enum/closure-construction layer above
  `RawTable` (which does differ in source between `std::collections::hash_map::Entry` and
  `hashbrown::hash_map::Entry`, unlike the underlying `RawTable`) was not disassembled/compared
  the same way.
- `perf stat`'s hardware branch-misprediction counters (real CPU, not callgrind's simulated
  predictor) are unavailable in this sandbox (no `perf`, confirmed by prior candidates) - only
  callgrind's own branch-sim model was available for the misprediction comparison above, which
  uses a simplified predictor, not this machine's actual branch predictor.
- **Incidental, non-performance finding, not this candidate's own patch/harness**: mid-session,
  `git stash` (used transiently while trying to get back to a clean baseline before switching to
  `git apply`/`git diff` instead) grabbed and applied a *different, unrelated* stash entry
  (`hayro-jpeg2000/src/j2c/idwt.rs`, 514-line diff) belonging to what looks like a concurrently-running
  sibling perf-optimize candidate's own worktree, because `refs/stash` is a single ref **shared
  across all `git worktree` checkouts of the same repository** - it is not per-worktree. This was
  caught immediately (the popped diff touched a file this candidate never modified) and reverted
  in this worktree before it could affect anything measured here; this candidate's own change was
  independently recoverable from its already-saved `patch.diff`, so no measurement in this folder
  was affected. But the reverse direction is a real risk this session cannot rule out: if a
  sibling candidate's own concurrent `git stash`/`git stash pop` sequence in *its* worktree
  happened to pop *this* candidate's stash entry instead of its own, that sibling's own worktree
  could have silently gained an unrelated, uncommitted change it never asked for and might not
  notice. Recommendation for this skill's own tooling: never use `git stash` inside a
  perf-optimize worktree when multiple candidates may run concurrently against worktrees of the
  same repository - use `git diff > patch.diff` + `git checkout -- <files>` (and `git apply` to
  restore) instead, exactly as MEASUREMENT.md's own patch-capture step already does; this
  candidate's own `RESULT.md`/`justfile` were written using that safer pattern throughout, this
  note is flagging the tooling gap for future sessions, not something wrong with this candidate's
  own recorded numbers.
