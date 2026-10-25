use serde_yaml::Mapping;

fn main() {
    let mapping = Mapping::with_capacity(1000000000usize);
    let _ = mapping.iter();
}
