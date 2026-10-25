fn main() {
    let value = <serde_json::Map<String, serde_json::Value>>::with_capacity(1000000000usize).insert("".to_string(), <serde_json::Value as ::core::str::FromStr>::from_str("").unwrap()).unwrap();
    let _ = value.get::<usize>(1usize);
}
