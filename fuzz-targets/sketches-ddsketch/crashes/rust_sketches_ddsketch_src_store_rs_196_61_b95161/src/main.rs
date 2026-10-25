use sketches_ddsketch::DDSketch;

fn main() {
    let mut d_d_sketch = DDSketch::default();
    d_d_sketch.add(131.0f64);
    d_d_sketch.add(f64::INFINITY);
    let vec = sketches_ddsketch::encoding::encode_to_java_bytes(&d_d_sketch);
    let _ = d_d_sketch.merge(&DDSketch::from_java_bytes(&vec).unwrap());
}
