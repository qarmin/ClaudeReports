# Optimization wins (Strong win / Small win only)

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results and
regressions) and the full methodology behind each number.

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 003 | Small win | `copy_to_slice_interleaved`'s stereo copy loop had a runtime-unknown interleave stride, blocking vectorization | +21/-0 | wav ~-4-5%, flac/ogg ~-1-3.5%, mp3 flat/small |
| 004 | Small win | `BitReaderLtr`/`BitReaderRtl::fetch_bits` missing `#[inline]`, un-inlined call in FLAC/ALAC's hottest loops | +2/-0 | alac ~2.6-3.2% (paired 4/4), flac ~0.5-1.5% (paired 8/8); ogg/wav/mp3 noise-level |
