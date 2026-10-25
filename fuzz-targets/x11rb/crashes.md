# x11rb - crashes

Source: https://github.com/psychon/x11rb @ 86c4252fe023fddd523f0ac3c7c07c40ba266ce0

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let set_key_type = SetKeyType { mask: ModMask::M1, real_mods: ModMask::ANY,
    virtual_mods: VMod::M7, num_levels: 0u8, preserve: false, entries: vec![],
    preserve_entries: vec![] };
let u8_v = set_key_type.n_map_entries();
&lt;x11rb_protocol::protocol::xproto::SetupFailed as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;SetupFailed { status: u8_v,
    protocol_major_version: 0u16, protocol_minor_version: 0u16, length: 0u16,
    reason: include_bytes!("data_0.bin").to_vec() }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xproto.rs:840:58:
`reason` has too many elements: TryFromIntError(PosOverflow)
backtrace:
  &lt;x11rb_protocol::protocol::xproto::SetupFailed as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xproto.rs:840:58)
  repro_x11rb_protocol_src_protocol_xproto_rs_840_58_81b9bc::main (src/main.rs:9:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xfixes::
    GetCursorImageAndNameReply as x11rb_protocol::x11_utils::Serialize&gt;::
    serialize_into(&amp;GetCursorImageAndNameReply { sequence: 0u16, length: 0u32,
    x: 0i16, y: 0i16, width: 0u16, height: 1u16, xhot: 0u16, yhot: 0u16,
    cursor_serial: 0u32, cursor_atom: 0u32, cursor_image: vec![0u32],
    name: vec![] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xfixes.rs:2916:9:
assertion `left == right` failed: `cursor_image` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xfixes::GetCursorImageAndNameReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xfixes.rs:2916:9)
  repro_x11rb_protocol_src_protocol_xfixes_rs_2916_9_a7b6bd::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xfixes::GetCursorImageReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(&amp;GetCursorImageReply { sequence: 0u16,
    length: 0u32, x: 0i16, y: 0i16, width: 0u16, height: 0u16, xhot: 0u16,
    yhot: 1u16, cursor_serial: 0u32, cursor_image: vec![0u32] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xfixes.rs:1200:9:
assertion `left == right` failed: `cursor_image` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xfixes::GetCursorImageReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xfixes.rs:1200:9)
  repro_x11rb_protocol_src_protocol_xfixes_rs_1200_9_89e41c::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let change_feedback_control_request = ChangeFeedbackControlRequest { mask: ChangeFeedbackControlMask::PERCENT,
    device_id: 0u8, feedback_id: 0u8, feedback: FeedbackCtl { feedback_id: 0u8,
    len: 0u16, data: FeedbackCtlData::InvalidValue(0u8) } };
let _ = change_feedback_control_request.clone().serialize(0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:5432:49:
attempted to serialize invalid switch case
backtrace:
  &lt;x11rb_protocol::protocol::xinput::FeedbackCtlData&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:5432:49)
  &lt;x11rb_protocol::protocol::xinput::FeedbackCtl as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:5482:19)
  &lt;x11rb_protocol::protocol::xinput::FeedbackCtl as x11rb_protocol::x11_utils::Serialize&gt;::serialize (x11rb-protocol/src/protocol/xinput.rs:5473:14)
  &lt;x11rb_protocol::protocol::xinput::ChangeFeedbackControlRequest&gt;::serialize (x11rb-protocol/src/protocol/xinput.rs:5590:44)
  repro_x11rb_protocol_src_protocol_xinput_rs_5432_49_4362bc::main (src/main.rs:8:53)</pre></td></tr>
<tr><td valign="top"><pre>let get_image_reply = GetImageReply::default();
let u32_v = get_image_reply.length();
let mut id_allocator = IdAllocator::new(u32_v, 1u32).unwrap();
let _ = id_allocator.update_xid_range(&amp;GetXIDRangeReply { sequence: 0u16,
    length: u32_v, start_id: 10u32, count: u32::MAX });</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/id_allocator.rs:54:23:
attempt to add with overflow
backtrace:
  &lt;x11rb_protocol::id_allocator::IdAllocator&gt;::update_xid_range (x11rb-protocol/src/id_allocator.rs:54:23)
  repro_x11rb_protocol_src_id_allocator_rs_54_23_d9f484::main (src/main.rs:9:26)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::randr::GetProviderPropertyReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetProviderPropertyReply { format: 74u8, sequence: 0u16, length: 0u32,
    type_: 0u32, bytes_after: 0u32, num_items: 1000000000u32, data: vec![] },
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/randr.rs:5974:142:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;x11rb_protocol::protocol::randr::GetProviderPropertyReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/randr.rs:5974:142)
  repro_x11rb_protocol_src_protocol_randr_rs_5974_142_0bb8cc::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::InputInfo as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;InputInfo { len: 0u8,
    info: InputInfoInfo::InvalidValue(0u8) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:894:47:
attempted to serialize invalid switch case
backtrace:
  &lt;x11rb_protocol::protocol::xinput::InputInfoInfo&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:894:47)
  &lt;x11rb_protocol::protocol::xinput::InputInfo as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:938:19)
  repro_x11rb_protocol_src_protocol_xinput_rs_894_47_34de06::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::glx::GetPolygonStippleReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetPolygonStippleReply { sequence: 0u16, data: vec![66u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/glx.rs:7471:9:
assertion `left == right` failed: `data` has an incorrect length, must be a multiple of 4
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::glx::GetPolygonStippleReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/glx.rs:7471:9)
  repro_x11rb_protocol_src_protocol_glx_rs_7471_9_8be374::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::GetDevicePropertyReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetDevicePropertyReply { xi_reply_type: 0u8, sequence: 0u16, length: 0u32,
    type_: 0u32, bytes_after: 0u32, num_items: 0u32, device_id: 0u8,
    items: GetDevicePropertyItems::Data16(vec![0u16]) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:9842:17:
assertion `left == right` failed: `data16` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::GetDevicePropertyItems&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:9842:17)
  &lt;x11rb_protocol::protocol::xinput::GetDevicePropertyReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:9924:20)
  repro_x11rb_protocol_src_protocol_xinput_rs_9842_17_db456a::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let get_query_objectuiv_a_r_b_reply = GetQueryObjectuivARBReply { sequence: 0u16,
    length: 0u32, datum: 0u32, data: vec![] };
let u32_v = get_query_objectuiv_a_r_b_reply.n();
&lt;x11rb_protocol::protocol::xinput::RawKeyPressEvent as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;RawKeyPressEvent { response_type: 0u8, extension: 0u8, sequence: 0u16,
    length: 1u32, event_type: 1u16, deviceid: 0u16, time: u32_v, detail: 0u32,
    sourceid: 0u16, flags: KeyEventFlags::from(0u32), valuator_mask: vec![],
    axisvalues: vec![Fp3232 { integral: 0i32, frac: 0u32 }],
    axisvalues_raw: vec![] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:18245:9:
assertion `left == right` failed: `axisvalues` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::RawKeyPressEvent as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:18245:9)
  repro_x11rb_protocol_src_protocol_xinput_rs_18245_9_8f3289::main (src/main.rs:9:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut id_allocator = IdAllocator::new(0u32, 10000u32).unwrap();
let _ = id_allocator.update_xid_range(&amp;GetXIDRangeReply { sequence: 0u16,
    length: 0u32, start_id: 0u32, count: 1000000000u32 });</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/id_allocator.rs:54:31:
attempt to multiply with overflow
backtrace:
  &lt;x11rb_protocol::id_allocator::IdAllocator&gt;::update_xid_range (x11rb-protocol/src/id_allocator.rs:54:31)
  repro_x11rb_protocol_src_id_allocator_rs_54_31_ac869d::main (src/main.rs:6:26)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::TouchBeginEvent as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(&amp;TouchBeginEvent { response_type: 0u8,
    extension: 0u8, sequence: 0u16, length: 0u32, event_type: 0u16,
    deviceid: 0u16, time: 0u32, detail: 1u32, root: 0u32, event: 0u32,
    child: 0u32, root_x: 0i32, root_y: 0i32, event_x: 0i32, event_y: 0i32,
    sourceid: 0u16, flags: TouchEventFlags::TOUCH_EMULATING_POINTER,
    mods: ModifierInfo::default(), group: GroupInfo::default(),
    button_mask: vec![], valuator_mask: vec![],
    axisvalues: vec![Fp3232::default()] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:18520:9:
assertion `left == right` failed: `axisvalues` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::TouchBeginEvent as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:18520:9)
  repro_x11rb_protocol_src_protocol_xinput_rs_18520_9_9a7a1f::main (src/main.rs:8:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::DeviceClass as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;DeviceClass { len: 0u16, sourceid: 0u16,
    data: DeviceClassData::InvalidValue(0u16) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:12872:49:
attempted to serialize invalid switch case
backtrace:
  &lt;x11rb_protocol::protocol::xinput::DeviceClassData&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:12872:49)
  &lt;x11rb_protocol::protocol::xinput::DeviceClass as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:12927:19)
  repro_x11rb_protocol_src_protocol_xinput_rs_12872_49_ef9a64::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xproto::GetModifierMappingReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetModifierMappingReply { sequence: 0u16, length: 0u32,
    keycodes: vec![0u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xproto.rs:26378:9:
assertion `left == right` failed: `keycodes` has an incorrect length, must be a multiple of 8
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xproto::GetModifierMappingReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xproto.rs:26378:9)
  repro_x11rb_protocol_src_protocol_xproto_rs_26378_9_777203::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xkb::GetKbdByNameRepliesGeometry as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetKbdByNameRepliesGeometry { geometry_type: 0u8, geometry_device_id: 0u8,
    geometry_sequence: 0u16, geometry_length: 0u32, name: 0u32,
    geometry_found: false, width_mm: 0u16, height_mm: 0u16, n_properties: 0u16,
    n_colors: 0u16, n_shapes: 0u16, n_sections: 0u16, n_doodads: 0u16,
    n_key_aliases: 0u16, base_color_ndx: 0u8, label_color_ndx: 0u8,
    label_font: CountedString16::default() }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xkb.rs:2433:9:
assertion `left == right` failed: `alignment_pad` has an incorrect length
  left: 0
 right: 2
backtrace:
  &lt;x11rb_protocol::protocol::xkb::CountedString16 as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xkb.rs:2433:9)
  &lt;x11rb_protocol::protocol::xkb::GetKbdByNameRepliesGeometry as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xkb.rs:12188:25)
  repro_x11rb_protocol_src_protocol_xkb_rs_2433_9_50b257::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::record::EnableContextReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(&amp;EnableContextReply { category: 0u8,
    sequence: 0u16, element_header: 0u8, client_swapped: false, xid_base: 0u32,
    server_time: 0u32, rec_sequence_num: 0u32, data: vec![0u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/record.rs:1045:9:
assertion `left == right` failed: `data` has an incorrect length, must be a multiple of 4
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::record::EnableContextReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/record.rs:1045:9)
  repro_x11rb_protocol_src_protocol_record_rs_1045_9_eb88cd::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xf86vidmode::GetDotClocksReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(&amp;GetDotClocksReply { sequence: 0u16,
    length: 0u32, flags: ClockFlag::from(0u16), clocks: 0u32, maxclocks: 0u32,
    clock: vec![0u32] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xf86vidmode.rs:2401:9:
assertion `left == right` failed: `clock` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xf86vidmode::GetDotClocksReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xf86vidmode.rs:2401:9)
  repro_x11rb_protocol_src_protocol_xf86vidmode_rs_2401_9_e8b6ec::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xproto::SetupAuthenticate as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(&amp;SetupAuthenticate { status: 0u8,
    reason: vec![51u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xproto.rs:894:9:
assertion `left == right` failed: `reason` has an incorrect length, must be a multiple of 4
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xproto::SetupAuthenticate as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xproto.rs:894:9)
  repro_x11rb_protocol_src_protocol_xproto_rs_894_9_b27485::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::glx::GetTexImageReply as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;GetTexImageReply { sequence: 0u16, width: 0i32,
    height: 0i32, depth: 0i32, data: vec![73u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/glx.rs:8488:9:
assertion `left == right` failed: `data` has an incorrect length, must be a multiple of 4
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::glx::GetTexImageReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/glx.rs:8488:9)
  repro_x11rb_protocol_src_protocol_glx_rs_8488_9_730667::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xkb::GetNamesReply as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;GetNamesReply { device_id: 0u8, sequence: 0u16,
    length: 0u32, min_key_code: 0u8, max_key_code: 0u8, n_types: 0u8,
    group_names: SetOfGroup::GROUP2, virtual_mods: VMod::M3, first_key: 0u8,
    n_keys: 0u8, indicators: 0u32, n_radio_groups: 0u8, n_key_aliases: 0u8,
    n_kt_levels: 0u16, value_list: GetNamesValueList { keycodes_name: None,
    geometry_name: None, symbols_name: None, phys_symbols_name: None,
    types_name: None, compat_name: None, type_names: Some(vec![0u32]),
    kt_level_names: None, indicator_names: None, virtual_mod_names: None,
    groups: None, key_names: None, key_aliases: None,
    radio_group_names: None } }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xkb.rs:10067:13:
assertion `left == right` failed: `type_names` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xkb::GetNamesValueList&gt;::serialize_into (x11rb-protocol/src/protocol/xkb.rs:10067:13)
  &lt;x11rb_protocol::protocol::xkb::GetNamesReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xkb.rs:10230:25)
  repro_x11rb_protocol_src_protocol_xkb_rs_10067_13_25ad36::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::glx::
    GetCompressedTexImageARBReply as x11rb_protocol::x11_utils::Serialize&gt;::
    serialize_into(&amp;GetCompressedTexImageARBReply { sequence: 0u16, size: 0i32,
    data: vec![0u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/glx.rs:11949:9:
assertion `left == right` failed: `data` has an incorrect length, must be a multiple of 4
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::glx::GetCompressedTexImageARBReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/glx.rs:11949:9)
  repro_x11rb_protocol_src_protocol_glx_rs_11949_9_d1443f::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::glx::GetMinmaxReply as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;GetMinmaxReply { sequence: 1u16,
    data: vec![0u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/glx.rs:11522:9:
assertion `left == right` failed: `data` has an incorrect length, must be a multiple of 4
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::glx::GetMinmaxReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/glx.rs:11522:9)
  repro_x11rb_protocol_src_protocol_glx_rs_11522_9_c3dd04::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xv::Image as x11rb_protocol::x11_utils::Serialize&gt;::
    serialize_into(&amp;Image { id: 0u32, width: 0u16, height: 0u16,
    pitches: vec![0u32], offsets: vec![], data: vec![] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xv.rs:741:9:
assertion `left == right` failed: `offsets` has an incorrect length
  left: 0
 right: 1
backtrace:
  &lt;x11rb_protocol::protocol::xv::Image as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xv.rs:741:9)
  repro_x11rb_protocol_src_protocol_xv_rs_741_9_574a6b::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::
    GetDeviceMotionEventsReply as x11rb_protocol::x11_utils::Serialize&gt;::
    serialize_into(&amp;GetDeviceMotionEventsReply { xi_reply_type: 0u8,
    sequence: 0u16, length: 1u32, num_axes: 0u8,
    device_mode: ValuatorMode::ABSOLUTE,
    events: vec![DeviceTimeCoord { time: 0u32, axisvalues: vec![0i32] }] },
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:1998:9:
assertion `left == right` failed: `axisvalues` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::DeviceTimeCoord&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:1998:9)
  &lt;x11rb_protocol::protocol::xinput::GetDeviceMotionEventsReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:2139:21)
  repro_x11rb_protocol_src_protocol_xinput_rs_1998_9_871381::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::GetDevicePropertyReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetDevicePropertyReply { xi_reply_type: 0u8, sequence: 0u16, length: 0u32,
    type_: 0u32, bytes_after: 0u32, num_items: 0u32, device_id: 0u8,
    items: GetDevicePropertyItems::Data8(vec![35u8]) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:9837:17:
assertion `left == right` failed: `data8` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::GetDevicePropertyItems&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:9837:17)
  &lt;x11rb_protocol::protocol::xinput::GetDevicePropertyReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:9924:20)
  repro_x11rb_protocol_src_protocol_xinput_rs_9837_17_55431b::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xkb::DeviceLedInfo as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;DeviceLedInfo { led_class: LedClass::from(0u8),
    led_id: 0u16, names_present: 0u32, maps_present: 0u32,
    phys_indicators: 0u32, state: 0u32, names: vec![0u32], maps: vec![] },
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xkb.rs:3754:9:
assertion `left == right` failed: `names` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xkb::DeviceLedInfo as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xkb.rs:3754:9)
  repro_x11rb_protocol_src_protocol_xkb_rs_3754_9_8ac65d::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::ButtonClass as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;ButtonClass { type_: DeviceClassType::from(
    0u16), len: 0u16, sourceid: 0u16, state: vec![0u32], labels: vec![] },
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:12075:9:
assertion `left == right` failed: `state` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::ButtonClass as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:12075:9)
  repro_x11rb_protocol_src_protocol_xinput_rs_12075_9_505ddd::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::glx::GetDrawableAttributesReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetDrawableAttributesReply { sequence: 0u16, length: 0u32,
    attribs: vec![0u32] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/glx.rs:3475:9:
assertion `left == right` failed: `attribs` has an incorrect length, must be a multiple of 2
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::glx::GetDrawableAttributesReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/glx.rs:3475:9)
  repro_x11rb_protocol_src_protocol_glx_rs_3475_9_349ee4::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xproto::GetPropertyReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(&amp;GetPropertyReply { format: 0u8,
    sequence: 0u16, length: 1u32, type_: 0u32, bytes_after: 0u32,
    value_len: 0u32, value: vec![66u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xproto.rs:10891:9:
assertion `left == right` failed: `value` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xproto::GetPropertyReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xproto.rs:10891:9)
  repro_x11rb_protocol_src_protocol_xproto_rs_10891_9_6dbfff::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::DeviceClass as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;DeviceClass { len: 0u16, sourceid: 0u16,
    data: DeviceClassData::Button(DeviceClassDataButton { state: vec![0u32],
    labels: vec![] }) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:12515:9:
assertion `left == right` failed: `state` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::DeviceClassDataButton as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:12515:9)
  &lt;x11rb_protocol::protocol::xinput::DeviceClassData&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:12867:55)
  &lt;x11rb_protocol::protocol::xinput::DeviceClass as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:12927:19)
  repro_x11rb_protocol_src_protocol_xinput_rs_12515_9_939a83::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::FeedbackState as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;FeedbackState { feedback_id: 0u8, len: 0u16,
    data: FeedbackStateData::InvalidValue(0u8) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:4486:51:
attempted to serialize invalid switch case
backtrace:
  &lt;x11rb_protocol::protocol::xinput::FeedbackStateData&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:4486:51)
  &lt;x11rb_protocol::protocol::xinput::FeedbackState as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:4536:19)
  repro_x11rb_protocol_src_protocol_xinput_rs_4486_51_8f8c15::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xkb::GetIndicatorMapReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetIndicatorMapReply { device_id: 0u8, sequence: 0u16, length: 0u32,
    which: 0u32, real_indicators: 0u32, n_indicators: 0u8,
    maps: vec![IndicatorMap::default()] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xkb.rs:9340:9:
assertion `left == right` failed: `maps` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xkb::GetIndicatorMapReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xkb.rs:9340:9)
  repro_x11rb_protocol_src_protocol_xkb_rs_9340_9_43bc40::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::RawTouchBeginEvent as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;RawTouchBeginEvent { response_type: 0u8, extension: 0u8, sequence: 0u16,
    length: 0u32, event_type: 0u16, deviceid: 0u16, time: 0u32, detail: 0u32,
    sourceid: 0u16, flags: TouchEventFlags::from(0u16), valuator_mask: vec![],
    axisvalues: vec![Fp3232::default()], axisvalues_raw: vec![] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:18810:9:
assertion `left == right` failed: `axisvalues` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::RawTouchBeginEvent as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:18810:9)
  repro_x11rb_protocol_src_protocol_xinput_rs_18810_9_9fc344::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![82u8, 73u8, 70u8, 70u8, 0u8, 0u8, 0u8, 0u8, 87u8, 69u8, 66u8,
    80u8, 86u8, 80u8, 56u8, 76u8];
let _ = CreateContextRequest::try_parse_request(
    RequestHeader { major_opcode: 0u8, minor_opcode: 1u8,
    remaining_length: 0u32 }, &amp;vec);</pre></td><td valign="top"><pre>out of memory: memory allocation of 5386081628 bytes failed
backtrace:
  x11rb_protocol::x11_utils::parse_list&lt;u32&gt; (src/x11_utils.rs:542)
  x11rb_protocol::protocol::record::CreateContextRequest::try_parse_request (src/protocol/record.rs:597)
  repro_out_of_memory_in_x11rb_protocol_protocol_record_createcontex_291a8e::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::XIGetPropertyReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(&amp;XIGetPropertyReply { sequence: 0u16,
    length: 0u32, type_: 0u32, bytes_after: 0u32, num_items: 0u32,
    items: XIGetPropertyItems::Data16(vec![0u16]) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:14997:17:
assertion `left == right` failed: `data16` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::XIGetPropertyItems&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:14997:17)
  &lt;x11rb_protocol::protocol::xinput::XIGetPropertyReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:15075:20)
  repro_x11rb_protocol_src_protocol_xinput_rs_14997_17_1fac48::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let feedback_ctl_data_string = FeedbackCtlDataString { keysyms: vec![] };
let u16_v = feedback_ctl_data_string.num_keysyms();
&lt;x11rb_protocol::protocol::xinput::DeviceState as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;DeviceState { len: u16_v,
    data: DeviceStateData::InvalidValue(1u16) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:8119:49:
attempted to serialize invalid switch case
backtrace:
  &lt;x11rb_protocol::protocol::xinput::DeviceStateData&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:8119:49)
  &lt;x11rb_protocol::protocol::xinput::DeviceState as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:8165:19)
  repro_x11rb_protocol_src_protocol_xinput_rs_8119_49_c556c4::main (src/main.rs:8:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xf86vidmode::GetGammaRampReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(&amp;GetGammaRampReply { sequence: 0u16,
    length: 0u32, size: 0u16, red: vec![0u16], green: vec![], blue: vec![] },
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xf86vidmode.rs:2843:9:
assertion `left == right` failed: `red` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xf86vidmode::GetGammaRampReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xf86vidmode.rs:2843:9)
  repro_x11rb_protocol_src_protocol_xf86vidmode_rs_2843_9_306e37::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::randr::GetProviderPropertyReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetProviderPropertyReply { format: 0u8, sequence: 1u16, length: 0u32,
    type_: 0u32, bytes_after: 1u32, num_items: 0u32, data: vec![0u8] },
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/randr.rs:5974:9:
assertion `left == right` failed: `data` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::randr::GetProviderPropertyReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/randr.rs:5974:9)
  repro_x11rb_protocol_src_protocol_randr_rs_5974_9_67bda8::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::InputState as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;InputState { len: 0u8,
    data: InputStateData::InvalidValue(0u8) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:6957:48:
attempted to serialize invalid switch case
backtrace:
  &lt;x11rb_protocol::protocol::xinput::InputStateData&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:6957:48)
  &lt;x11rb_protocol::protocol::xinput::InputState as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:7001:19)
  repro_x11rb_protocol_src_protocol_xinput_rs_6957_48_0161d9::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::DeviceState as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;DeviceState { len: 0u16,
    data: DeviceStateData::Resolution(
    DeviceStateDataResolution { resolution_values: vec![],
    resolution_min: vec![0u32], resolution_max: vec![] }) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:7791:9:
assertion `left == right` failed: `resolution_min` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::DeviceStateDataResolution as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:7791:9)
  &lt;x11rb_protocol::protocol::xinput::DeviceStateData&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:8110:67)
  &lt;x11rb_protocol::protocol::xinput::DeviceState as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:8165:19)
  repro_x11rb_protocol_src_protocol_xinput_rs_7791_9_c87574::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::randr::GetScreenInfoReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetScreenInfoReply { rotations: Rotation::ROTATE270, sequence: 0u16,
    length: 0u32, root: 0u32, timestamp: 0u32, config_timestamp: 0u32,
    size_id: 0u16, rotation: Rotation::from(0u16), rate: 0u16, n_info: 1u16,
    sizes: vec![], rates: vec![] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/randr.rs:885:9:
assertion `left == right` failed: `rates` has an incorrect length
  left: 0
 right: 1
backtrace:
  &lt;x11rb_protocol::protocol::randr::GetScreenInfoReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/randr.rs:885:9)
  repro_x11rb_protocol_src_protocol_randr_rs_885_9_629ecf::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::HierarchyChange as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(&amp;HierarchyChange { len: 1u16,
    data: HierarchyChangeData::InvalidValue(0u16) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:11050:53:
attempted to serialize invalid switch case
backtrace:
  &lt;x11rb_protocol::protocol::xinput::HierarchyChangeData&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:11050:53)
  &lt;x11rb_protocol::protocol::xinput::HierarchyChange as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:11095:19)
  repro_x11rb_protocol_src_protocol_xinput_rs_11050_53_0d4624::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::randr::GetOutputPropertyReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(&amp;GetOutputPropertyReply { format: 0u8,
    sequence: 0u16, length: 0u32, type_: 0u32, bytes_after: 0u32,
    num_items: 0u32, data: vec![35u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/randr.rs:2470:9:
assertion `left == right` failed: `data` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::randr::GetOutputPropertyReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/randr.rs:2470:9)
  repro_x11rb_protocol_src_protocol_randr_rs_2470_9_30b034::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::glx::GetConvolutionFilterReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetConvolutionFilterReply { sequence: 0u16, width: 0i32, height: 0i32,
    data: vec![104u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/glx.rs:10464:9:
assertion `left == right` failed: `data` has an incorrect length, must be a multiple of 4
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::glx::GetConvolutionFilterReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/glx.rs:10464:9)
  repro_x11rb_protocol_src_protocol_glx_rs_10464_9_607e3f::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::DeviceResolutionState as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;DeviceResolutionState { control_id: DeviceControl::ENABLE, len: 0u16,
    resolution_values: vec![], resolution_min: vec![0u32],
    resolution_max: vec![] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:7459:9:
assertion `left == right` failed: `resolution_min` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::DeviceResolutionState as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:7459:9)
  repro_x11rb_protocol_src_protocol_xinput_rs_7459_9_d55f8b::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;x11rb_protocol::protocol::randr::GetCrtcGammaReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize(&amp;GetCrtcGammaReply { sequence: 0u16,
    length: 0u32, red: vec![0u16], green: vec![], blue: vec![] });</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/randr.rs:3485:9:
assertion `left == right` failed: `green` has an incorrect length
  left: 0
 right: 1
backtrace:
  &lt;x11rb_protocol::protocol::randr::GetCrtcGammaReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/randr.rs:3485:9)
  &lt;x11rb_protocol::protocol::randr::GetCrtcGammaReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize (x11rb-protocol/src/protocol/randr.rs:3471:14)
  repro_x11rb_protocol_src_protocol_randr_rs_3485_9_60d207::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::RawButtonPressEvent as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;RawButtonPressEvent { response_type: 0u8, extension: 0u8, sequence: 0u16,
    length: 0u32, event_type: 0u16, deviceid: 0u16, time: 0u32, detail: 0u32,
    sourceid: 0u16, flags: PointerEventFlags::POINTER_EMULATED,
    valuator_mask: vec![1u32], axisvalues: vec![], axisvalues_raw: vec![] },
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:18341:9:
assertion `left == right` failed: `axisvalues` has an incorrect length
  left: 0
 right: 1
backtrace:
  &lt;x11rb_protocol::protocol::xinput::RawButtonPressEvent as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:18341:9)
  repro_x11rb_protocol_src_protocol_xinput_rs_18341_9_1815ca::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::DeviceCtl as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;DeviceCtl { len: 0u16,
    data: DeviceCtlData::InvalidValue(1u16) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:8968:47:
attempted to serialize invalid switch case
backtrace:
  &lt;x11rb_protocol::protocol::xinput::DeviceCtlData&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:8968:47)
  &lt;x11rb_protocol::protocol::xinput::DeviceCtl as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:9014:19)
  repro_x11rb_protocol_src_protocol_xinput_rs_8968_47_423182::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::GetDevicePropertyReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetDevicePropertyReply { xi_reply_type: 0u8, sequence: 0u16, length: 1u32,
    type_: 0u32, bytes_after: 0u32, num_items: 0u32, device_id: 0u8,
    items: GetDevicePropertyItems::Data32(vec![0u32]) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:9847:17:
assertion `left == right` failed: `data32` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::GetDevicePropertyItems&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:9847:17)
  &lt;x11rb_protocol::protocol::xinput::GetDevicePropertyReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:9924:20)
  repro_x11rb_protocol_src_protocol_xinput_rs_9847_17_ba2c72::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::glx::AreTexturesResidentReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;AreTexturesResidentReply { sequence: 0u16, ret_val: 0u32,
    data: vec![false] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/glx.rs:9530:9:
assertion `left == right` failed: `data` has an incorrect length, must be a multiple of 4
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::glx::AreTexturesResidentReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/glx.rs:9530:9)
  repro_x11rb_protocol_src_protocol_glx_rs_9530_9_6724c0::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::ButtonPressEvent as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;ButtonPressEvent { response_type: 0u8, extension: 0u8, sequence: 0u16,
    length: 0u32, event_type: 1u16, deviceid: 0u16, time: 0u32, detail: 0u32,
    root: 0u32, event: 1u32, child: 0u32, root_x: 0i32, root_y: 0i32,
    event_x: 0i32, event_y: 0i32, sourceid: 0u16,
    flags: PointerEventFlags::from(0u16), mods: ModifierInfo::default(),
    group: GroupInfo { base: 0u8, latched: 0u8, locked: 0u8, effective: 0u8 },
    button_mask: vec![], valuator_mask: vec![],
    axisvalues: vec![Fp3232::default()] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:17510:9:
assertion `left == right` failed: `axisvalues` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::ButtonPressEvent as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:17510:9)
  repro_x11rb_protocol_src_protocol_xinput_rs_17510_9_cd870f::main (src/main.rs:8:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::randr::GetScreenInfoReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetScreenInfoReply { rotations: Rotation::from(0u16), sequence: 0u16,
    length: 0u32, root: 0u32, timestamp: 0u32, config_timestamp: 0u32,
    size_id: 0u16, rotation: Rotation::from(0u8), rate: 0u16, n_info: 1u16,
    sizes: vec![ScreenSize::default(), ScreenSize::default()], rates: vec![] },
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/randr.rs:885:109:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;x11rb_protocol::protocol::randr::GetScreenInfoReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/randr.rs:885:109)
  repro_x11rb_protocol_src_protocol_randr_rs_885_109_416f5c::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xproto::GetImageReply as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;GetImageReply { depth: 0u8, sequence: 1u16,
    visual: 0u32, data: vec![39u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xproto.rs:19958:9:
assertion `left == right` failed: `data` has an incorrect length, must be a multiple of 4
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xproto::GetImageReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xproto.rs:19958:9)
  repro_x11rb_protocol_src_protocol_xproto_rs_19958_9_67afdb::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::glx::QueryContextReply as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;QueryContextReply { sequence: 1u16,
    length: 0u32, attribs: vec![0u32] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/glx.rs:3036:9:
assertion `left == right` failed: `attribs` has an incorrect length, must be a multiple of 2
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::glx::QueryContextReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/glx.rs:3036:9)
  repro_x11rb_protocol_src_protocol_glx_rs_3036_9_78cb56::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let list_selections_reply = ListSelectionsReply::default();
let u32_v = list_selections_reply.selections_len();
&lt;x11rb_protocol::protocol::xinput::DeviceStateDataResolution as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;DeviceStateDataResolution { resolution_values: vec![0u32, 0u32, 0u32, 0u32,
    0u32], resolution_min: vec![0u32, 0u32, 0u32, u32_v, 1u32],
    resolution_max: vec![] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:7793:9:
assertion `left == right` failed: `resolution_max` has an incorrect length
  left: 0
 right: 5
backtrace:
  &lt;x11rb_protocol::protocol::xinput::DeviceStateDataResolution as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:7793:9)
  repro_x11rb_protocol_src_protocol_xinput_rs_7793_9_b9bb5a::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::XIGetPropertyReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(&amp;XIGetPropertyReply { sequence: 0u16,
    length: 0u32, type_: 0u32, bytes_after: 0u32, num_items: 0u32,
    items: XIGetPropertyItems::InvalidValue(0u8) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:15005:52:
attempted to serialize invalid switch case
backtrace:
  &lt;x11rb_protocol::protocol::xinput::XIGetPropertyItems&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:15005:52)
  &lt;x11rb_protocol::protocol::xinput::XIGetPropertyReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:15075:20)
  repro_x11rb_protocol_src_protocol_xinput_rs_15005_52_a16fcf::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::glx::ReadPixelsReply as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;ReadPixelsReply { sequence: 0u16,
    data: vec![0u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/glx.rs:5150:9:
assertion `left == right` failed: `data` has an incorrect length, must be a multiple of 4
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::glx::ReadPixelsReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/glx.rs:5150:9)
  repro_x11rb_protocol_src_protocol_glx_rs_5150_9_09bf66::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xproto::GetPropertyReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(&amp;GetPropertyReply { format: 100u8,
    sequence: 0u16, length: 0u32, type_: 0u32, bytes_after: 0u32,
    value_len: 1000000000u32, value: vec![] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xproto.rs:10891:143:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;x11rb_protocol::protocol::xproto::GetPropertyReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xproto.rs:10891:143)
  repro_x11rb_protocol_src_protocol_xproto_rs_10891_143_1942a2::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xv::QueryImageAttributesReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;QueryImageAttributesReply { sequence: 0u16, length: 0u32, data_size: 0u32,
    width: 0u16, height: 0u16, pitches: vec![], offsets: vec![0u32] },
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xv.rs:3289:9:
assertion `left == right` failed: `offsets` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xv::QueryImageAttributesReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xv.rs:3289:9)
  repro_x11rb_protocol_src_protocol_xv_rs_3289_9_db7bdd::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xkb::SetKeyType as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;SetKeyType { mask: ModMask::M1,
    real_mods: ModMask::ANY, virtual_mods: VMod::default(), num_levels: 0u8,
    preserve: true, entries: vec![KTSetMapEntry::default()],
    preserve_entries: vec![] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xkb.rs:3223:9:
assertion `left == right` failed: `preserve_entries` has an incorrect length
  left: 0
 right: 1
backtrace:
  &lt;x11rb_protocol::protocol::xkb::SetKeyType as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xkb.rs:3223:9)
  repro_x11rb_protocol_src_protocol_xkb_rs_3223_9_d6f392::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::GetDevicePropertyReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetDevicePropertyReply { xi_reply_type: 0u8, sequence: 0u16, length: 0u32,
    type_: 0u32, bytes_after: 0u32, num_items: 0u32, device_id: 0u8,
    items: GetDevicePropertyItems::InvalidValue(0u8) }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:9850:56:
attempted to serialize invalid switch case
backtrace:
  &lt;x11rb_protocol::protocol::xinput::GetDevicePropertyItems&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:9850:56)
  &lt;x11rb_protocol::protocol::xinput::GetDevicePropertyReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:9924:20)
  repro_x11rb_protocol_src_protocol_xinput_rs_9850_56_685563::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::glx::GetSeparableFilterReply as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;GetSeparableFilterReply { sequence: 0u16, row_w: 0i32, col_h: 0i32,
    rows_and_cols: vec![84u8] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/glx.rs:10912:9:
assertion `left == right` failed: `rows_and_cols` has an incorrect length, must be a multiple of 4
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::glx::GetSeparableFilterReply as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/glx.rs:10912:9)
  repro_x11rb_protocol_src_protocol_glx_rs_10912_9_75c866::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let image = Image::allocate(0u16, 0u16, ScanlinePad::Pad8, 0u8,
    BitsPerPixel::B8, ImageOrder::MsbFirst);
let image2 = image.clone().into_owned();
let mut image3 = image2.clone().into_owned();
image3.put_pixel(0u16, 0u16, 1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb/src/image.rs:862:9:
assertion failed: x &lt; self.width
backtrace:
  &lt;x11rb::image::Image&gt;::put_pixel (x11rb/src/image.rs:862:9)
  repro_x11rb_src_image_rs_862_9_1b6e7d::main (src/main.rs:10:12)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::DeviceName as x11rb_protocol::x11_utils::
    Serialize&gt;::serialize_into(&amp;DeviceName { string: include_bytes!(
    "data_0.bin").to_vec() }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:966:51:
`string` has too many elements: TryFromIntError(PosOverflow)
backtrace:
  &lt;x11rb_protocol::protocol::xinput::DeviceName as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:966:51)
  repro_x11rb_protocol_src_protocol_xinput_rs_966_51_b50d82::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;x11rb_protocol::protocol::xinput::DeviceResolutionState as x11rb_protocol::
    x11_utils::Serialize&gt;::serialize_into(
    &amp;DeviceResolutionState { control_id: DeviceControl::ENABLE, len: 0u16,
    resolution_values: vec![], resolution_min: vec![],
    resolution_max: vec![0u32] }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/protocol/xinput.rs:7461:9:
assertion `left == right` failed: `resolution_max` has an incorrect length
  left: 1
 right: 0
backtrace:
  &lt;x11rb_protocol::protocol::xinput::DeviceResolutionState as x11rb_protocol::x11_utils::Serialize&gt;::serialize_into (x11rb-protocol/src/protocol/xinput.rs:7461:9)
  repro_x11rb_protocol_src_protocol_xinput_rs_7461_9_dcd5ea::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut connection = Connection::default();
connection.enqueue_packet(vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/connection/mod.rs:194:26:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[u8]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[u8] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;x11rb_protocol::connection::Connection&gt;::enqueue_packet (x11rb-protocol/src/connection/mod.rs:194:26)
  repro_x11rb_protocol_src_connection_mod_rs_194_26_1f14d4::main (src/main.rs:5:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut packet_reader = PacketReader::default();
let _ = packet_reader.advance(10000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at x11rb-protocol/src/packet_reader.rs:83:9:
assertion failed: self.already_read &lt;= self.pending_packet.len()
backtrace:
  &lt;x11rb_protocol::packet_reader::PacketReader&gt;::advance (x11rb-protocol/src/packet_reader.rs:83:9)
  repro_x11rb_protocol_src_packet_reader_rs_83_9_87eda5::main (src/main.rs:5:27)</pre></td></tr>
</table>
