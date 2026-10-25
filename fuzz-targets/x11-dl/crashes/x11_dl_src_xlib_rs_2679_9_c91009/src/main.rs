use x11_dl::xlib::ClientMessageData;

fn main() {
    let client_message_data = ClientMessageData::from([0i64, 0i64, 0i64, 0i64, 0i64]);
    let _ = client_message_data.get_long(1000000000usize);
}
