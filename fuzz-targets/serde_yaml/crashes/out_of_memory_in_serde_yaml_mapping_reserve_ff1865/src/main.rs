use serde_yaml::Mapping;

fn main() {
    let mut mapping = Mapping::from(Mapping::with_capacity(0usize));
    mapping.reserve(1000000000usize);
}
