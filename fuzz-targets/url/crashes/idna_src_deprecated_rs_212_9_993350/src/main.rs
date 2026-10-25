use idna::Config;

fn main() {
    let _ = Config::default().use_idna_2008_rules(true);
}
