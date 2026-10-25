use sketches_ddsketch::DDSketch;

fn main() {
    let _ = DDSketch::from_java_bytes(&include_bytes!("data_0.bin").to_vec());
}
