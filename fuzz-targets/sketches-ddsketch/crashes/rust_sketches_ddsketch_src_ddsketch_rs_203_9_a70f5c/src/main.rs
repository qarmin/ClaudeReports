use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    let vec = sketches_ddsketch::encoding::encode_to_java_bytes(&DDSketch::new(Config::default()));
    let mut d_d_sketch = DDSketch::default();
    let mut d_d_sketch2 = sketches_ddsketch::encoding::decode_from_java_bytes(&vec).unwrap();
    d_d_sketch2.add_with_count(0.0f64, u64::MAX);
    d_d_sketch.add(0.0f64);
    let _ = d_d_sketch.merge(&d_d_sketch2);
}
