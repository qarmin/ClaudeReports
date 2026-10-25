use x11rb_protocol::protocol::xv::QueryImageAttributesReply;

fn main() {
    <x11rb_protocol::protocol::xv::QueryImageAttributesReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&QueryImageAttributesReply { sequence: 0u16, length: 0u32, data_size: 0u32, width: 0u16, height: 0u16, pitches: vec![], offsets: vec![0u32] }, &mut vec![]);
}
