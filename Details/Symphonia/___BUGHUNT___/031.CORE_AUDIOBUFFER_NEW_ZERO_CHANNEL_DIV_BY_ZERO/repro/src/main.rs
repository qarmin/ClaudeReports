//! Standalone reproduction for: `attempt to divide by zero` panic in
//! `symphonia_core::audio::AudioBuffer::new` (`symphonia-core/src/audio/buf.rs:84`) whenever the
//! `AudioSpec` it is given has zero channels.
//!
//! ```text
//! assert!(capacity <= (usize::MAX / (std::mem::size_of::<S>() * num_channels)), "capacity too large");
//! ```
//!
//! With `num_channels == 0` the divisor is `0`, so instead of the intended "capacity too large"
//! assertion the constructor panics with `attempt to divide by zero` - and integer division by
//! zero panics in release builds too, not only under debug assertions/overflow checks.
//!
//! Two independent, real callers are exercised, both through public APIs only:
//!
//!   * Part A - the affected entry point called directly, the way any third-party
//!     format/codec crate built on `symphonia-core` calls it. A zero-channel `AudioSpec` is not
//!     an illegal state in this API: `Channels::None`, `Channels::Discrete(0)` and an empty
//!     `Channels::Custom` all report `count() == 0`, and `AudioBuffer::is_unused` explicitly
//!     documents "no channels" as a legitimate condition.
//!
//!   * Part B - the same panic reached end-to-end from a real file, with no hand-built internal
//!     state at all: a crafted CAF file is fed to symphonia's real public
//!     probe -> demux -> `make_audio_decoder` pipeline (the same calls `symphonia-play` makes).
//!     `symphonia-format-caf` turns a `chan` (Channel Layout) chunk that declares
//!     `kAudioChannelLayoutTag_UseChannelDescriptions` with zero channel descriptions into
//!     `Channels::Custom([])`, and `symphonia-codec-adpcm` passes that straight into
//!     `AudioBuffer::new`.
//!
//! Exit code is 1 if the divide-by-zero in `buf.rs` was observed, 0 otherwise.

use std::panic::{self, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

use symphonia::core::audio::{Audio, AudioBuffer, AudioSpec, Channels};
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

/// Runs `f`, returning the panic message + location if it panicked.
fn catch<F: FnOnce()>(f: F) -> Option<String> {
    let captured: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    let sink = Arc::clone(&captured);
    let prev = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let msg = info
            .payload()
            .downcast_ref::<&str>()
            .map(|s| (*s).to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "<non-string panic payload>".to_string());

        let loc = info.location().map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()));

        *sink.lock().unwrap() = Some(match loc {
            Some(loc) => format!("{loc}: {msg}"),
            None => msg,
        });
    }));

    let result = panic::catch_unwind(AssertUnwindSafe(f));
    panic::set_hook(prev);

    match result {
        Ok(()) => None,
        Err(_) => Some(
            captured.lock().unwrap().clone().unwrap_or_else(|| "<panic, no message>".to_string()),
        ),
    }
}

fn is_div_by_zero_in_audio_buf(msg: &str) -> bool {
    msg.contains("attempt to divide by zero") && msg.contains("audio/buf.rs")
}

fn chunk(ctype: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 8 + payload.len());
    out.extend_from_slice(ctype);
    out.extend_from_slice(&(payload.len() as i64).to_be_bytes());
    out.extend_from_slice(payload);
    out
}

/// Build a CAF file with a valid single-channel `ima4` (Apple IMA4 / ADPCM IMA QT) audio
/// description. When `zero_channel_layout` is set, a `chan` chunk is appended that declares
/// `mChannelLayoutTag = kAudioChannelLayoutTag_UseChannelDescriptions` (0) with
/// `mNumberChannelDescriptions = 0`.
fn build_caf(zero_channel_layout: bool) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"caff");
    out.extend_from_slice(&1u16.to_be_bytes()); // file version
    out.extend_from_slice(&0u16.to_be_bytes()); // file flags

    let mut desc = Vec::new();
    desc.extend_from_slice(&44100.0f64.to_be_bytes()); // mSampleRate
    desc.extend_from_slice(b"ima4"); // mFormatID
    desc.extend_from_slice(&0u32.to_be_bytes()); // mFormatFlags
    desc.extend_from_slice(&34u32.to_be_bytes()); // mBytesPerPacket (real IMA4 packet size)
    desc.extend_from_slice(&64u32.to_be_bytes()); // mFramesPerPacket (real IMA4 block length)
    desc.extend_from_slice(&1u32.to_be_bytes()); // mChannelsPerFrame
    desc.extend_from_slice(&16u32.to_be_bytes()); // mBitsPerChannel
    assert_eq!(desc.len(), 32);
    out.extend_from_slice(&chunk(b"desc", &desc));

    if zero_channel_layout {
        let mut chan = Vec::new();
        chan.extend_from_slice(&0u32.to_be_bytes()); // mChannelLayoutTag
        chan.extend_from_slice(&0u32.to_be_bytes()); // mChannelBitmap
        chan.extend_from_slice(&0u32.to_be_bytes()); // mNumberChannelDescriptions
        assert_eq!(chan.len(), 12);
        out.extend_from_slice(&chunk(b"chan", &chan));
    }

    let mut data_payload = Vec::new();
    data_payload.extend_from_slice(&0u32.to_be_bytes()); // mEditCount
    data_payload.extend_from_slice(&[0u8; 34]); // one 34-byte IMA4 packet
    out.extend_from_slice(&chunk(b"data", &data_payload));

    out
}

/// Drive the crafted file through the real public pipeline.
fn decode_caf(data: Vec<u8>) {
    let mss = MediaSourceStream::new(Box::new(std::io::Cursor::new(data)), Default::default());

    let mut format = symphonia::default::get_probe()
        .probe(&Hint::new(), mss, FormatOptions::default(), MetadataOptions::default())
        .expect("probe should recognize the crafted CAF file");

    let track = format.default_track(TrackType::Audio).expect("should have an audio track");
    let codec_params = track.codec_params.as_ref().unwrap().audio().unwrap();

    println!(
        "    demuxed channels = {:?} (count = {})",
        codec_params.channels,
        codec_params.channels.as_ref().map_or(0, |c| c.count())
    );

    // For the crafted file this is where AudioBuffer::new is reached with 0 channels.
    let mut decoder = match symphonia::default::get_codecs()
        .make_audio_decoder(codec_params, &Default::default())
    {
        Ok(decoder) => decoder,
        Err(err) => {
            println!("    make_audio_decoder returned an error (no panic): {err}");
            return;
        }
    };

    println!("    make_audio_decoder succeeded");

    loop {
        let packet = match format.next_packet() {
            Ok(Some(packet)) => packet,
            Ok(None) => break,
            Err(err) => {
                println!("    demux error (no panic): {err}");
                break;
            }
        };
        match decoder.decode(&packet) {
            Ok(_) => println!("    decoded a packet OK"),
            Err(err) => println!("    decode error (no panic): {err}"),
        }
    }
}

fn main() {
    let mut bug_seen = false;

    println!("=== Part A: the affected public entry point, called directly ===");

    for (label, channels) in [
        ("Channels::None", Channels::None),
        ("Channels::Discrete(0)", Channels::Discrete(0)),
        ("Channels::Custom([])", Channels::Custom(Vec::new().into_boxed_slice())),
    ] {
        println!("  AudioBuffer::<f32>::new(AudioSpec::new(44100, {label}), 1024)");
        println!("    Channels::count() = {}", channels.count());

        let spec = AudioSpec::new(44100, channels);
        match catch(move || {
            let buf = AudioBuffer::<f32>::new(spec, 1024);
            println!(
                "    ok: num_planes = {}, capacity = {}, is_unused = {}",
                buf.num_planes(),
                buf.capacity(),
                buf.is_unused()
            );
        }) {
            Some(msg) => {
                println!("    PANIC {msg}");
                if is_div_by_zero_in_audio_buf(&msg) {
                    bug_seen = true;
                }
            }
            None => println!("    no panic"),
        }
    }

    println!();
    println!("=== Part B: end-to-end, a real crafted CAF file through the public pipeline ===");

    for (label, zero_channel_layout) in
        [("baseline CAF, no chan chunk (1 channel)", false), ("crafted CAF, chan chunk with 0 channel descriptions", true)]
    {
        let data = build_caf(zero_channel_layout);
        println!("  {label}: {} bytes", data.len());

        match catch(move || decode_caf(data)) {
            Some(msg) => {
                println!("    PANIC {msg}");
                if is_div_by_zero_in_audio_buf(&msg) {
                    bug_seen = true;
                }
                else {
                    println!(
                        "    (not this candidate's bug - see RESULT.md, \"What's not done\": \
                         symphonia-codec-adpcm accepting a 0-channel spec at all is a separate, \
                         distinct defect)"
                    );
                }
            }
            None => println!("    no panic"),
        }
    }

    println!();
    if bug_seen {
        println!("=== RESULT: BUG PRESENT - AudioBuffer::new divided by zero on a 0-channel spec");
        std::process::exit(1);
    }

    println!("=== RESULT: BUG GONE - AudioBuffer::new did not divide by zero on a 0-channel spec");
}
