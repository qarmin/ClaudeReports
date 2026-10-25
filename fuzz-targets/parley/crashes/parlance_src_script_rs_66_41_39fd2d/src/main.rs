use parlance::Script;

fn main() {
    let script = Script::from_bytes([255u8, 0u8, 0u8, 0u8]);
    let _ = script.as_str();
}
