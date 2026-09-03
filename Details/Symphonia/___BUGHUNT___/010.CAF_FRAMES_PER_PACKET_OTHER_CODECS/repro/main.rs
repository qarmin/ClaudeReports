//! Verification harness for candidate 010 (Not reproducible): does CAF's untrusted
//! `frames_per_packet` field, passed straight through by `symphonia-format-caf` as
//! `max_frames_per_packet`/`frames_per_block` in `AudioCodecParameters`, actually get *consumed*
//! by the AAC / ALAC / FLAC / MP3 decoders in a way that could cause an out-of-bounds write - the
//! same root cause candidate 003 confirmed for the ADPCM IMA QT ("ima4") codec?
//!
//! For each codec, this builds a real, minimal CAF file selecting that codec with a deliberately
//! implausible `frames_per_packet` (0xFFFFFFF0), then runs it through the real
//! `symphonia::default::get_probe()` / `get_codecs()` public API pipeline - the same pipeline a
//! real application (e.g. symphonia-play) uses to play an untrusted file - and prints whether
//! anything panics.
//!
//! Unlike candidate 003's `repro/`, this one is expected to (and does) complete every case
//! without a panic: see `RESULT.md` for why (each of these decoders either doesn't read
//! `frames_per_block`/`max_frames_per_packet` from `codec_params` at all, or overwrites it with a
//! value derived from its own codec-specific extra data before using it).

use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

fn chunk(ctype: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 8 + payload.len());
    out.extend_from_slice(ctype);
    out.extend_from_slice(&(payload.len() as i64).to_be_bytes());
    out.extend_from_slice(payload);
    out
}

fn caf_header() -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"caff");
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes());
    out
}

fn desc_chunk(
    format_id: &[u8; 4],
    format_flags: u32,
    bytes_per_packet: u32,
    frames_per_packet: u32,
    channels: u32,
    bits_per_channel: u32,
) -> Vec<u8> {
    let mut desc = Vec::new();
    desc.extend_from_slice(&44100.0f64.to_be_bytes());
    desc.extend_from_slice(format_id);
    desc.extend_from_slice(&format_flags.to_be_bytes());
    desc.extend_from_slice(&bytes_per_packet.to_be_bytes());
    desc.extend_from_slice(&frames_per_packet.to_be_bytes());
    desc.extend_from_slice(&channels.to_be_bytes());
    desc.extend_from_slice(&bits_per_channel.to_be_bytes());
    assert_eq!(desc.len(), 32);
    chunk(b"desc", &desc)
}

fn pakt_chunk_one_packet(size: u8, frames: i64) -> Vec<u8> {
    let mut pakt = Vec::new();
    pakt.extend_from_slice(&1i64.to_be_bytes()); // total_packets
    pakt.extend_from_slice(&frames.to_be_bytes()); // valid_frames
    pakt.extend_from_slice(&0i32.to_be_bytes()); // priming_frames
    pakt.extend_from_slice(&0i32.to_be_bytes()); // remainder_frames
    pakt.push(size); // one packet's byte size, as a single-byte VLQ (must be < 0x80)
    chunk(b"pakt", &pakt)
}

fn run_case(name: &str, data: Vec<u8>) {
    println!("== {name} ({} bytes) ==", data.len());
    let mss = MediaSourceStream::new(Box::new(std::io::Cursor::new(data)), Default::default());

    let mut format = match symphonia::default::get_probe().probe(
        &Hint::new(),
        mss,
        FormatOptions::default(),
        MetadataOptions::default(),
    ) {
        Ok(f) => f,
        Err(e) => {
            println!("  probe failed (no crash): {e}");
            return;
        }
    };

    let track = match format.default_track(TrackType::Audio) {
        Some(t) => t,
        None => {
            println!("  no audio track (no crash)");
            return;
        }
    };
    let codec_params = track.codec_params.as_ref().unwrap().audio().unwrap();
    println!(
        "  codec_params: max_frames_per_packet={:?} frames_per_block={:?}",
        codec_params.max_frames_per_packet, codec_params.frames_per_block
    );

    let mut decoder = match symphonia::default::get_codecs()
        .make_audio_decoder(codec_params, &Default::default())
    {
        Ok(d) => d,
        Err(e) => {
            println!("  no decoder registered / rejected params (no crash): {e}");
            return;
        }
    };

    loop {
        let packet = match format.next_packet() {
            Ok(Some(p)) => p,
            Ok(None) => break,
            Err(e) => {
                println!("  demux error (no crash): {e}");
                break;
            }
        };
        match decoder.decode(&packet) {
            Ok(_) => println!("  decoded a packet OK"),
            Err(e) => println!("  decode error (no crash): {e}"),
        }
    }
    println!("  finished (no crash)");
}

fn main() {
    // --- AAC: force the "Fixed audio packet" path with an absurd frames_per_packet, no magic
    // cookie (decoder falls back to a default LC AudioSpecificConfig, 1024 samples/frame,
    // regardless of what CAF claims). Packet payload is garbage AAC raw data block bytes.
    {
        let mut out = caf_header();
        out.extend_from_slice(&desc_chunk(b"aac ", 0, 7, 0xFFFF_FFF0, 1, 16));
        let mut data_payload = Vec::new();
        data_payload.extend_from_slice(&0u32.to_be_bytes());
        data_payload.extend_from_slice(&[0xAAu8; 7]); // one bogus "packet"
        out.extend_from_slice(&chunk(b"data", &data_payload));
        run_case("AAC, frames_per_packet=0xFFFFFFF0 (Fixed path)", out);
    }

    // --- ALAC: needs a valid magic cookie (24 bytes), whose OWN frame_length field governs
    // buffer sizing - CAF's frames_per_packet is set to something totally different.
    {
        let mut out = caf_header();
        out.extend_from_slice(&desc_chunk(b"alac", 0, 0, 0xFFFF_FFF0, 1, 16));
        let mut cookie = Vec::new();
        cookie.extend_from_slice(&4096u32.to_be_bytes()); // frame_length (real value used by decoder)
        cookie.push(0); // compatible_version
        cookie.push(16); // bit_depth
        cookie.push(40); // pb
        cookie.push(10); // mb
        cookie.push(14); // kb
        cookie.push(1); // num_channels
        cookie.extend_from_slice(&255u16.to_be_bytes()); // max_run
        cookie.extend_from_slice(&0u32.to_be_bytes()); // max_frame_bytes
        cookie.extend_from_slice(&0u32.to_be_bytes()); // avg_bit_rate
        cookie.extend_from_slice(&44100u32.to_be_bytes()); // sample_rate
        assert_eq!(cookie.len(), 24);
        out.extend_from_slice(&chunk(b"kuki", &cookie));
        out.extend_from_slice(&pakt_chunk_one_packet(8, 4096));
        let mut data_payload = Vec::new();
        data_payload.extend_from_slice(&0u32.to_be_bytes());
        data_payload.extend_from_slice(&[0x00u8; 8]); // bogus ALAC packet bytes
        out.extend_from_slice(&chunk(b"data", &data_payload));
        run_case("ALAC, frames_per_packet=0xFFFFFFF0, valid cookie (frame_length=4096)", out);
    }

    // --- FLAC: needs a valid STREAMINFO (34 bytes); its own block_len_max OVERWRITES whatever
    // CAF claims (see FlacDecoder::try_new). frames_per_packet set to an absurd Fixed value.
    {
        let mut out = caf_header();
        out.extend_from_slice(&desc_chunk(b"flac", 0, 0, 0xFFFF_FFF0, 1, 16));
        let mut info = Vec::new();
        info.extend_from_slice(&4096u16.to_be_bytes()); // block_len_min
        info.extend_from_slice(&4096u16.to_be_bytes()); // block_len_max
        info.extend_from_slice(&[0, 0, 0]); // frame_byte_len_min (24-bit)
        info.extend_from_slice(&[0, 0, 0]); // frame_byte_len_max (24-bit)
        // Bit-packed: sample_rate=44100 (20b), channels-1=0 (3b), bits_per_sample-1=15 (5b),
        // n_samples=0 (36b).
        info.extend_from_slice(&[0x0a, 0xc4, 0x40, 0xf0, 0x00, 0x00, 0x00, 0x00]);
        info.extend_from_slice(&[0u8; 16]); // md5 (all zero = "no checksum")
        assert_eq!(info.len(), 34);
        out.extend_from_slice(&chunk(b"kuki", &info));
        out.extend_from_slice(&pakt_chunk_one_packet(8, 4096));
        let mut data_payload = Vec::new();
        data_payload.extend_from_slice(&0u32.to_be_bytes());
        data_payload.extend_from_slice(&[0x00u8; 8]); // bogus FLAC frame bytes
        out.extend_from_slice(&chunk(b"data", &data_payload));
        run_case("FLAC, frames_per_packet=0xFFFFFFF0, valid STREAMINFO (block_len_max=4096)", out);
    }

    // --- MP3: no extra data required; frame size is derived from the MPEG frame header inside
    // the packet bytes themselves, never from codec_params.
    {
        let mut out = caf_header();
        out.extend_from_slice(&desc_chunk(b".mp3", 0, 7, 0xFFFF_FFF0, 1, 16));
        let mut data_payload = Vec::new();
        data_payload.extend_from_slice(&0u32.to_be_bytes());
        data_payload.extend_from_slice(&[0x00u8; 7]); // bogus (invalid) MPEG frame bytes
        out.extend_from_slice(&chunk(b"data", &data_payload));
        run_case("MP3, frames_per_packet=0xFFFFFFF0 (Fixed path)", out);
    }

    // --- Opus: check whether it's even registered as a default decoder in the `symphonia`
    // facade crate at all (it is not, as of this codebase - no `opus` feature is wired up in
    // symphonia/Cargo.toml or symphonia/src/lib.rs::register_enabled_codecs).
    {
        let mut out = caf_header();
        out.extend_from_slice(&desc_chunk(b"opus", 0, 0, 0xFFFF_FFF0, 1, 16));
        let mut data_payload = Vec::new();
        data_payload.extend_from_slice(&0u32.to_be_bytes());
        data_payload.extend_from_slice(&[0x00u8; 8]);
        out.extend_from_slice(&chunk(b"data", &data_payload));
        run_case("Opus, frames_per_packet=0xFFFFFFF0", out);
    }

    println!("ALL CASES COMPLETED WITHOUT A PANIC");
}
