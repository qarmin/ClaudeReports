# DataToken::ItemValue: Vec<u8> -> Arc<[u8]> (builds on candidate 001)

Candidate 001 (`InMemFragment: Vec<u8> -> Arc<[u8]>`) removed the per-write-call *clone* of each
pixel fragment when tokenizing an `InMemDicomObject` for writing, but its own RESULT.md flagged a
still-open remaining copy: `ItemValueTokens::next()` (`parser/src/dataset/mod.rs`) still did
`value.as_ref().to_owned()` to produce a `DataToken::ItemValue(Vec<u8>)` - a second real
full-buffer copy per fragment per write call, on top of the final `write_bytes` copy into the
output buffer. This candidate is a self-contained follow-on that includes candidate 001's patch as
a prerequisite (this fix has no effect without it - `DataToken::ItemValue` can only become a cheap
refcount bump if the fragment it's built from is already `Arc`-backed), consistent with how this
workspace's own candidates build on each other elsewhere (see e.g. candidate 003's guard extended
in candidate 006). Base commit `df2b9e4236298efa3fa37a1f360fc4bf284d586a`.

## What was tried

Changed `DataToken::ItemValue`'s field type from `Vec<u8>` to `Arc<[u8]>` (same type as
`InMemFragment`). Changed `ItemValueTokens<P>`'s bound from `P: AsRef<[u8]>` to
`P: AsRef<[u8]> + Into<Arc<[u8]>>` (and propagated the same bound to `DataElementTokens<I, P>`,
`IntoTokens for DataElement<I, P>`, and `IntoTokens for ItemValue<P>`, all in
`parser/src/dataset/mod.rs`, since they all funnel into the same generic `P`). Changed
`ItemValueTokens::Value(P)` to `Value(Option<P>)` (matching the existing `Start(Option<P>, bool)`
pattern) so ownership of `P` can be taken in `next()` and converted via `.into()` instead of
`.as_ref().to_owned()` - for `P = InMemFragment` (`Arc<[u8]>`, real-world case for
`InMemDicomObject`'s write path), `Arc<[u8]>: Into<Arc<[u8]>>` is the reflexive identity
conversion (free, no copy); for any other `P: AsRef<[u8]> + Into<Arc<[u8]>>>` (e.g. a plain
`Vec<u8>`), `.into()` does whatever `Vec<u8> -> Arc<[u8]>` normally costs (unchanged from before -
still a real conversion, just via `Into` instead of `AsRef::to_owned`). Every other construction
site of `DataToken::ItemValue` (read-side: `parser/src/dataset/{mod,read}.rs`, freshly-read
`Vec<u8>` bytes now wrapped via `.into()`; various test literals) was updated mechanically.

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s `write-only`/`parse`/`roundtrip` modes
against `color3d_jpeg_baseline.dcm` (6.14MB, JPEG-encapsulated, 120 fragments) - same fixture and
methodology as candidate 001, but **baseline for this candidate is candidate 001 alone** (not
pristine upstream), to isolate this incremental fix's own effect.

| Variant | write-only p50 (multi-core) | write-only p50 (single-core) | parse p50 | roundtrip p50 |
|---|---|---|---|---|
| baseline (candidate 001 alone) | 0.266/0.267 ms (2 rounds) | 0.290 ms | 1.957 ms | 4.042 ms |
| patched (001 + this fix) | 0.158/0.156 ms (2 rounds) | 0.163 ms | 1.955 ms (no change) | 0.492 ms |

## Result

**write-only: -40.8%/-41.5% multi-core (2 rounds, 9/9 both), -43.8% single-core (`taskset -c 0`,
9/9), all consistent** - a substantial further reduction on top of candidate 001's already-large
write-side win. **roundtrip (parse+write every iteration): -87.8%, 9/9, consistent** - much larger
than write-only's own improvement, because candidate 001 alone left `roundtrip` showing "no
consistent change" (the one remaining per-write copy this candidate removes was large enough to
offset write-only's isolated win once combined with a real per-iteration parse). **parse alone: no
consistent change** (-0.1%, 6/9 - not consistent), confirming the one-time read-side
`Vec<u8> -> Arc<[u8]>` wrap this candidate adds (`.into()` on freshly-decoded bytes) costs
effectively the same as before.

## Does it change behavior/output?

- **Bit-exact?** Yes. `sha256sum`/FNV-1a checksums of `dump`/`write-only`/`roundtrip` output are
  identical between baseline (candidate 001) and this candidate's patched build.
- **Is this a breaking change?** Yes, `DataToken` is a public enum
  (`dicom_parser::dataset::DataToken`) and `ItemValue`'s field type changing from `Vec<u8>` to
  `Arc<[u8]>` is a public-API break for any external code pattern-matching on it expecting
  `Vec<u8>` specifically (same category of break as candidate 001's `InMemFragment` change, and
  compounds with it - this candidate cannot be taken without candidate 001). The `ItemValueTokens<P>`/
  `DataElementTokens<I, P>` trait bound additions (`+ Into<Arc<[u8]>>`) are also a breaking change
  for any external `IntoTokens`/`DataDictionary`-adjacent implementation supplying a custom `P`
  that doesn't implement `Into<Arc<[u8]>>` (unlikely in practice, since `Vec<u8>` and `Arc<[u8]>`
  both do, but not guaranteed for an arbitrary third-party `P`).
- Project test suite: **537 passed / 0 failed / 6 ignored, both before (candidate 001 alone) and
  after (with this fix)** (full `cargo test --workspace`, including doctests) - no new failures.

## Why the result looks the way it does

This fix removes the second (and, after candidate 001, last) real full-buffer copy in the
write-a-decoded-object pipeline - `write_all`'s only remaining data movement is the actual
`write_bytes` serialization into the output buffer, which is unavoidable (the caller needs the
bytes somewhere). The disproportionately large `roundtrip` improvement (-87.8%, much bigger than
`write-only`'s own -41%) makes sense once the two candidates' costs are laid out per-iteration:
candidate 001 alone left `roundtrip` = (parse cost, including one Vec->Arc wrap per fragment) +
(write cost, including one Arc-clone-then-to_owned-copy per fragment) - the write-side copy this
candidate removes was large enough that its removal is far more visible in the *combined*
loop than it was in isolation, because with it gone, `roundtrip`'s total per-iteration cost is now
dominated by the parse-side JPEG-adjacent decode work rather than by two redundant pixel-buffer
copies stacked on top of it.

## Peak memory follow-up (2026-08-26)

Requested as a backlog item ("measure peak RSS for candidates 001/008/016, only ever timed
before") - see candidate 001's own matching follow-up section for the full methodology (new
`gen-fragmented`/`write-only-peak-mem` harness modes, 200 MB synthetic 40-fragment fixture,
`/proc/self/status` `VmHWM`, both glibc and mimalloc).

| Variant | glibc (3 runs) | mimalloc (3 runs) |
|---|---|---|
| baseline (neither fix) | ~818.3 MB avg | ~865.1 MB avg |
| candidate 001 alone | ~828.4 MB avg (no reduction) | ~867.3 MB avg (no reduction) |
| **this candidate (001+008 combined)** | ~629.6 MB avg (**-23.1%**) | ~654.2 MB avg (**-24.4%**) |

**This candidate is where the peak-RSS win actually materializes**, consistently under both
allocators, unlike candidate 001 alone (see its RESULT.md for why: 001 removes a real memcpy's
*CPU cost* but not its *peak byte count*, since the copy it eliminates was already draining
against the still-real `to_owned()` copy downstream of it). This candidate's own change - making
`DataToken::ItemValue` itself `Arc<[u8]>`, which removes that `to_owned()` copy - is what finally
leaves only the original pixel buffer live, with no transient duplicate at all. Bit-exact
(checksum identical to baseline on the synthetic fixture, same technique as the original
candidate's own correctness check).

## What's not done / open questions

- This candidate's `patch.diff` **includes candidate 001's patch as a prerequisite** (the two
  cannot be meaningfully separated - see "What was tried"). A reader wanting only this candidate's
  incremental change, not candidate 001's, would need to diff this patch against candidate 001's
  own `patch.diff` by hand.
- Not measured: whether the `+ Into<Arc<[u8]>>` bound addition on `DataElementTokens`/`IntoTokens`
  affects compile times or binary size meaningfully - unlikely given the bound is only ever
  monomorphized for the same concrete `P` types already in use, but not checked.
- Same breaking-change caveat as candidate 001 applies compounded: taking this patch means taking
  both public-API breaks together.
