use sketches_ddsketch::DDSketch;

fn main() {
    let mut d_d_sketch = DDSketch::default();
    d_d_sketch.add(-0.0f64);
    d_d_sketch.add_with_count(1.0f64, u64::MAX);
    let _ = d_d_sketch.sum();
}
