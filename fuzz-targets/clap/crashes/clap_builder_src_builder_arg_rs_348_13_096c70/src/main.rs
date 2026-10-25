use clap_builder::Arg;

fn main() {
    let arg = Arg::default();
    let _ = arg.clone().short_aliases(vec!['-']);
}
