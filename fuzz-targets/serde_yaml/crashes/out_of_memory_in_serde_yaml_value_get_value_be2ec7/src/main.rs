use serde_yaml::Mapping;
use serde_yaml::Value;
use serde_yaml::value::Tag;
use serde_yaml::value::TaggedValue;

fn main() {
    let value = Value::Tagged(Box::new(TaggedValue { tag: Tag::new("a".to_string()), value: Value::from(0i16) }));
    let _ = value.get::<serde_yaml::Value>(Mapping::with_capacity(1000000000usize).remove::<String>("".to_string()).unwrap());
}
