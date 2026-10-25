use htmlparser::Tokenizer;

fn main() {
    let string = "".to_string();
    let tokenizer = Tokenizer::from_fragment(&string, 0usize..1usize);
    let mut stream = tokenizer.stream();
    let _ = stream.consume_name();
}
