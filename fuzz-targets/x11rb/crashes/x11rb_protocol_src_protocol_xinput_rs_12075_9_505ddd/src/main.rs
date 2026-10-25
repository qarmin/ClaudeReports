use x11rb_protocol::protocol::xinput::ButtonClass;
use x11rb_protocol::protocol::xinput::DeviceClassType;

fn main() {
    <x11rb_protocol::protocol::xinput::ButtonClass as x11rb_protocol::x11_utils::Serialize>::serialize_into(&ButtonClass { type_: DeviceClassType::from(0u16), len: 0u16, sourceid: 0u16, state: vec![0u32], labels: vec![] }, &mut vec![]);
}
