fn main() {
    let map = <toml::map::Map<String, toml::Value>>::with_capacity(1000000000000usize);
    let string = "".to_string();
    let _ = map.get::<String>(&string);
}
