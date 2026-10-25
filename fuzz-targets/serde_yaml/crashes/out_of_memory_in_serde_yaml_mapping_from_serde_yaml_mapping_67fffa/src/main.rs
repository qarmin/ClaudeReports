use serde_yaml::Mapping;

fn main() {
    let _ = Mapping::from(Mapping::with_capacity(1000000000usize));
}
