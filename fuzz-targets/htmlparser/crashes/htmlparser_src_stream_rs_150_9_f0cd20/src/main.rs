use htmlparser::Stream;

fn main() {
    let string = "".to_string();
    let mut stream = Stream::from_substr(&string, 0usize..0usize);
    stream.advance(1usize);
}
