use serde_json::Value;

fn main() {
    let value = <serde_json::Map<String, serde_json::Value>>::with_capacity(10000000000usize).shift_insert(0usize, "".to_string(), Value::from(0u8)).unwrap();
    let _ = value.as_object();
}
