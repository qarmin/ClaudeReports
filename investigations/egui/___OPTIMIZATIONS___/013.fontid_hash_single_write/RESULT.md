# `FontId` hashed with one write (size bits + family kind), instead of separate float and enum writes

Follow-up of candidate 012: `FontId::hash` is called once per text section of every layout job (the syntax-highlighted code views in the demo have hundreds of sections). Packed `size` (with `OrderedFloat`'s NaN/-0.0 equivalence) and the `FontFamily` variant kind into one `u64` write; the name of `FontFamily::Name` is still hashed separately.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metric: wall-clock time.

## What was tried

Interleaved A/B of prebuilt criterion binaries (`tools/ab.sh`, pinned core). Ratios patched/base, 5 rounds:

| Benchmark | ratios |
|---|---|
| demo_no_tessellate | 0.9966, 0.9871, 0.9452, 0.9932, 0.9875 |
| text_layout_cached | 0.9963, 1.0075, 0.9946, 0.9945, 0.9955 |

(Round 3 of demo_no_tessellate has a noisy base sample, A = 342 vs ~317 in the other rounds.)

## Result

Within noise: demo_no_tessellate about -0.3% to -1.3% ignoring the noisy round, text_layout_cached -0.5% (4 of 5 rounds marginally faster). Single-section label hashing only saves one write; the multi-section code views were already helped most by candidate 012 (patched-012 vs patched-013 comparison was not meaningful because 013 alone does not contain 012).

## Does it change behavior/output?

- **Bit-exact?** Yes; only `FontId` `Hash` values change. Hash/Eq consistency preserved (family kind plus name for `Name`, same float equivalence as `OrderedFloat`).
- Breaking change: no.
- Tests: `cargo test -p epaint --all-features` 95 + 4 passed (1 ignored), same as unpatched.
- Real change to the tested project's source: `patch.diff` (+15/-2).

## Why the result looks the way it does

One write saved per section is ~1-2 ns; only a job with many sections shows it and the demo has only two such views.

## What's not done / open questions

- Combined with 012 not measured; expected to be a small addition for multi-section jobs.
