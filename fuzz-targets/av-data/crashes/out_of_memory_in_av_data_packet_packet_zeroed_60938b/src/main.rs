use av_data::packet::Packet;

fn main() {
    let _ = Packet::zeroed(2147483648usize);
}
