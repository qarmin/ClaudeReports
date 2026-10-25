use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    let mut d_d_sketch = DDSketch::new(Config::new(0.103f64, 1000000000u32, 1.0f64));
    d_d_sketch.add(f64::MIN);
    d_d_sketch.add(f64::NEG_INFINITY);
}
