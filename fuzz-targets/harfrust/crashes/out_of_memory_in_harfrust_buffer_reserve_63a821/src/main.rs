use harfrust::Buffer;

fn main() {
    let mut buffer = Buffer::new();
    let _ = buffer.reserve(100000000usize);
}
