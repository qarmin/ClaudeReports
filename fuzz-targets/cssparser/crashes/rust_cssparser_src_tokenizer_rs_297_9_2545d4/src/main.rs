use cssparser::Parser;

fn main() {
    let string = "a".to_string();
    let mut parser = Parser::new(&string);
    let _ = cssparser::parse_nth(&mut parser);
    let string2 = "".to_string();
    let parser2 = Parser::new(&string2);
    let source_position = parser.position();
    let _ = parser2.slice_from(source_position);
}
