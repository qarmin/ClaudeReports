# drm-fourcc - crashes

Source: https://github.com/danielzfranklin/drm-fourcc-rs @ 1ad1a978a9aaa78ff0016cc7013787e4d1d68682

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let unrecognized_fourcc = UnrecognizedFourcc(2173415155u32);
let _ = unrecognized_fourcc.string_form();</pre></td><td valign="top"><pre>thread 'main' panicked at drm-fourcc-rs/src/lib.rs:203:31:
called `Option::unwrap()` on a `None` value
backtrace:
  drm_fourcc::fourcc_display_form (drm-fourcc-rs/src/lib.rs:203:31)
  drm_fourcc::fourcc_string_form (drm-fourcc-rs/src/lib.rs:195:5)
  &lt;drm_fourcc::UnrecognizedFourcc&gt;::string_form (drm-fourcc-rs/src/lib.rs:154:9)
  repro_drm_fourcc_rs_src_lib_rs_203_31_1f7db6::main (src/main.rs:5:33)</pre></td></tr>
<tr><td valign="top"><pre>let drm_fourcc = DrmFourcc::R16f;
let _ = drm_fourcc.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at drm-fourcc-rs/src/lib.rs:84:43:
Must be valid fourcc
backtrace:
  &lt;drm_fourcc::as_enum::DrmFourcc&gt;::display_form (drm-fourcc-rs/src/lib.rs:84:43)
  &lt;drm_fourcc::as_enum::DrmFourcc as core::fmt::Display&gt;::fmt (drm-fourcc-rs/src/lib.rs:98:28)
  &lt;drm_fourcc::as_enum::DrmFourcc as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;drm_fourcc::as_enum::DrmFourcc as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_drm_fourcc_rs_src_lib_rs_84_43_2544ec::main (src/main.rs:5:24)</pre></td></tr>
</table>
