use semver_parser::parser::Parser;

fn main() {
    let string = include_str!("string_0.txt").to_string();
    let mut parser = Parser::new(&string).unwrap();
    let _ = parser.version();
}
