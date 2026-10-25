use fluent_bundle::FluentArgs;
use fluent_bundle::builtins::NUMBER;

fn main() {
    let vec = vec![];
    let fluent_args = FluentArgs::with_capacity(1000000000usize);
    let _ = NUMBER(&vec, &fluent_args);
}
