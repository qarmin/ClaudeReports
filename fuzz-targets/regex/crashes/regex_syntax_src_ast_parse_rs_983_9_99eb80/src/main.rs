use regex_syntax::ast::parse::Parser;

fn main() {
    let mut parser = Parser::new();
    let _ = parser.parse_with_comments("a");
    let _ = parser.parse("");
}
