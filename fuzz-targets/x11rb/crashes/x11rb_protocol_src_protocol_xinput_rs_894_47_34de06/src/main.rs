use x11rb_protocol::protocol::xinput::InputInfo;
use x11rb_protocol::protocol::xinput::InputInfoInfo;

fn main() {
    <x11rb_protocol::protocol::xinput::InputInfo as x11rb_protocol::x11_utils::Serialize>::serialize_into(&InputInfo { len: 0u8, info: InputInfoInfo::InvalidValue(0u8) }, &mut vec![]);
}
