use x11_dl::xlib::ClientMessageData;

fn main() {
    let client_message_data = ClientMessageData::from([1u64, 0u64, 0u64, 0u64, 1u64]);
    let _ = client_message_data.get_byte(1000000000usize);
}
