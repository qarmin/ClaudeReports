# dicom-json deserialize: avoid a double-deserialization pass via RawValue

`json/src/de/mod.rs`'s `DataElementVisitor::visit_map` captures the `"Value"` JSON field as an
eagerly-parsed `serde_json::Value` DOM tree while scanning the surrounding map (the target VR
isn't known until the whole object has been scanned, so the value can't be interpreted yet). Once
the VR is known, **every one of the 15 VR-dependent match arms then calls
`serde_json::from_value(value)`** to convert that already-built `Value` tree into the final typed
`Vec<T>`/`Vec<DicomJson<InMemDicomObject<D>>>`/etc. This is a genuine double pass over every
multi-valued element's content: once to build the generic `Value` tree (one `serde_json::Value`
allocated per array element, plus the outer `Vec<Value>`), and again to walk that tree and convert
each element into the concrete target type (a second `Vec<T>` allocation and a second full
iteration). Found while sweeping for a decode-side counterpart to candidate 010's own
InlineBinary/`RawValue` serialize-side fix - the same `RawValue` mechanism candidate 010 already
proved works in this crate applies naturally here too, on the opposite (deserialize) side.

Unlike candidate 010 (which only sped up the `InlineBinary` base64 string case), this fix applies
to **every** VR that carries a JSON `"Value"` array - the common case for essentially all
non-binary elements in a typical DICOM JSON document (numeric VRs, text VRs, `PN`, `AT`, and
nested `SQ` items), not a narrow edge case.

## What was tried

- `json/Cargo.toml`: enabled serde_json's `raw_value` feature (already used, and reverted, by
  candidate 010's serialize-side patch - confirmed working in this crate).
- `json/src/de/mod.rs`: changed the captured field from `Option<serde_json::Value>` to
  `Option<Box<serde_json::value::RawValue>>` (captures raw JSON text with a cheap "remember the
  byte span" operation, no per-element tree construction) and replaced all 15
  `serde_json::from_value(value).map_err(A::Error::custom)?` call sites with
  `serde_json::from_str(value.get()).map_err(A::Error::custom)?` - a single, direct
  text-to-typed-value deserialization once the VR is known, instead of two passes.

Harness: added a new `json-decode <file> <iters>` mode to `___OPTIMIZATIONS___/harness/
bench_pipeline` (parses the DICOM file once, encodes to JSON once outside the timed loop, then
repeatedly times `dicom_json::from_str` alone). The checksum is the FNV-1a hash of the
last-decoded object re-serialized back to JSON (a full content round-trip check, not just an
element count) - a stronger correctness signal than most of this workspace's other checksums,
chosen deliberately given how many VR branches this patch touches.

Fixtures: `seq_10k.dcm` (synthetic, 10,000 SQ items x 2 short UI strings each - candidate 005/016's
own fixture, chosen here to stress the text-array VR branch at scale), and two real
`dicom-test-files` fixtures already in this workspace's testdata (`liver.dcm`, 102KB;
`rtplan.dcm`, a real radiotherapy plan with nested sequences and numeric arrays).

| Variant | seq_10k.dcm p50 (2 rounds) | liver.dcm p50 | rtplan.dcm p50 |
|---|---|---|---|
| baseline (Value DOM + from_value) | 14.3-14.8 ms | 0.126 ms | 0.070 ms |
| patched (RawValue + from_str) | 5.2-5.7 ms | 0.088 ms | 0.040 ms |

## Result

**-64.0%/-64.5% on `seq_10k.dcm` across two independent rounds (9/9 paired wins both times,
multi-core), -64.4% single-core (`taskset -c 0`, 9/9) - fully consistent.** On real
`dicom-test-files` fixtures, smaller but still large and fully consistent wins: **-29.7% on
`liver.dcm`** (9/9) and **-42.3% on `rtplan.dcm`** (9/9). The magnitude scales with how many
multi-valued/text elements a file has relative to its total size (`seq_10k.dcm`'s 20,000 short
string values is close to a worst case for the old double-pass approach; `liver.dcm`'s smaller,
more scalar-heavy content sees a real but proportionally smaller win) - all three fixtures showed
a large, unambiguous win, not just the synthetic one.

## Does it change behavior/output?

- **Bit-exact?** Yes - the harness's checksum (FNV-1a of the full re-serialized JSON from the
  decoded object) is identical between baseline and patched builds on all three fixtures,
  confirming the actual decoded content (not just structure/element count) matches exactly.
- **Is this a breaking change?** No public API/type changes - `JsonDataElement`'s `value` field
  and the `DataElementVisitor`'s internal state are private to the `json` crate.
- Project test suite: `cargo test --release -p dicom-json -p dicom-dump`: **12 + 3 + 5 (json) + 4 +
  1 (dump) = 25 passed / 0 failed**, identical counts before and after - including
  `can_resolve_nan_and_inf_float` (verifies the `NumberOrText` textual-fallback parsing, e.g.
  `"NaN"`/`"-inf"`/`"inf"` string literals in a numeric VR's array, still resolves correctly
  through `RawValue`+`from_str`) and `can_resolve_bulk_data`/`can_parse_null_values` (edge cases
  unrelated to the `"Value"` array path, confirming no regression elsewhere). `cargo build
  --release --workspace`: clean, no errors, no new warnings. Also confirmed the crate-level
  `dicom_json::from_value` entry point (a *different* code path - deserializing directly from an
  already-constructed `serde_json::Value`, e.g. via the `json!` macro in tests) still works
  correctly through the same `RawValue`-based `MapAccess` visitor (serde_json's `RawValue` handling
  is implemented generically across all of its `Deserializer` impls, not just the str/reader ones).

## Why the result looks the way it does

Building a `serde_json::Value` tree for an array of N elements allocates N `Value` entries (each a
small tagged enum, cheap individually but not free) into a `Vec<Value>`, then `from_value` walks
that same Vec again, converting each `Value::Number`/`Value::String` into the target `T` and
collecting into a *second* `Vec<T>`. Capturing a `RawValue` instead skips the first allocation
entirely - it holds the original byte span more or less for free) - so the total work drops from
"parse text into a generic tree, then convert the tree into a typed vec" to "parse text directly
into a typed vec," removing one full allocation + one full iteration per multi-valued element,
which is precisely the mechanism candidate 010 already validated for the narrower `InlineBinary`
case.

## Peak-RSS follow-up (2026-08-26)

Added a `json-decode-peak-mem <file>` mode to the harness (same `/proc/self/status` `VmHWM`
technique as `write-only-peak-mem`/candidates 001/008/016's own follow-ups): a single
`dicom_json::from_str` call, then report peak RSS. Measured on `seq_10k.dcm` (3 runs each):

| Variant | run 1 | run 2 | run 3 | avg |
|---|---|---|---|---|
| baseline (Value DOM + from_value) | 56,312 kB | 56,200 kB | 56,480 kB | 56,331 kB |
| patched (RawValue + from_str) | 38,948 kB | 38,772 kB | 39,016 kB | 38,912 kB |

**-30.9% peak RSS reduction, consistent across all 3 runs (no overlap between the two
distributions).** This directly confirms the mechanism's predicted memory effect: eliminating the
intermediate `Vec<serde_json::Value>` DOM tree (one heap-allocated `Value` per array element, for
every multi-valued/text element in the document) removes real peak memory, not just CPU time -
consistent with `projects/dicom-rs.md`'s standing "measure both wall-clock and peak RSS for any
clone/allocation-avoidance mechanism" guidance, and a materially larger relative RSS win than this
workspace has seen on most of its other allocation-count-reducing candidates (016's own
allocation-count fix showed no RSS movement at all, since its avoided allocations were each tiny -
here, by contrast, the avoided `Value` tree scales with the whole document's element count, so the
RSS effect is large). Checksum identical to the wall-clock measurement's own checksum, confirming
this didn't silently change correctness.

## What's not done / open questions
- The `SQ` (sequence) branch benefits the same way (`serde_json::from_str::<Vec<DicomJson<
  InMemDicomObject<D>>>>`), but this recurses back into the same `visit_map`/`DataElementVisitor`
  machinery for each nested object - not separately isolated/measured from the flat-element case
  in this session's fixtures (both `seq_10k.dcm` and `rtplan.dcm` do exercise nested `SQ` content
  as part of their overall win, but no fixture isolated the `SQ` branch alone).
- `crate::from_value` (the top-level function taking an already-built `serde_json::Value`) was
  confirmed to still work correctly (see "Does it change behavior/output?" above) but its own
  performance was not benchmarked - it's a fundamentally different call path (JSON text was never
  a `&str` in the first place) where this fix's mechanism doesn't obviously apply the same way,
  out of scope for this candidate.
