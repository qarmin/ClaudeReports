use htmlparser::Stream;

fn main() {
    let string = "".to_string();
    let stream = Stream::from_substr(&string, usize::MAX..0usize);
    let _ = stream.next_byte();
}
