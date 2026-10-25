use x11rb_protocol::protocol::xkb::KTSetMapEntry;
use x11rb_protocol::protocol::xkb::SetKeyType;
use x11rb_protocol::protocol::xkb::VMod;
use x11rb_protocol::protocol::xproto::ModMask;

fn main() {
    <x11rb_protocol::protocol::xkb::SetKeyType as x11rb_protocol::x11_utils::Serialize>::serialize_into(&SetKeyType { mask: ModMask::M1, real_mods: ModMask::ANY, virtual_mods: VMod::default(), num_levels: 0u8, preserve: true, entries: vec![KTSetMapEntry::default()], preserve_entries: vec![] }, &mut vec![]);
}
