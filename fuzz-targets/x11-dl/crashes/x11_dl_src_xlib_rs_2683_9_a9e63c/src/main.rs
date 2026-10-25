use x11_dl::xlib::ClientMessageData;

fn main() {
    let client_message_data = ClientMessageData::default();
    let _ = client_message_data.get_short(100usize);
}
