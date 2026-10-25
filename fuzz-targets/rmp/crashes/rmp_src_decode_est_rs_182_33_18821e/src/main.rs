use rmp::decode::MessageLen;

fn main() {
    let _ = MessageLen::len_of(&include_bytes!("data_0.bin").to_vec());
}
