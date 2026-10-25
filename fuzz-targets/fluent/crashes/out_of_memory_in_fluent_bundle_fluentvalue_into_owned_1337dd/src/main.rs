use fluent_bundle::FluentArgs;
use fluent_bundle::builtins::NUMBER;

fn main() {
    let vec = vec![];
    let fluent_args = FluentArgs::with_capacity(1000000000usize);
    let fluent_value = NUMBER(&vec, &fluent_args);
    let _ = fluent_value.into_owned();
}
