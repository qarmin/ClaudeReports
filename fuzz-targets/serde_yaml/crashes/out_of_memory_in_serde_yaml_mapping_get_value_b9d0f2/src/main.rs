use serde_yaml::Mapping;

fn main() {
    let mapping = Mapping::default();
    let _ = mapping.get::<serde_yaml::Value>(Mapping::with_capacity(1000000000usize).swap_remove::<String>("".to_string()).unwrap());
}
