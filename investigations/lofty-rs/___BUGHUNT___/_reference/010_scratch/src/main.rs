// Boundary/exhaustive sweep of the `esds` AudioSpecificConfig byte_a/byte_b bitfields consumed by
// `mp4a_properties` (candidate 010: frequency_index==0x0F explicit-sample-rate path + extended
// object_type computation). Builds a minimal real MP4/M4A file per combo and reads it back through
// the real `lofty::read_from_path` entry point, catching any panic.
//
// object_type == 36 (ALS) is skipped: that's the already-confirmed, separate bug from candidate
// 003 (`___BUGHUNT___/003.mp4_als_channels_overflow/`) and would drown this sweep's results.

use lofty::file::AudioFile;

use std::io::Write;
use std::panic;

fn atom(fourcc: &[u8; 4], content: &[u8]) -> Vec<u8> {
    let len = (content.len() + 8) as u32;
    let mut out = Vec::with_capacity(content.len() + 8);
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(fourcc);
    out.extend_from_slice(content);
    out
}

fn build_mp4(byte_a: u8, byte_b: u8, byte_c: u8, explicit_sample_rate: u32) -> Vec<u8> {
    let mut ftyp_content = Vec::new();
    ftyp_content.extend_from_slice(b"M4A ");
    ftyp_content.extend_from_slice(&0u32.to_be_bytes());
    let ftyp = atom(b"ftyp", &ftyp_content);

    let mut mdhd_content = Vec::new();
    mdhd_content.extend_from_slice(&0u32.to_be_bytes());
    mdhd_content.extend_from_slice(&0u32.to_be_bytes());
    mdhd_content.extend_from_slice(&0u32.to_be_bytes());
    mdhd_content.extend_from_slice(&1000u32.to_be_bytes());
    mdhd_content.extend_from_slice(&0u32.to_be_bytes());
    let mdhd = atom(b"mdhd", &mdhd_content);

    let mut hdlr_content = Vec::new();
    hdlr_content.extend_from_slice(&[0u8; 8]);
    hdlr_content.extend_from_slice(b"soun");
    let hdlr = atom(b"hdlr", &hdlr_content);

    let mut esds_content = Vec::new();
    esds_content.extend_from_slice(&0u32.to_be_bytes());

    esds_content.push(0x03);
    esds_content.push(0x00);
    esds_content.extend_from_slice(&[0u8; 3]);

    esds_content.push(0x04);
    esds_content.push(0x00);
    esds_content.push(0x40);
    esds_content.extend_from_slice(&[0u8; 8]);
    esds_content.extend_from_slice(&0u32.to_be_bytes());

    esds_content.push(0x05);
    esds_content.push(0x00);

    esds_content.push(byte_a);
    esds_content.push(byte_b);

    // Always append the bytes every branch might consume (byte_c, then the explicit 24-bit
    // sample rate, then one more trailing byte for the non-extended explicit-sample-rate path's
    // byte_c read) - harmless extra bytes are simply never read on branches that don't need them.
    let sr_bytes = explicit_sample_rate.to_be_bytes();
    esds_content.push(sr_bytes[1]);
    esds_content.push(sr_bytes[2]);
    esds_content.push(sr_bytes[3]);
    esds_content.push(byte_c);
    esds_content.push(byte_c);

    let esds = atom(b"esds", &esds_content);

    let mut mp4a_content = Vec::new();
    mp4a_content.extend_from_slice(&[0u8; 6]);
    mp4a_content.extend_from_slice(&1u16.to_be_bytes());
    mp4a_content.extend_from_slice(&0u16.to_be_bytes());
    mp4a_content.extend_from_slice(&[0u8; 2]);
    mp4a_content.extend_from_slice(&[0u8; 4]);
    mp4a_content.extend_from_slice(&2u16.to_be_bytes());
    mp4a_content.extend_from_slice(&16u16.to_be_bytes());
    mp4a_content.extend_from_slice(&[0u8; 2]);
    mp4a_content.extend_from_slice(&[0u8; 2]);
    mp4a_content.extend_from_slice(&(44100u32 << 16).to_be_bytes());
    mp4a_content.extend_from_slice(&esds);
    let mp4a = atom(b"mp4a", &mp4a_content);

    let mut stsd_content = Vec::new();
    stsd_content.extend_from_slice(&0u32.to_be_bytes());
    stsd_content.extend_from_slice(&1u32.to_be_bytes());
    stsd_content.extend_from_slice(&mp4a);
    let stsd = atom(b"stsd", &stsd_content);

    let stbl = atom(b"stbl", &stsd);
    let minf = atom(b"minf", &stbl);

    let mut mdia_content = Vec::new();
    mdia_content.extend_from_slice(&mdhd);
    mdia_content.extend_from_slice(&hdlr);
    mdia_content.extend_from_slice(&minf);
    let mdia = atom(b"mdia", &mdia_content);

    let trak = atom(b"trak", &mdia);
    let moov = atom(b"moov", &trak);

    let mut out = Vec::new();
    out.extend_from_slice(&ftyp);
    out.extend_from_slice(&moov);
    out
}

fn object_type_of(byte_a: u8, byte_b: u8) -> u16 {
    let object_type = byte_a >> 3;
    if object_type == 31 {
        32 + ((byte_a & 7) | (byte_b >> 5)) as u16
    } else {
        object_type as u16
    }
}

fn main() {
    panic::set_hook(Box::new(|_| {}));

    let mut panics = 0u64;
    let mut checked = 0u64;

    let sample_rate_values = [0x000000u32, 0x7FFFFFu32, 0x800000u32, 0xFFFFFFu32, 0x0000FFu32];
    let byte_c_values = [0x00u8, 0x7Fu8, 0x80u8, 0xFFu8];

    for byte_a in 0u16..256 {
        let byte_a = byte_a as u8;
        for byte_b in 0u16..256 {
            let byte_b = byte_b as u8;

            if object_type_of(byte_a, byte_b) == 36 {
                // Skip: candidate 003's already-confirmed ALS overflow, not this candidate.
                continue;
            }

            for &explicit_sample_rate in &sample_rate_values {
                for &byte_c in &byte_c_values {
                    checked += 1;
                    let bytes = build_mp4(byte_a, byte_b, byte_c, explicit_sample_rate);

                    let result = panic::catch_unwind(|| {
                        let mut cursor = std::io::Cursor::new(bytes);
                        let _ = lofty::mp4::Mp4File::read_from(
                            &mut cursor,
                            lofty::config::ParseOptions::new(),
                        );
                    });

                    if result.is_err() {
                        panics += 1;
                        eprintln!(
                            "PANIC: byte_a={byte_a:#04x} byte_b={byte_b:#04x} byte_c={byte_c:#04x} explicit_sample_rate={explicit_sample_rate:#08x}"
                        );
                    }
                }
            }
        }
    }

    std::io::stdout().flush().ok();
    println!("checked {checked} combinations, {panics} panicked");
}
