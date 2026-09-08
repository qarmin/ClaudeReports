# Optimization wins (Strong win / Small win only)

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results and
regressions) and the full methodology behind each number.

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 003 | Small win | PNG DEFLATE compression level (Fast to Fastest) | +1/-0 | -20.7% isolated vs. Fast baseline (-88.5% vs. current Balanced default) |
| 002 | Strong win | PNG DEFLATE compression level (Balanced to Fast) | +5/-0 | -87.8% isolated, -48.6% end-to-end (real img.pdf) |
| 001 | Small win | Pixmap::take_unpremultiplied (float division to integer LUT) | +31/-14 | -25.4% isolated; within noise end-to-end |
