# Optimization wins (Strong win / Small win only)

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results and
regressions) and the full methodology behind each number.

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 001 | Strong win | InMemFragment: Vec\<u8\> -> Arc<[u8]> | +58/-47 | -90% write-only on encapsulated pixel data |
| 002 | Strong win | YBR colorspace conversion: f32+floor -> fixed-point | +44/-36 | -55-64% single-core, -17-19% multi-core |
| 003 | Small win | rayon threshold+guard for colorspace conversion | +82/-50 | -32% below 16,384px threshold, no change above |
| 008 | Strong win | DataToken::ItemValue: Vec\<u8\> -> Arc<[u8]> (builds on 001) | +88/-68 | -87% roundtrip, -95% write-only vs pristine (001+008 combined) |
| 009 | Small win | skip zero-init in read_value_ob before read_exact | +20/-2 | -2.6 to -3.8% parse on native (non-JPEG) files |
| 010 | Small win | JSON InlineBinary via RawValue (skip escape scan) | +30/-4 | -11 to -12% json-encode with large inline binary content |
| 011 | Small win | RLE decode: remove per-element bounds checks | +10/-3 | -9 to -11% RLE Lossless decode |
| 012 | Strong win | Lut::new_with_fn rayon threshold+guard | +41/-18 | -37% for 8-bit (256-entry) LUT construction |
| 013 | Strong win | RLE decode_frame: remove per-element bounds checks | +9/-3 | -27 to -31% single-frame RLE Lossless decode |
| 014 | Small win | StatefulDecoder string buffer: skip zero-init (7 sites) | +28/-7 | -4 to -6% parse for string-heavy DICOM files |
| 018 | Small win | Lut::new_rescale_and_normalize: fuse two-pass min/max fold into one | +3/-2 | -15-18% for VoiLutOption::Normalize on a 1M-pixel image |
| 019 | Strong win | InMemDicomObject::write_dataset*: borrow instead of clone (issue #641) | +230/-16 | -40% write time, -25% peak RSS on a ~490MB native-pixel object |
| 020 | Small win | PDataWriter (DIMSE sender): skip copy-into-buffer for large writes | +46/-7 | -3 to -6% isolated from socket noise, 9/9; real loopback TCP: inconclusive |
| 021 | Strong win | Transcode::decode_inline: consume decoded pixels instead of `.data().to_vec()` | +17/-1 | -20 to -24% on a 120-frame JPEG-to-native transcode, 9/9 |
| 022 | Strong win | unpack_overlay_bits: table-driven byte unpack instead of bit-at-a-time | +27/-3 | -88% on a 4096x4096 overlay plane decode, 9/9 |
| 023 | Strong win | Lut::new_rescale_and_normalize: fused fold + rayon threshold-guard, slice signature | +60/-20 | -44% multi-core, -23% single-core (fusion alone) on a 1M-sample VoiLutOption::Normalize |
| 026 | Strong win | dicom-json deserialize: avoid double-deserialization via RawValue + from_str | +21/-17 | -64% on a 10k-item sequence fixture, -30 to -42% on real dicom-test-files fixtures |
| 029 | Small win | AsyncPDataWriter: direct dispatch for large single writes (async counterpart to candidate 020) | +130/-0 | -2.3 to -2.7% at 300MB isolated in-memory, no effect at 100MB (scale-dependent) |
| 035 | Strong win | AsAttributeTags (AT-VR list) JSON serialization: hand-written hex table instead of format!/Formatter (candidate 027's AT-VR half, previously applied but never benchmarked) | +28/-6 | -72.6 to -71.9% at 50,000 tags, -43.5% at a realistic 10-tag element size |
| 036 | Strong win | dicom-json deserialize RawValue fix (candidate 026), isolated to its SQ-recursion path at equal element count vs. a flat-element equivalent | +21/-17 | -57.9 to -65.7% for SQ-nested content vs. -12.1 to -18.9% for flat content, same element count |
| 037 | Small win | dicom-json deserialize RawValue fix (candidate 026), measured for the from_value entry point (never has raw JSON text) | +21/-17 | -23.2 to -37.3% across 3 rounds, smaller/noisier than the from_str path's own -57 to -66% |
| 042 | Small win | StatefulDecoder::read_value_strs single-value fast path (decode-side counterpart to candidate 024, which found null) | +20/-0 | -8.8 to -15.5% across short/long/real-world fixtures, all consistent |
| 043 | Small win | Same single-value fast path extended to read_value_da/ds/dt/is/tm (DA/DS/DT/IS/TM VRs) | +50/-0 | -5.9 to -8.3% across 3 rounds, all consistent |
| 045 | Strong win | Hand-written SSE4.1/SSSE3 SIMD rewrite of YCbCr colorspace conversion (candidate 002's fixed-point loop never auto-vectorized) | +309/-48 | -71.6 to -71.9% single-core, -28.5 to -29.6% multi-core |
