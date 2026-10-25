use cfg_expr::ParseError;
use cfg_expr::error::Reason;

fn main() {
    let parse_error = ParseError { original: "".to_string(), span: 11usize..11660897532026927812usize, reason: Reason::InvalidNot(18446744073709551614usize) };
    let _ = parse_error.to_string();
}
