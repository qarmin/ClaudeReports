use xmlparser::Tokenizer;

fn main() {
    let string = "".to_string();
    let tokenizer = Tokenizer::from_fragment(&string, usize::MAX..0usize);
    let stream = tokenizer.stream();
    let _ = stream.next_byte();
}
