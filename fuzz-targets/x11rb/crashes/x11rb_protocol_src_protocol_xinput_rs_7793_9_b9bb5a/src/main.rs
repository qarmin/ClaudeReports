use x11rb_protocol::protocol::xinput::DeviceStateDataResolution;
use x11rb_protocol::protocol::xselinux::ListSelectionsReply;

fn main() {
    let list_selections_reply = ListSelectionsReply::default();
    let u32_v = list_selections_reply.selections_len();
    <x11rb_protocol::protocol::xinput::DeviceStateDataResolution as x11rb_protocol::x11_utils::Serialize>::serialize_into(&DeviceStateDataResolution { resolution_values: vec![0u32, 0u32, 0u32, 0u32, 0u32], resolution_min: vec![0u32, 0u32, 0u32, u32_v, 1u32], resolution_max: vec![] }, &mut vec![]);
}
