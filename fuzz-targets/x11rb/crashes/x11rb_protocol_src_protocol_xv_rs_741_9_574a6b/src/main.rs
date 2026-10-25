use x11rb_protocol::protocol::xv::Image;

fn main() {
    <x11rb_protocol::protocol::xv::Image as x11rb_protocol::x11_utils::Serialize>::serialize_into(&Image { id: 0u32, width: 0u16, height: 0u16, pitches: vec![0u32], offsets: vec![], data: vec![] }, &mut vec![]);
}
