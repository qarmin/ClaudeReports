use x11rb_protocol::protocol::xinput::DeviceCtl;
use x11rb_protocol::protocol::xinput::DeviceCtlData;

fn main() {
    <x11rb_protocol::protocol::xinput::DeviceCtl as x11rb_protocol::x11_utils::Serialize>::serialize_into(&DeviceCtl { len: 0u16, data: DeviceCtlData::InvalidValue(1u16) }, &mut vec![]);
}
