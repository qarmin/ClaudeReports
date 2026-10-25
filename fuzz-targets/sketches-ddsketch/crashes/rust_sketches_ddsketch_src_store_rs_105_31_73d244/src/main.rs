use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    let mut d_d_sketch = DDSketch::new(Config::new(-0.0f64, 0u32, 1.0f64));
    d_d_sketch.add(1e300f64);
}
