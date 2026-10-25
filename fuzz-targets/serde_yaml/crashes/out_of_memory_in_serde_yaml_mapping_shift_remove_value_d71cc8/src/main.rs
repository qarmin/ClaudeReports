use serde_yaml::Mapping;
use serde_yaml::Value;

fn main() {
    let _ = Mapping::with_capacity(1000000000usize).shift_remove::<serde_yaml::Value>(Value::from(0u64));
}
