use toml::Value;

fn main() {
    let value = Value::Table(<toml::map::Map<String, toml::Value>>::with_capacity(100000000000000000usize));
    let _ = value.as_array();
}
