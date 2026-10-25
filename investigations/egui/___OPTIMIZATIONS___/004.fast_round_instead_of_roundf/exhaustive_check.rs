#[inline(always)]
fn fast_round(x: f32) -> f32 {
    if x.abs() < 8_388_608.0 {
        let t = x as i32 as f32;
        let d = x - t;
        let r = if d >= 0.5 { t + 1.0 } else if d <= -0.5 { t - 1.0 } else { t };
        r.copysign(x)
    } else {
        x
    }
}
fn main() {
    let mut bad = 0u64;
    for bits in 0..=u32::MAX {
        let x = f32::from_bits(bits);
        let a = x.round();
        let b = fast_round(x);
        if a.to_bits() != b.to_bits() && !(a.is_nan() && b.is_nan()) { bad += 1; if bad < 5 { println!("{x:?} {a:?} {b:?}"); } }
    }
    println!("mismatches: {bad}");
}
