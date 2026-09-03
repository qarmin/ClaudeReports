// Repro: `ElementChannel::predict` in symphonia-codec-alac panics with "attempt to subtract with
// overflow" while running the FIR filter (and again in the coefficient-adaptation loop) when the
// coded bit depth is 32.
//
// End-to-end path exercised: a real CAF file on disk -> symphonia::default::get_probe() ->
// the real symphonia-format-caf demuxer -> symphonia::default::get_codecs() -> the real
// symphonia-codec-alac decoder. Nothing internal is reached into: only the public `symphonia`
// API is used, exactly as symphonia/examples/getting-started.rs does.
//
// Why bit depth 32 matters: for a 32-bit ALAC stream `bps == 32`, so `num_clip_bits == 0` and
// `clip_msbs()` becomes the identity function. The rice-decoded residuals therefore span the full
// i32 range instead of being clipped into a narrower one, and `predict()`'s plain (non-wrapping)
// `i32` arithmetic overflows. A bit depth of 32 is legal ALAC and is accepted by
// symphonia-common's MagicCookie parser (only `> 32` is rejected).

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::TrackType;
use symphonia::core::formats::probe::Hint;
use symphonia::core::io::MediaSourceStream;

// ---------------------------------------------------------------------------------------------
// Bitstream writer (MSb first), used to author the ALAC packet payload.
// ---------------------------------------------------------------------------------------------

struct BitWriter {
    buf: Vec<u8>,
    acc: u8,
    n: u32,
}

impl BitWriter {
    fn new() -> Self {
        BitWriter { buf: Vec::new(), acc: 0, n: 0 }
    }

    fn put(&mut self, val: u64, bits: u32) {
        for i in (0..bits).rev() {
            let b = ((val >> i) & 1) as u8;
            self.acc = (self.acc << 1) | b;
            self.n += 1;
            if self.n == 8 {
                self.buf.push(self.acc);
                self.acc = 0;
                self.n = 0;
            }
        }
    }

    fn finish(mut self) -> Vec<u8> {
        if self.n > 0 {
            self.acc <<= 8 - self.n;
            self.buf.push(self.acc);
        }
        self.buf
    }
}

// ---------------------------------------------------------------------------------------------
// ALAC "magic cookie" (24-byte form) - what a real .caf/.m4a carries in its `kuki`/`alac` box.
// ---------------------------------------------------------------------------------------------

fn magic_cookie(frame_length: u32, bit_depth: u8, num_channels: u8, sample_rate: u32) -> Vec<u8> {
    let mut c = Vec::new();
    c.extend_from_slice(&frame_length.to_be_bytes());
    c.push(0); // compatible version
    c.push(bit_depth);
    c.push(40); // pb  (reference encoder default)
    c.push(10); // mb  (reference encoder default)
    c.push(14); // kb  (reference encoder default)
    c.push(num_channels);
    c.extend_from_slice(&255u16.to_be_bytes()); // max run
    c.extend_from_slice(&0u32.to_be_bytes()); // max frame bytes
    c.extend_from_slice(&0u32.to_be_bytes()); // avg bit rate
    c.extend_from_slice(&sample_rate.to_be_bytes());
    assert_eq!(c.len(), 24);
    c
}

// ---------------------------------------------------------------------------------------------
// Minimal, valid CAF container around one fixed-size ALAC packet.
// ---------------------------------------------------------------------------------------------

fn caf_chunk(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(kind);
    v.extend_from_slice(&(data.len() as i64).to_be_bytes());
    v.extend_from_slice(data);
    v
}

fn caf_file(
    sample_rate: u32,
    num_channels: u8,
    bit_depth: u8,
    frame_length: u32,
    packet: &[u8],
) -> Vec<u8> {
    // 'desc' - audio description chunk (always exactly 32 bytes).
    let mut desc = Vec::new();
    desc.extend_from_slice(&f64::from(sample_rate).to_be_bytes());
    desc.extend_from_slice(b"alac");
    desc.extend_from_slice(&0u32.to_be_bytes()); // format flags
    desc.extend_from_slice(&(packet.len() as u32).to_be_bytes()); // bytes per packet
    desc.extend_from_slice(&frame_length.to_be_bytes()); // frames per packet
    desc.extend_from_slice(&u32::from(num_channels).to_be_bytes());
    desc.extend_from_slice(&u32::from(bit_depth).to_be_bytes());
    assert_eq!(desc.len(), 32);

    // 'data' - edit count followed by the audio data.
    let mut data = Vec::new();
    data.extend_from_slice(&0u32.to_be_bytes());
    data.extend_from_slice(packet);

    let mut file = Vec::new();
    file.extend_from_slice(b"caff");
    file.extend_from_slice(&1u16.to_be_bytes()); // file version
    file.extend_from_slice(&0u16.to_be_bytes()); // file flags
    file.extend_from_slice(&caf_chunk(b"desc", &desc));
    file.extend_from_slice(&caf_chunk(
        b"kuki",
        &magic_cookie(frame_length, bit_depth, num_channels, sample_rate),
    ));
    file.extend_from_slice(&caf_chunk(b"data", &data));
    file
}

// ---------------------------------------------------------------------------------------------
// ALAC packet payload authoring.
// ---------------------------------------------------------------------------------------------

/// Writes an escaped rice code that decodes to the raw 32-bit value `raw`: a 9-bit unary prefix of
/// ones (the escape) followed by `bps` raw bits. `rice_code_to_signed(0xffff_ffff) == i32::MIN`.
fn put_escaped_residual(w: &mut BitWriter, raw: u32, bps: u32) {
    w.put(0x1ff, 9);
    w.put(u64::from(raw), bps);
}

struct ElemHeader {
    mode: u32,
    shift: u32,
    pb_bits: u32,
    lpc_order: u32,
    coeffs: Vec<i32>,
}

fn put_elem_header(w: &mut BitWriter, h: &ElemHeader) {
    w.put(u64::from(h.mode), 4);
    w.put(u64::from(h.shift), 4);
    w.put(u64::from(h.pb_bits), 3);
    w.put(u64::from(h.lpc_order), 5);
    for i in 0..h.lpc_order as usize {
        w.put(u64::from(h.coeffs.get(i).copied().unwrap_or(0) as u16), 16);
    }
}

/// A Single Channel Element (mono) frame.
fn sce_packet(num_samples: u32, bps: u32, h: &ElemHeader, residuals: &[u32]) -> Vec<u8> {
    let mut w = BitWriter::new();
    w.put(0, 3); // ALAC_ELEM_TAG_SCE
    w.put(0, 4); // element instance tag
    w.put(0, 12); // unused header bits (must be 0)
    w.put(0, 1); // is_partial_frame
    w.put(0, 2); // shift / 8 -> 0
    w.put(0, 1); // is_uncompressed
    w.put(0, 8); // mid-side shift (must be 0 for SCE)
    w.put(0, 8); // mid-side weight (must be 0 for SCE)
    put_elem_header(&mut w, h);
    for i in 0..num_samples as usize {
        put_escaped_residual(&mut w, residuals[i % residuals.len()], bps);
    }
    w.finish()
}

/// A Channel Pair Element (stereo) frame with the mid-side weight left at 0, so mid-side
/// decorrelation is skipped entirely and only `predict()` is exercised.
fn cpe_packet(num_samples: u32, bps: u32, h: &ElemHeader, residuals: &[u32]) -> Vec<u8> {
    let mut w = BitWriter::new();
    w.put(1, 3); // ALAC_ELEM_TAG_CPE
    w.put(0, 4);
    w.put(0, 12);
    w.put(0, 1);
    w.put(0, 2);
    w.put(0, 1);
    w.put(0, 8); // mid-side shift
    w.put(0, 8); // mid-side weight == 0 -> decorrelate_mid_side() is not called
    put_elem_header(&mut w, h);
    put_elem_header(&mut w, h);
    for _ch in 0..2 {
        for i in 0..num_samples as usize {
            put_escaped_residual(&mut w, residuals[i % residuals.len()], bps);
        }
    }
    w.finish()
}

// ---------------------------------------------------------------------------------------------
// The real end-to-end decode path: probe -> demux -> decode, public API only.
// ---------------------------------------------------------------------------------------------

fn decode_caf(path: &Path) -> Result<usize, Error> {
    let src = File::open(path).expect("failed to open the generated CAF file");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("caf");

    let mut format = symphonia::default::get_probe().probe(
        &hint,
        mss,
        Default::default(),
        Default::default(),
    )?;

    let track = format.default_track(TrackType::Audio).expect("no audio track");
    let track_id = track.id;
    let params = track.codec_params.clone().expect("codec parameters missing");
    let params = params.audio().expect("not an audio codec").clone();

    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(&params, &AudioDecoderOptions::default())?;

    let mut frames = 0;
    while let Some(packet) = format.next_packet()? {
        if packet.track_id != track_id {
            continue;
        }
        frames += decoder.decode(&packet)?.frames();
    }

    Ok(frames)
}

fn write_temp(name: &str, bytes: &[u8]) -> PathBuf {
    let path = std::env::temp_dir().join(format!("alac_predict_repro_{name}.caf"));
    let mut f = File::create(&path).expect("failed to create temp file");
    f.write_all(bytes).expect("failed to write temp file");
    path
}

/// Panic site ("file:line:col: message") -> how many of the generated files panicked there. The
/// panic hook records into this instead of printing, so one line per distinct site is reported
/// rather than hundreds of identical backtrace headers.
static PANIC_SITES: Mutex<BTreeMap<String, usize>> = Mutex::new(BTreeMap::new());
static LAST_PANIC: Mutex<String> = Mutex::new(String::new());

fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let msg = info
            .payload()
            .downcast_ref::<&str>()
            .map(|s| (*s).to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "<non-string panic payload>".to_string());

        let site = match info.location() {
            Some(loc) => format!("{}:{}:{}: {msg}", loc.file(), loc.line(), loc.column()),
            None => format!("<unknown location>: {msg}"),
        };

        *PANIC_SITES.lock().unwrap().entry(site.clone()).or_insert(0) += 1;
        *LAST_PANIC.lock().unwrap() = site;
    }));
}

#[derive(Default)]
struct Tally {
    panics: usize,
    errors: usize,
    ok: usize,
}

fn run_case(tally: &mut Tally, name: &str, bytes: &[u8], verbose: bool) {
    let path = write_temp(name, bytes);
    let result = catch_unwind(AssertUnwindSafe(|| decode_caf(&path)));
    let _ = std::fs::remove_file(&path);

    match result {
        Err(_) => {
            tally.panics += 1;
            if verbose {
                println!(
                    "  {name}: PANIC ({} byte file) at {}",
                    bytes.len(),
                    LAST_PANIC.lock().unwrap()
                );
            }
        }
        Ok(Ok(frames)) => {
            tally.ok += 1;
            if verbose {
                println!("  {name}: decoded {frames} frames, no panic");
            }
        }
        Ok(Err(e)) => {
            tally.errors += 1;
            if verbose {
                println!("  {name}: rejected with a decode error: {e}");
            }
        }
    }
}

struct Rng(u64);

impl Rng {
    fn next_u32(&mut self) -> u32 {
        // xorshift64*
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        (x.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 32) as u32
    }
}

fn main() {
    println!("symphonia-codec-alac: ElementChannel::predict() i32 overflow, 32-bit ALAC");
    println!("path: real .caf file -> get_probe() -> caf demuxer -> get_codecs() -> alac decoder\n");

    install_panic_hook();

    let mut tally = Tally::default();

    // -----------------------------------------------------------------------------------------
    // Case 1 - the minimal deterministic trigger: 32-bit mono, lpc_order = 1, every residual
    // decodes to i32::MIN. The warm-up pass makes out[1] == 0, then the FIR filter computes
    // `0 - i32::MIN` (lib.rs `.map(|(&coeff, &s)| coeff.wrapping_mul(s - past0))`).
    // -----------------------------------------------------------------------------------------
    println!("case 1: mono, bit_depth=32, lpc_order=1, residuals = i32::MIN");
    let h = ElemHeader { mode: 0, shift: 0, pb_bits: 0, lpc_order: 1, coeffs: vec![0] };
    let pkt = sce_packet(4, 32, &h, &[0xffff_ffff]);
    let file1 = caf_file(44100, 1, 32, 4, &pkt);
    run_case(&mut tally, "case1_mono_bd32", &file1, true);

    // -----------------------------------------------------------------------------------------
    // Case 2 - same class, different shape: a higher LPC order with non-zero coefficients and a
    // non-zero quantisation shift, so the coefficient-adaptation loop (`let val = past0 - sample`
    // / `res -= (1 + j as i32) * ((sign * val) >> self.shift)`) is reached as well.
    // -----------------------------------------------------------------------------------------
    println!("case 2: mono, bit_depth=32, lpc_order=4, shift=4, non-zero coefficients");
    let h = ElemHeader {
        mode: 0,
        shift: 4,
        pb_bits: 0,
        lpc_order: 4,
        coeffs: vec![-32768, 32767, -32768, 32767],
    };
    let pkt = sce_packet(16, 32, &h, &[0xffff_ffff, 0x0000_0001, 0xffff_fffe, 0x0000_0000]);
    let file2 = caf_file(44100, 1, 32, 16, &pkt);
    run_case(&mut tally, "case2_mono_bd32_order4", &file2, true);

    // -----------------------------------------------------------------------------------------
    // Case 3 - the CPE variant: bit_depth = 31 stereo gives bps = 31 - 0 + 1 = 32 as well, so
    // `num_clip_bits` is 0 again. The mid-side weight is 0 here, so this is purely `predict()`.
    // -----------------------------------------------------------------------------------------
    println!("case 3: stereo CPE, bit_depth=31 (bps=32), lpc_order=1, residuals = i32::MIN");
    let h = ElemHeader { mode: 0, shift: 0, pb_bits: 0, lpc_order: 1, coeffs: vec![0] };
    let pkt = cpe_packet(4, 32, &h, &[0xffff_ffff]);
    let file3 = caf_file(44100, 2, 31, 4, &pkt);
    run_case(&mut tally, "case3_stereo_bd31", &file3, true);

    // -----------------------------------------------------------------------------------------
    // Case 4 - a bounded, deterministic sweep over the same bug class (random element headers and
    // random escaped residual payloads at bps = 32) so a fix is not judged by the one input that
    // happened to trigger it first. Files that the demuxer/decoder legitimately reject are
    // counted separately; only a panic is a failure.
    // -----------------------------------------------------------------------------------------
    const SWEEP: usize = 400;
    println!("case 4: {SWEEP} generated 32-bit ALAC files (random headers/residuals)");
    let mut rng = Rng(0x1234_5678_9abc_def0);
    let before = tally.panics;
    for i in 0..SWEEP {
        let lpc_order = 1 + rng.next_u32() % 8;
        let coeffs: Vec<i32> =
            (0..lpc_order).map(|_| (rng.next_u32() % 0x1_0000) as i16 as i32).collect();
        let h = ElemHeader {
            mode: 0,
            shift: rng.next_u32() % 16,
            pb_bits: rng.next_u32() % 8,
            lpc_order,
            coeffs,
        };
        let num_samples = 4 + rng.next_u32() % 28;
        let residuals: Vec<u32> = (0..8)
            .map(|_| match rng.next_u32() % 4 {
                0 => 0xffff_ffff, // i32::MIN
                1 => 0xffff_fffe, // i32::MAX
                2 => rng.next_u32(),
                _ => rng.next_u32() % 8,
            })
            .collect();
        let stereo = i % 2 == 1;
        let (packet, channels, bit_depth) = if stereo {
            (cpe_packet(num_samples, 32, &h, &residuals), 2, 31)
        }
        else {
            (sce_packet(num_samples, 32, &h, &residuals), 1, 32)
        };
        let file = caf_file(44100, channels, bit_depth, num_samples, &packet);
        run_case(&mut tally, &format!("sweep{i}"), &file, false);
    }
    println!("  sweep: {} panics", tally.panics - before);

    println!(
        "\ntotal: {} panics, {} decode errors, {} decoded without panicking",
        tally.panics, tally.errors, tally.ok
    );

    let sites = PANIC_SITES.lock().unwrap();
    if !sites.is_empty() {
        println!("\ndistinct panic sites:");
        for (site, count) in sites.iter() {
            println!("  {count:>4}x {site}");
        }
    }
    drop(sites);

    if tally.panics > 0 {
        println!("RESULT: BUG PRESENT - the decoder panicked on valid-shaped 32-bit ALAC input");
        std::process::exit(1);
    }

    println!("RESULT: no panics - the decoder either decoded or cleanly rejected every input");
}
