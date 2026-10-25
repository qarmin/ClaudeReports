fn main() {
    let map = <toml::map::Map<String, toml::Value>>::with_capacity(1000000000usize);
    let _ = map.values();
}
