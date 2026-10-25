fn main() {
    let _ = <toml::map::Map<String, toml::Value>>::with_capacity(1000000000usize);
}
