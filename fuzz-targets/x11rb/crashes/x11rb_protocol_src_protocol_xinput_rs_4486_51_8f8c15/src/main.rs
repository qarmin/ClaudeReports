use x11rb_protocol::protocol::xinput::FeedbackState;
use x11rb_protocol::protocol::xinput::FeedbackStateData;

fn main() {
    <x11rb_protocol::protocol::xinput::FeedbackState as x11rb_protocol::x11_utils::Serialize>::serialize_into(&FeedbackState { feedback_id: 0u8, len: 0u16, data: FeedbackStateData::InvalidValue(0u8) }, &mut vec![]);
}
