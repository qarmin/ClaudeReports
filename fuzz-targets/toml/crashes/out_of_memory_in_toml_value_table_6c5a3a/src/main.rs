use toml::Value;

fn main() {
    let _ = Value::Table(<toml::map::Map<String, toml::Value>>::with_capacity(1000000000usize));
}
