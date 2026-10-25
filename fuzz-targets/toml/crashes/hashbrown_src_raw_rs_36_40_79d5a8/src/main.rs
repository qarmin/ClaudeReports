fn main() {
    let _ = <toml::map::Map<String, toml::Value>>::with_capacity(10000000000000000000usize);
}
