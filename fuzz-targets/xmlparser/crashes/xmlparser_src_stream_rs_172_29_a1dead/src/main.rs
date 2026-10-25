use xmlparser::Tokenizer;

fn main() {
    let string = "".to_string();
    let tokenizer = Tokenizer::from_fragment(&string, 0usize..0usize);
    let stream = tokenizer.stream();
    let text_pos = stream.gen_text_pos_from(0usize);
    let string2 = text_pos.to_string();
    let tokenizer2 = Tokenizer::from_fragment(&string2, 1usize..0usize);
    let stream2 = tokenizer2.stream();
    let _ = stream2.starts_with(&vec![]);
}
