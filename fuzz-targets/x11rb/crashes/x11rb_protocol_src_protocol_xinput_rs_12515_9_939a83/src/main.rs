use x11rb_protocol::protocol::xinput::DeviceClass;
use x11rb_protocol::protocol::xinput::DeviceClassData;
use x11rb_protocol::protocol::xinput::DeviceClassDataButton;

fn main() {
    <x11rb_protocol::protocol::xinput::DeviceClass as x11rb_protocol::x11_utils::Serialize>::serialize_into(&DeviceClass { len: 0u16, sourceid: 0u16, data: DeviceClassData::Button(DeviceClassDataButton { state: vec![0u32], labels: vec![] }) }, &mut vec![]);
}
