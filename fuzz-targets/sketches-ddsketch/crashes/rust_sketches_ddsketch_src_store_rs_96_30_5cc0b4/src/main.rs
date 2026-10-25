use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    let config = Config::defaults();
    let mut d_d_sketch = DDSketch::new(config);
    d_d_sketch.add(1.0f64);
    d_d_sketch.add(f64::INFINITY);
}
