// End-to-end repro: a crafted MP4/M4A file on disk carries an `stsd` -> `fLaC` sample entry
// whose child `dfLa` atom declares a size of exactly 8 (`ATOM_HEADER_LEN`, the 4-byte atom size
// field + the 4-byte "dfLa" fourcc, and nothing else). `AtomInfo::read` only guarantees
// `len >= ATOM_HEADER_LEN` (8) for any atom, so this is a perfectly legal (if useless) `dfLa`
// atom as far as the general atom parser is concerned.
//
// The file is read back with lofty's real public entry point, `lofty::read_from_path`, which
// drives the actual `Mp4File`/`AtomReader`/`read_stsd`/`flac_properties` parsing path - the same
// path any caller opening an untrusted `.m4a`/`.mp4` file goes through. This is the "malformed
// file handed to a parser's real public entry point" case from CONFIRMATION.md section 11: the
// crafted bytes on disk ARE the genuine external input, there is no second participant to fake
// here.
//
// `lofty/src/mp4/properties.rs`'s `flac_properties` does:
//     if dfla.len - 12 < 18 {
// which subtracts 12 from the atom's on-disk `len` *before* checking it's even that large,
// panicking with "attempt to subtract with overflow" under the default dev/test profile's
// `overflow-checks = true` whenever `dfla.len < 12`, and silently wrapping to a huge `u64` in
// release builds (which then fails the STREAMINFO read for an unrelated reason instead).

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

    // fLaC sample entry: 28 bytes of fixed `stsd` sample-entry fields (reserved, data reference
    // index, entry version, revision level, vendor, channels, sample size, compression id,
    // packet size, sample rate), followed by a single, malformed `dfLa` child atom.
    let mut flac_entry_content = Vec::new();
    flac_entry_content.extend_from_slice(&[0u8; 6]); // reserved
    flac_entry_content.extend_from_slice(&1u16.to_be_bytes()); // data reference index
    flac_entry_content.extend_from_slice(&0u16.to_be_bytes()); // stsd entry version 0
    flac_entry_content.extend_from_slice(&[0u8; 2]); // revision level
    flac_entry_content.extend_from_slice(&[0u8; 4]); // vendor
    flac_entry_content.extend_from_slice(&2u16.to_be_bytes()); // channels (initial, unrelated field)
    flac_entry_content.extend_from_slice(&16u16.to_be_bytes()); // sample size
    flac_entry_content.extend_from_slice(&[0u8; 2]); // compression id
    flac_entry_content.extend_from_slice(&[0u8; 2]); // packet size
    flac_entry_content.extend_from_slice(&(44100u32 << 16).to_be_bytes()); // sample rate, 16.16 fixed

    // The overflow target: a `dfLa` atom whose declared size is exactly `ATOM_HEADER_LEN` (8) -
    // just the size field and fourcc, no version/flags, no STREAMINFO content at all.
    flac_entry_content.extend_from_slice(&8u32.to_be_bytes());
    flac_entry_content.extend_from_slice(b"dfLa");

    let flac_entry = atom(b"fLaC", &flac_entry_content);

    // stsd
    let mut stsd_content = Vec::new();
    stsd_content.extend_from_slice(&0u32.to_be_bytes()); // version+flags
    stsd_content.extend_from_slice(&1u32.to_be_bytes()); // num sample entries
    stsd_content.extend_from_slice(&flac_entry);
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
    let path = dir.join("mp4_flac_dfla_len_underflow_repro.m4a");
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

    // Only reached if the underflow did NOT panic (i.e. the bug is fixed).
    println!("RESULT: no panic. properties = {:?}", properties);
}
