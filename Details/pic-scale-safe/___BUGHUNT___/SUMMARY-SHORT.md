# Confirmed bugs (Status: Confirmed only)

Filtered from `SUMMARY.md` - see that file for every candidate investigated (including
Not-reproducible and Inconclusive ones) and the full methodology behind each finding.

| # | Class | Candidate | Root cause |
|---|---|---|---|
| 002 | crash | `has_non_constant_alpha`/`*_f32_impl` zero-width panic | `width == 0` on a non-empty buffer makes the `chunks_exact` chunk size `0`, which panics |
| 004 | crash | `div_by_2pn_m1` `u32` overflow on out-of-range 16-bit input | Full-range `u16` premultiply input under a small `bit_depth` overflows the `u32` accumulator (panics debug, wraps release) |
| 013 | logic | `resize_nearest` `k_x`/`k_y` truncate to 0 for a source dimension `>= 2^32` | `u64` left-shift silently drops high bits, collapsing every destination pixel onto source index 0; reachable with a ~4 GiB buffer |
| 014 | crash | `has_non_constant_alpha`/`*_f32_impl` `width * CHANNELS` overflow | Separate plain-multiplication overflow from candidate 002, panics or silently mis-scans with a large `width` |
| 015 | crash | `generate_weights` `usize->f32->usize` precision loss lets bounds exceed the source size | Round-trip cast on `in_size > 2^24` can round up, causing a slice-OOB panic in the horizontal convolution; reproduced with a ~16 MB image |
