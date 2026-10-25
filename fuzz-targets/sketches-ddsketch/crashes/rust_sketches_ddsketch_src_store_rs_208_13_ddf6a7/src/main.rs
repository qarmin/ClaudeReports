use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    let mut d_d_sketch = DDSketch::new(Config::default());
    d_d_sketch.add_with_count(1.0f64, u64::MAX);
    let vec = sketches_ddsketch::encoding::encode_to_java_bytes(&d_d_sketch);
    let mut d_d_sketch2 = sketches_ddsketch::encoding::decode_from_java_bytes(&vec).unwrap();
    let _ = d_d_sketch2.merge(&d_d_sketch);
}
