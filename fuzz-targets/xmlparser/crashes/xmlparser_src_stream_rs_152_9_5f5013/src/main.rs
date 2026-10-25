use xmlparser::Tokenizer;

fn main() {
    let string = "".to_string();
    let tokenizer = Tokenizer::from_fragment(&string, 0usize..0usize);
    let mut stream = tokenizer.stream();
    stream.advance(1usize);
}
