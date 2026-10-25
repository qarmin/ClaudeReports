use fluent_bundle::FluentArgs;

fn main() {
    let fluent_args = FluentArgs::with_capacity(1000000000usize);
    let _ = fluent_args.iter();
}
