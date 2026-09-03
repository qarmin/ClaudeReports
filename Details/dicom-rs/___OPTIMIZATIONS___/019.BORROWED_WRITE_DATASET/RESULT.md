# InMemDicomObject::write_dataset*: borrow element values instead of cloning them

Direct attempt at [GitHub issue #641](https://github.com/Enet4/dicom-rs/issues/641)
("`write_dataset_with_ts` and `write_dataset_with_ts_cs` should have versions that consume
`InMemDicomObject`"), following the maintainer's own suggested direction from the issue thread
rather than the issue body's proposed fix. Enet4 (repo owner) wrote: *"I think we can optimize
data set writing and benefit from the existing API by producing an iterator of shallow tokens
instead... Once this is done, there may be no need for a consuming version of these methods."*
This candidate implements exactly that for `InMemDicomObject::write_dataset`,
`write_dataset_with_ts_cs`, and `write_dataset_with_ts_cs_options` (`write_dataset_with_ts` and
`write_dataset_with_ts_options` already just delegate to `_cs`/`_cs_options`, so they're covered
for free) - **the recommended fix adds no new consuming (`self`) API**, matching what the
maintainer said he wanted instead. `patch.diff` also includes a working, measured implementation
of the issue's own originally-proposed alternative (a `self`-consuming duplicate), clearly marked
as comparison-only code to be deleted - see "Comparison against the issue's own proposed fix"
below for the numbers this produced.

## Root cause (confirmed, not assumed)

`IntoTokens for &InMemDicomObject` (`object/src/tokens.rs`) does `self.into_iter().cloned()` -
every element's value is cloned into an owned `DataToken` before it reaches the writer. For a
large native (non-fragmented) `PixelData` element - exactly the "1 GB object" scenario in the
issue - this means cloning the single largest buffer in the object just to hand it to a writer
that immediately serializes it and drops the clone.

Read `parser/src/dataset/write.rs`'s `write_impl` (the sole consumer of every `DataToken` during
writing) to check whether that clone was ever load-bearing: it isn't. Every payload-carrying
match arm only ever needs a borrow at the point of use -
`self.printer.encode_primitive_element(&last_de, value)` where `value: &PrimitiveValue`,
`self.printer.write_bytes(data)` where `data: &[u8]`, `self.printer.encode_offset_table(table)`
where `table: &[u32]`. The clone exists purely because `DataToken`'s variants hold owned data,
not because encoding itself needs ownership.

## What was tried

1. **`parser/src/dataset/write.rs`**: added three new public methods on `DataSetWriter` -
   `write_primitive_value_borrowed(&PrimitiveValue)`, `write_item_value_borrowed(&[u8])`,
   `write_offset_table_borrowed(&[u32])` - each a thin wrapper around the exact same
   `self.printer.*` call `write_impl` already made for the matching owned `DataToken` variant,
   just skipping the owned envelope. A new error variant `UnexpectedPrimitiveValueBorrowed`
   mirrors `UnexpectedToken` for the one case that can fail (`write_primitive_value_borrowed`
   called without a preceding element header) without needing to carry an owned `DataToken` for
   the error message.
2. **`object/src/tokens.rs`**: added `pub(crate) fn write_dataset_borrowed` and a private
   `write_element_borrowed` helper - a hand-written recursive walk over `&InMemDicomObject`
   (which already yields `&InMemElement` for free via its existing `IntoIterator` impl, no
   cloning needed to get there) that drives a `DataSetWriter` directly: structural tokens
   (`ElementHeader`, `SequenceStart`, `ItemStart`, etc. - all `Copy`, free to construct) still go
   through the existing `DataSetWriter::write`, but the three payload-carrying tokens go through
   the new borrowed methods instead. Recurses into nested `SQ` items via
   `DataSetSequence::items() -> &[I]` and into pixel fragments via
   `PixelFragmentSequence::fragments() -> &[P]`/`offset_table() -> &[u32]` - both already
   reference-returning accessors, no new plumbing needed in `dicom-core`.
3. **`object/src/mem.rs`**: `write_dataset`, `write_dataset_with_ts_cs` (both branches: with and
   without a dataset adapter), and `write_dataset_with_ts_cs_options` now call
   `crate::tokens::write_dataset_borrowed(&mut dset_writer, self, options)` instead of
   `dset_writer.write_sequence(self.into_tokens_with_options(options))`.

This is **additive-only** in the public API surface: three new `pub` methods on `DataSetWriter`,
one new error variant, zero signature changes to any existing public function
(`write_dataset*` still take `&self`, same as before). `IntoTokens for &InMemDicomObject` itself
is untouched and still works exactly as before for any other caller that uses it directly.

### Scope note: `FileDicomObject::write_all`/`write_to_file` are NOT covered by this fix

The actual most-used write entry point - `FileDicomObject<O>::write_all`/`write_to_file`
(`object/src/lib.rs`), which is what `obj.write_all(...)` calls and what candidates 001/008/016 in
this workspace measured - goes through the *same* cloning `(&self.obj).into_tokens()` call, but
on a **generic** `impl<O> FileDicomObject<O> where for<'a> &'a O: IntoTokens`. This candidate does
not touch that path, and here is exactly why: Rust has no stable specialization. Giving
`InMemDicomObject` a faster borrowed implementation while keeping the generic bound working for
every other `O: IntoTokens` implementor would require either the unstable `specialization`
feature, or replacing the bound with a new trait that every existing external `O` implementor
would have to explicitly opt into (a breaking change, since today they only need `IntoTokens`).
Neither is something a bounded perf candidate should decide unilaterally. This is a genuine
finding for the maintainer to weigh, not an oversight: **the generic `FileDicomObject<O>` design
is itself an obstacle to extending this fix to `write_all`/`write_to_file` without either an
unstable compiler feature or a breaking API change.** `write_dataset`/`write_dataset_with_ts*`
(the exact methods named in issue #641's title) are unaffected by this limitation because they are
plain inherent methods on the concrete `InMemDicomObject<D>` type, not part of the generic trait
bound.

## Correctness verification

**Bit-exact, checked three independent ways, all passing:**

1. **11 real `pydicom` test fixtures** (`MR_small`, `nested_priv_SQ` [nested private sequences],
   `SC_ybr_full_422_uncompressed`, `vlut_04`, `eCT_Supplemental`, `CT_small`,
   `color3d_jpeg_baseline` [encapsulated JPEG pixel data], `liver`, `reportsi`,
   `SC_ybr_full_uncompressed`, `rtplan` [deeply nested SQ, RT plan]), compared via a new
   `dump-dataset` harness mode (`write_dataset_with_ts` output, byte-for-byte) between a baseline
   build and a patched build: **`cmp` reports identical output on all 11**, sizes ranging from 116
   bytes to 6.1 MB.
2. **A synthetic edge-case object**, built in-process by a new `verify-borrowed-edge-cases`
   harness mode specifically to exercise corners no available real fixture combines: a
   `SpecificCharacterSet` element (triggers `charset_changed = true` ->
   `force_invalidate_sq_length`), a two-level nested `SQ`, and encapsulated pixel data with a
   non-empty offset table plus a mix of non-empty **and empty** fragments. Compared across **all
   four** `write_dataset*` methods (`write_dataset_with_ts`, `write_dataset_with_ts_cs`,
   `write_dataset_with_ts_cs_options`, `write_dataset`): all four checksums identical between
   baseline and patched.
3. **Project test suite**: `cargo test --workspace`: **537 passed / 0 failed, both before and
   after** (includes several existing `write_dataset_with_ts`/`write_dataset_with_ts_cs` tests in
   `object/src/mem.rs` that assert exact output bytes - these already caught any regression during
   development before the dedicated differential checks above were even written).

The implementation replicates two real, pre-existing quirks in the original token-producing code
(confirmed by reading it, not assumed) rather than "fixing" them as a drive-by: encapsulated pixel
data fragments and the offset table never have their own item length invalidated by
`force_invalidate_sq_length`, regardless of the surrounding object's `charset_changed` state
(the original code constructs their token iterator via `.into_tokens()`, which always uses
`IntoTokensOptions::default()`, not the enclosing `options`) - see the code comments in
`object/src/tokens.rs` for the exact lines this was confirmed against.

## What was tried (performance)

Benchmark: `___OPTIMIZATIONS___/harness/bench_pipeline`'s new `write-dataset-only <file> <iters>`
mode (parses once, then repeats `obj.write_dataset_with_ts(&mut out, ts)` - unlike the existing
`write-only` mode, which calls `write_all` and would NOT exercise this fix, per the scope note
above). Fixture: a synthetic ~490 MB native (non-encapsulated) `PixelData` object, generated with
the harness's existing `gen-mono` mode from `pydicom/CT_small.dcm` (row-multiplier 15000) -
deliberately sized to match issue #641's own "1 GB object" scenario as closely as practical.

| Variant | single-core p50 (`taskset -c 0`) | multi-core p50 | notes |
|---|---|---|---|
| baseline (cloning `IntoTokens`) | 482.7 ms | 473.2 ms / 469.0 ms (2 rounds) | |
| patched (borrowed write path) | 284.6 ms | 281.4 ms / 282.4 ms (2 rounds) | |

## Result

**-41.0% single-core, -40.5%/-39.8% multi-core across two independent rounds, 9/9 paired wins in
all three rounds, fully consistent.** `uptime` load average was 1.3-1.5 throughout (light
background load, but the paired alternating-run discipline and the fully consistent 9/9 verdicts
across all three rounds leave no ambiguity). This is directly the effect predicted by the root
cause: for a write dominated by one very large primitive value, eliminating that value's clone
removes close to half the total wall-clock cost (one full memcpy-sized pass over the buffer, plus
its allocation, no longer happens).

## Does it change behavior/output?

- **Bit-exact?** Yes - see "Correctness verification" above (three independent checks, all
  passing).
- **Is this a breaking change?** No. Purely additive: three new `pub` methods on `DataSetWriter`,
  one new error variant (`dicom_parser::dataset::write::Error::UnexpectedPrimitiveValueBorrowed`,
  and the enum is already `#[non_exhaustive]` so this can't break exhaustive external matches),
  zero changes to any existing public function signature. `write_dataset`,
  `write_dataset_with_ts_cs`, `write_dataset_with_ts_cs_options`, `write_dataset_with_ts`, and
  `write_dataset_with_ts_options` all keep taking `&self` exactly as before - **the recommended
  fix** deliberately does not add a `self`-consuming duplicate API, per the maintainer's own
  stated preference in the issue thread. `patch.diff` as shipped in this folder additionally
  contains `write_dataset_with_ts_cs2`/`write_dataset_with_ts2` - a `self`-consuming pair, clearly
  fenced with `BEGIN`/`END comparison-only code` comments - added solely to produce the comparison
  numbers below; whoever applies this patch for real should delete that fenced block, it is not
  part of the recommendation.
- Project test suite: **537 passed / 0 failed, both before and after**, including the comparison
  code (`cargo test --workspace`) - no existing test exercises the new comparison-only methods
  directly (nothing in the suite calls `write_dataset_with_ts_cs2`), so this counts as "doesn't
  break anything else", not as test coverage of the comparison code itself; the comparison code's
  own correctness was checked separately (see below).

## Why the result looks the way it does

The win size (~40%) is close to "half the wall-clock cost of the write disappears" because, for
this fixture, the write is completely dominated by one giant `PrimitiveValue` (the native pixel
data): the baseline path does (1) clone that buffer (`SmallVec`/heap allocation + full memcpy),
then (2) serialize it to the output writer, so total work is roughly `2 * buffer_size` worth of
memory traffic; the patched path does only (2). This matches the peak-memory argument from the
issue thread almost exactly, just observed as CPU/wall-clock time instead of peak RSS (this
candidate did not instrument peak memory directly - see "What's not done" below).

## Peak memory (the issue's actual original complaint, measured directly)

`/usr/bin/time -v` isn't available in this environment, so a small one-shot `write-dataset-peak-mem`
harness mode was added instead: one `write_dataset_with_ts` call on the same ~490 MB fixture,
then reads this process's own `VmHWM` (peak resident set size) from `/proc/self/status`.

| Variant | VmHWM (4 runs each) |
|---|---|
| baseline (cloning) | 1,926,056 / 1,925,096 / 1,925,648 KB (~1.88 GB) |
| patched (borrowed) | 1,446,440 / 1,446,140 / 1,446,456 KB (~1.41 GB) |

**-479,600 KB peak RSS (~468 MB), fully consistent across repeated runs (near-zero run-to-run
variance, unlike wall-clock timing).** This is within 0.5% of the fixture's own pixel data size
(490,519,808 bytes raw pixel bytes ≈ 479,023 KB) - i.e. the entire transient clone the issue
complained about is gone, almost exactly, not just reduced. This is a direct, quantified answer to
the issue's own framing ("1GB object -> 2GB peak memory... will this new approach also limit peak
memory usage to 1GB?"): yes, for this fixture, peak RSS drops by very close to the size of the
object's own largest element.

## Comparison against the issue's own proposed fix (consuming `self`)

Requested explicitly: how much of this candidate's benefit could the issue body's own original
proposal (a `self`-consuming duplicate, `write_dataset_with_ts_cs2`) have gotten on its own,
without the borrowed-writer machinery above? `patch.diff` includes a working implementation of
that alternative too - `write_dataset_with_ts_cs2`/`write_dataset_with_ts2`, clearly fenced with
`BEGIN`/`END comparison-only code` comments in `object/src/mem.rs` - **purely to produce this
comparison, not as a second recommended fix.** Whoever applies this patch for real should delete
that fenced block; keeping both a borrowed and a consuming write path in the same codebase forever
would be pure duplication for no benefit, once the numbers below are known. It works by consuming
`self` through the *existing*, untouched, owned `IntoTokens for InMemDicomObject<D>` impl (not
`&InMemDicomObject`) - since consuming ownership all the way through already avoids the clone for
the same underlying reason the borrowed path does (`InMemDicomObject<D>: IntoIterator` moves
elements out of the map, no `.cloned()` needed), this was expected to get most of the same benefit
with far less new code. Measured on the same ~490 MB fixture, same harness, same paired-run
discipline as above (new harness modes: `write-dataset-only-consuming`, `dump-dataset-consuming`,
`write-dataset-peak-mem-consuming` - each parses a fresh object per call, since the consuming API
can't be reused, but only the write call itself is timed, so the comparison against the reused-
object borrowed loop stays fair).

**Bit-exact against the borrowed path too**: `dump-dataset` and `dump-dataset-consuming` produce
identical output (`cmp` reports no difference).

**Peak RSS: statistically identical to the borrowed path**, both ways: 1,445,124 / 1,445,160 KB
(borrowed) vs. 1,443,824 / 1,444,096 KB (consuming) - a ~1 MB difference, well within noise. This
confirms Enet4's underlying claim in the issue thread: it is genuinely the *clone* that drove peak
memory, not *how* the clone is avoided - borrowing and consuming are equally effective at solving
the issue's original complaint.

**Wall-clock: consuming is consistently, measurably slower than borrowing** - not faster, not
equal:

| Variant | single-core p50 (`taskset -c 0`, 9 runs) | multi-core p50 (9 runs) |
|---|---|---|
| borrowed (this candidate's real fix) | 288.6 ms | 283.1 ms |
| consuming (issue's own proposal) | 321.7 ms | 315.7 ms |

**+11.5% slower than the borrowed path, 9/9 paired losses in both single-core and multi-core
rounds, fully consistent.** Consuming is still a large win over the original cloning baseline
(~470-490 ms) - roughly **-33%** - just a meaningfully smaller one than borrowing's ~-40%. The
likely cause (not proven with a profiler in this candidate, stated as the leading hypothesis, not
fact): `write_dataset_with_ts_cs2` reaches the writer through the *existing*, more general
token-producing machinery (`InMemObjectTokens` / `DataElementTokens` / `FlattenTokens` / their
`VecDeque`-buffered, enum-state-machine `Iterator::next()` implementations in
`parser/src/dataset/mod.rs`), which was written to also serve the *reading* side's generic
`IntoTokens` abstraction and has more layers of indirection than this candidate's hand-written
`write_element_borrowed`, which calls `DataSetWriter`'s encode methods close to directly. Neither
path clones the pixel data - the timing gap is abstraction overhead on top of an equally
clone-free base, not a difference in what gets copied.

**Net comparison**: the borrowed approach (this candidate's actual fix) wins on both dimensions
tested - equal peak-RSS reduction, and a real wall-clock edge on top - while also not requiring a
second, `self`-consuming public API next to the existing borrowing one. This is an empirical,
not just principled, confirmation that Enet4's suggested direction was the better one of the two
under consideration in the issue thread.

## What's not done / open questions

- **`FileDicomObject::write_all`/`write_to_file` are not covered** - see the dedicated scope note
  above. This is the more commonly used entry point in practice (preamble + meta group + dataset
  in one call) and still pays the full clone cost. Extending this fix there requires a maintainer
  decision (accept a breaking trait-bound change on `FileDicomObject<O>`, or wait for stable
  specialization) that is out of scope for a single perf candidate to make unilaterally.
- The three new `DataSetWriter` methods are individually straightforward but their combined
  correctness rests entirely on `write_element_borrowed`/`write_dataset_borrowed` in
  `object/src/tokens.rs` calling them in exactly the right sequence relative to
  `DataSetWriter::write`'s structural-token bookkeeping (`last_de`, `seq_tokens` stack) - this is
  the same contract `write_impl` already relies on internally, just now also relied upon from a
  second, external call site. Not exposed as a public trait/generic abstraction (deliberately
  scoped to `InMemDicomObject` only, per the scope note) - a hypothetical future generalization to
  other `I: IntoTokens` implementors would need to turn this into a proper trait, not copy-paste
  the recursive function.
