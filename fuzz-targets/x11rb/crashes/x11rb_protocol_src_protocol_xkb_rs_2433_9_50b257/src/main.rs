use x11rb_protocol::protocol::xkb::CountedString16;
use x11rb_protocol::protocol::xkb::GetKbdByNameRepliesGeometry;

fn main() {
    <x11rb_protocol::protocol::xkb::GetKbdByNameRepliesGeometry as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetKbdByNameRepliesGeometry { geometry_type: 0u8, geometry_device_id: 0u8, geometry_sequence: 0u16, geometry_length: 0u32, name: 0u32, geometry_found: false, width_mm: 0u16, height_mm: 0u16, n_properties: 0u16, n_colors: 0u16, n_shapes: 0u16, n_sections: 0u16, n_doodads: 0u16, n_key_aliases: 0u16, base_color_ndx: 0u8, label_color_ndx: 0u8, label_font: CountedString16::default() }, &mut vec![]);
}
