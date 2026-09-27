use std::hint::black_box;

#[inline(never)]
fn compute(seed: f32) -> f32 {
    let a = black_box(seed);
    let b = a * a;
    let c = -(b * b) * a * b;
    let d = c * c;
    let e = d * -d;
    e * e
}

fn main() {
    let seeds: [f32; 12] = [
        1e30, 1e35, 1e38, 3.4028235e38, -1e30, -3.4028235e38, 2623078656.0, 51216.0,
        1.0, 0.0, f32::NAN, f32::INFINITY,
    ];
    for s in seeds {
        let v = compute(s);
        let u128v = v as u128;
        let i128v = v as i128;
        let u64v = v as u64;
        let i64v = v as i64;
        let u32v = v as u32;
        println!("seed={s:?} val={v:?} bits={:08x} u128={u128v} i128={i128v} u64={u64v} i64={i64v} u32={u32v}", v.to_bits());
    }
}
