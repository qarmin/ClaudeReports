use sketches_ddsketch::DDSketch;

fn main() {
    let mut d_d_sketch = DDSketch::default();
    d_d_sketch.add(12.0f64);
    d_d_sketch.add(f64::INFINITY);
}
