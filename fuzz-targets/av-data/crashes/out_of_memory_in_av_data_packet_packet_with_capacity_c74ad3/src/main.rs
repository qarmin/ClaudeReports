use av_data::packet::Packet;

fn main() {
    let _ = Packet::with_capacity(4294967296usize);
}
