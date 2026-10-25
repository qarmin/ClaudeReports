#![no_main]
use jxl_fuzz_formats::{FuzzConfig, fuzz_decode};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(whole) = fuzz_decode(data, FuzzConfig::default()) else { return };
    let cfg = FuzzConfig { progressive: true, flush_intermediate: true, ..Default::default() };
    let Ok(chunked) = fuzz_decode(data, cfg) else {
        panic!("chunked decode failed where whole decode succeeded");
    };
    assert_eq!(whole.len(), chunked.len(), "frame count differs");
    for (f, (a, b)) in whole.iter().zip(chunked.iter()).enumerate() {
        assert_eq!(a.len(), b.len());
        for (c, (x, y)) in a.iter().zip(b.iter()).enumerate() {
            assert_eq!(x.size(), y.size());
            for row in 0..x.size().1 {
                for (i, (p, q)) in x.row(row).iter().zip(y.row(row).iter()).enumerate() {
                    assert!(p == q || (p.is_nan() && q.is_nan()), "frame {f} ch {c} ({i},{row}): {p} vs {q}");
                }
            }
        }
    }
});
