use hayro_jpeg2000::{DecodeSettings, DecoderContext, Image};
use std::time::Instant;

fn peak_rss_kb() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmHWM:") {
            return rest.trim().trim_end_matches(" kB").trim().parse().ok();
        }
    }
    None
}

fn main() {
    let path = std::env::args().nth(1).expect("path to j2c");
    let target_w: u32 = std::env::args().nth(2).map(|s| s.parse().unwrap()).unwrap_or(20);
    let target_h: u32 = std::env::args().nth(3).map(|s| s.parse().unwrap()).unwrap_or(20);
    let data = std::fs::read(&path).expect("read file");

    let settings = DecodeSettings {
        target_resolution: Some((target_w, target_h)),
        ..Default::default()
    };

    let start_all = Instant::now();
    let image = match Image::new(&data, &settings) {
        Ok(i) => i,
        Err(e) => {
            println!("Image::new failed: {e:?}");
            return;
        }
    };
    println!("header parsed: {}x{} declared", image.width(), image.height());

    let mut ctx = DecoderContext::default();
    let start = Instant::now();
    let result = image.decode(&mut ctx);
    let elapsed = start.elapsed();
    let total_elapsed = start_all.elapsed();

    match result {
        Ok(_) => println!("decode() succeeded"),
        Err(e) => println!("decode() returned Err: {e:?}"),
    }
    println!("decode() took {:?} (total incl. header: {:?})", elapsed, total_elapsed);
    if let Some(kb) = peak_rss_kb() {
        println!("peak RSS (VmHWM): {kb} kB (~{:.2} MB)", kb as f64 / 1024.0);
    }
}
