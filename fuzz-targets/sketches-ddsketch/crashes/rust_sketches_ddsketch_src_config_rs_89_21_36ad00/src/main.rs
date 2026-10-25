use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    let vec = sketches_ddsketch::encoding::encode_to_java_bytes(&DDSketch::new(Config::new(0.0f64, 0u32, 1.0f64)));
    let _ = sketches_ddsketch::encoding::decode_from_java_bytes(&vec);
}
