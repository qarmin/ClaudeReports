use roxmltree::Document;

fn main() {
    let string = include_str!("string_0.txt").to_string();
    let document = Document::parse(&string).unwrap();
    let node = document.root_element();
    let document2 = node.document();
    let _ = document2.text_pos_at(1389usize);
}
