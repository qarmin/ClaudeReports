use x11rb_protocol::id_allocator::IdAllocator;
use x11rb_protocol::protocol::xc_misc::GetXIDRangeReply;

fn main() {
    let mut id_allocator = IdAllocator::new(0u32, 10000u32).unwrap();
    let _ = id_allocator.update_xid_range(&GetXIDRangeReply { sequence: 0u16, length: 0u32, start_id: 0u32, count: 1000000000u32 });
}
