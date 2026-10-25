use serde_yaml::Mapping;
use serde_yaml::Value;

fn main() {
    let value = Mapping::with_capacity(1000000000usize).insert(Value::from(0i16), Value::Bool(false)).unwrap();
    let _ = value.as_mapping();
}
