use sketches_ddsketch::DDSketch;

fn main() {
    let mut d_d_sketch = DDSketch::default();
    d_d_sketch.add_with_count(0.0f64, u64::MAX);
    d_d_sketch.add_with_count(0.0f64, 1u64);
}
