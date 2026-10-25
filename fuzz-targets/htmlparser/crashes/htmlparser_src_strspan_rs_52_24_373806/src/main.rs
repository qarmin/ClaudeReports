use htmlparser::Stream;

fn main() {
    let string = "".to_string();
    let stream = Stream::from_substr(&string, 1usize..0usize);
    let _ = stream.slice_back(0usize);
}
