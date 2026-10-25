use toml::Value;

fn main() {
    let value = Value::Table(<toml::map::Map<String, toml::Value>>::with_capacity(1000000000usize));
    let _ = value.get::<String>("".to_string());
}
