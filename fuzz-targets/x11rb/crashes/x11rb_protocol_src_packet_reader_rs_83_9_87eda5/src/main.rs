use x11rb_protocol::packet_reader::PacketReader;

fn main() {
    let mut packet_reader = PacketReader::default();
    let _ = packet_reader.advance(10000usize);
}
