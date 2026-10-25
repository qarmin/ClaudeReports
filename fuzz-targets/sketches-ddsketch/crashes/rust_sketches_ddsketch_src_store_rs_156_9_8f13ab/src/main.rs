use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    let mut d_d_sketch = DDSketch::new(Config::new(0.0f64, 100u32, -15.0f64));
    d_d_sketch.add_with_count(-0.0f64, 1u64);
}
