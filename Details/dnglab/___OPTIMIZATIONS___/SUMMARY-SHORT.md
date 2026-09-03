# Optimization wins (Strong win / Small win only)

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results and
regressions) and the full methodology behind each number.

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 006 | Strong win | X-Trans bilinear demosaic output buffer was `f32::NAN`-pre-filled (a full serial write) before every element got unconditionally overwritten; skip the fill via `MaybeUninit` | +31/-7 | multi-core ~20-45% faster wall time (criterion 412-419ms -> 282-340ms), bit-exact |
