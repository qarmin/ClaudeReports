# rav1e - crashes

Source: https://github.com/xiph/rav1e @ 31435de9d76fddd38f6dcc31d4014574cebb2092

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::inverse::av1_iidentity8(&amp;[], &amp;mut vec![], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/inverse.rs:300:9:
range end index 8 out of range for slice of length 0
backtrace:
  &lt;[i32] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  rav1e::transform::inverse::av1_iidentity8 (rav1e/src/transform/inverse.rs:300:9)
  repro_rav1e_src_transform_inverse_rs_300_9_36ebc1::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::mc::MotionVector as ::core::ops::Add&lt;rav1e::bench::mc::
    MotionVector&gt;&gt;::add(MotionVector { row: 0i16, col: i16::MAX },
    MotionVector { row: 0i16, col: 1i16 });</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/mc.rs:94:50:
attempt to add with overflow
backtrace:
  &lt;rav1e::mc::MotionVector as core::ops::arith::Add&gt;::add (rav1e/src/mc.rs:94:50)
  repro_rav1e_src_mc_rs_94_50_400970::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterCounter&gt; as rav1e::bench::ec::Writer&gt;::count_signed_subexp_with_ref(
    &amp;WriterCounter::new(), 0i32, i32::MIN, 0i32, 0u8, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:768:7:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::count_signed_subexp_with_ref (rav1e/src/ec.rs:768:7)
  repro_rav1e_src_ec_rs_768_7_c7f220::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterCounter::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterCounter&gt; as rav1e::bench::
    ec::Writer&gt;::bool(&amp;mut writer_base, false, u16::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:487:5:
assertion failed: f &lt; 32768
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::bool (rav1e/src/ec.rs:487:5)
  repro_rav1e_src_ec_rs_487_5_058992::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterEncoder&gt; as rav1e::bench::ec::Writer&gt;::count_quniform(
    &amp;WriterEncoder::new(), u32::MAX, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:643:15:
attempt to shift left with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::count_quniform (rav1e/src/ec.rs:643:15)
  repro_rav1e_src_ec_rs_643_15_cb89ed::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::inverse::av1_iidentity4(&amp;[0i32, 0i32, 0i32,
    -100000000i32], &amp;mut vec![0i32, 0i32, 0i32, 0i32], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/inverse.rs:154:49:
attempt to multiply with overflow
backtrace:
  rav1e::transform::inverse::av1_iidentity4::{closure#0} (rav1e/src/transform/inverse.rs:154:49)
  rav1e::transform::inverse::av1_iidentity4 (rav1e/src/transform/inverse.rs:154:6)
  repro_rav1e_src_transform_inverse_rs_154_49_e8cf69::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut segmentation_state = SegmentationState { enabled: false,
    update_data: false, update_map: false, preskip: false,
    last_active_segid: 0u8, features: [[false, false, false, false, false,
    false, false, false], [false, false, false, false, false, false, false,
    false], [false, false, false, false, false, false, false, false], [false,
    false, false, false, false, false, false, false], [false, false, false,
    false, false, false, false, false], [false, false, false, false, false,
    false, false, false], [false, false, false, false, false, false, false,
    false], [false, false, false, false, false, false, false, false]],
    data: [[0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16], [0i16, 0i16, 0i16,
    0i16, 0i16, 0i16, 0i16, 0i16], [0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16,
    0i16], [0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16], [0i16, 0i16, 0i16,
    0i16, 0i16, 0i16, 0i16, 0i16], [0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16,
    0i16], [0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16], [0i16, 0i16, 0i16,
    0i16, 0i16, 0i16, 0i16, 0i16]],
    threshold: [rav1e::bench::rdo::distortion_scale_for(0.0f64, 0.0f64),
    rav1e::bench::rdo::distortion_scale_for(0.0f64, 1.0f64),
    DistortionScale::from(0.0f64), DistortionScale::default(),
    DistortionScale(0u32), rav1e::bench::rdo::distortion_scale_for(0.0f64,
    1.0f64), DistortionScale::from(0.0f64)], min_segment: 0u8,
    max_segment: 10u8 };
segmentation_state.update_threshold(1u8, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/encoder.rs:569:16:
range end index 10 out of range for slice of length 8
backtrace:
  &lt;[[i16; 8]] as core::ops::index::Index&lt;core::ops::range::RangeToInclusive&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;[[i16; 8]; 8] as core::ops::index::Index&lt;core::ops::range::RangeToInclusive&lt;usize&gt;&gt;&gt;::index (core/src/array/mod.rs:391:9)
  &lt;rav1e::encoder::SegmentationState&gt;::update_threshold (rav1e/src/encoder.rs:569:16)
  repro_rav1e_src_encoder_rs_569_16_7db361::main (src/main.rs:6:24)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::mc::MotionVector as ::core::ops::Shr&lt;u8&gt;&gt;::shr(
    MotionVector::default(), 100u8);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/mc.rs:76:25:
attempt to shift right with overflow
backtrace:
  &lt;rav1e::mc::MotionVector as core::ops::bit::Shr&lt;u8&gt;&gt;::shr (rav1e/src/mc.rs:76:25)
  repro_rav1e_src_mc_rs_76_25_76eb2f::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterCounter&gt; as rav1e::bench::ec::Writer&gt;::count_unsigned_subexp_with_ref(
    &amp;WriterCounter::new(), 1u32, u32::MAX, 1u8, 1000000000u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:691:15:
attempt to add with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::count_subexp (rav1e/src/ec.rs:691:15)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::count_unsigned_subexp_with_ref (rav1e/src/ec.rs:734:12)
  repro_rav1e_src_ec_rs_691_15_991867::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterCounter&gt; as rav1e::bench::ec::Writer&gt;::symbol_bits(
    &amp;WriterCounter::new(), 2u32, &amp;[0u16, u16::MAX, 0u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:378:5:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt;&gt;::frac_compute (rav1e/src/ec.rs:378:5)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::symbol_bits (rav1e/src/ec.rs:604:5)
  repro_rav1e_src_ec_rs_378_5_86bb85::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterEncoder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterEncoder&gt; as rav1e::bench::
    ec::Writer&gt;::bool(&amp;mut writer_base, false, 0u16);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:486:5:
assertion failed: 0 &lt; f
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::bool (rav1e/src/ec.rs:486:5)
  repro_rav1e_src_ec_rs_486_5_8b798b::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterRecorder&gt; as rav1e::bench::ec::Writer&gt;::count_signed_subexp_with_ref(
    &amp;WriterRecorder::new(), 0i32, 0i32, 0i32, 0u8, i32::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:387:7:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterRecorder&gt;&gt;::recenter (rav1e/src/ec.rs:387:7)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterRecorder&gt; as rav1e::ec::Writer&gt;::count_unsigned_subexp_with_ref (rav1e/src/ec.rs:734:31)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterRecorder&gt; as rav1e::ec::Writer&gt;::count_signed_subexp_with_ref (rav1e/src/ec.rs:767:10)
  repro_rav1e_src_ec_rs_387_7_890ed0::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::rdo::RawDistortion as ::core::ops::Mul&lt;rav1e::bench::
    rdo::DistortionScale&gt;&gt;::mul(RawDistortion::new(10000000000000000000u64),
    rav1e::bench::rdo::distortion_scale_for(0.866f64, 0.9999999999999999f64));</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/rdo.rs:614:6:
attempt to multiply with overflow
backtrace:
  &lt;rav1e::rdo::DistortionScale&gt;::mul_u64 (rav1e/src/rdo.rs:614:6)
  &lt;rav1e::rdo::RawDistortion as core::ops::arith::Mul&lt;rav1e::rdo::DistortionScale&gt;&gt;::mul (rav1e/src/rdo.rs:678:20)
  repro_rav1e_src_rdo_rs_614_6_28a9aa::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::av1_round_shift_array(&amp;mut vec![0i32], 1usize,
    i8::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/mod.rs:328:14:
attempt to negate with overflow
backtrace:
  rav1e::transform::av1_round_shift_array::{closure#1} (rav1e/src/transform/mod.rs:328:14)
  rav1e::transform::av1_round_shift_array (rav1e/src/transform/mod.rs:327:31)
  repro_rav1e_src_transform_mod_rs_328_14_82f515::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterEncoder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterEncoder&gt; as rav1e::bench::
    ec::Writer&gt;::write_unsigned_subexp_with_ref(&amp;mut writer_base, u32::MAX,
    u32::MAX, 0u8, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:662:15:
attempt to add with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::write_subexp (rav1e/src/ec.rs:662:15)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::write_unsigned_subexp_with_ref (rav1e/src/ec.rs:717:12)
  repro_rav1e_src_ec_rs_662_15_21f58c::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterCounter::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterCounter&gt; as rav1e::bench::
    ec::Writer&gt;::write_signed_subexp_with_ref(&amp;mut writer_base, -1000000000i32,
    i32::MAX, 0i32, 0u8, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:750:7:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::write_signed_subexp_with_ref (rav1e/src/ec.rs:750:7)
  repro_rav1e_src_ec_rs_750_7_a20214::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterCounter::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterCounter&gt; as rav1e::bench::
    ec::Writer&gt;::write_unsigned_subexp_with_ref(&amp;mut writer_base, u32::MAX,
    2258794559u32, 0u8, 2122739179u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:719:57:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::write_unsigned_subexp_with_ref (rav1e/src/ec.rs:719:57)
  repro_rav1e_src_ec_rs_719_57_f20660::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::rdo::estimate_rate(0u8, TxSize::TX_8X4,
    10000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/rdo.rs:138:10:
attempt to multiply with overflow
backtrace:
  rav1e::rdo::estimate_rate (rav1e/src/rdo.rs:138:10)
  repro_rav1e_src_rdo_rs_138_10_3f7d78::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterCounter::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterCounter&gt; as rav1e::bench::
    ec::Writer&gt;::write_subexp(&amp;mut writer_base, 0u32, 49u8, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:661:15:
attempt to shift left with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::write_subexp (rav1e/src/ec.rs:661:15)
  repro_rav1e_src_ec_rs_661_15_6ceafe::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::mc::MotionVector as ::core::ops::Mul&lt;u16&gt;&gt;::mul(
    MotionVector { row: 0i16, col: 10000i16 }, 10000u16);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/mc.rs:67:53:
attempt to multiply with overflow
backtrace:
  &lt;rav1e::mc::MotionVector as core::ops::arith::Mul&lt;u16&gt;&gt;::mul (rav1e/src/mc.rs:67:53)
  repro_rav1e_src_mc_rs_67_53_449a78::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::av1_round_shift_array(&amp;mut vec![0i32], 1usize, -100i8);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/mod.rs:328:7:
attempt to shift left with overflow
backtrace:
  rav1e::transform::av1_round_shift_array::{closure#1} (rav1e/src/transform/mod.rs:328:7)
  rav1e::transform::av1_round_shift_array (rav1e/src/transform/mod.rs:327:31)
  repro_rav1e_src_transform_mod_rs_328_7_b2585f::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let writer_base = WriterCounter::new();
let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterCounter&gt; as rav1e::bench::ec::Writer&gt;::count_signed_subexp_with_ref(
    &amp;writer_base, 0i32, 1i32, 0i32, 0u8, i32::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:771:7:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::count_signed_subexp_with_ref (rav1e/src/ec.rs:771:7)
  repro_rav1e_src_ec_rs_771_7_abe7a1::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::rdo::clip_visible_bsize(0usize, 0usize,
    BlockSize::BLOCK_16X16, 0usize, usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/rdo.rs:242:29:
attempt to add with overflow
backtrace:
  rav1e::rdo::clip_visible_bsize (rav1e/src/rdo.rs:242:29)
  repro_rav1e_src_rdo_rs_242_29_757384::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterEncoder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterEncoder&gt; as rav1e::bench::
    ec::Writer&gt;::write_unsigned_subexp_with_ref(&amp;mut writer_base, 0u32, 0u32,
    0u8, 1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:719:46:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::write_unsigned_subexp_with_ref (rav1e/src/ec.rs:719:46)
  repro_rav1e_src_ec_rs_719_46_0b4748::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterEncoder&gt; as rav1e::bench::ec::Writer&gt;::count_signed_subexp_with_ref(
    &amp;WriterEncoder::new(), 0i32, 1i32, i32::MIN, 0u8, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:769:7:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::count_signed_subexp_with_ref (rav1e/src/ec.rs:769:7)
  repro_rav1e_src_ec_rs_769_7_8aab09::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterEncoder&gt; as rav1e::bench::ec::Writer&gt;::count_signed_subexp_with_ref(
    &amp;WriterEncoder::new(), 1i32, -1000000000i32, 1i32, 0u8, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:736:57:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::count_unsigned_subexp_with_ref (rav1e/src/ec.rs:736:57)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::count_signed_subexp_with_ref (rav1e/src/ec.rs:767:10)
  repro_rav1e_src_ec_rs_736_57_142b98::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterCounter::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterCounter&gt; as rav1e::bench::
    ec::Writer&gt;::literal(&amp;mut writer_base, 40u8, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:508:21:
attempt to shift right with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::literal (rav1e/src/ec.rs:508:21)
  repro_rav1e_src_ec_rs_508_21_9486ce::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TxSize::by_dims(0usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/mod.rs:263:12:
internal error: entered unreachable code
backtrace:
  &lt;rav1e::transform::TxSize&gt;::by_dims (rav1e/src/transform/mod.rs:263:12)
  repro_rav1e_src_transform_mod_rs_263_12_4e92cc::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterEncoder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterEncoder&gt; as rav1e::bench::
    ec::Writer&gt;::write_golomb(&amp;mut writer_base, u32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:610:13:
attempt to add with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::write_golomb (rav1e/src/ec.rs:610:13)
  repro_rav1e_src_ec_rs_610_13_4c4b1a::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterEncoder&gt; as rav1e::bench::ec::Writer&gt;::symbol_bits(
    &amp;WriterEncoder::new(), 0u32, &amp;[u16::MAX, 0u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:584:7:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::symbol_bits (rav1e/src/ec.rs:584:7)
  repro_rav1e_src_ec_rs_584_7_bc0694::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let writer_base = WriterCounter::new();
let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterCounter&gt; as rav1e::bench::ec::Writer&gt;::symbol_bits(&amp;writer_base, 1u32,
    &amp;[0u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:577:14:
index out of bounds: the len is 1 but the index is 1
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::symbol_bits (rav1e/src/ec.rs:577:14)
  repro_rav1e_src_ec_rs_577_14_36d3ab::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterEncoder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterEncoder&gt; as rav1e::bench::
    ec::Writer&gt;::write_signed_subexp_with_ref(&amp;mut writer_base, 0i32, 1i32,
    i32::MIN, 0u8, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:751:7:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::write_signed_subexp_with_ref (rav1e/src/ec.rs:751:7)
  repro_rav1e_src_ec_rs_751_7_3a805f::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::inverse::av1_iflipadst4(&amp;[0i32, 0i32, 0i32,
    100000000i32], &amp;mut vec![0i32, 0i32, 0i32, 0i32], 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/inverse.rs:119:12:
attempt to multiply with overflow
backtrace:
  rav1e::transform::inverse::av1_iadst4 (rav1e/src/transform/inverse.rs:119:12)
  rav1e::transform::inverse::av1_iflipadst4 (rav1e/src/transform/inverse.rs:94:3)
  repro_rav1e_src_transform_inverse_rs_119_12_83c9e4::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterCounter&gt; as rav1e::bench::ec::Writer&gt;::symbol_bits(
    &amp;WriterCounter::new(), 0u32, &amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:574:23:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::symbol_bits (rav1e/src/ec.rs:574:23)
  repro_rav1e_src_ec_rs_574_23_7e5b3c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DistortionScale::inv_mean(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/rdo.rs:589:36:
attempt to divide by zero
backtrace:
  &lt;rav1e::rdo::DistortionScale&gt;::inv_mean (rav1e/src/rdo.rs:589:36)
  repro_rav1e_src_rdo_rs_589_36_f17ca7::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterEncoder&gt; as rav1e::bench::ec::Writer&gt;::symbol_bits(
    &amp;WriterEncoder::new(), 0u32, &amp;[1000u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:574:5:
assertion failed: cdf[cdf.len() - 1] &lt; (1 &lt;&lt; EC_PROB_SHIFT)
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::symbol_bits (rav1e/src/ec.rs:574:5)
  repro_rav1e_src_ec_rs_574_5_314efd::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterCounter::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterCounter&gt; as rav1e::bench::
    ec::StorageBackend&gt;::store(&amp;mut writer_base, 0u16, 0u16, 0u16);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:344:23:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt;&gt;::lr_compute (rav1e/src/ec.rs:344:23)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::StorageBackend&gt;::store (rav1e/src/ec.rs:196:24)
  repro_rav1e_src_ec_rs_344_23_16d1fa::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterEncoder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterEncoder&gt; as rav1e::bench::
    ec::Writer&gt;::write_signed_subexp_with_ref(&amp;mut writer_base, 0i32, 1i32,
    0i32, 0u8, i32::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:753:7:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::write_signed_subexp_with_ref (rav1e/src/ec.rs:753:7)
  repro_rav1e_src_ec_rs_753_7_90349b::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::inverse::av1_iflipadst4(&amp;[100000000i32, 0i32, 0i32,
    0i32], &amp;mut vec![0i32, 0i32, 0i32, 0i32], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/inverse.rs:114:12:
attempt to multiply with overflow
backtrace:
  rav1e::transform::inverse::av1_iadst4 (rav1e/src/transform/inverse.rs:114:12)
  rav1e::transform::inverse::av1_iflipadst4 (rav1e/src/transform/inverse.rs:94:3)
  repro_rav1e_src_transform_inverse_rs_114_12_a18915::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::mc::MotionVector as ::core::ops::Mul&lt;i16&gt;&gt;::mul(
    &lt;rav1e::bench::mc::MotionVector as ::core::ops::Shl&lt;u8&gt;&gt;::shl(
    MotionVector { row: 1i16, col: 0i16 }, 10u8), 100i16);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/mc.rs:58:25:
attempt to multiply with overflow
backtrace:
  &lt;rav1e::mc::MotionVector as core::ops::arith::Mul&lt;i16&gt;&gt;::mul (rav1e/src/mc.rs:58:25)
  repro_rav1e_src_mc_rs_58_25_da9bae::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterCounter::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterCounter&gt; as rav1e::bench::
    ec::StorageBackend&gt;::store(&amp;mut writer_base, 0u16, 0u16, 10000u16);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:345:6:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt;&gt;::lr_compute (rav1e/src/ec.rs:345:6)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::StorageBackend&gt;::store (rav1e/src/ec.rs:196:24)
  repro_rav1e_src_ec_rs_345_6_a7d56a::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::mc::MotionVector as ::core::ops::Add&lt;rav1e::bench::mc::
    MotionVector&gt;&gt;::add(MotionVector { row: i16::MIN, col: 0i16 },
    MotionVector { row: -10000i16, col: 0i16 });</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/mc.rs:94:25:
attempt to add with overflow
backtrace:
  &lt;rav1e::mc::MotionVector as core::ops::arith::Add&gt;::add (rav1e/src/mc.rs:94:25)
  repro_rav1e_src_mc_rs_94_25_043bf0::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::rdo::Distortion as ::core::ops::Mul&lt;rav1e::bench::rdo::
    DistortionScale&gt;&gt;::mul(Distortion(u64::MAX),
    DistortionScale::inv_mean(&amp;[DistortionScale(945063371u32)]));</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/rdo.rs:614:5:
attempt to add with overflow
backtrace:
  &lt;rav1e::rdo::DistortionScale&gt;::mul_u64 (rav1e/src/rdo.rs:614:5)
  &lt;rav1e::rdo::Distortion as core::ops::arith::Mul&lt;rav1e::rdo::DistortionScale&gt;&gt;::mul (rav1e/src/rdo.rs:693:26)
  repro_rav1e_src_rdo_rs_614_5_68e4d2::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::mc::MotionVector as ::core::ops::Mul&lt;i16&gt;&gt;::mul(
    MotionVector { row: 0i16, col: -5549i16 }, 10i16);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/mc.rs:58:46:
attempt to multiply with overflow
backtrace:
  &lt;rav1e::mc::MotionVector as core::ops::arith::Mul&lt;i16&gt;&gt;::mul (rav1e/src/mc.rs:58:46)
  repro_rav1e_src_mc_rs_58_46_e583b0::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::mc::MotionVector as ::core::ops::Shl&lt;u8&gt;&gt;::shl(
    MotionVector { row: 0i16, col: 0i16 }, 53u8);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/mc.rs:85:25:
attempt to shift left with overflow
backtrace:
  &lt;rav1e::mc::MotionVector as core::ops::bit::Shl&lt;u8&gt;&gt;::shl (rav1e/src/mc.rs:85:25)
  repro_rav1e_src_mc_rs_85_25_b55d4c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::inverse::av1_iidentity4(&amp;[], &amp;mut vec![], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/inverse.rs:151:9:
range end index 4 out of range for slice of length 0
backtrace:
  &lt;[i32] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  rav1e::transform::inverse::av1_iidentity4 (rav1e/src/transform/inverse.rs:151:9)
  repro_rav1e_src_transform_inverse_rs_151_9_1be1f2::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterCounter::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterCounter&gt; as rav1e::bench::
    ec::StorageBackend&gt;::store(&amp;mut writer_base, 1u16, 1000u16, 1u16);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:345:13:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt;&gt;::lr_compute (rav1e/src/ec.rs:345:13)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::StorageBackend&gt;::store (rav1e/src/ec.rs:196:24)
  repro_rav1e_src_ec_rs_345_13_c8b8bc::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterEncoder&gt; as rav1e::bench::ec::Writer&gt;::count_signed_subexp_with_ref(
    &amp;WriterEncoder::new(), 0i32, 0i32, 0i32, 100u8, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:690:15:
attempt to shift left with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::count_subexp (rav1e/src/ec.rs:690:15)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::count_unsigned_subexp_with_ref (rav1e/src/ec.rs:734:12)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::count_signed_subexp_with_ref (rav1e/src/ec.rs:767:10)
  repro_rav1e_src_ec_rs_690_15_4c9caf::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::inverse::av1_iflipadst16(&amp;[], &amp;mut vec![], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/inverse.rs:410:3:
assertion failed: input.len() &gt;= 16
backtrace:
  rav1e::transform::inverse::av1_iadst16 (rav1e/src/transform/inverse.rs:410:3)
  rav1e::transform::inverse::av1_iflipadst16 (rav1e/src/transform/inverse.rs:404:3)
  repro_rav1e_src_transform_inverse_rs_410_3_3409d2::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterRecorder&gt; as rav1e::bench::ec::Writer&gt;::count_signed_subexp_with_ref(
    &amp;WriterRecorder::new(), 0i32, 0i32, 0i32, 0u8, 1i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:736:46:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterRecorder&gt; as rav1e::ec::Writer&gt;::count_unsigned_subexp_with_ref (rav1e/src/ec.rs:736:46)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterRecorder&gt; as rav1e::ec::Writer&gt;::count_signed_subexp_with_ref (rav1e/src/ec.rs:767:10)
  repro_rav1e_src_ec_rs_736_46_34787e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::mc::MotionVector as ::core::ops::Mul&lt;u16&gt;&gt;::mul(
    MotionVector { row: 100i16, col: 0i16 }, 10000u16);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/mc.rs:67:25:
attempt to multiply with overflow
backtrace:
  &lt;rav1e::mc::MotionVector as core::ops::arith::Mul&lt;u16&gt;&gt;::mul (rav1e/src/mc.rs:67:25)
  repro_rav1e_src_mc_rs_67_25_3201c7::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::rdo::clip_visible_bsize(0usize, 0usize,
    BlockSize::BLOCK_64X64, usize::MAX, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/rdo.rs:234:29:
attempt to add with overflow
backtrace:
  rav1e::rdo::clip_visible_bsize (rav1e/src/rdo.rs:234:29)
  repro_rav1e_src_rdo_rs_234_29_516e5b::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let u64_v = rav1e::version::major();
let _ = DistortionScale::new(0u64, u64_v);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/rdo.rs:580:15:
attempt to divide by zero
backtrace:
  &lt;rav1e::rdo::DistortionScale&gt;::new (rav1e/src/rdo.rs:580:15)
  repro_rav1e_src_rdo_rs_580_15_b32a6f::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::inverse::av1_iidentity4(&amp;[], &amp;mut vec![0i32, 0i32,
    0i32, 0i32], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/inverse.rs:153:15:
range end index 4 out of range for slice of length 0
backtrace:
  &lt;[i32] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  rav1e::transform::inverse::av1_iidentity4 (rav1e/src/transform/inverse.rs:153:15)
  repro_rav1e_src_transform_inverse_rs_153_15_750562::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let u16_v = CDEF_VERY_LARGE;
let mut writer_base = WriterEncoder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterEncoder&gt; as rav1e::bench::
    ec::StorageBackend&gt;::store(&amp;mut writer_base, u16::MAX, u16_v, 1u16);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:286:16:
attempt to shift left with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::StorageBackend&gt;::store (rav1e/src/ec.rs:286:16)
  repro_rav1e_src_ec_rs_286_16_578c8b::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterEncoder&gt; as rav1e::bench::ec::Writer&gt;::symbol_bits(
    &amp;WriterEncoder::new(), 1u32, &amp;[0u16, 10000u16, 0u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:580:7:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::symbol_bits (rav1e/src/ec.rs:580:7)
  repro_rav1e_src_ec_rs_580_7_2704b9::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let sequence = Sequence::new(&amp;EncoderConfig::with_speed_preset(0u8));
let _ = sequence.get_relative_dist(2498367332u32, 1000000000u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/encoder.rs:334:16:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::encoder::Sequence&gt;::get_relative_dist (rav1e/src/encoder.rs:334:16)
  repro_rav1e_src_encoder_rs_334_16_82d9dd::main (src/main.rs:6:22)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterCounter&gt; as rav1e::bench::ec::Writer&gt;::count_unsigned_subexp_with_ref(
    &amp;WriterCounter::new(), 0u32, 0u32, 31u8, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:691:20:
attempt to multiply with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::count_subexp (rav1e/src/ec.rs:691:20)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::count_unsigned_subexp_with_ref (rav1e/src/ec.rs:734:12)
  repro_rav1e_src_ec_rs_691_20_4f0b94::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterEncoder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterEncoder&gt; as rav1e::bench::
    ec::Writer&gt;::write_unsigned_subexp_with_ref(&amp;mut writer_base, 0u32, 0u32,
    31u8, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:662:20:
attempt to multiply with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::write_subexp (rav1e/src/ec.rs:662:20)
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::write_unsigned_subexp_with_ref (rav1e/src/ec.rs:717:12)
  repro_rav1e_src_ec_rs_662_20_6aa3ad::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::inverse::av1_iflipadst4(&amp;[-1000000i32, 0i32, 0i32,
    0i32], &amp;mut vec![0i32, 0i32, 0i32, 0i32], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/inverse.rs:115:12:
attempt to multiply with overflow
backtrace:
  rav1e::transform::inverse::av1_iadst4 (rav1e/src/transform/inverse.rs:115:12)
  rav1e::transform::inverse::av1_iflipadst4 (rav1e/src/transform/inverse.rs:94:3)
  repro_rav1e_src_transform_inverse_rs_115_12_896f93::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterCounter::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterCounter&gt; as rav1e::bench::
    ec::Writer&gt;::add_bits_frac(&amp;mut writer_base, u32::MAX);
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterCounter&gt; as rav1e::bench::
    ec::Writer&gt;::add_bits_frac(&amp;mut writer_base, 1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:499:5:
attempt to add with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::add_bits_frac (rav1e/src/ec.rs:499:5)
  repro_rav1e_src_ec_rs_499_5_f55c52::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>let u16_v = CDEF_VERY_LARGE;
let mut writer_base = WriterCounter::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterCounter&gt; as rav1e::bench::
    ec::StorageBackend&gt;::store(&amp;mut writer_base, u16::MAX, u16_v, 1u16);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:200:16:
attempt to shift left with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::StorageBackend&gt;::store (rav1e/src/ec.rs:200:16)
  repro_rav1e_src_ec_rs_200_16_2181db::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::inverse::av1_iflipadst4(&amp;[0i32, 0i32, 0i32, 791351i32],
    &amp;mut vec![0i32, 0i32, 0i32, 0i32], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/inverse.rs:120:12:
attempt to multiply with overflow
backtrace:
  rav1e::transform::inverse::av1_iadst4 (rav1e/src/transform/inverse.rs:120:12)
  rav1e::transform::inverse::av1_iflipadst4 (rav1e/src/transform/inverse.rs:94:3)
  repro_rav1e_src_transform_inverse_rs_120_12_f35efa::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterEncoder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterEncoder&gt; as rav1e::bench::
    ec::Writer&gt;::write_quniform(&amp;mut writer_base, 10000u32, 0u32);
let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterEncoder&gt; as rav1e::bench::ec::Writer&gt;::symbol_bits(&amp;writer_base, 1u32,
    &amp;[u16::MAX, 10000u16, 0u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:604:5:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::symbol_bits (rav1e/src/ec.rs:604:5)
  repro_rav1e_src_ec_rs_604_5_d46867::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterRecorder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterRecorder&gt; as rav1e::
    bench::ec::Writer&gt;::write_quniform(&amp;mut writer_base, u32::MAX, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:627:15:
attempt to shift left with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterRecorder&gt; as rav1e::ec::Writer&gt;::write_quniform (rav1e/src/ec.rs:627:15)
  repro_rav1e_src_ec_rs_627_15_6b4082::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterRecorder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterRecorder&gt; as rav1e::
    bench::ec::Writer&gt;::add_bits_frac(&amp;mut writer_base, u32::MAX);
let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterRecorder&gt; as rav1e::bench::ec::Writer&gt;::tell_frac(&amp;mut writer_base);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:797:5:
attempt to add with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterRecorder&gt; as rav1e::ec::Writer&gt;::tell_frac (rav1e/src/ec.rs:797:5)
  repro_rav1e_src_ec_rs_797_5_7e7ae7::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let u16_v = CDEF_VERY_LARGE;
let mut writer_base = WriterRecorder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterRecorder&gt; as rav1e::
    bench::ec::StorageBackend&gt;::store(&amp;mut writer_base, u16::MAX, u16_v, 1u16);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:235:16:
attempt to shift left with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterRecorder&gt; as rav1e::ec::StorageBackend&gt;::store (rav1e/src/ec.rs:235:16)
  repro_rav1e_src_ec_rs_235_16_b4d4b4::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterRecorder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterRecorder&gt; as rav1e::
    bench::ec::Writer&gt;::write_quniform(&amp;mut writer_base, 10000u32, 0u32);
let _ = &lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::
    WriterRecorder&gt; as rav1e::bench::ec::Writer&gt;::symbol_bits(&amp;writer_base,
    1u32, &amp;[u16::MAX, 0u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:591:13:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterRecorder&gt; as rav1e::ec::Writer&gt;::symbol_bits (rav1e/src/ec.rs:591:13)
  repro_rav1e_src_ec_rs_591_13_c07c4a::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::kmeans::&lt;i64, 4&gt;(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/kmeans.rs:22:17:
attempt to subtract with overflow
backtrace:
  rav1e::util::kmeans::kmeans::&lt;i64, 4&gt; (rav1e/src/util/kmeans.rs:22:17)
  repro_rav1e_src_util_kmeans_rs_22_17_1e062e::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::kmeans::&lt;i64, 1&gt;(&amp;[0i64]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/kmeans.rs:22:12:
attempt to divide by zero
backtrace:
  rav1e::util::kmeans::kmeans::&lt;i64, 1&gt; (rav1e/src/util/kmeans.rs:22:12)
  repro_rav1e_src_util_kmeans_rs_22_12_25893e::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterRecorder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterRecorder&gt; as rav1e::
    bench::ec::Writer&gt;::symbol::&lt;4&gt;(&amp;mut writer_base, 1u32, &amp;[u16::MAX, 0u16,
    0u16, 0u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:535:5:
assertion failed: fl &lt;= 32768
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterRecorder&gt; as rav1e::ec::Writer&gt;::symbol::&lt;4&gt; (rav1e/src/ec.rs:535:5)
  repro_rav1e_src_ec_rs_535_5_0c9b86::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::ec::update_cdf::&lt;1&gt;(&amp;mut [10000u16], 1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:941:17:
attempt to subtract with overflow
backtrace:
  rav1e::ec::rust::update_cdf::&lt;1&gt; (rav1e/src/ec.rs:941:17)
  rav1e::asm::x86::ec::update_cdf::&lt;1&gt; (rav1e/src/asm/x86/ec.rs:24:3)
  repro_rav1e_src_ec_rs_941_17_d0d874::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::kmeans::&lt;i64, 4&gt;(&amp;[i64::MAX]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/kmeans.rs:35:11:
attempt to add with overflow
backtrace:
  rav1e::util::kmeans::kmeans::&lt;i64, 4&gt;::{closure#1} (rav1e/src/util/kmeans.rs:35:11)
  rav1e::util::kmeans::kmeans::&lt;i64, 4&gt; (rav1e/src/util/kmeans.rs:33:42)
  repro_rav1e_src_util_kmeans_rs_35_11_e9dce3::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterCounter::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterCounter&gt; as rav1e::bench::
    ec::Writer&gt;::symbol::&lt;4&gt;(&amp;mut writer_base, 0u32, &amp;[u16::MAX, 0u16, 0u16,
    1u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:534:5:
assertion failed: (fh &gt;&gt; EC_PROB_SHIFT) &lt;= (fl &gt;&gt; EC_PROB_SHIFT)
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterCounter&gt; as rav1e::ec::Writer&gt;::symbol::&lt;4&gt; (rav1e/src/ec.rs:534:5)
  repro_rav1e_src_ec_rs_534_5_f8415a::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::kmeans::&lt;i64, 0&gt;(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/kmeans.rs:27:8:
attempt to subtract with overflow
backtrace:
  rav1e::util::kmeans::kmeans::&lt;i64, 0&gt; (rav1e/src/util/kmeans.rs:27:8)
  repro_rav1e_src_util_kmeans_rs_27_8_2621a7::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterRecorder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterRecorder&gt; as rav1e::
    bench::ec::Writer&gt;::symbol::&lt;1&gt;(&amp;mut writer_base, 0u32, &amp;[74u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:521:5:
assertion failed: cdf[cdf.len() - 1] &lt; (1 &lt;&lt; EC_PROB_SHIFT)
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterRecorder&gt; as rav1e::ec::Writer&gt;::symbol::&lt;1&gt; (rav1e/src/ec.rs:521:5)
  repro_rav1e_src_ec_rs_521_5_58727d::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterEncoder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterEncoder&gt; as rav1e::bench::
    ec::Writer&gt;::symbol::&lt;0&gt;(&amp;mut writer_base, 0u32, &amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:521:23:
attempt to subtract with overflow
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterEncoder&gt; as rav1e::ec::Writer&gt;::symbol::&lt;0&gt; (rav1e/src/ec.rs:521:23)
  repro_rav1e_src_ec_rs_521_23_f30c23::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_base = WriterRecorder::new();
&lt;rav1e::bench::ec::WriterBase&lt;rav1e::bench::ec::WriterRecorder&gt; as rav1e::
    bench::ec::Writer&gt;::symbol::&lt;1&gt;(&amp;mut writer_base, 1u32, &amp;[0u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/ec.rs:523:5:
assertion failed: s &lt; cdf.len()
backtrace:
  &lt;rav1e::ec::WriterBase&lt;rav1e::ec::WriterRecorder&gt; as rav1e::ec::Writer&gt;::symbol::&lt;1&gt; (rav1e/src/ec.rs:523:5)
  repro_rav1e_src_ec_rs_523_5_a87dcb::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::kmeans::&lt;i64, 4&gt;(&amp;[0i64, 0i64, 0i64, i64::MAX,
    1i64]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/kmeans.rs:97:5:
attempt to add with overflow
backtrace:
  rav1e::util::kmeans::scan::&lt;i64&gt; (rav1e/src/util/kmeans.rs:97:5)
  rav1e::util::kmeans::kmeans::&lt;i64, 4&gt; (rav1e/src/util/kmeans.rs:41:9)
  repro_rav1e_src_util_kmeans_rs_97_5_74d507::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::kmeans::&lt;i64, 4&gt;(&amp;[0i64, i64::MIN, 0i64]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/kmeans.rs:93:5:
attempt to subtract with overflow
backtrace:
  rav1e::util::kmeans::scan::&lt;i64&gt; (rav1e/src/util/kmeans.rs:93:5)
  rav1e::util::kmeans::kmeans::&lt;i64, 4&gt; (rav1e/src/util/kmeans.rs:41:9)
  repro_rav1e_src_util_kmeans_rs_93_5_57dce2::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::kmeans::&lt;i64, 4&gt;(&amp;[1i64, 9223372036854775806i64]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/kmeans.rs:35:10:
attempt to add with overflow
backtrace:
  rav1e::util::kmeans::kmeans::&lt;i64, 4&gt;::{closure#1} (rav1e/src/util/kmeans.rs:35:10)
  rav1e::util::kmeans::kmeans::&lt;i64, 4&gt; (rav1e/src/util/kmeans.rs:33:42)
  repro_rav1e_src_util_kmeans_rs_35_10_16fd5b::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::inverse::av1_iflipadst8(&amp;[], &amp;mut vec![], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/inverse.rs:219:3:
assertion failed: input.len() &gt;= 8
backtrace:
  rav1e::transform::inverse::av1_iadst8 (rav1e/src/transform/inverse.rs:219:3)
  rav1e::transform::inverse::av1_iflipadst8 (rav1e/src/transform/inverse.rs:210:3)
  repro_rav1e_src_transform_inverse_rs_219_3_586501::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::cdf::&lt;1, 4&gt;([u16::MAX]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/cdf.rs:19:5:
assertion failed: vars[i] &lt;= 32768
backtrace:
  rav1e::util::cdf::cdf::&lt;1, 4&gt; (rav1e/src/util/cdf.rs:19:5)
  repro_rav1e_src_util_cdf_rs_19_5_b0e580::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::inverse::av1_iflipadst4(&amp;[0i32, 1000000000i32, 0i32,
    0i32], &amp;mut vec![0i32, 0i32, 0i32, 0i32], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/inverse.rs:116:12:
attempt to multiply with overflow
backtrace:
  rav1e::transform::inverse::av1_iadst4 (rav1e/src/transform/inverse.rs:116:12)
  rav1e::transform::inverse::av1_iflipadst4 (rav1e/src/transform/inverse.rs:94:3)
  repro_rav1e_src_transform_inverse_rs_116_12_db196f::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::kmeans::&lt;i64, 4&gt;(&amp;[0i64, -1000000000000000000i64,
    0i64]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/kmeans.rs:48:19:
attempt to subtract with overflow
backtrace:
  &lt;usize as core::ops::arith::Sub&gt;::sub (core/src/ops/arith.rs:216:45)
  &lt;&amp;usize as core::ops::arith::Sub&gt;::sub (core/src/internal_macros.rs:50:17)
  rav1e::util::kmeans::kmeans::&lt;i64, 4&gt; (rav1e/src/util/kmeans.rs:48:19)
  repro_rav1e_src_util_kmeans_rs_48_19_dfb1c2::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::cdf::&lt;1, 1&gt;([0u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/cdf.rs:14:3:
assertion failed: CDF_LEN &gt; VARS
backtrace:
  rav1e::util::cdf::cdf::&lt;1, 1&gt; (rav1e/src/util/cdf.rs:14:3)
  repro_rav1e_src_util_cdf_rs_14_3_7da3fd::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::kmeans::&lt;i64, 4&gt;(&amp;[0i64, i64::MIN,
    -1000000000000000000i64]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/kmeans.rs:84:5:
attempt to add with overflow
backtrace:
  rav1e::util::kmeans::scan::&lt;i64&gt; (rav1e/src/util/kmeans.rs:84:5)
  rav1e::util::kmeans::kmeans::&lt;i64, 4&gt; (rav1e/src/util/kmeans.rs:41:9)
  repro_rav1e_src_util_kmeans_rs_84_5_d2e0a8::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::rdo::estimate_rate(0u8, TxSize::TX_16X64,
    9223372036854775810u64);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/rdo.rs:138:11:
attempt to subtract with overflow
backtrace:
  rav1e::rdo::estimate_rate (rav1e/src/rdo.rs:138:11)
  repro_rav1e_src_rdo_rs_138_11_1f838c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::kmeans::&lt;i64, 4&gt;(&amp;[0i64, 0i64, -1i64,
    -9223372036854775807i64, 0i64, 0i64]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/kmeans.rs:80:5:
attempt to subtract with overflow
backtrace:
  rav1e::util::kmeans::scan::&lt;i64&gt; (rav1e/src/util/kmeans.rs:80:5)
  rav1e::util::kmeans::kmeans::&lt;i64, 4&gt; (rav1e/src/util/kmeans.rs:41:9)
  repro_rav1e_src_util_kmeans_rs_80_5_5f024e::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rav1e::bench::util::kmeans::&lt;i64, 4&gt;(&amp;[0i64, 0i64, 0i64, i64::MAX,
    0i64]);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/util/kmeans.rs:53:10:
attempt to add with overflow
backtrace:
  &lt;i64 as core::ops::arith::Add&gt;::add (core/src/ops/arith.rs:104:45)
  &lt;&amp;i64 as core::ops::arith::Add&lt;i64&gt;&gt;::add (core/src/internal_macros.rs:28:17)
  rav1e::util::kmeans::kmeans::&lt;i64, 4&gt; (rav1e/src/util/kmeans.rs:53:10)
  repro_rav1e_src_util_kmeans_rs_53_10_69bab6::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::inverse::av1_iflipadst4(&amp;[65536i32, 0i32, 0i32,
    -524288i32], &amp;mut vec![0i32, 0i32, 0i32, 0i32], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rav1e/src/transform/inverse.rs:133:12:
attempt to subtract with overflow
backtrace:
  rav1e::transform::inverse::av1_iadst4 (rav1e/src/transform/inverse.rs:133:12)
  rav1e::transform::inverse::av1_iflipadst4 (rav1e/src/transform/inverse.rs:94:3)
  repro_rav1e_src_transform_inverse_rs_133_12_2d5079::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::av1_round_shift_array(&amp;mut vec![i32::MAX], 1usize,
    1i8);</pre></td><td valign="top"><pre>thread 'main' panicked at v_frame/src/math.rs:70:5:
attempt to add with overflow
backtrace:
  v_frame::math::round_shift (v_frame/src/math.rs:70:5)
  rav1e::transform::av1_round_shift_array::{closure#0} (rav1e/src/transform/mod.rs:324:12)
  rav1e::transform::av1_round_shift_array (rav1e/src/transform/mod.rs:323:31)
  repro_v_frame_src_math_rs_70_5_b487ab::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>rav1e::bench::transform::av1_round_shift_array(&amp;mut vec![0i32], 1usize, 37i8);</pre></td><td valign="top"><pre>thread 'main' panicked at v_frame/src/math.rs:70:15:
attempt to shift left with overflow
backtrace:
  v_frame::math::round_shift (v_frame/src/math.rs:70:15)
  rav1e::transform::av1_round_shift_array::{closure#0} (rav1e/src/transform/mod.rs:324:12)
  rav1e::transform::av1_round_shift_array (rav1e/src/transform/mod.rs:323:31)
  repro_v_frame_src_math_rs_70_15_e93b47::main (src/main.rs:2:5)</pre></td></tr>
</table>
