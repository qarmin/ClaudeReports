use x11rb_protocol::protocol::xkb::SetKeyType;
use x11rb_protocol::protocol::xkb::VMod;
use x11rb_protocol::protocol::xproto::ModMask;
use x11rb_protocol::protocol::xproto::SetupFailed;

fn main() {
    let set_key_type = SetKeyType { mask: ModMask::M1, real_mods: ModMask::ANY, virtual_mods: VMod::M7, num_levels: 0u8, preserve: false, entries: vec![], preserve_entries: vec![] };
    let u8_v = set_key_type.n_map_entries();
    <x11rb_protocol::protocol::xproto::SetupFailed as x11rb_protocol::x11_utils::Serialize>::serialize_into(&SetupFailed { status: u8_v, protocol_major_version: 0u16, protocol_minor_version: 0u16, length: 0u16, reason: include_bytes!("data_0.bin").to_vec() }, &mut vec![]);
}
