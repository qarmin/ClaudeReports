use x11rb_protocol::protocol::xinput::DeviceName;

fn main() {
    <x11rb_protocol::protocol::xinput::DeviceName as x11rb_protocol::x11_utils::Serialize>::serialize_into(&DeviceName { string: include_bytes!("data_0.bin").to_vec() }, &mut vec![]);
}
