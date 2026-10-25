use x11rb_protocol::protocol::xkb::GetNamesReply;
use x11rb_protocol::protocol::xkb::GetNamesValueList;
use x11rb_protocol::protocol::xkb::SetOfGroup;
use x11rb_protocol::protocol::xkb::VMod;

fn main() {
    <x11rb_protocol::protocol::xkb::GetNamesReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetNamesReply { device_id: 0u8, sequence: 0u16, length: 0u32, min_key_code: 0u8, max_key_code: 0u8, n_types: 0u8, group_names: SetOfGroup::GROUP2, virtual_mods: VMod::M3, first_key: 0u8, n_keys: 0u8, indicators: 0u32, n_radio_groups: 0u8, n_key_aliases: 0u8, n_kt_levels: 0u16, value_list: GetNamesValueList { keycodes_name: None, geometry_name: None, symbols_name: None, phys_symbols_name: None, types_name: None, compat_name: None, type_names: Some(vec![0u32]), kt_level_names: None, indicator_names: None, virtual_mod_names: None, groups: None, key_names: None, key_aliases: None, radio_group_names: None } }, &mut vec![]);
}
