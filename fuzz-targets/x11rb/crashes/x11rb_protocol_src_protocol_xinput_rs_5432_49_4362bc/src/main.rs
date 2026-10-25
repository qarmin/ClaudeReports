use x11rb_protocol::protocol::xinput::ChangeFeedbackControlMask;
use x11rb_protocol::protocol::xinput::ChangeFeedbackControlRequest;
use x11rb_protocol::protocol::xinput::FeedbackCtl;
use x11rb_protocol::protocol::xinput::FeedbackCtlData;

fn main() {
    let change_feedback_control_request = ChangeFeedbackControlRequest { mask: ChangeFeedbackControlMask::PERCENT, device_id: 0u8, feedback_id: 0u8, feedback: FeedbackCtl { feedback_id: 0u8, len: 0u16, data: FeedbackCtlData::InvalidValue(0u8) } };
    let _ = change_feedback_control_request.clone().serialize(0u8);
}
