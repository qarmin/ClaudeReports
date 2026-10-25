# Scalar vs SSE4.2 vs AVX2 vs AVX-512 output

Open issue #966 says the CI 'avx512' job may not actually run AVX-512, so that backend (81 unsafe sites) is the least exercised. This machine has AVX-512.

Base commit 7f539c8 (jxl-rs main); the main tree was not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Five builds (no SIMD feature, sse42, avx, avx512, all-simd) of the same replay binary, outputs dumped as f32 and compared with tolerance | `harnesses/simdvar`; 109 test + conformance files (83 decodable) and 5811 fuzz-corpus inputs (163 decodable) | ok/err status identical for every input at every level; worst relative difference 4.6e-5 (avx, avx512, all) and 9.9e-4 (sse42) against the scalar build |

## Status

**Not reproducible**

**Fully reproducible**: n/a (no finding).

## Root cause

No behavioural difference between SIMD levels on the decodable inputs.

## What's not done / open questions

- Does not prove the AVX-512 build is really selected at run time on this CPU (the runtime dispatch was not inspected). aarch64/wasm32 backends were not run.
