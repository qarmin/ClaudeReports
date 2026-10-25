use x11rb_protocol::protocol::xinput::DeviceTimeCoord;
use x11rb_protocol::protocol::xinput::GetDeviceMotionEventsReply;
use x11rb_protocol::protocol::xinput::ValuatorMode;

fn main() {
    <x11rb_protocol::protocol::xinput::GetDeviceMotionEventsReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetDeviceMotionEventsReply { xi_reply_type: 0u8, sequence: 0u16, length: 1u32, num_axes: 0u8, device_mode: ValuatorMode::ABSOLUTE, events: vec![DeviceTimeCoord { time: 0u32, axisvalues: vec![0i32] }] }, &mut vec![]);
}
