const SRGB_GAMMA: f32 = 2.4;
const GAMMA_EXP: f32 = 1.0 / SRGB_GAMMA;
const MANTISSA_LUT_SIZE: usize = 1024;
const EXP_LUT_MIN: i32 = -16;
const EXP_LUT_MAX: i32 = 16;
const EXP_LUT_SIZE: usize = (EXP_LUT_MAX - EXP_LUT_MIN + 1) as usize;

fn mantissa_lut() -> [f32; MANTISSA_LUT_SIZE + 1] {
    let mut table = [0f32; MANTISSA_LUT_SIZE + 1];
    for (i, v) in table.iter_mut().enumerate() {
        let m = 1.0 + i as f32 / MANTISSA_LUT_SIZE as f32;
        *v = m.powf(GAMMA_EXP);
    }
    table
}
fn exp_lut() -> [f32; EXP_LUT_SIZE] {
    let mut table = [0f32; EXP_LUT_SIZE];
    for (i, v) in table.iter_mut().enumerate() {
        let e = EXP_LUT_MIN + i as i32;
        *v = 2f32.powf(e as f32 * GAMMA_EXP);
    }
    table
}

fn fast_pow_gamma(v: f32, mlut: &[f32], elut: &[f32]) -> f32 {
    let bits = v.to_bits();
    let exponent = ((bits >> 23) & 0xff) as i32 - 127;
    if exponent < EXP_LUT_MIN || exponent > EXP_LUT_MAX {
        return v.powf(GAMMA_EXP);
    }
    let mantissa_bits = (bits & 0x007f_ffff) | (127u32 << 23);
    let m = f32::from_bits(mantissa_bits);
    let scaled = (m - 1.0) * MANTISSA_LUT_SIZE as f32;
    let idx = scaled as usize;
    let frac = scaled - idx as f32;
    let m_pow = mlut[idx] + frac * (mlut[idx + 1] - mlut[idx]);
    m_pow * elut[(exponent - EXP_LUT_MIN) as usize]
}

fn main() {
    let mlut = mantissa_lut();
    let elut = exp_lut();
    let mut max_abs_err = 0f64;
    let mut max_rel_err = 0f64;
    let mut max_u16_diff = 0i32;
    let mut worst_v = 0f32;
    // Dense scan over the realistic domain plus some extreme values.
    let n = 20_000_000u64;
    for i in 0..=n {
        let v = 0.00304f32 + (i as f32 / n as f32) * (3.0f32 - 0.00304f32);
        let exact = v.powf(GAMMA_EXP) as f64;
        let approx = fast_pow_gamma(v, &mlut, &elut) as f64;
        let abs_err = (exact - approx).abs();
        if abs_err > max_abs_err {
            max_abs_err = abs_err;
            worst_v = v;
        }
        let rel_err = abs_err / exact.abs().max(1e-12);
        if rel_err > max_rel_err {
            max_rel_err = rel_err;
        }
        // Simulate full downstream: gamma gain/offset then scale to u16.
        let exact_out = (exact * 1.055 - 0.055).clamp(0.0, 1.0);
        let approx_out = (approx * 1.055 - 0.055).clamp(0.0, 1.0);
        let exact_u16 = (exact_out * 65535.0) as i32;
        let approx_u16 = (approx_out * 65535.0) as i32;
        let d = (exact_u16 - approx_u16).abs();
        if d > max_u16_diff {
            max_u16_diff = d;
        }
    }
    println!("max_abs_err={:.3e} at v={:.6}", max_abs_err, worst_v);
    println!("max_rel_err={:.3e}", max_rel_err);
    println!("max_u16_diff={}", max_u16_diff);

    // extreme / edge values
    for &v in &[0.00304f32, 0.0031f32, 1e-4, 1e-6, 1.0, 2.0, 5.0, 100.0, 1e5, f32::MIN_POSITIVE] {
        let exact = v.powf(GAMMA_EXP);
        let approx = fast_pow_gamma(v, &mlut, &elut);
        println!("v={:e} exact={:e} approx={:e} absdiff={:e}", v, exact, approx, (exact-approx).abs());
    }
}
