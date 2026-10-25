# moxcms - crashes

Source: https://github.com/awxkee/moxcms @ a61db8babddcb47f8cb46314f2c6577756d7496f

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let vec = vec![];
let cube = Cube::new_cube(&amp;vec, [0u8, 0u8, 0u8], 0usize).unwrap();
let _ = cube.trilinear_vec4(0.0f32, 0.0f32, 1.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/nd_array.rs:859:28:
range start index 17179869180 out of range for slice of length 0
backtrace:
  &lt;[f32] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;moxcms::nd_array::ArrayFetchVector4f as moxcms::nd_array::ArrayFetch&lt;moxcms::matrix::Vector4&lt;f32&gt;&gt;&gt;::fetch (moxcms/src/nd_array.rs:859:28)
  &lt;moxcms::nd_array::Cube&gt;::trilinear::&lt;moxcms::matrix::Vector4&lt;f32&gt;, moxcms::nd_array::ArrayFetchVector4f&gt; (moxcms/src/nd_array.rs:972:26)
  &lt;moxcms::nd_array::Cube&gt;::trilinear_vec4 (moxcms/src/nd_array.rs:1292:14)
  repro_moxcms_src_nd_array_rs_859_28_4ee956::main (src/main.rs:6:18)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Layout::from(10u8);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:293:18:
not implemented
backtrace:
  &lt;moxcms::transform::Layout as core::convert::From&lt;u8&gt;&gt;::from (moxcms/src/transform.rs:293:18)
  repro_moxcms_src_transform_rs_293_18_d46f46::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let layout = Layout::Gray;
let _ = layout.r_i();</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:189:29:
not implemented
backtrace:
  &lt;moxcms::transform::Layout&gt;::r_i (moxcms/src/transform.rs:189:29)
  repro_moxcms_src_transform_rs_189_29_0449d0::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = moxcms::rounding_div_ceil(0i32, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/math/mod.rs:35:5:
attempt to divide by zero
backtrace:
  moxcms::math::rounding_div_ceil (moxcms/src/math/mod.rs:35:5)
  repro_moxcms_src_math_mod_rs_35_5_cb456b::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ToneReprCurve::Lut(vec![]).inverse();</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/trc.rs:738:47:
attempt to subtract with overflow
backtrace:
  moxcms::trc::lut_interp_linear16 (moxcms/src/trc.rs:738:47)
  moxcms::trc::lut_inverse_interp16 (moxcms/src/trc.rs:889:15)
  moxcms::trc::invert_lut (moxcms/src/trc.rs:940:16)
  &lt;moxcms::trc::ToneReprCurve&gt;::inverse (moxcms/src/trc.rs:48:39)
  repro_moxcms_src_trc_rs_738_47_e30302::main (src/main.rs:4:40)</pre></td></tr>
<tr><td valign="top"><pre>let l_ch = &lt;moxcms::LCh as ::core::ops::Div&lt;f32&gt;&gt;::div(
    &lt;moxcms::LCh as ::core::ops::Add&lt;f32&gt;&gt;::add(LCh::default(), 1.0f32),
    0.0f32);
assert_eq!(l_ch == l_ch,
    l_ch.partial_cmp(&amp;l_ch) == Some(std::cmp::Ordering::Equal), "PartialOrd \
    inconsistent with PartialEq");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:5:5:
assertion `left == right` failed: PartialOrd inconsistent with PartialEq
  left: false
 right: true
backtrace:
  repro_contract_partialordeq_for_moxcms_lch_af2ca2::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![0.0f32];
let cube = Cube::new(&amp;vec, 65536usize, 0usize).unwrap();
let _ = cube.prism_vec3(0.0f32, 0.0f32, 0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/nd_array.rs:841:49:
attempt to multiply with overflow
backtrace:
  &lt;moxcms::nd_array::ArrayFetchVector3f as moxcms::nd_array::ArrayFetch&lt;moxcms::matrix::Vector3&lt;f32&gt;&gt;&gt;::fetch (moxcms/src/nd_array.rs:841:49)
  &lt;moxcms::nd_array::Cube&gt;::prism::&lt;moxcms::matrix::Vector3&lt;f32&gt;, moxcms::nd_array::ArrayFetchVector3f&gt; (moxcms/src/nd_array.rs:1192:24)
  &lt;moxcms::nd_array::Cube&gt;::prism_vec3 (moxcms/src/nd_array.rs:1249:14)
  repro_moxcms_src_nd_array_rs_841_49_590ddf::main (src/main.rs:6:18)</pre></td></tr>
<tr><td valign="top"><pre>let lut_store = LutStore::Store16(vec![]);
let vec = lut_store.to_clut_f32();
let hypercube = Hypercube::new(&amp;vec, 0usize, 0usize).unwrap();
let _ = hypercube.prism_vec4(0.0f32, 0.0f32, 0.0f32, 0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/nd_array.rs:211:28:
range start index 17179869180 out of range for slice of length 0
backtrace:
  &lt;[f32] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;moxcms::nd_array::Fetch4Vec4 as moxcms::nd_array::Fetcher4&lt;moxcms::matrix::Vector4&lt;f32&gt;&gt;&gt;::fetch (moxcms/src/nd_array.rs:211:28)
  &lt;moxcms::nd_array::Hypercube&gt;::prism::&lt;moxcms::matrix::Vector4&lt;f32&gt;, moxcms::nd_array::Fetch4Vec4&gt; (moxcms/src/nd_array.rs:535:20)
  &lt;moxcms::nd_array::Hypercube&gt;::prism_vec4 (moxcms/src/nd_array.rs:639:14)
  repro_moxcms_src_nd_array_rs_211_28_e37e69::main (src/main.rs:8:23)</pre></td></tr>
<tr><td valign="top"><pre>let layout = Layout::Inks14;
let _ = layout.b_i();</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:215:18:
not implemented
backtrace:
  &lt;moxcms::transform::Layout&gt;::b_i (moxcms/src/transform.rs:215:18)
  repro_moxcms_src_transform_rs_215_18_e0cfa7::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Layout::resolve(42u8);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:317:18:
not implemented
backtrace:
  &lt;moxcms::transform::Layout&gt;::resolve (moxcms/src/transform.rs:317:18)
  repro_moxcms_src_transform_rs_317_18_551bdf::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let layout = Layout::GrayAlpha;
let _ = layout.r_i();</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:190:34:
not implemented
backtrace:
  &lt;moxcms::transform::Layout&gt;::r_i (moxcms/src/transform.rs:190:34)
  repro_moxcms_src_transform_rs_190_34_e45b05::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let layout = Layout::Rgb;
let _ = layout.a_i();</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:222:28:
not implemented
backtrace:
  &lt;moxcms::transform::Layout&gt;::a_i (moxcms/src/transform.rs:222:28)
  repro_moxcms_src_transform_rs_222_28_4ad97c::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![0.0f32];
let _ = Hypercube::new(&amp;vec, 10000usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/nd_array.rs:98:24:
attempt to multiply with overflow
backtrace:
  &lt;moxcms::nd_array::Hypercube&gt;::new (moxcms/src/nd_array.rs:98:24)
  repro_moxcms_src_nd_array_rs_98_24_b180c7::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = moxcms::rounding_div_ceil(i32::MAX, 1i32);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/math/mod.rs:35:6:
attempt to add with overflow
backtrace:
  moxcms::math::rounding_div_ceil (moxcms/src/math/mod.rs:35:6)
  repro_moxcms_src_math_mod_rs_35_6_701d13::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let layout = Layout::Inks13;
let _ = layout.a_i();</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:226:18:
not implemented
backtrace:
  &lt;moxcms::transform::Layout&gt;::a_i (moxcms/src/transform.rs:226:18)
  repro_moxcms_src_transform_rs_226_18_d41bf3::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let layout = Layout::Inks5;
let _ = layout.g_i();</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:203:18:
not implemented
backtrace:
  &lt;moxcms::transform::Layout&gt;::g_i (moxcms/src/transform.rs:203:18)
  repro_moxcms_src_transform_rs_203_18_f40374::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let layout = Layout::Gray;
let _ = layout.a_i();</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:224:29:
not implemented
backtrace:
  &lt;moxcms::transform::Layout&gt;::a_i (moxcms/src/transform.rs:224:29)
  repro_moxcms_src_transform_rs_224_29_5394dc::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let cube = Cube::new(&amp;vec, 0usize, 0usize).unwrap();
let _ = cube.trilinear_vec3(0.0f32, 0.0f32, 0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/nd_array.rs:842:28:
range start index 12884901885 out of range for slice of length 0
backtrace:
  &lt;[f32] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;moxcms::nd_array::ArrayFetchVector3f as moxcms::nd_array::ArrayFetch&lt;moxcms::matrix::Vector3&lt;f32&gt;&gt;&gt;::fetch (moxcms/src/nd_array.rs:842:28)
  &lt;moxcms::nd_array::Cube&gt;::trilinear::&lt;moxcms::matrix::Vector3&lt;f32&gt;, moxcms::nd_array::ArrayFetchVector3f&gt; (moxcms/src/nd_array.rs:972:26)
  &lt;moxcms::nd_array::Cube&gt;::trilinear_vec3 (moxcms/src/nd_array.rs:1234:14)
  repro_moxcms_src_nd_array_rs_842_28_93af91::main (src/main.rs:6:18)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let hypercube = Hypercube::new(&amp;vec, 1usize, 0usize).unwrap();
let _ = hypercube.prism_vec3(0.0f32, 0.0f32, 0.0f32, 0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/nd_array.rs:196:28:
range start index 12884901885 out of range for slice of length 0
backtrace:
  &lt;[f32] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;moxcms::nd_array::Fetch4Vec3 as moxcms::nd_array::Fetcher4&lt;moxcms::matrix::Vector3&lt;f32&gt;&gt;&gt;::fetch (moxcms/src/nd_array.rs:196:28)
  &lt;moxcms::nd_array::Hypercube&gt;::prism::&lt;moxcms::matrix::Vector3&lt;f32&gt;, moxcms::nd_array::Fetch4Vec3&gt; (moxcms/src/nd_array.rs:535:20)
  &lt;moxcms::nd_array::Hypercube&gt;::prism_vec3 (moxcms/src/nd_array.rs:621:14)
  repro_moxcms_src_nd_array_rs_196_28_33865c::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let layout = Layout::Gray;
let _ = layout.b_i();</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:213:29:
not implemented
backtrace:
  &lt;moxcms::transform::Layout&gt;::b_i (moxcms/src/transform.rs:213:29)
  repro_moxcms_src_transform_rs_213_29_8f6051::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![1.0f32];
let _ = Hypercube::new(&amp;vec, 1000000000usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/nd_array.rs:97:24:
attempt to multiply with overflow
backtrace:
  &lt;moxcms::nd_array::Hypercube&gt;::new (moxcms/src/nd_array.rs:97:24)
  repro_moxcms_src_nd_array_rs_97_24_00ded6::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ParametricCurve::new(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/trc.rs:237:22:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;moxcms::trc::ParametricCurve&gt;::new (moxcms/src/trc.rs:237:22)
  repro_moxcms_src_trc_rs_237_22_681e1c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let layout = Layout::GrayAlpha;
let _ = layout.g_i();</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:202:34:
not implemented
backtrace:
  &lt;moxcms::transform::Layout&gt;::g_i (moxcms/src/transform.rs:202:34)
  repro_moxcms_src_transform_rs_202_34_1bc070::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![0.0f32];
let _ = Cube::new(&amp;vec, 4294967296usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/nd_array.rs:877:24:
attempt to multiply with overflow
backtrace:
  &lt;moxcms::nd_array::Cube&gt;::new (moxcms/src/nd_array.rs:877:24)
  repro_moxcms_src_nd_array_rs_877_24_208c81::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let layout = Layout::Inks11;
let _ = layout.r_i();</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:191:18:
not implemented
backtrace:
  &lt;moxcms::transform::Layout&gt;::r_i (moxcms/src/transform.rs:191:18)
  repro_moxcms_src_transform_rs_191_18_401548::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let layout = Layout::Gray;
let _ = layout.g_i();</pre></td><td valign="top"><pre>thread 'main' panicked at moxcms/src/transform.rs:201:29:
not implemented
backtrace:
  &lt;moxcms::transform::Layout&gt;::g_i (moxcms/src/transform.rs:201:29)
  repro_moxcms_src_transform_rs_201_29_5ae74a::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let matrix4f = Matrix4f { v: [[0.0f32, 0.0f32, 0.0f32, 0.0f32], [f32::MAX,
    0.0f32, 0.0f32, 1.0f32], [0.0f32, 1.0f32, 0.0f32, 57.0f32], [0.0f32, 1.0f32,
    1.0f32, 1.0f32]] };
let f32_v = matrix4f.determinant().unwrap();
let luv = Luv { l: 0.0f32, u: f32_v, v: 0.0f32 };
assert_eq!(luv == luv, luv.partial_cmp(&amp;luv) == Some(std::cmp::Ordering::Equal),
    "PartialOrd inconsistent with PartialEq");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:8:5:
assertion `left == right` failed: PartialOrd inconsistent with PartialEq
  left: true
 right: false
backtrace:
  repro_contract_partialordeq_for_moxcms_luv_f9f689::main (src/main.rs:8:5)</pre></td></tr>
</table>
