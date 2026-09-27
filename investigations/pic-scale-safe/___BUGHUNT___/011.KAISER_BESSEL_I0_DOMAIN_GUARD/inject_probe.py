import sys

path = "src/math/kaiser.rs"
marker = "bessel_i0(6.33f64.as_() * (1.0f64.as_() - x * x).sqrt()) * i0a\n}"
probe_mod = """

#[cfg(test)]
mod bughunt_probe {
    use super::*;

    #[test]
    fn probe_negative_out_of_domain() {
        let v = kaiser::<f32>(-2.0);
        eprintln!("kaiser(-2.0) = {v}");
        assert_eq!(v, 0.0);
    }

    #[test]
    fn probe_bessel_i0_nan() {
        let v: f32 = bessel_i0(f32::NAN);
        eprintln!("bessel_i0(NaN) = {v}");
    }

    #[test]
    fn probe_bessel_i0_large_f32_overflow() {
        let v: f32 = bessel_i0(1.0e30_f32);
        eprintln!("bessel_i0(1e30_f32) = {v}");
    }
}
"""

with open(path) as f:
    s = f.read()

if marker not in s:
    sys.exit("marker not found - kaiser.rs source changed shape, update inject_probe.py")

with open(path, "w") as f:
    f.write(s.replace(marker, marker + probe_mod))
