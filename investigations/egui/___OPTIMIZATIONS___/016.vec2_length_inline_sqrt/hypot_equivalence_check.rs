fn fast(x: f32, y: f32) -> f32 { let (x, y) = (f64::from(x), f64::from(y)); (x * x + y * y).sqrt() as f32 }
fn main() {
    let mut s: u64 = 0x9E3779B97F4A7C15;
    let mut next = || { s ^= s << 13; s ^= s >> 7; s ^= s << 17; s };
    let mut bad = 0u64; let mut n = 0u64;
    for round in 0..400_000_000u64 {
        let r = next();
        // mix of magnitudes: normal range, small, large, specials
        let (a, b) = match round % 4 {
            0 => (f32::from_bits(r as u32), f32::from_bits((r >> 32) as u32)),
            1 => (((r as u32) as f32) / 1e3, (((r >> 32) as u32) as f32) / 1e3 - 1e6),
            2 => (((r % 2000) as f32) * 0.03125, (((r >> 20) % 2000) as f32) * 0.03125 - 30.0),
            _ => (((r as i32) as f32) * 1e-9, (((r >> 32) as i32) as f32) * 1e-9),
        };
        let h = a.hypot(b); let f = fast(a, b);
        n += 1;
        if h.to_bits() != f.to_bits() && !(h.is_nan() && f.is_nan()) { bad += 1; if bad < 6 { println!("{a:e} {b:e} {h:e} {f:e}"); } }
    }
    println!("pairs {n} mismatches {bad}");
}
