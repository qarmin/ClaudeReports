# InMemFragment: Vec<u8> -> Arc<[u8]>

`dicom_core::value::InMemFragment` (`core/src/value/mod.rs:32`) is the element type used for
encapsulated pixel-data fragments throughout `dicom-object`/`dicom-core`. `InMemDicomObject::write_all`/
`write_dataset` take `&self`, so producing an owned `DataToken` stream from a borrowed object
requires cloning every element - for an encapsulated `PixelSequence`, this means cloning every
pixel fragment's full byte buffer on every single `write_all` call. This was carried over as a
well-specified lead from a prior session's notes (`~/.claude/skills/perf-optimize/projects/dicom-rs.md`,
candidate 006) whose real artifacts weren't found on disk this session - re-implemented and
re-measured fresh here, at the same base commit `df2b9e4236298efa3fa37a1f360fc4bf284d586a`.

## What was tried

Changed `InMemFragment` from `Vec<u8>` to `std::sync::Arc<[u8]>`, so the per-element clone in
`InMemObjectTokens::next` becomes a refcount bump instead of a full byte-buffer copy. This is a
one-time `Vec<u8> -> Arc<[u8]>` conversion cost paid exactly once when a fragment is first created
(decode side: `object::collector.rs`/`object::mem.rs`'s `DataToken::ItemValue`/`LazyDataToken::LazyItemValue`
handling; construction side: `pixeldata::transcode`, `fromimage`, test code) - never repeated on
subsequent writes/clones.

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`, `write-only` mode (parses
`testdata/color3d_jpeg_baseline.dcm` - 6.14MB, JPEG-encapsulated, 120 frames - once outside the
timed loop, then repeats `obj.write_all()` into a fresh buffer each iteration, hashing the output).
Measured with `tools/paired.sh` (alternating A/B runs, RUNS=9), per MEASUREMENT.md's paired-comparison
discipline - this box has real background load from outside this container (`uptime` load average
23-26 on 24 cores throughout this session), so unpaired before/after medians would be unreliable.

| Variant | single-core p50 (taskset -c 0) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| baseline (`Vec<u8>`) | 5.554 ms | 4.905 ms / 5.503 ms (2 rounds) | full fragment-buffer clone every write |
| patched (`Arc<[u8]>`) | 0.523 ms | 0.365 ms / 0.360 ms (2 rounds) | refcount bump only |

## Result

**-90.6% single-core (taskset -c 0), 9/9 paired wins, consistent.**
**-92.6% and -93.5% multi-core (unpinned) across two independent rounds, 9/9 and 9/9 paired wins,
both consistent.** All three rounds agree in direction and rough magnitude (~90-94% reduction),
well above this box's own noise floor for this comparison (individual same-binary run-to-run
variance was under ~15% in these runs; the effect size here is roughly 6x that).

`roundtrip` mode (parse + write every iteration) was also checked and shows **no consistent
change** (+2.9%, only 2/9 paired wins - not consistent), as expected: re-parsing every iteration
pays the one-time `Vec -> Arc` conversion cost on every iteration too, which roughly offsets the
write-side saving. This fix only helps workloads that write the same in-memory object more than
once (or write once after a decode-then-hold pattern), not a tight parse-then-write-once loop.

## Does it change behavior/output?

- **Bit-exact?** Yes. `sha256sum`-equivalent (FNV-1a full-buffer hash) of `write_all` output is
  identical between baseline and patched builds on `color3d_jpeg_baseline.dcm`
  (`0xdef147e27b943369` both ways). `parse`/`roundtrip`/`dump` modes were also spot-checked and
  produced identical output.
- **Is this a breaking change?** Yes, for external consumers. `InMemFragment` is a *public* type
  alias (`dicom_core::value::InMemFragment`), used directly in this crate's own tests, `object`,
  `pixeldata::transcode`, `encoding::adapters` test helpers, and `parser`. The change itself is
  mechanical everywhere in this repo (rustc's own error messages point at every call site that
  needs `.into()`/`InMemFragment::from(...)`/`Arc::from(...)`, or a `.to_vec()` at a decode
  boundary - see `patch.diff`), but is a real semver break for any external consumer that
  pattern-matches on `Vec<u8>` specifically rather than going through `AsRef<[u8]>`/`Deref`.
  `encoding::adapters::RawPixelData::fragments` (a *different*, non-generic `SmallVec<[Vec<u8>; 2]>`
  field used by pixel-data-adapter implementers) was deliberately left as `Vec<u8>`, with a
  `.to_vec()` conversion added at the two call sites that build it from `InMemFragment`
  (`object::raw_pixel_data()`, and the equivalent test helpers in `encoding/src/adapters.rs` and
  `transfer-syntax-registry/tests/adapters/mod.rs`) - that field has a much wider blast radius
  (every transfer-syntax codec adapter) for a copy that only happens at decode time anyway, not on
  every `write_all`.
- Project test suite: **537 passed / 0 failed / 6 ignored, identical before and after** (full
  `cargo test --workspace`, including doctests). No new failures; no behavior change beyond the
  type signature change itself.

## Why the result looks the way it does

Turning a full-buffer memcpy into an atomic refcount increment removes real work that scales with
total pixel-data size, not a fixed per-call overhead - so the win is large (~90%+) specifically
because the test fixture is pixel-data-heavy (6.14MB, 120 JPEG-encapsulated fragments) relative to
the rest of the dataset. A file with little/no encapsulated pixel data would see a much smaller
absolute effect from this same change (nothing to clone cheaply or expensively). `roundtrip`'s null
result confirms the win is specifically a *write-without-reparsing* win, not a blanket parse+write
speedup - the fix targets exactly the "hold a decoded object and write it (possibly repeatedly)"
usage pattern, which matches how `roundtrip`'s prior session write-only isolation was designed to
test in the first place.

## Peak memory follow-up (2026-08-26, this candidate applied alone - real result, not the one predicted)

Requested as a backlog item ("measure peak RSS for candidates 001/008/016, only ever timed
before") and measured directly with the `/proc/self/status` `VmHWM` technique built for candidate
019, on a synthetic 200 MB encapsulated (40 fragments x 5 MB, via a new `gen-fragmented` harness
mode) fixture - `color3d_jpeg_baseline.dcm` (6.14 MB) turned out too small for either candidate's
memory effect to clear this process's own fixed footprint.

| Variant | glibc (3 runs) | mimalloc (3 runs, `--features diag-mimalloc`) |
|---|---|---|
| baseline (neither fix) | ~818.3 MB avg | ~865.1 MB avg |
| **this candidate alone** | ~828.4 MB avg (**+1.2%, i.e. no reduction**) | ~867.3 MB avg (**+0.3%, no reduction**) |
| candidate 008 (001+008 combined) | ~629.6 MB avg (**-23.1%**) | ~654.2 MB avg (**-24.4%**) |

**This candidate alone does not reduce peak RSS, under either allocator - consistent, not noise.**
This is not a contradiction of the wall-clock result above; it is explained by the "What's not
done" note below, confirmed rather than just predicted: `.cloned()` is a *lazy* per-element
iterator adapter, and the per-element token expansion that follows it (`elem.into_tokens_with_
options(options).collect()`) *drains* that clone fragment-by-fragment into a `VecDeque<DataToken>`
via `ItemValueTokens`'s own `value.as_ref().to_owned()` copy - so the clone this candidate makes
free (Arc refcount instead of a real `Vec` duplicate) was already being incrementally consumed and
released as the *second*, still-real `to_owned()` copy was being built up, not held fully alongside
it. Removing the first copy's *CPU cost* (a full memcpy pass) is real and large (the wall-clock
win above) - but it barely changes the *peak byte count* alive at once, because the second copy
was already the dominant contributor to that peak, with or without this fix. Candidate 008's
additional change (`DataToken::ItemValue` itself becoming `Arc<[u8]>`, eliminating that second
`to_owned()` copy too) is what actually moves peak RSS - see its own RESULT.md's matching
follow-up section for the same numbers from that side. Bit-exact in both configurations (checksum
identical across baseline/001-alone/008 on the synthetic fixture, same technique as the original
candidate).

## What's not done / open questions

- `DataToken::ItemValue` (`parser/src/dataset/mod.rs`) is itself still `Vec<u8>`, not
  `Arc<[u8]>`/`Cow<[u8]>` - so `ItemValueTokens::next()` still does one `value.as_ref().to_owned()`
  copy per fragment per write call, on top of what this fix removed. This candidate only removes
  the *clone-for-tokenization* copy; a single `write_all` call still does one real copy of the
  pixel data even after this fix (down from two). Fixing that too would need `DataToken` itself to
  hold something borrow/refcount-friendly, a bigger change since `DataToken` is shared with the
  read path (where a freshly-owned `Vec<u8>` from just-read bytes is the natural thing to
  produce) - left as an open backlog item in `IDEAS.md`.
- Not benchmarked: the one-time `Vec -> Arc` conversion cost itself, in isolation (how much of
  `roundtrip`'s null result is "write got faster, parse got correspondingly slower" vs. "both
  roughly unchanged, offsetting by coincidence"). The `parse`-only mode's numbers were not compared
  paired before/after in this candidate; only `write-only` and `roundtrip` were paired-measured.
- Whether this patch is actually worth taking, given the public semver break, is left to the
  reader - this file states only measured facts.
