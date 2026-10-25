use x11rb_protocol::protocol::xinput::DeviceState;
use x11rb_protocol::protocol::xinput::DeviceStateData;
use x11rb_protocol::protocol::xinput::FeedbackCtlDataString;

fn main() {
    let feedback_ctl_data_string = FeedbackCtlDataString { keysyms: vec![] };
    let u16_v = feedback_ctl_data_string.num_keysyms();
    <x11rb_protocol::protocol::xinput::DeviceState as x11rb_protocol::x11_utils::Serialize>::serialize_into(&DeviceState { len: u16_v, data: DeviceStateData::InvalidValue(1u16) }, &mut vec![]);
}
