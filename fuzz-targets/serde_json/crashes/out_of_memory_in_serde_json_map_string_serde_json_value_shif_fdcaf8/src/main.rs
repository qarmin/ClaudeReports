use serde_json::Value;

fn main() {
    let mut map = <serde_json::Map<String, serde_json::Value> as ::core::default::Default>::default();
    let _ = map.shift_insert(0usize, "".to_string(), Value::from(<serde_json::Map<String, serde_json::Value>>::with_capacity(1000000000usize)));
}
