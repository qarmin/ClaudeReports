use x11rb_protocol::protocol::xinput::DeviceState;
use x11rb_protocol::protocol::xinput::DeviceStateData;
use x11rb_protocol::protocol::xinput::DeviceStateDataResolution;

fn main() {
    <x11rb_protocol::protocol::xinput::DeviceState as x11rb_protocol::x11_utils::Serialize>::serialize_into(&DeviceState { len: 0u16, data: DeviceStateData::Resolution(DeviceStateDataResolution { resolution_values: vec![], resolution_min: vec![0u32], resolution_max: vec![] }) }, &mut vec![]);
}
