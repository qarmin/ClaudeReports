use toml::Value;

fn main() {
    let _ = Value::from(<toml::map::Map<String, toml::Value>>::with_capacity(1000000000usize));
}
