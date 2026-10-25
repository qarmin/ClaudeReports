use xmlparser::Tokenizer;

fn main() {
    let string = "".to_string();
    let tokenizer = Tokenizer::from_fragment(&string, 1usize..0usize);
    let mut stream = tokenizer.stream();
    let _ = stream.skip_name();
}
