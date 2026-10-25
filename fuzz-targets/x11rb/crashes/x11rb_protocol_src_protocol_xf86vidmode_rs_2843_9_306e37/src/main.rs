use x11rb_protocol::protocol::xf86vidmode::GetGammaRampReply;

fn main() {
    <x11rb_protocol::protocol::xf86vidmode::GetGammaRampReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetGammaRampReply { sequence: 0u16, length: 0u32, size: 0u16, red: vec![0u16], green: vec![], blue: vec![] }, &mut vec![]);
}
