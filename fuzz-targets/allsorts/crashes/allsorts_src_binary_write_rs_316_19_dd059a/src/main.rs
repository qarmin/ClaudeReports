use allsorts::binary::write::WriteBuffer;

fn main() {
    let mut write_buffer = WriteBuffer::new();
    let _ = <allsorts::binary::write::WriteBuffer as allsorts::binary::write::WriteContext>::write_zeros(&mut write_buffer, 10000000000000000000usize);
}
