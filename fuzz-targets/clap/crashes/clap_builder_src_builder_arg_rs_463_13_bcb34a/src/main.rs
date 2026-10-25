use clap_builder::Arg;
use clap_builder::Id;

fn main() {
    let arg = Arg::new(Id::default());
    let _ = arg.clone().visible_short_aliases(vec!['-']);
}
