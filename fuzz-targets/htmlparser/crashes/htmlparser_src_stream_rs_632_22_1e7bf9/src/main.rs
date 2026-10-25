use htmlparser::Stream;

fn main() {
    let string = "Zażółć".to_string();
    let mut stream = Stream::from_substr(&string, 9usize..10000000000000000000usize);
    let _ = stream.consume_quote();
}
