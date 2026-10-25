use htmlparser::Tokenizer;

fn main() {
    let string = "".to_string();
    let tokenizer = Tokenizer::from_fragment(&string, 1usize..0usize);
    let stream = tokenizer.stream();
    let _ = stream.slice_back(1000000000usize);
}
