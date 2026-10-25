use htmlparser::Tokenizer;

fn main() {
    let string = "".to_string();
    let tokenizer = Tokenizer::from_fragment(&string, 0usize..1000000000usize);
    let stream = tokenizer.stream();
    let _ = stream.next_byte();
}
