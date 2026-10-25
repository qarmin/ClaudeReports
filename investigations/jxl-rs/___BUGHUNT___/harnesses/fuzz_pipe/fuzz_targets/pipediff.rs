#![no_main]
use jxl_fuzz_pipe::{FuzzConfig, fuzz_decode};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(a) = fuzz_decode(data, FuzzConfig::default()) else { return };
    let b = match fuzz_decode(data, FuzzConfig { simple_pipeline: true, ..Default::default() }) {
        Ok(b) => b,
        Err(()) => panic!("simple pipeline failed where low-memory pipeline succeeded"),
    };
    assert_eq!(a.len(), b.len(), "frame count");
    for (f, (x, y)) in a.iter().zip(b.iter()).enumerate() {
        for (c, (p, q)) in x.iter().zip(y.iter()).enumerate() {
            assert_eq!(p.size(), q.size());
            for row in 0..p.size().1 {
                for (i, (u, v)) in p.row(row).iter().zip(q.row(row).iter()).enumerate() {
                    assert!(u == v || (u.is_nan() && v.is_nan()), "frame {f} ch {c} ({i},{row}): low-mem {u} vs simple {v}");
                }
            }
        }
    }
});
