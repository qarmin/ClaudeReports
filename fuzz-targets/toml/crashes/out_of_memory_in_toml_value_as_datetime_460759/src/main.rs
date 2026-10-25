use toml::Value;

fn main() {
    let value = Value::from(<toml::map::Map<String, toml::Value>>::with_capacity(1000000000usize));
    let _ = value.as_datetime();
}
