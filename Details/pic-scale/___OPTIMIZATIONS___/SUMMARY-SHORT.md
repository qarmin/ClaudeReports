# Optimization wins (Strong win / Small win only)

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results and
regressions) and the full methodology behind each number.

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 005 | Strong win | AVX2 horizontal convolution tier for CbCr (2-channel) u16, bit_depth<=12 (over existing SSE4.1) | +882/-0 | -39% to -46% median wall-clock, single-core and unpinned; 3 of 4 runs consistent |
| 002 | Strong win | SSE4.1 horizontal convolution kernel for CbCr u16 (bit_depth<=12) | +373/-0 | -18% to -26% median ~-23%, single-core and unpinned |
| 003 | Strong win | AVX2 horizontal convolution kernel for planar (1-channel) u8 | +357/-0 | -14% to -20%, all 4 paired.sh runs consistent |
| 008 | Strong win | Candidate 005's AVX2 CbCr16 LB horizontal patch, re-measured under a wide-filter (`kernel_size=96`) criterion for `resize_cbcr` | +882/-0 | -44.7% to -47.4%, all 4 paired.sh runs consistent (9/9) |
| 004 | Small win | AVX2 horizontal convolution kernel for CbCr (2-channel) u8 | +505/-0 | -4% to -7% median, 4 of 5 repeats consistent |
| 010 | Small win | Candidate 004's AVX2 CbCr8 horizontal patch, re-measured under a wide-filter (`kernel_size~96`) criterion for `resize_cbcr` | +505/-0 | -8.7%/-5.7% unpinned (consistent); pinned repeats agree in direction but land just under the consistency bar |
| 014 | Small win | AVX-512VNNI dispatch branch for Plane/CbCr/RGB/RGBA u16 vertical/column convolution (bit_depth<=12) | +487/-1 | -5.0% to -9.0% median ~-7.5%, patched faster in all 4 repeats |
| 015 | Small win | avxvnni + AVX-512VNNI dispatch branches for signed-s16 Plane vertical/column convolution (bit_depth<=12) | +546/-1 | -5.4% to -12.5% median ~-9.4%, patched faster in all 6 repeats |
| 016 | Small win | Candidate 014's AVX-512VNNI vertical/column patch, re-measured against CN=2/3/4 bench cases | +487/-1 | CbCr10 -2.3% to -3.7%; RGB10 -1.8% to -4.3%; RGBA10 -6.5% to -7.4%, direction generalizes across CN |
