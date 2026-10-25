# Confirmed bugs (Status: Confirmed only)

Filtered from `SUMMARY.md` - see that file for every candidate investigated (including
Not-reproducible and Inconclusive ones) and the full methodology behind each finding.

| # | Class | Candidate | Fix lines | Root cause | Introduced by |
|---|---|---|---|---|---|
| 001 | other | zune-png decoder.rs doctest examples reference a phantom `post_process_image_apng` function and an undefined `decoder` variable | +18/-25 | APNG doc examples were never updated when the compositing API became the `ApngContext` struct, leaving both a nonexistent-import and an out-of-scope-variable doctest failure | 6332c284 |
| 002 | logic | zune-jpeg test compares fn-pointer addresses of `#[inline]` function `idct_int_1x1` across two call sites | +11/-4 | `#[inline]` lets rustc emit separate local copies of the same function per codegen unit under the default 16-codegen-unit release profile, so the two independently-obtained addresses legitimately differ | ca04b109 |
| 003 | logic | zune-inflate `StreamingDecoder::decode_chunk` never resets its chunk-relative read position between calls, corrupting any decode spanning more than one chunk | +32/-2 | `position` indexes into the current chunk slice but is never reset to 0 for a new chunk, so `refill` silently stops reading further input after the first call; zune-png avoids it only via its own undocumented manual workaround | 2901c37b |
