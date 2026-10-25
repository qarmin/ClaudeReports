use getopts::Options;

fn main() {
    let mut options = Options::default();
    let _ = options.optmulti("", "a", "", "");
}
