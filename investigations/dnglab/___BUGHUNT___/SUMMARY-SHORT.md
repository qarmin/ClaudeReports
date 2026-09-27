# Confirmed bugs (Status: Confirmed only)

Filtered from `SUMMARY.md` - see that file for every candidate investigated (including
Not-reproducible and Inconclusive ones) and the full methodology behind each finding.

| # | Class | Candidate | Fix lines | Root cause |
|---|---|---|---|---|
| 001 | memory-safety | ppg.rs PPG demosaic \`.take(h - 6)\`/\`.take(w - 6)\` underflow guard for sub-6px images | +6/-6 | usize underflow removes the bound protecting unchecked raw-pointer pixel reads; ASAN-confirmed heap-buffer-overflow read in release |
| 002 | crash | ppg.rs \`interpolate_borders\` border-jump guard for 4-5px-wide images | +3/-1 | border-skip jump moves backward for narrow images, looping forever |
| 003 | crash | x3f.rs \`X3fFile\`/\`X3fDirectory\`/\`X3fImage::new\` bounds-check attacker-controlled slice offsets | +12/-3 | unchecked file-buffer slice at an offset read straight from file bytes panics on any truncated X3F file |
| 004 | crash | decompressors/mod.rs \`decompress_lines*\`/\`decompress_strips*\` zero-width/zero-stripsize guard | +12/-0 | file-tag-derived width of 0 becomes rayon's chunk size, which panics unconditionally |
| 005 | crash | ciff/mod.rs \`CiffEntry::new\`/\`CiffIFD::new\` bounds-check attacker-controlled directory offsets | +12/-2 | CRW directory entry's declared size/offset used to slice the file buffer with no bounds check |
| 006 | crash | bmff/vendor.rs \`VendorBox::read_box\` checked_add guard on offset+size | +6/-2 | unchecked \`u64\` add overflow panics in debug; reachable from any file via get_decoder's un-gated BMFF fallback |
| 007 | crash | imgop/mod.rs \`Rect::as_ltrb_offsets\`/\`as_tlbr_offsets\` saturating_sub guard | +12/-2 | DNG ActiveArea larger than the image underflows; matches real-user-reported issue #582 |
| 008 | crash | rawimage.rs \`RawImage::calc_black_levels\` fold \`image.is_empty()\` into the existing \`x == 0\` early return | +1/-2 | assert!(!image.is_empty()) fires whenever a file-tag-derived height==0 makes the decoded image empty but the CFA pattern is still non-degenerate |
| 010 | crash | imgop/mod.rs \`Rect::new_with_points\` saturating_sub instead of hard \`assert!(p1<=p2)\` | +6/-4 | DNG ActiveArea tag with top>bottom or left>right panics unconditionally in raw_metadata alone; matches real-user-reported issue #573 |
| 011 | crash | nef/lensdata.rs \`from_makernote\` return \`Result::Err\` instead of \`todo!()\` for an unrecognized LensData version | +1/-1 | todo!() panic bypasses raw_metadata's existing graceful-degradation Err-handling match arm; matches real-user-reported issue #583 |
| 012 | crash | cfa.rs \`CFA::new\` fallback to an empty/invalid pattern instead of \`panic!()\` on an unexpected length, plus a dng.rs divide-by-zero guard | +14/-2 | CFAPattern TIFF tag's file-controlled element count reaches CFA::new with no prior length validation; matches real-user-reported issue #586 |
| 013 | crash | pumps.rs \`BitPumpJPEG::peek_bits\` bounds-check before indexing \`pos+1\` for the JPEG stuffing byte | +1/-1 | truncated entropy segment ending in a bare 0xFF indexes one byte past the buffer; matches real-user-reported issue #587 |
| 014 | crash | tiff/ifd.rs \`IFD::new\` \`Value::Unknown\` SubIFD-offset lookup bounds check on \`offsets[0]\` | +5/-1 | empty offsets Vec indexed at [0] when a TIFF entry declares an unrecognized type code with count 0, reachable via RAF's main-IFD parse; matches real-user-reported issue #569 |
| 015 | crash | rw2/v4decompressor.rs \`BitPumpPanasonic::peek_bits\` and \`decode_panasonic_v4\` strip-offset bounds check | +11/-3 | unchecked LEu16 read walks past the RW2 strip buffer once a truncated file runs out of data; matches real-user-reported issue #571 |
| 018 | crash | x3f.rs \`X3fDecoder::raw_image\` \`caminfo.doffset + 6\` slice bounds check | +7/-2 | unbounded slice panics when an IMA2 image block's 28-byte header ends within the file's last 5 bytes |
