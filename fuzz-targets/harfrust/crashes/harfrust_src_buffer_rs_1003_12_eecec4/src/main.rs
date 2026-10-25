use harfrust::Buffer;

fn main() {
    let mut buffer = Buffer::default();
    buffer.reverse_range(10000000000000000000usize, 255usize);
}
