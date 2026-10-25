use x11rb_protocol::protocol::xinput::InputState;
use x11rb_protocol::protocol::xinput::InputStateData;

fn main() {
    <x11rb_protocol::protocol::xinput::InputState as x11rb_protocol::x11_utils::Serialize>::serialize_into(&InputState { len: 0u8, data: InputStateData::InvalidValue(0u8) }, &mut vec![]);
}
