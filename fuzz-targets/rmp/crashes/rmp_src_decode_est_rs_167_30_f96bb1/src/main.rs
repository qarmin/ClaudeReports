use rmp::decode::MessageLen;

fn main() {
    let _ = MessageLen::len_of(&vec![201u8]);
}
