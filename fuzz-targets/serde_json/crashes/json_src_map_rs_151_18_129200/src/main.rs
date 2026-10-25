use serde_json::Value;

fn main() {
    let mut map = <serde_json::Map<String, serde_json::Value> as ::core::default::Default>::default();
    let _ = map.shift_insert(1usize, "".to_string(), Value::Object(<serde_json::Map<String, serde_json::Value>>::with_capacity(0usize)));
}
