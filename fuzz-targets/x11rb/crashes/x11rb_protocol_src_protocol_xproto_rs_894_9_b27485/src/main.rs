use x11rb_protocol::protocol::xproto::SetupAuthenticate;

fn main() {
    <x11rb_protocol::protocol::xproto::SetupAuthenticate as x11rb_protocol::x11_utils::Serialize>::serialize_into(&SetupAuthenticate { status: 0u8, reason: vec![51u8] }, &mut vec![]);
}
