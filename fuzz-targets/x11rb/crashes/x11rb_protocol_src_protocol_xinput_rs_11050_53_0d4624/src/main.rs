use x11rb_protocol::protocol::xinput::HierarchyChange;
use x11rb_protocol::protocol::xinput::HierarchyChangeData;

fn main() {
    <x11rb_protocol::protocol::xinput::HierarchyChange as x11rb_protocol::x11_utils::Serialize>::serialize_into(&HierarchyChange { len: 1u16, data: HierarchyChangeData::InvalidValue(0u16) }, &mut vec![]);
}
