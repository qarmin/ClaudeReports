use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    let mut d_d_sketch = DDSketch::new(Config::default());
    d_d_sketch.add(1.0f64);
    let mut d_d_sketch2 = DDSketch::default();
    d_d_sketch2.add_with_count(3.922934895103397e30f64, u64::MAX);
    let _ = d_d_sketch.merge(&d_d_sketch2);
}
