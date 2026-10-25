use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    let mut d_d_sketch = DDSketch::new(Config::new(0.063f64, 1000000000u32, 1.0f64));
    d_d_sketch.add(f64::MAX);
    d_d_sketch.add_with_count(f64::INFINITY, 1u64);
}
