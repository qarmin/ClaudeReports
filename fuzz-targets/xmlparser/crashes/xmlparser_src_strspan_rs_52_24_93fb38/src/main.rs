use xmlparser::Stream;

fn main() {
    let string = "".to_string();
    let stream = Stream::from_substr(&string, 0usize..1usize);
    let _ = stream.slice_tail();
}
