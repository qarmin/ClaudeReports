use serde_json::Value;

fn main() {
    let _ = Value::Object(<serde_json::Map<String, serde_json::Value>>::with_capacity(100000000000000usize));
}
