use x11rb_protocol::protocol::randr::GetCrtcGammaReply;

fn main() {
    let _ = <x11rb_protocol::protocol::randr::GetCrtcGammaReply as x11rb_protocol::x11_utils::Serialize>::serialize(&GetCrtcGammaReply { sequence: 0u16, length: 0u32, red: vec![0u16], green: vec![], blue: vec![] });
}
