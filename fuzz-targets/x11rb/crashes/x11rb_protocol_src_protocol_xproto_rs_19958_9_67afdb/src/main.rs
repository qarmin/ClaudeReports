use x11rb_protocol::protocol::xproto::GetImageReply;

fn main() {
    <x11rb_protocol::protocol::xproto::GetImageReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetImageReply { depth: 0u8, sequence: 1u16, visual: 0u32, data: vec![39u8] }, &mut vec![]);
}
