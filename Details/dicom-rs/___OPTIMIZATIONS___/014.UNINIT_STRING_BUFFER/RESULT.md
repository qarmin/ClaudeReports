# StatefulDecoder's scratch string buffer: skip zero-init before read_exact (7 sites)

Direct follow-on to candidate 009's discovery (`read_value_ob`'s `smallvec![0u8; len]` immediately
overwritten by `read_exact`), found while re-profiling `parse` on `seq_10k.dcm` (candidate 005's
text-heavy synthetic fixture, built for a different purpose but useful here too). The exact same
anti-pattern - zero-initializing a buffer that gets fully overwritten a few lines later - exists at
**7 separate call sites** in `parser/src/stateful/decode.rs`, all sharing `self.buffer` (a
persistent `Vec<u8>` scratch buffer reused across calls, unlike `read_value_ob`'s fresh
per-call `SmallVec`): `read_value_strs`, `read_value_str`, `read_value_da`, `read_value_ds`,
`read_value_dt`, `read_value_is`, `read_value_tm` - i.e. every string-shaped VR (multi-valued
strings, single strings, dates, decimal strings, datetimes, integer strings, times). This covers a
much broader and more commonly-hit set of DICOM value types than `read_value_ob` alone (which only
covers `OB`/raw-byte values).

## What was tried

Added a `resize_buffer_uninit(&mut self, len: usize)` method that does
`self.buffer.clear(); self.buffer.reserve(len); unsafe { self.buffer.set_len(len) }` instead of
`self.buffer.resize_with(len, Default::default)` - skipping the zero-fill of any newly-added
capacity, since every call site immediately follows this with a full `read_exact(&mut self.buffer)`
that overwrites every byte (or errors, in which case the buffer is never read as meaningful data).
Sound for the same reason as candidate 009: `u8` has no `Drop` glue and every bit pattern is a
valid `u8`, so an unwritten tail is never unsound to *hold*, only unsound to *read as meaningful
data* before it's written - which no caller here does. Verified with **Miri**
(`cargo +nightly miri test -p dicom-parser --lib`, `MIRIFLAGS=-Zmiri-disable-isolation`): all 49
tests pass with no UB reported.

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s `parse` mode against `seq_10k.dcm` (candidate
005's synthetic fixture - 10,000 sequence items x 2 `UI`-VR strings = 20,000 small string reads,
all going through `read_value_strs`) and `pydicom/liver.dcm` (a real, mixed-content native file,
less string-heavy but with real Date/Decimal-String elements in its header).

| Variant | seq_10k.dcm parse p50 | liver.dcm parse p50 |
|---|---|---|
| baseline (`resize_with`, zero-init) | 8.169/8.119 ms (2 rounds) | 0.026 ms |
| patched (`resize_buffer_uninit`) | 7.831/7.792 ms (2 rounds) | 0.024 ms |

## Result

**-4.1%/-4.0% on `seq_10k.dcm` across two independent rounds, 9/9 paired wins both times,
consistent. -5.7% on `liver.dcm`, 9/9, consistent.** A real, modest, broadly-applicable win -
smaller in relative terms than candidate 009's `read_value_ob` fix (which targets one much larger
single buffer, a whole pixel-data element) but touching far more call sites and value types, so it
applies to essentially every real-world DICOM file (every file has some string-typed elements;
not every file has large `OB` values). `write-only`/`roundtrip` (which don't exercise this
read-only code path) show no reliable change, as expected (`write-only`: +1.7%, 1/9 - technically
"consistent" per the `wins<=1` rule but a small enough magnitude to plausibly be noise;
`roundtrip`: -1.6%, 6/9 - not consistent).

## Does it change behavior/output?

- **Bit-exact?** Yes. `sha256sum` of the fully parsed-then-dumped object is identical between
  baseline and patched builds on both fixtures.
- **Is this a breaking change?** No public API/type changes - `resize_buffer_uninit` is a private
  method on `StatefulDecoder`.
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures. Additionally verified with
  **Miri** (49/49 `dicom-parser` tests pass, no UB reported) - the same extra scrutiny applied to
  candidate 009's similar unsafe usage.

## Why the result looks the way it does

Same mechanism as candidate 009, applied more broadly: removing a memset that's immediately
overwritten removes real, if individually small, work from a code path hit by every string-typed
element in every DICOM file. The relative win is smaller than `read_value_ob`'s because each
individual string is typically much shorter than a pixel-data buffer (tens of bytes vs tens of
kilobytes) - the *fixed* cost of a memset call (setup/teardown) matters more here relative to the
*bytes-scaled* cost that dominates for a large buffer, which likely also explains why this didn't
show the same magnitude of win as `read_value_ob`. Unlike candidate 009's numeric-array sibling
sites (which regressed when given the same treatment), these string-buffer sites all share one
*reused* buffer via a plain `Vec<u8>` (not a generic `SmallVec<[T; N]>` monomorphized helper), which
plausibly avoids whatever generic-dispatch inlining cost caused that regression - not confirmed via
disassembly this candidate, but consistent with the pattern.

A third check (this candidate's own `just test`, from a fresh clone) reproduced the `seq_10k.dcm`
win cleanly (-4.8%, 8/9, consistent) but `liver.dcm` came back borderline (-6.5%, 7/9 - not
consistent by one pair) that run, against 9/9 twice in the original live-repo testing above - given
`liver.dcm`'s effect is small in absolute terms (~26 microseconds total), this is plausibly just
noise at that timescale on a fresh-clone build rather than a real inconsistency; the `seq_10k.dcm`
result (a larger, more clearly-separated effect) is the more load-bearing evidence for this
candidate's win either way.

## What's not done / open questions

- Not root-caused: exactly why this win (~4-6%) is smaller in relative terms than a naive
  read of "7 sites instead of 1" might suggest - no per-site breakdown was done to see whether some
  of the 7 (e.g. `read_value_strs`, hit 20,000 times on `seq_10k.dcm`) contribute disproportionately
  more than others (e.g. `read_value_da`/`read_value_tm`, likely hit only a handful of times per
  file).
- `object/src/meta.rs`'s `read_str_body` (file-meta UID/string reading, a similar but separate
  buffer) still has the same anti-pattern, per candidate 009's own open item - not addressed here
  either, since it's a different struct's buffer in a different crate.
