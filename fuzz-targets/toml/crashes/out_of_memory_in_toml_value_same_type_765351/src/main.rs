use toml::Value;

fn main() {
    let value = Value::from(0i64);
    let _ = value.same_type(&Value::from(<toml::map::Map<String, toml::Value>>::with_capacity(1000000000usize)));
}
