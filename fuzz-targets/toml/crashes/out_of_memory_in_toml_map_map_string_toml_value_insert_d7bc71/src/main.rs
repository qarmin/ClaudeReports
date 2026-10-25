use toml::Value;

fn main() {
    let mut map = <toml::map::Map<String, toml::Value> as ::core::default::Default>::default();
    let _ = map.insert("".to_string(), Value::Table(<toml::map::Map<String, toml::Value>>::with_capacity(1000000000usize)));
}
