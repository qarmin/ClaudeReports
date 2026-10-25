use htmlparser::Tokenizer;

fn main() {
    let string = "".to_string();
    let tokenizer = Tokenizer::from_fragment(&string, 1usize..0usize);
    let stream = tokenizer.stream();
    let _ = stream.gen_text_pos();
}
