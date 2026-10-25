use fluent_bundle::FluentArgs;
use fluent_bundle::types::FluentNumberOptions;

fn main() {
    let mut fluent_number_options = FluentNumberOptions::default();
    let fluent_args = FluentArgs::with_capacity(1000000000usize);
    fluent_number_options.merge(&fluent_args);
}
