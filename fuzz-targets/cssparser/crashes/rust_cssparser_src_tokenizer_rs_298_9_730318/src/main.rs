use cssparser::Parser;

fn main() {
    let string = "".to_string();
    let parser = Parser::new(&string);
    let parser_state = parser.state();
    let string2 = "Y".to_string();
    let mut parser2 = Parser::new(&string2);
    let _ = cssparser::parse_nth(&mut parser2);
    let _ = parser.slice(parser_state.position()..parser2.position());
}
