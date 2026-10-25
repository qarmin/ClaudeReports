use getopts::Fail;
use getopts::Options;

fn main() {
    let mut options = Options::default();
    let fail = Fail::OptionDuplicated("".to_string());
    let string = fail.to_string();
    let _ = options.optflag("م", &string, "");
}
