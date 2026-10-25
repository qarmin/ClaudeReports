use serde_yaml::Mapping;

fn main() {
    let mut mapping = Mapping::default();
    let _ = mapping.swap_remove_entry::<serde_yaml::Value>(Mapping::with_capacity(1000000000usize).remove::<String>("".to_string()).unwrap());
}
