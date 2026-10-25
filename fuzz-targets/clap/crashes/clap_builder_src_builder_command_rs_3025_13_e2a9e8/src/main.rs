use clap_builder::Command;
use clap_builder::builder::Str;

fn main() {
    let command = Command::new(Str::default());
    let _ = command.clone().visible_short_flag_aliases(vec!['-']);
}
