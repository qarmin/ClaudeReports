fn main() {
    let map = <serde_json::Map<String, serde_json::Value>>::with_capacity(1000000000usize);
    let string = "".to_string();
    let _ = map.get::<String>(&string);
}
