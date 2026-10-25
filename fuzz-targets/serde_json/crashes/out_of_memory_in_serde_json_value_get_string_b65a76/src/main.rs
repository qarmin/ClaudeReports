use serde_json::Value;

fn main() {
    let value = Value::from(<serde_json::Map<String, serde_json::Value>>::with_capacity(1000000000usize));
    let _ = value.get::<String>("".to_string());
}
