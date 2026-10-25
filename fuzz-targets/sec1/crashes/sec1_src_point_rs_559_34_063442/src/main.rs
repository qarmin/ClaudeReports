use sec1::point::Tag;

fn main() {
    let tag = Tag::Uncompressed;
    let _ = tag.message_len(usize::MAX);
}
