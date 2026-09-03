//! Verification harness for candidate 038 (APEV2_ITEM_LEN_AND_KEY_UNBOUNDED).
//!
//! The suspicion under test: `ApeItem::read` in `symphonia-metadata/src/ape.rs` takes a raw 32-bit
//! item length straight out of the file and hands it to `read_boxed_slice_exact` /
//! `read_utf8_value`, `read_key` grows a `String` until a NUL byte with no cap, and
//! `ApeHeader::num_items` (also a raw `u32`) drives the item loop with nothing cross-checking it
//! against the tag's declared `size` - so a crafted tag should be able to force a multi-GB
//! allocation or an unbounded read.
//!
//! This harness drives real crafted files through the real public entry points
//! (`ApeReader::try_new` + `MetadataReader::read_all`, and the real
//! `symphonia::default::get_probe().probe()` for the end-to-end path) and reports, for each case,
//! the peak RSS actually reached against the size of the file that produced it.
//!
//! Each case runs in its own child process, because `VmHWM` is a per-process high-water mark and
//! would otherwise be permanently polluted by whichever case ran first.
//!
//! Expected output if the suspicion is WRONG (which is what was actually observed):
//!   - `item-len-max` / `key-no-nul`: peak RSS ~1x the file size, not 4 GiB. `read_boxed_slice_exact`
//!     is hardened (progressive growth + `try_reserve_exact`), so the declared length only bounds
//!     the read from above; the real bound is how many bytes the stream actually has.
//!   - `num-items-past-extent` vs `num-items-honest`: the same peak RSS, so the missing
//!     `num_items`-vs-`size` cross-check costs nothing extra on top of an honestly-declared tag.
//!   - `id3v2-baseline`: a comparable per-byte amplification for an equally sized ID3v2 tag, i.e.
//!     the amplification is a generic property of "one `Tag` per parsed item/frame", not an APE bug.

use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};

use symphonia::core::formats::probe::Hint;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, MetadataReader};
use symphonia::default::meta::{ApeReader, Id3v2Reader};

const HAS_HEADER: u32 = 0x8000_0000;
const HAS_FOOTER: u32 = 0x4000_0000;
const IS_HEADER: u32 = 0x2000_0000;

const CASES: &[&str] = &[
    "item-len-max",
    "key-no-nul",
    "num-items-past-extent",
    "num-items-honest",
    "id3v2-baseline",
    "probe-trailing",
    "probe-leading",
];

fn vm_hwm_kb() -> u64 {
    let s = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    for line in s.lines() {
        if let Some(rest) = line.strip_prefix("VmHWM:") {
            let digits: String = rest.chars().filter(|c| c.is_ascii_digit()).collect();
            return digits.parse().unwrap_or(0);
        }
    }
    0
}

fn ape_hdr(version: u32, size: u32, num_items: u32, flags: u32) -> [u8; 32] {
    let mut v = [0u8; 32];
    v[0..8].copy_from_slice(b"APETAGEX");
    v[8..12].copy_from_slice(&version.to_le_bytes());
    v[12..16].copy_from_slice(&size.to_le_bytes());
    v[16..20].copy_from_slice(&num_items.to_le_bytes());
    v[20..24].copy_from_slice(&flags.to_le_bytes());
    v
}

/// A minimal, genuinely valid 8-bit mono PCM WAV file.
fn minimal_wav(data_len: u32) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(b"RIFF");
    v.extend_from_slice(&(36 + data_len).to_le_bytes());
    v.extend_from_slice(b"WAVEfmt ");
    v.extend_from_slice(&16u32.to_le_bytes());
    v.extend_from_slice(&1u16.to_le_bytes()); // PCM
    v.extend_from_slice(&1u16.to_le_bytes()); // mono
    v.extend_from_slice(&8000u32.to_le_bytes()); // sample rate
    v.extend_from_slice(&8000u32.to_le_bytes()); // byte rate
    v.extend_from_slice(&1u16.to_le_bytes()); // block align
    v.extend_from_slice(&8u16.to_le_bytes()); // bits per sample
    v.extend_from_slice(b"data");
    v.extend_from_slice(&data_len.to_le_bytes());
    v.resize(v.len() + data_len as usize, 0x80);
    v
}

/// Write a leading APEv2 tag (`is_header` set, so a reader starting at offset 0 sees a header)
/// whose 32-byte header is followed by `body`.
fn write_leading_ape(path: &Path, declared_size: u32, num_items: u32, body: &[u8]) -> u64 {
    let mut f = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
    f.write_all(&ape_hdr(2000, declared_size, num_items, HAS_FOOTER | IS_HEADER)).unwrap();
    f.write_all(body).unwrap();
    f.flush().unwrap();
    std::fs::metadata(path).unwrap().len()
}

/// `n` copies of the smallest possible APE item: value length 0, flags 0 (UTF-8 value), an empty
/// key consisting of nothing but its NUL terminator. 9 bytes each - the densest item stream a
/// crafted tag can declare.
fn minimal_items(n: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(n * 9);
    for _ in 0..n {
        v.extend_from_slice(&[0u8; 9]);
    }
    v
}

fn read_all_ape(path: &Path) -> (String, usize) {
    let src = std::fs::File::open(path).unwrap();
    let mss = MediaSourceStream::new(Box::new(src), Default::default());
    let mut r = ApeReader::try_new(mss, MetadataOptions::default()).expect("ApeReader::try_new");
    match r.read_all() {
        Ok(m) => ("Ok".to_string(), m.revision.media.tags.len()),
        Err(e) => (format!("Err: {e}"), 0),
    }
}

fn run_case(case: &str, mb: usize, dir: &Path) {
    let bytes = mb << 20;
    let file = dir.join(format!("{case}.bin"));

    let base = vm_hwm_kb();
    let t = std::time::Instant::now();

    let (file_len, outcome, tags, note) = match case {
        // One item declaring a value length of u32::MAX (4 GiB) followed by `mb` MiB of real
        // stream bytes. If the declared length really drove the allocation this would try to
        // reserve 4 GiB.
        "item-len-max" => {
            let mut body = Vec::new();
            body.extend_from_slice(&u32::MAX.to_le_bytes()); // value length
            body.extend_from_slice(&0u32.to_le_bytes()); // flags -> UTF-8 value
            body.extend_from_slice(b"k\0"); // key + NUL terminator
            body.resize(body.len() + bytes, 0x41);
            let len = write_leading_ape(&file, (body.len() + 32) as u32, 1, &body);
            let (outcome, tags) = read_all_ape(&file);
            (len, outcome, tags, "declared item value length = u32::MAX (4 GiB)")
        }
        // One item whose key is never NUL-terminated, so `read_key` keeps pushing into its String
        // until the stream runs out. Tests the "no 255-character cap" TODO in read_key.
        "key-no-nul" => {
            let mut body = Vec::new();
            body.extend_from_slice(&0u32.to_le_bytes()); // value length 0
            body.extend_from_slice(&0u32.to_le_bytes()); // flags
            body.resize(body.len() + bytes, b'A'); // key bytes, no NUL anywhere
            let len = write_leading_ape(&file, (body.len() + 32) as u32, 1, &body);
            let (outcome, tags) = read_all_ape(&file);
            (len, outcome, tags, "key with no NUL terminator for the whole file")
        }
        // Header declares size = 32, i.e. a tag with room for zero items, yet claims u32::MAX
        // items. Every item read is therefore outside the extent the same header declares.
        "num-items-past-extent" => {
            let body = minimal_items(bytes / 9);
            let len = write_leading_ape(&file, 32, u32::MAX, &body);
            let (outcome, tags) = read_all_ape(&file);
            (len, outcome, tags, "declared size = 32 bytes, declared num_items = u32::MAX")
        }
        // The same item stream, but with size/num_items declared truthfully. If this costs the
        // same as the case above, then cross-checking num_items against size buys nothing.
        "num-items-honest" => {
            let n = bytes / 9;
            let body = minimal_items(n);
            let len = write_leading_ape(&file, (body.len() + 32) as u32, n as u32, &body);
            let (outcome, tags) = read_all_ape(&file);
            (len, outcome, tags, "size and num_items both declared truthfully")
        }
        // An equally sized ID3v2.3 tag of minimal 11-byte TIT2 frames, through the other
        // standalone reader in the same crate, as a baseline for the per-byte amplification.
        "id3v2-baseline" => {
            let n = bytes / 11;
            let body_len = n * 11;
            {
                let mut f = std::io::BufWriter::new(std::fs::File::create(&file).unwrap());
                f.write_all(b"ID3").unwrap();
                f.write_all(&[3, 0, 0]).unwrap();
                let size = body_len as u32;
                f.write_all(&[
                    ((size >> 21) & 0x7f) as u8,
                    ((size >> 14) & 0x7f) as u8,
                    ((size >> 7) & 0x7f) as u8,
                    (size & 0x7f) as u8,
                ])
                .unwrap();
                let mut frame = Vec::new();
                frame.extend_from_slice(b"TIT2");
                frame.extend_from_slice(&1u32.to_be_bytes());
                frame.extend_from_slice(&[0, 0]);
                frame.push(0); // text encoding byte, empty text
                for _ in 0..n {
                    f.write_all(&frame).unwrap();
                }
                f.flush().unwrap();
            }
            let len = std::fs::metadata(&file).unwrap().len();
            let src = std::fs::File::open(&file).unwrap();
            let mss = MediaSourceStream::new(Box::new(src), Default::default());
            let mut r = Id3v2Reader::try_new(mss, MetadataOptions::default()).unwrap();
            let (outcome, tags) = match r.read_all() {
                Ok(m) => ("Ok".to_string(), m.revision.media.tags.len()),
                Err(e) => (format!("Err: {e}"), 0),
            };
            (len, outcome, tags, "ID3v2.3 tag of minimal TIT2 frames, same file size")
        }
        // Real end-to-end path #1: a genuinely valid WAV file with a hostile APEv2 tag appended at
        // its end. The default probe searches anchor -32 for trailing metadata and instantiates
        // the registered ApeReader itself.
        "probe-trailing" => {
            let wav = minimal_wav(1024);
            {
                let mut f = std::io::BufWriter::new(std::fs::File::create(&file).unwrap());
                f.write_all(&wav).unwrap();
                f.write_all(&minimal_items(bytes / 9)).unwrap();
                // Footer: claims u32::MAX items and a tag size covering the item stream.
                let size = (bytes / 9 * 9 + 32) as u32;
                f.write_all(&ape_hdr(2000, size, u32::MAX, HAS_FOOTER)).unwrap();
                f.flush().unwrap();
            }
            let len = std::fs::metadata(&file).unwrap().len();
            let src = std::fs::File::open(&file).unwrap();
            let mss = MediaSourceStream::new(Box::new(src), Default::default());
            let outcome = match symphonia::default::get_probe().probe(
                &Hint::new(),
                mss,
                Default::default(),
                Default::default(),
            ) {
                Ok(_) => "Ok: probe returned a format reader".to_string(),
                Err(e) => format!("Err: {e}"),
            };
            (len, outcome, 0, "real probe(), APEv2 footer appended to a valid WAV")
        }
        // Real end-to-end path #2: the hostile APEv2 tag leads the file (APEv2 anchors are
        // Supplemental, so the main probe loop instantiates ApeReader at offset 0) and the WAV
        // follows it.
        "probe-leading" => {
            let wav = minimal_wav(1024);
            {
                let mut f = std::io::BufWriter::new(std::fs::File::create(&file).unwrap());
                f.write_all(&ape_hdr(2000, 32, u32::MAX, HAS_FOOTER | IS_HEADER)).unwrap();
                f.write_all(&minimal_items(bytes / 9)).unwrap();
                f.write_all(&wav).unwrap();
                f.flush().unwrap();
            }
            let len = std::fs::metadata(&file).unwrap().len();
            let src = std::fs::File::open(&file).unwrap();
            let mss = MediaSourceStream::new(Box::new(src), Default::default());
            let outcome = match symphonia::default::get_probe().probe(
                &Hint::new(),
                mss,
                Default::default(),
                Default::default(),
            ) {
                Ok(_) => "Ok: probe returned a format reader".to_string(),
                Err(e) => format!("Err: {e}"),
            };
            (len, outcome, 0, "real probe(), APEv2 header leading a valid WAV")
        }
        other => panic!("unknown case {other}"),
    };

    let peak = vm_hwm_kb();
    let delta_kb = peak.saturating_sub(base);
    println!(
        "{case:<22} file={:>7.1} MiB  peakRSS={:>7.1} MiB  amplification={:>5.1}x  elapsed={:>8?}  tags={tags:<9} {outcome}\n{:24}({note})",
        file_len as f64 / 1048576.0,
        delta_kb as f64 / 1024.0,
        (delta_kb as f64 * 1024.0) / file_len as f64,
        t.elapsed(),
        "",
    );

    let _ = std::fs::remove_file(&file);
}

/// A hand-rolled sanity check that does not depend on any crafted file: an in-memory APEv2 tag
/// with a single 4 GiB-declared item and only a handful of real bytes behind it must fail with a
/// clean error rather than panicking or reserving 4 GiB.
fn tiny_stream_check() {
    let mut body = Vec::new();
    body.extend_from_slice(&u32::MAX.to_le_bytes());
    body.extend_from_slice(&0u32.to_le_bytes());
    body.extend_from_slice(b"k\0");
    body.extend_from_slice(b"short");
    let mut data = ape_hdr(2000, (body.len() + 32) as u32, 1, HAS_FOOTER | IS_HEADER).to_vec();
    data.extend_from_slice(&body);

    let mss = MediaSourceStream::new(Box::new(Cursor::new(data)), Default::default());
    let mut r = ApeReader::try_new(mss, MetadataOptions::default()).unwrap();
    match r.read_all() {
        Ok(_) => println!("tiny-stream check: Ok (unexpected, but no crash)"),
        Err(e) => println!("tiny-stream check: clean error, no panic, no 4 GiB reservation: {e}"),
    }
    // Same shape with the has_header flag set, which is the one place ApeHeader::read does any
    // arithmetic on the untrusted size (`size + 32`).
    let mut data = ape_hdr(2000, u32::MAX, 1, HAS_FOOTER | HAS_HEADER | IS_HEADER).to_vec();
    data.extend_from_slice(&body);
    let mss = MediaSourceStream::new(Box::new(Cursor::new(data)), Default::default());
    let mut r = ApeReader::try_new(mss, MetadataOptions::default()).unwrap();
    match r.read_all() {
        Ok(_) => println!("size=u32::MAX+has_header check: Ok (unexpected, but no crash)"),
        Err(e) => println!("size=u32::MAX+has_header check: clean error, no overflow panic: {e}"),
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let first = args.next();

    let mb: usize = std::env::var("REPRO_MB").ok().and_then(|s| s.parse().ok()).unwrap_or(64);

    if let Some(case) = first {
        // Child process: run exactly one case so VmHWM reflects only that case.
        let dir = PathBuf::from(args.next().unwrap_or_else(|| ".".to_string()));
        run_case(&case, mb, &dir);
        return;
    }

    let dir = std::env::current_dir().unwrap().join("work");
    std::fs::create_dir_all(&dir).unwrap();
    let exe = std::env::current_exe().unwrap();

    println!("candidate 038 - APE item length / key length / num_items bounds check");
    println!("crafted-file size per case: {mb} MiB (override with REPRO_MB)\n");

    tiny_stream_check();
    println!();

    for case in CASES {
        let status = std::process::Command::new(&exe)
            .arg(case)
            .arg(&dir)
            .status()
            .expect("failed to spawn case");
        if !status.success() {
            println!("{case:<22} CHILD FAILED: {status}");
        }
    }

    let _ = std::fs::remove_dir_all(&dir);

    println!(
        "\nReading of the numbers above:\n\
         - `item-len-max` and `key-no-nul` at a small constant multiple of the file size (the\n  \
           multiple is the doubling-realloc transient, not the declared length) means the 32-bit\n  \
           item length and the uncapped key loop are both bounded by how many bytes the stream\n  \
           actually has, not by the declared 32-bit value:\n  \
           `ReadBytes::read_boxed_slice_exact` grows progressively (4 MiB first, then doubling)\n  \
           with `try_reserve_exact`, so a 4 GiB claim behind a short file fails with a clean error\n  \
           instead of reserving 4 GiB.\n\
         - `num-items-past-extent` and `num-items-honest` costing the same means the missing\n  \
           num_items-vs-size cross-check does not change the resource cost: a tag can declare the\n  \
           same item stream honestly and get the same result.\n\
         - `id3v2-baseline` in the same ballpark means the per-byte amplification is a property of\n  \
           storing one `Tag` per parsed item/frame, shared with the crate's other standalone\n  \
           reader, not an APE-specific defect."
    );
}
