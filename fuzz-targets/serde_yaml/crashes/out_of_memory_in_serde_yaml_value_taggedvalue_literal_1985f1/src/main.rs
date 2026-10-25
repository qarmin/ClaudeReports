use serde_yaml::Mapping;
use serde_yaml::value::Tag;
use serde_yaml::value::TaggedValue;

fn main() {
    let _ = TaggedValue { tag: Tag::new("K".to_string()), value: Mapping::with_capacity(1000000000usize).remove::<String>("".to_string()).unwrap() };
}
