# fraction - crashes

Source: https://github.com/dnsl48/fraction @ 53d04cddb4640d1fe7756c289b3ce34be14da504

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = fraction::division::divide_to_ascii_vec::&lt;u8&gt;(0u8, 0u8,
    10000000000000000000usize, false);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from fraction/src/division.rs:494:22)
capacity overflow
backtrace:
  fraction::division::divide_to_ascii_vec::&lt;u8&gt; (fraction/src/division.rs:494:22)
  repro_fraction_src_division_rs_494_22_c25c8f::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = fraction::division::divide_to_string::&lt;u16&gt;(0u16, 0u16,
    1000000000000000000usize, false);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000000000002 bytes failed
backtrace:
  fraction::division::divide_to_string&lt;u16&gt; (fraction/src/division.rs:448)
  repro_out_of_memory_in_fraction_division_divide_to_string_u16_af1fab::main (src/main.rs:2)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = fraction::division::divide_to_ascii_vec::&lt;u16&gt;(0u16, 0u16, usize::MAX,
    false);</pre></td><td valign="top"><pre>thread 'main' panicked at fraction/src/division.rs:409:5:
attempt to add with overflow
backtrace:
  fraction::division::division_result_max_char_length::&lt;u16&gt; (fraction/src/division.rs:409:5)
  fraction::division::divide_to_ascii_vec::&lt;u16&gt; (fraction/src/division.rs:494:41)
  repro_fraction_src_division_rs_409_5_ddfabc::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = fraction::division::divide_to_string::&lt;u16&gt;(0u16, 1u16,
    10000000000000000000usize, false);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from fraction/src/division.rs:448:22)
capacity overflow
backtrace:
  fraction::division::divide_to_string::&lt;u16&gt; (fraction/src/division.rs:448:22)
  repro_fraction_src_division_rs_448_22_16d084::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = fraction::generic::read_generic_integer::&lt;fraction::BigUint,
    u16&gt;(22u16);
let _ = RawApprox::from(
    &lt;fraction::prelude::GenericFraction&lt;u16&gt; as fraction::Zero&gt;::
    zero().sqrt_abs_raw(4080515371u32));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_bigint::biguint::subtraction::sbb (src/biguint/subtraction.rs:24)
  num_bigint::biguint::subtraction::sub2 (src/biguint/subtraction.rs:58)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:274)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:255)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:242)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:355)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:358)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:349)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:358)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::biguint::power::{impl#12}::pow (src/biguint/power.rs:86)
  fraction::fraction::approx::Accuracy::decimal_places&lt;u32&gt; (fraction/src/fraction/approx.rs:67)
  fraction::fraction::generic_fraction::GenericFraction&lt;u16&gt;::sqrt_abs_raw&lt;u16&gt; (fraction/src/fraction/approx/sqrt.rs:423)
  repro_timeout_in_fraction_approx_sqrt_rawapprox_from_fraction_appr_9b700c::main (src/main.rs:5)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let generic_fraction = &lt;fraction::prelude::GenericFraction&lt;fraction::
    BigUint&gt; as ::core::convert::From&lt;u64&gt;&gt;::from(0u64).sqrt(1000000000u32);
let _ = generic_fraction.denom();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_bigint::biguint::multiplication::mac_digit (src/biguint/multiplication.rs:46)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:105)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:242)
... (previous line repeated 2 more times)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:349)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:358)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:358)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::biguint::power::{impl#12}::pow (src/biguint/power.rs:86)
  fraction::fraction::approx::Accuracy::decimal_places&lt;u32&gt; (fraction/src/fraction/approx.rs:67)
  fraction::fraction::generic_fraction::GenericFraction&lt;num_bigint::biguint::BigUint&gt;::sqrt_raw&lt;num_bigint::biguint::BigUint&gt; (fraction/src/fraction/approx/sqrt.rs:498)
  fraction::fraction::generic_fraction::GenericFraction&lt;num_bigint::biguint::BigUint&gt;::sqrt&lt;num_bigint::biguint::BigUint&gt; (fraction/src/fraction/approx/sqrt.rs:508)
  repro_timeout_in_fraction_prelude_genericfraction_fraction_biguint_d633b9::main (src/main.rs:2)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = fraction::division::divide_to_string::&lt;u8&gt;(0u8, 0u8, 4294967296usize,
    false);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967298 bytes failed
backtrace:
  fraction::division::divide_to_string&lt;u8&gt; (fraction/src/division.rs:448)
  repro_out_of_memory_in_fraction_division_divide_to_string_u8_674ad7::main (src/main.rs:2)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = fraction::division::divide_to_ascii_vec::&lt;u16&gt;(0u16, 0u16,
    1000000000000000000usize, false);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000000000002 bytes failed
backtrace:
  fraction::division::divide_to_ascii_vec&lt;u16&gt; (fraction/src/division.rs:494)
  repro_out_of_memory_in_fraction_division_divide_to_ascii_vec_u16_8e24df::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = fraction::division::divide_to_ascii_vec::&lt;u8&gt;(1u8, 0u8,
    1000000000000000000usize, false);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000000000002 bytes failed
backtrace:
  fraction::division::divide_to_ascii_vec&lt;u8&gt; (fraction/src/division.rs:494)
  repro_out_of_memory_in_fraction_division_divide_to_ascii_vec_u8_4f367e::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;fraction::prelude::GenericFraction&lt;fraction::BigUint&gt; as fraction::
    ToPrimitive&gt;::to_u64(
    &amp;&lt;fraction::prelude::GenericFraction&lt;fraction::BigInt&gt; as fraction::Zero&gt;::
    zero().sqrt(1000000000u32));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_bigint::biguint::multiplication::mac_with_carry (src/biguint/multiplication.rs:21)
  num_bigint::biguint::multiplication::mac_digit (src/biguint/multiplication.rs:47)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:105)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:255)
... (previous line repeated 1 more times)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:271)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:355)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:355)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:355)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::biguint::power::{impl#12}::pow (src/biguint/power.rs:86)
  fraction::fraction::approx::Accuracy::decimal_places&lt;u32&gt; (fraction/src/fraction/approx.rs:67)
  fraction::fraction::generic_fraction::GenericFraction&lt;num_bigint::bigint::BigInt&gt;::sqrt_raw&lt;num_bigint::bigint::BigInt&gt; (fraction/src/fraction/approx/sqrt.rs:498)
  fraction::fraction::generic_fraction::GenericFraction&lt;num_bigint::bigint::BigInt&gt;::sqrt&lt;num_bigint::bigint::BigInt&gt; (fraction/src/fraction/approx/sqrt.rs:508)
  repro_timeout_in_fraction_prelude_genericfraction_fraction_biguint_3523fe::main (src/main.rs:2)</pre></td></tr>
<tr><td valign="top"><pre>let generic_fraction = &lt;fraction::prelude::GenericFraction&lt;fraction::
    BigUint&gt; as ::core::convert::From&lt;i32&gt;&gt;::from(0i32);
let _ = generic_fraction.sqrt_abs(1000000000u32);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_bigint::biguint::multiplication::mac_digit (src/biguint/multiplication.rs:46)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:105)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:242)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:255)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:271)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:355)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:349)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:355)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::biguint::power::{impl#12}::pow (src/biguint/power.rs:86)
  fraction::fraction::approx::Accuracy::decimal_places&lt;u32&gt; (fraction/src/fraction/approx.rs:67)
  fraction::fraction::generic_fraction::GenericFraction&lt;num_bigint::biguint::BigUint&gt;::sqrt_abs_raw&lt;num_bigint::biguint::BigUint&gt; (fraction/src/fraction/approx/sqrt.rs:423)
  fraction::fraction::generic_fraction::GenericFraction&lt;num_bigint::biguint::BigUint&gt;::sqrt_abs&lt;num_bigint::biguint::BigUint&gt; (fraction/src/fraction/approx/sqrt.rs:432)
  repro_timeout_in_fraction_prelude_genericfraction_fraction_biguint_2cbdf1::main (src/main.rs:3)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;fraction::prelude::GenericFraction&lt;fraction::BigUint&gt; as fraction::
    Signed&gt;::is_positive(
    &amp;&lt;fraction::prelude::GenericFraction&lt;fraction::BigUint&gt; as ::core::convert::
    From&lt;fraction::approx::sqrt::RawApprox&gt;&gt;::from(
    &lt;fraction::prelude::GenericFraction&lt;u16&gt;&gt;::nan().sqrt_raw(1688321246u32)));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_bigint::biguint::multiplication::mac_digit (src/biguint/multiplication.rs:46)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:105)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:242)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:271)
... (previous line repeated 1 more times)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:358)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::biguint::power::{impl#12}::pow (src/biguint/power.rs:86)
  fraction::fraction::approx::Accuracy::decimal_places&lt;u32&gt; (fraction/src/fraction/approx.rs:67)
  fraction::fraction::generic_fraction::GenericFraction&lt;u16&gt;::sqrt_raw&lt;u16&gt; (fraction/src/fraction/approx/sqrt.rs:498)
  repro_timeout_in_fraction_prelude_genericfraction_fraction_biguint_390deb::main (src/main.rs:2)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let generic_fraction = &lt;fraction::prelude::GenericFraction&lt;u16&gt; as fraction::
    Bounded&gt;::min_value();
let generic_fraction2 = generic_fraction.fract();
let _ = generic_fraction2.sqrt(1000000000u32);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_bigint::biguint::multiplication::mac_with_carry (src/biguint/multiplication.rs:23)
  num_bigint::biguint::multiplication::mac_digit (src/biguint/multiplication.rs:47)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:105)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:242)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:255)
... (previous line repeated 1 more times)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:358)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:355)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:358)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:349)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::biguint::power::{impl#12}::pow (src/biguint/power.rs:86)
  fraction::fraction::approx::Accuracy::decimal_places&lt;u32&gt; (fraction/src/fraction/approx.rs:67)
  fraction::fraction::generic_fraction::GenericFraction&lt;u16&gt;::sqrt_raw&lt;u16&gt; (fraction/src/fraction/approx/sqrt.rs:498)
  fraction::fraction::generic_fraction::GenericFraction&lt;u16&gt;::sqrt&lt;u16&gt; (fraction/src/fraction/approx/sqrt.rs:508)
  repro_timeout_in_fraction_prelude_genericfraction_u16_sqrt_f01a4d::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let generic_fraction = &lt;fraction::prelude::GenericFraction&lt;u16&gt;&gt;::new_neg::&lt;u16,
    u16&gt;(u16::MAX, 1u16);
let generic_fraction2 = generic_fraction.abs();
let _ = generic_fraction2.ceil();</pre></td><td valign="top"><pre>thread 'main' panicked at num-rational/src/lib.rs:221:18:
attempt to add with overflow
backtrace:
  &lt;u16 as core::ops::arith::Add&gt;::add (core/src/ops/arith.rs:104:45)
  &lt;num_rational::Ratio&lt;u16&gt;&gt;::ceil (num-rational/src/lib.rs:221:18)
  &lt;fraction::fraction::generic_fraction::GenericFraction&lt;u16&gt;&gt;::ceil (fraction/src/fraction/generic_fraction.rs:1027:57)
  repro_num_rational_src_lib_rs_221_18_e12ae0::main (src/main.rs:4:31)</pre></td></tr>
</table>
