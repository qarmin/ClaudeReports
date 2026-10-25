use x11rb_protocol::id_allocator::IdAllocator;
use x11rb_protocol::protocol::xc_misc::GetXIDRangeReply;
use x11rb_protocol::protocol::xproto::GetImageReply;

fn main() {
    let get_image_reply = GetImageReply::default();
    let u32_v = get_image_reply.length();
    let mut id_allocator = IdAllocator::new(u32_v, 1u32).unwrap();
    let _ = id_allocator.update_xid_range(&GetXIDRangeReply { sequence: 0u16, length: u32_v, start_id: 10u32, count: u32::MAX });
}
