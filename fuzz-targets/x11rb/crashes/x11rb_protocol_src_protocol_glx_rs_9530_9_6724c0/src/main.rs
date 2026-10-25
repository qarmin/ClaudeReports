use x11rb_protocol::protocol::glx::AreTexturesResidentReply;

fn main() {
    <x11rb_protocol::protocol::glx::AreTexturesResidentReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&AreTexturesResidentReply { sequence: 0u16, ret_val: 0u32, data: vec![false] }, &mut vec![]);
}
