use serde_json::Value;

fn main() {
    let _ = Value::from(<serde_json::Map<String, serde_json::Value>>::with_capacity(1000000000usize));
}
