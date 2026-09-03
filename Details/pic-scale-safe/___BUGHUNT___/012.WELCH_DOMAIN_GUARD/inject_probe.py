import sys

path = "src/math/welch.rs"
marker = "1f32.as_() - x * x\n    }\n}"
probe_mod = """

#[cfg(test)]
mod bughunt_probe {
    use super::welch;

    #[test]
    fn negative_domain_guard() {
        let v = welch::<f32>(-2.0);
        eprintln!("welch(-2.0) = {v}");
        assert_eq!(v, 0.0f32, "welch(-2.0) should be 0.0 outside its support");
    }

    #[test]
    fn negative_boundary() {
        let v = welch::<f32>(-1.0);
        eprintln!("welch(-1.0) = {v}");
        assert_eq!(v, 0.0f32, "welch(-1.0) should be 0.0 at its support boundary");
    }
}
"""

with open(path) as f:
    s = f.read()

if marker not in s:
    sys.exit("marker not found - welch.rs source changed shape, update inject_probe.py")

with open(path, "w") as f:
    f.write(s.replace(marker, marker + probe_mod))
