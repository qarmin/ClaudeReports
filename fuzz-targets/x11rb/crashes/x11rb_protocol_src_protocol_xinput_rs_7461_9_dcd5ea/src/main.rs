use x11rb_protocol::protocol::xinput::DeviceControl;
use x11rb_protocol::protocol::xinput::DeviceResolutionState;

fn main() {
    <x11rb_protocol::protocol::xinput::DeviceResolutionState as x11rb_protocol::x11_utils::Serialize>::serialize_into(&DeviceResolutionState { control_id: DeviceControl::ENABLE, len: 0u16, resolution_values: vec![], resolution_min: vec![], resolution_max: vec![0u32] }, &mut vec![]);
}
