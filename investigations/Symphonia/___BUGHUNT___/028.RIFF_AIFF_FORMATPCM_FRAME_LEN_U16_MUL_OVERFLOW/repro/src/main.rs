// Candidate 028: symphonia-format-riff AIFF FormatPcm::make_packet_info() frame_len u16 overflow.
//
// This drives the real, public, end-to-end path a player uses: open a file from disk as a
// MediaSourceStream and hand it to symphonia's default Probe, which selects AiffReader and calls
// AiffReader::try_new. Nothing internal is called directly and no state is hand-constructed - the
// only input is a structurally valid .aiff file on disk (see gen_aiff.py).
//
// symphonia-format-riff/src/common.rs:215
//     let frame_len = (self.bits_per_sample / 8) * num_channels;   // both u16
// bits_per_sample is at most 32 (4 bytes) and num_channels comes from the AIFF COMM chunk's
// 16-bit channel-count field, which map_aiff_channel_count() intentionally accepts for any value
// >= 4. 4 * 16384 == 65536 does not fit in u16.

use std::env;
use std::fs::File;
use std::path::Path;

use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;

fn open(path: &Path) -> i32 {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            println!("[{}] could not open file: {}", path.display(), err);
            return 2;
        }
    };

    let mss = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

    let mut hint = Hint::new();
    hint.with_extension("aiff");

    println!("[{}] probing...", path.display());

    match symphonia::default::get_probe().probe(
        &hint,
        mss,
        FormatOptions::default(),
        MetadataOptions::default(),
    ) {
        Ok(reader) => {
            let tracks = reader.tracks();
            println!(
                "[{}] OK: probe returned a reader with {} track(s)",
                path.display(),
                tracks.len()
            );
            0
        }
        Err(err) => {
            // A graceful decode error is the correct behaviour for a nonsensical channel count.
            println!("[{}] Err (graceful, no panic): {}", path.display(), err);
            0
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let assets: Vec<String> = if args.is_empty() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
        vec![
            dir.join("aiff_chan2_size16.aiff").to_string_lossy().into_owned(),
            dir.join("aiff_chan30000_size8.aiff").to_string_lossy().into_owned(),
            dir.join("aiff_chan16384_size32.aiff").to_string_lossy().into_owned(),
            dir.join("aiff_chan21846_size24.aiff").to_string_lossy().into_owned(),
            dir.join("aiff_chan65535_size32.aiff").to_string_lossy().into_owned(),
        ]
    }
    else {
        args
    };

    let mut worst = 0;
    for asset in &assets {
        let code = open(Path::new(asset));
        worst = worst.max(code);
    }

    println!("done, all {} file(s) handled without a panic", assets.len());
    std::process::exit(worst);
}
