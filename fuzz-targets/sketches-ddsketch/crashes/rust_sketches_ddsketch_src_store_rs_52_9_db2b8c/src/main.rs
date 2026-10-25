use sketches_ddsketch::DDSketch;

fn main() {
    let mut d_d_sketch = DDSketch::default();
    d_d_sketch.add_with_count(41.0f64, u64::MAX);
    d_d_sketch.add_with_count(f64::INFINITY, 18446744073709551614u64);
}
