# dasp_sample - crashes

Source: https://github.com/rustaudio/sample @ d0892971a3b72dee9b3ff0d16f94b0962605e858

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let i20 = I20::from(0u16);
let _ = &lt;dasp_sample::types::I20 as ::core::ops::Shr&lt;dasp_sample::types::I20&gt;&gt;::
    shr(i20, MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/types.rs:245:5:
attempt to shift right with overflow
backtrace:
  &lt;dasp_sample::types::i20::I20 as core::ops::bit::Shr&gt;::shr (dasp_sample/src/types.rs:205:20)
  repro_dasp_sample_src_types_rs_245_5_15cb03::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let u48 = U48::new_unchecked(i64::MIN);
let _ = dasp_sample::conv::u48::to_i48(u48);</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/conv.rs:495:28:
attempt to subtract with overflow
backtrace:
  dasp_sample::conv::u48::to_i48 (dasp_sample/src/conv.rs:495:28)
  repro_dasp_sample_src_conv_rs_495_28_cf847e::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = dasp_sample::conv::u24::to_i32(U24::new_unchecked(i32::MIN));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/conv.rs:446:9:
attempt to subtract with overflow
backtrace:
  dasp_sample::conv::u24::to_i32 (dasp_sample/src/conv.rs:446:9)
  repro_dasp_sample_src_conv_rs_446_9_4acbc8::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = dasp_sample::conv::u24::to_f64(U24::new_unchecked(i32::MIN));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/conv.rs:443:28:
attempt to subtract with overflow
backtrace:
  dasp_sample::conv::u24::to_i24 (dasp_sample/src/conv.rs:443:28)
  dasp_sample::conv::u24::to_f64 (dasp_sample/src/conv.rs:460:35)
  repro_dasp_sample_src_conv_rs_443_28_b326e7::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = dasp_sample::conv::u48::to_i64(U48::new_unchecked(i64::MIN));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/conv.rs:498:9:
attempt to subtract with overflow
backtrace:
  dasp_sample::conv::u48::to_i64 (dasp_sample/src/conv.rs:498:9)
  repro_dasp_sample_src_conv_rs_498_9_e62f4c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = dasp_sample::conv::i24::to_u64(dasp_sample::conv::f32::to_i24(
    289.0f32));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/conv.rs:259:10:
attempt to add with overflow
backtrace:
  dasp_sample::conv::i24::to_u64 (dasp_sample/src/conv.rs:259:10)
  repro_dasp_sample_src_conv_rs_259_10_a8df45::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;dasp_sample::I24 as ::core::ops::Rem&lt;dasp_sample::I24&gt;&gt;::rem(
    dasp_sample::conv::u24::to_i24(dasp_sample::conv::i64::to_u24(0i64)),
    I24::from(0u16));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/types.rs:251:5:
attempt to calculate the remainder with a divisor of zero
backtrace:
  &lt;dasp_sample::types::i24::I24 as core::ops::arith::Rem&gt;::rem (dasp_sample/src/types.rs:187:20)
  repro_dasp_sample_src_types_rs_251_5_cbdb1e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let i16 = dasp_sample::conv::u48::to_i16(U48::new(0i64).unwrap());
let _ = &lt;dasp_sample::types::U11 as ::core::ops::Neg&gt;::neg(U11::new_unchecked(
    i16));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/types.rs:266:5:
attempt to negate with overflow
backtrace:
  &lt;dasp_sample::types::u11::U11 as core::ops::arith::Neg&gt;::neg (dasp_sample/src/types.rs:55:20)
  repro_dasp_sample_src_types_rs_266_5_d90a0e::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = dasp_sample::conv::i48::to_u64(dasp_sample::conv::f32::to_i48(
    16777216.0f32));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/conv.rs:331:10:
attempt to add with overflow
backtrace:
  dasp_sample::conv::i48::to_u64 (dasp_sample/src/conv.rs:331:10)
  repro_dasp_sample_src_conv_rs_331_10_a94e07::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;dasp_sample::types::I11 as ::core::ops::Shl&lt;dasp_sample::types::I11&gt;&gt;::
    shl(MAX, I11::from(69u8));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/types.rs:238:5:
attempt to shift left with overflow
backtrace:
  &lt;dasp_sample::types::i11::I11 as core::ops::bit::Shl&gt;::shl (dasp_sample/src/types.rs:196:20)
  repro_dasp_sample_src_types_rs_238_5_b7b90e::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;dasp_sample::U24 as ::core::ops::Rem&lt;dasp_sample::U24&gt;&gt;::rem(U24::new(
    0i32).unwrap(), U24::from(0i32));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/types.rs:276:5:
attempt to calculate the remainder with a divisor of zero
backtrace:
  &lt;dasp_sample::types::u24::U24 as core::ops::arith::Rem&gt;::rem (dasp_sample/src/types.rs:187:20)
  repro_dasp_sample_src_types_rs_276_5_150d7c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;dasp_sample::U48 as ::core::ops::Add&lt;dasp_sample::U48&gt;&gt;::add(
    dasp_sample::conv::i24::to_u48(MAX),
    dasp_sample::conv::u64::to_u48(1000000000000000000u64));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/types.rs:282:5:
arithmetic operation overflowed
backtrace:
  &lt;dasp_sample::types::u48::U48 as core::ops::arith::Add&gt;::add (dasp_sample/src/types.rs:136:47)
  repro_dasp_sample_src_types_rs_282_5_dcc374::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;dasp_sample::I24 as ::core::ops::Neg&gt;::neg(
    dasp_sample::conv::f32::to_i24(-1e30f32));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/types.rs:253:5:
attempt to negate with overflow
backtrace:
  &lt;dasp_sample::types::i24::I24 as core::ops::arith::Neg&gt;::neg (dasp_sample/src/types.rs:55:20)
  repro_dasp_sample_src_types_rs_253_5_36c31c::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;dasp_sample::types::I11 as ::core::ops::Neg&gt;::neg(I11::new_unchecked(
    i16::MIN));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/types.rs:240:5:
attempt to negate with overflow
backtrace:
  &lt;dasp_sample::types::i11::I11 as core::ops::arith::Neg&gt;::neg (dasp_sample/src/types.rs:55:20)
  repro_dasp_sample_src_types_rs_240_5_30ba7d::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = dasp_sample::conv::f64::to_u24(9007199254740992.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/conv.rs:250:28:
attempt to add with overflow
backtrace:
  dasp_sample::conv::i24::to_u24 (dasp_sample/src/conv.rs:250:28)
  dasp_sample::conv::f64::to_u24 (dasp_sample/src/conv.rs:560:16)
  repro_dasp_sample_src_conv_rs_250_28_49333e::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let i48 = dasp_sample::conv::f64::to_i48(1.0f64);
let _ = &lt;dasp_sample::I48 as ::core::ops::Mul&lt;dasp_sample::I48&gt;&gt;::mul(
    dasp_sample::conv::u64::to_i48(1u64), i48);</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/types.rs:258:5:
attempt to multiply with overflow
backtrace:
  &lt;dasp_sample::types::i48::I48 as core::ops::arith::Mul&gt;::mul (dasp_sample/src/types.rs:160:29)
  repro_dasp_sample_src_types_rs_258_5_7e2363::main (src/main.rs:3:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = dasp_sample::conv::f64::to_u48(f64::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/conv.rs:328:28:
attempt to add with overflow
backtrace:
  dasp_sample::conv::i48::to_u48 (dasp_sample/src/conv.rs:328:28)
  dasp_sample::conv::f64::to_u48 (dasp_sample/src/conv.rs:562:16)
  repro_dasp_sample_src_conv_rs_328_28_02d242::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;dasp_sample::types::U20 as ::core::ops::Sub&lt;dasp_sample::types::U20&gt;&gt;::
    sub(U20::from(0u16), U20::new_unchecked(1i32));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/types.rs:270:5:
arithmetic operation overflowed
backtrace:
  &lt;dasp_sample::types::u20::U20 as core::ops::arith::Sub&gt;::sub (dasp_sample/src/types.rs:148:47)
  repro_dasp_sample_src_types_rs_270_5_6b04ea::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;dasp_sample::types::U11 as ::core::ops::Div&lt;dasp_sample::types::U11&gt;&gt;::
    div(MIN, MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/types.rs:264:5:
attempt to divide by zero
backtrace:
  &lt;dasp_sample::types::u11::U11 as core::ops::arith::Div&gt;::div (dasp_sample/src/types.rs:171:20)
  repro_dasp_sample_src_types_rs_264_5_12541e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;dasp_sample::I48 as ::core::ops::Neg&gt;::neg(
    &lt;dasp_sample::I48 as ::core::ops::Not&gt;::not(dasp_sample::conv::f64::to_i48(
    1e300f64)));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/types.rs:260:5:
attempt to negate with overflow
backtrace:
  &lt;dasp_sample::types::i48::I48 as core::ops::arith::Neg&gt;::neg (dasp_sample/src/types.rs:55:20)
  repro_dasp_sample_src_types_rs_260_5_e6c110::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = dasp_sample::conv::i24::to_u32(dasp_sample::conv::f64::to_i24(
    65536.0f64));</pre></td><td valign="top"><pre>thread 'main' panicked at dasp_sample/src/conv.rs:253:10:
attempt to add with overflow
backtrace:
  dasp_sample::conv::i24::to_u32 (dasp_sample/src/conv.rs:253:10)
  repro_dasp_sample_src_conv_rs_253_10_640dd1::main (src/main.rs:2:13)</pre></td></tr>
</table>
