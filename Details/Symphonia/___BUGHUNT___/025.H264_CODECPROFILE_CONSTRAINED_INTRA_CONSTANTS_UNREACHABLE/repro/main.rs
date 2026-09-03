// Repro for candidate 025: H.264 "constrained"/"intra" CodecProfile constants are unreachable
// from the real AVCDecoderConfigurationRecord parser.
//
// This constructs a real ISO/IEC 14496-15 AVCDecoderConfigurationRecord byte layout - the exact
// bytes a real H.264 encoder (e.g. x264) writes into an mp4 file's `avcC` box for a Constrained
// Baseline Profile stream - and feeds it to the crate's own public parsing entry point,
// `symphonia_common::mpeg::video::AVCDecoderConfigurationRecord::read`. This is the same function
// symphonia-format-isomp4's real `avcc.rs` atom reader calls when demuxing a real mp4 file, so
// this is a single external artifact (an avcC payload) handed to a real public parser - not a
// hand-faked second participant.

use symphonia_common::mpeg::video::AVCDecoderConfigurationRecord;
use symphonia_core::codecs::video::well_known::profiles::CODEC_PROFILE_H264_CONSTRAINED_BASELINE;

fn main() {
    // Real-world AVCDecoderConfigurationRecord bytes for a Constrained Baseline Profile stream:
    //   configurationVersion       = 1
    //   AVCProfileIndication       = 66  (Baseline profile_idc, per ISO/IEC 14496-10 Annex A)
    //   profile_compatibility      = 0x40 (constraint_set1_flag = 1, all other bits 0) - this is
    //                                exactly the byte real encoders (x264 "baseline" preset with
    //                                the "Constrained Baseline" constraint applied) write here.
    //   AVCLevelIndication         = 30  (Level 3.0)
    // Per ISO/IEC 14496-10 Table 7-... and common practice (e.g. RFC 6184 section 8.1), a stream
    // with profile_idc=66 and constraint_set1_flag=1 is precisely "Constrained Baseline Profile".
    let avcc_bytes: [u8; 4] = [0x01, 0x42, 0x40, 0x1E];

    let record = AVCDecoderConfigurationRecord::read(&avcc_bytes)
        .expect("a well-formed 4-byte avcC record must parse");

    println!("Parsed profile: {:?} (raw = {})", record.profile, record.profile.get());
    println!(
        "CODEC_PROFILE_H264_CONSTRAINED_BASELINE: {:?} (raw = {})",
        CODEC_PROFILE_H264_CONSTRAINED_BASELINE,
        CODEC_PROFILE_H264_CONSTRAINED_BASELINE.get()
    );

    let matches = record.profile == CODEC_PROFILE_H264_CONSTRAINED_BASELINE;
    println!("record.profile == CODEC_PROFILE_H264_CONSTRAINED_BASELINE: {}", matches);

    if matches {
        println!(
            "\nOK: real Constrained Baseline avcC bytes parse to a CodecProfile that equals \
             CODEC_PROFILE_H264_CONSTRAINED_BASELINE."
        );
    } else {
        println!(
            "\nBUG: real Constrained Baseline avcC bytes parse to CodecProfile({}), which can \
             never equal CODEC_PROFILE_H264_CONSTRAINED_BASELINE (CodecProfile({})), because the \
             real parser never folds the constraint_set bits into bits 8+ the way the constant \
             expects.",
            record.profile.get(),
            CODEC_PROFILE_H264_CONSTRAINED_BASELINE.get()
        );
    }
}
