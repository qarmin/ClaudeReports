use serde_yaml::Mapping;
use serde_yaml::Value;

fn main() {
    let _ = Value::Mapping(Mapping::with_capacity(1000000000usize));
}
