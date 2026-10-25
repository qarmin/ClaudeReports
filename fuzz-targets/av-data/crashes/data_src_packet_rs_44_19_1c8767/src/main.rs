use av_data::packet::Packet;

fn main() {
    let _ = Packet::zeroed(10000000000000000000usize);
}
