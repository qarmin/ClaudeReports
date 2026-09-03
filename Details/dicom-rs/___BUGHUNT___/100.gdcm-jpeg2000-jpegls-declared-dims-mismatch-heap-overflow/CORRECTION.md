# Correction (from candidate 103)

Candidate 102 (checking GDCM's separate `NumberOfDimensions == 3` multi-frame decode branch for
the same bug) found that this candidate's original description of its finding as living in
GDCM's `NumberOfDimensions == 2` branch is inaccurate.

dicom-rs's own `gdcm-rs` C++ wrapper (`gdcm_wrapper.cc::c_decode_frames`, the function both
`gdcm_rs::decode_single_frame_compressed` and `gdcm_rs::decode_multi_frame_compressed` call into)
unconditionally calls `image.SetNumberOfDimensions(3)`, regardless of frame count - `dims[2]` is
just `1` for the single-frame case this candidate's repro used. There is no code path from this
crate's `gdcm` feature that ever reaches GDCM's true `NumberOfDimensions == 2` branch at all; this
candidate's original repro already went through the `NumberOfDimensions == 3` branch (with a
one-iteration loop, `Dimensions[2] == 1`), not a genuinely different "2-D" branch as originally
labeled.

**This does not change the original finding's status.** The two heap-buffer-overflow bugs
(`JPEG2000Codec::DecodeByStreamsCommon`'s write-overflow, `Bitmap::TryJPEGLSCodec`'s
read-overflow) are exactly as originally described - only the "which GDCM branch" part of the
root-cause narrative was wrong. Everywhere this `RESULT.md` says "the `NumberOfDimensions == 2`
branch", read it as "the `NumberOfDimensions == 3` branch with `Dimensions[2] == 1`" instead.

See candidate 103's own `RESULT.md` ("Root cause" section, point 2) for the full detail, and its
repro for confirmation that the same defect also fires with `Dimensions[2] > 1` (a genuine
multi-frame file), which was the actual open question this correction's source candidate set out
to answer.
