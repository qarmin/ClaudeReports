use xmlparser::Stream;

fn main() {
    let string = "".to_string();
    let stream = Stream::from_substr(&string, 0usize..10000000000000000000usize);
    let _ = stream.next_byte();
}
