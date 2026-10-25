# num-bigint-dig - crashes

Source: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;num_bigint_dig::BigInt as ::core::ops::Rem&lt;isize&gt;&gt;::rem(
    BigInt::from_signed_bytes_le(&amp;vec![]), 0isize);</pre></td><td valign="top"><pre>timeout: Program received signal SIGSEGV, Segmentation fault
backtrace:
  num_bigint_dig::bigint::{impl#128}::rem (num-bigint/src/bigint.rs:2085)
  num_bigint_dig::bigint::{impl#128}::rem (num-bigint/src/bigint.rs:2087)
... (previous line repeated 58 more times)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_bigint_dig::algorithms::div_rem_digit(
    &lt;num_bigint_dig::BigUint as ::core::ops::Shl&lt;usize&gt;&gt;::shl(
    BigUint::from_bytes_be(&amp;vec![]), usize::MAX), 1u64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2305843009213693952 bytes failed
backtrace:
  smallvec::infallible&lt;()&gt; (smallvec/src/lib.rs:331)
  smallvec::SmallVec&lt;[u64; 4]&gt;::reserve_exact&lt;[u64; 4]&gt; (smallvec/src/lib.rs:1375)
  smallvec::SmallVec&lt;[u64; 4]&gt;::with_capacity&lt;[u64; 4]&gt; (smallvec/src/lib.rs:880)
  num_bigint_dig::algorithms::shl::biguint_shl (num-bigint/src/algorithms/shl.rs:16)
  num_bigint_dig::biguint::{impl#20}::shl (num-bigint/src/biguint.rs:430)
  repro_out_of_memory_in_num_bigint_dig_algorithms_div_rem_digit_a4b4d8::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_bigint_dig::prime::probably_prime(
    &amp;&lt;num_bigint_dig::BigInt as num_bigint_dig::IntoBigUint&gt;::into_biguint(
    &lt;num_bigint_dig::BigInt as ::core::ops::Rem&lt;isize&gt;&gt;::rem(BigInt::from(0i8),
    0isize)).unwrap(), 0usize);</pre></td><td valign="top"><pre>timeout: Program received signal SIGSEGV, Segmentation fault
backtrace:
  num_bigint_dig::bigint::{impl#130}::rem (num-bigint/src/bigint.rs:2087)
... (previous line repeated 59 more times)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_bigint_dig::prime::probably_prime_miller_rabin(
    &amp;&lt;num_bigint_dig::BigUint as ::core::ops::Add&lt;u8&gt;&gt;::add(
    BigUint::from_bytes_le(&amp;vec![]), 7u8), 1000000000usize, false);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  smallvec::SmallVec&lt;[u64; 4]&gt;::inline_capacity&lt;[u64; 4]&gt; (smallvec/src/lib.rs:1002)
  smallvec::SmallVec&lt;[u64; 4]&gt;::spilled&lt;[u64; 4]&gt; (smallvec/src/lib.rs:1082)
  smallvec::SmallVec&lt;[u64; 4]&gt;::triple_mut&lt;[u64; 4]&gt; (smallvec/src/lib.rs:1065)
  smallvec::SmallVec&lt;[u64; 4]&gt;::truncate&lt;[u64; 4]&gt; (smallvec/src/lib.rs:1423)
  smallvec::SmallVec&lt;[u64; 4]&gt;::clear&lt;[u64; 4]&gt; (smallvec/src/lib.rs:1464)
  num_bigint_dig::biguint::monty::montgomery (src/monty.rs:59)
  num_bigint_dig::biguint::monty::monty_modpow (src/monty.rs:186)
  num_bigint_dig::biguint::BigUint::modpow (src/biguint.rs:2411)
  num_bigint_dig::prime::probably_prime_miller_rabin (src/prime.rs:201)
  repro_timeout_in_num_bigint_dig_prime_probably_prime_miller_rabin_206241::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>num_bigint_dig::algorithms::sub2(&amp;mut vec![], &amp;[1u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/sub.rs:43:5:
Cannot subtract b from a because b is larger than a.
backtrace:
  num_bigint_dig::algorithms::sub::sub2 (num-bigint/src/algorithms/sub.rs:43:5)
  repro_num_bigint_src_algorithms_sub_rs_43_5_750c6b::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_bigint_dig::algorithms::div_wide(0u64, 0u64, 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/div.rs:29:5:
assertion failed: hi &lt; divisor
backtrace:
  num_bigint_dig::algorithms::div::div_wide (num-bigint/src/algorithms/div.rs:29:5)
  repro_num_bigint_src_algorithms_div_rs_29_5_a58a31::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>num_bigint_dig::algorithms::mac3(&amp;mut vec![], &amp;[1u64], &amp;[1u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/mac.rs:27:28:
mid &gt; len
backtrace:
  &lt;[u64]&gt;::split_at_mut (core/src/slice/mod.rs:1998:21)
  num_bigint_dig::algorithms::mac::mac_digit (num-bigint/src/algorithms/mac.rs:27:28)
  num_bigint_dig::algorithms::mac::long (num-bigint/src/algorithms/mac.rs:68:9)
  num_bigint_dig::algorithms::mac::mac3 (num-bigint/src/algorithms/mac.rs:57:9)
  repro_num_bigint_src_algorithms_mac_rs_27_28_e02dfe::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_bigint_dig::algorithms::cmp_slice(&amp;[], &amp;[0u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/cmp.rs:7:5:
assertion failed: b.last() != Some(&amp;0)
backtrace:
  num_bigint_dig::algorithms::cmp::cmp_slice (num-bigint/src/algorithms/cmp.rs:7:5)
  repro_num_bigint_src_algorithms_cmp_rs_7_5_0d30b5::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>num_bigint_dig::algorithms::add2(&amp;mut vec![], &amp;[0u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/add.rs:16:5:
assertion failed: a.len() &gt;= b.len()
backtrace:
  num_bigint_dig::algorithms::add::__add2 (num-bigint/src/algorithms/add.rs:16:5)
  num_bigint_dig::algorithms::add::add2 (num-bigint/src/algorithms/add.rs:43:17)
  repro_num_bigint_src_algorithms_add_rs_16_5_df5694::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let big_int = &lt;num_bigint_dig::BigInt as ::core::ops::Rem&lt;u8&gt;&gt;::rem(
    BigInt::from(0u32), 0u8);
let _ = big_int.to_str_radix(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/biguint.rs:2024:5:
The radix must be within 2...36
backtrace:
  num_bigint_dig::biguint::to_str_radix_reversed (num-bigint/src/biguint.rs:2024:5)
  &lt;num_bigint_dig::bigint::BigInt&gt;::to_str_radix (num-bigint/src/bigint.rs:3006:21)
  repro_num_bigint_src_biguint_rs_2024_5_556326::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>num_bigint_dig::algorithms::sub2rev(&amp;[0u64], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/sub.rs:64:5:
assertion failed: b.len() &gt;= a.len()
backtrace:
  num_bigint_dig::algorithms::sub::sub2rev (num-bigint/src/algorithms/sub.rs:64:5)
  repro_num_bigint_src_algorithms_sub_rs_64_5_ef81df::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>num_bigint_dig::algorithms::sub2rev(&amp;[], &amp;mut vec![1u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/sub.rs:75:5:
Cannot subtract b from a because b is larger than a.
backtrace:
  num_bigint_dig::algorithms::sub::sub2rev (num-bigint/src/algorithms/sub.rs:75:5)
  repro_num_bigint_src_algorithms_sub_rs_75_5_1b3ea0::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BigUint::from_radix_le(&amp;vec![], 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/biguint.rs:2257:9:
The radix must be within 2...256
backtrace:
  &lt;num_bigint_dig::biguint::BigUint&gt;::from_radix_le (num-bigint/src/biguint.rs:2257:9)
  repro_num_bigint_src_biguint_rs_2257_9_efb809::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BigUint::parse_bytes(&amp;vec![], 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/biguint.rs:284:9:
The radix must be within 2...36
backtrace:
  &lt;num_bigint_dig::biguint::BigUint as num_traits::Num&gt;::from_str_radix (num-bigint/src/biguint.rs:284:9)
  &lt;num_bigint_dig::biguint::BigUint&gt;::parse_bytes::{closure#0} (num-bigint/src/biguint.rs:2194:27)
  &lt;num_bigint_dig::biguint::BigUint&gt;::parse_bytes (num-bigint/src/biguint.rs:2194:14)
  repro_num_bigint_src_biguint_rs_284_9_f7c15f::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_bigint_dig::algorithms::div_rem(&amp;BigUint::from_bytes_be(&amp;vec![]),
    &amp;BigUint::default());</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/div.rs:38:9:
explicit panic
backtrace:
  num_bigint_dig::algorithms::div::div_rem (num-bigint/src/algorithms/div.rs:38:9)
  repro_num_bigint_src_algorithms_div_rs_38_9_dff7b8::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_bigint_dig::algorithms::__sub2rev(&amp;[], &amp;mut vec![0u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/sub.rs:52:5:
assertion failed: b.len() == a.len()
backtrace:
  num_bigint_dig::algorithms::sub::__sub2rev (num-bigint/src/algorithms/sub.rs:52:5)
  repro_num_bigint_src_algorithms_sub_rs_52_5_9ce67a::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;num_bigint_dig::BigUint as ::core::ops::Neg&gt;::neg(
    num_bigint_dig::algorithms::mul3(&amp;[], &amp;[]));</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/biguint.rs:1343:9:
explicit panic
backtrace:
  &lt;num_bigint_dig::biguint::BigUint as core::ops::arith::Neg&gt;::neg (num-bigint/src/biguint.rs:1343:9)
  repro_num_bigint_src_biguint_rs_1343_9_592fc2::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_bigint_dig::algorithms::cmp_slice(&amp;[0u64], &amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/cmp.rs:6:5:
assertion failed: a.last() != Some(&amp;0)
backtrace:
  num_bigint_dig::algorithms::cmp::cmp_slice (num-bigint/src/algorithms/cmp.rs:6:5)
  repro_num_bigint_src_algorithms_cmp_rs_6_5_d57ed1::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let big_int = BigInt::from_biguint(Sign::Minus, BigUint::from(0u8));
let _ = num_bigint_dig::algorithms::jacobi(&amp;big_int,
    &amp;BigInt::from_signed_bytes_be(&amp;vec![]));</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/jacobi.rs:10:9:
invalid arguments, y must be an odd integer,but got BigInt { sign: NoSign, data: BigUint { data: [] } }
backtrace:
  num_bigint_dig::algorithms::jacobi::jacobi (num-bigint/src/algorithms/jacobi.rs:10:9)
  repro_num_bigint_src_algorithms_jacobi_rs_10_9_374abd::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>num_bigint_dig::algorithms::add2(&amp;mut vec![0u64, 0u64, 0u64, 1u64], &amp;[0u64,
    0u64, 0u64, u64::MAX]);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/add.rs:45:5:
assertion failed: carry == 0
backtrace:
  num_bigint_dig::algorithms::add::add2 (num-bigint/src/algorithms/add.rs:45:5)
  repro_num_bigint_src_algorithms_add_rs_45_5_8e5590::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let big_uint = BigUint::new(vec![1u32]);
let _ = big_uint.to_radix_le(1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/biguint.rs:2009:12:
attempt to calculate the remainder with a divisor of zero
backtrace:
  num_bigint_dig::biguint::to_radix_le (num-bigint/src/biguint.rs:2009:12)
  &lt;num_bigint_dig::biguint::BigUint&gt;::to_radix_le (num-bigint/src/biguint.rs:2374:9)
  repro_num_bigint_src_biguint_rs_2009_12_f34af3::main (src/main.rs:5:22)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BigInt::from_radix_be(Sign::Minus, &amp;vec![], 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/biguint.rs:2214:9:
The radix must be within 2...256
backtrace:
  &lt;num_bigint_dig::biguint::BigUint&gt;::from_radix_be (num-bigint/src/biguint.rs:2214:9)
  &lt;num_bigint_dig::bigint::BigInt&gt;::from_radix_be (num-bigint/src/bigint.rs:2884:9)
  repro_num_bigint_src_biguint_rs_2214_9_451c63::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BigUint::from_radix_be(&amp;vec![], 16u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/biguint.rs:187:5:
assertion failed: !v.is_empty() &amp;&amp; bits &lt;= 8 &amp;&amp; big_digit::BITS % bits == 0
backtrace:
  num_bigint_dig::biguint::from_bitwise_digits_le (num-bigint/src/biguint.rs:187:5)
  &lt;num_bigint_dig::biguint::BigUint&gt;::from_radix_be (num-bigint/src/biguint.rs:2229:17)
  repro_num_bigint_src_biguint_rs_187_5_4f5abf::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let big_int = &lt;num_bigint_dig::BigInt as ::core::ops::Div&lt;i64&gt;&gt;::div(
    BigInt::default(), 1i64);
let _ = big_int.get_limb(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/biguint.rs:2105:18:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[u64]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[u64] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;smallvec::SmallVec&lt;[u64; 4]&gt; as core::ops::index::Index&lt;usize&gt;&gt;::index (smallvec/src/lib.rs:2288:18)
  &lt;num_bigint_dig::biguint::BigUint&gt;::get_limb (num-bigint/src/biguint.rs:2105:18)
  &lt;num_bigint_dig::bigint::BigInt&gt;::get_limb (num-bigint/src/bigint.rs:3160:19)
  repro_num_bigint_src_biguint_rs_2105_18_cdc238::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BigUint::from_radix_be(&amp;vec![], 7u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/biguint.rs:242:5:
assertion failed: !v.is_empty() &amp;&amp; !radix.is_power_of_two()
backtrace:
  num_bigint_dig::biguint::from_radix_digits_be (num-bigint/src/biguint.rs:242:5)
  &lt;num_bigint_dig::biguint::BigUint&gt;::from_radix_be (num-bigint/src/biguint.rs:2234:13)
  repro_num_bigint_src_biguint_rs_242_5_9054db::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>num_bigint_dig::algorithms::mac_digit(&amp;mut vec![0u64, 0u64, 0u64, 0u64, 0u64,
    0u64], &amp;[0u64, 0u64, 0u64, 0u64, 0u64, 1000000000u64],
    10000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/mac.rs:35:26:
carry overflow during multiplication!
backtrace:
  num_bigint_dig::algorithms::mac::mac_digit (num-bigint/src/algorithms/mac.rs:35:26)
  repro_num_bigint_src_algorithms_mac_rs_35_26_56d8b8::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let big_int = BigInt::from_signed_bytes_le(&amp;vec![176u8]);
let _ = big_int.to_radix_be(1000000000u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/biguint.rs:2596:5:
The radix must be within 2...256
backtrace:
  num_bigint_dig::biguint::get_radix_base (num-bigint/src/biguint.rs:2596:5)
  num_bigint_dig::biguint::to_radix_digits_le (num-bigint/src/biguint.rs:1982:25)
  num_bigint_dig::biguint::to_radix_le (num-bigint/src/biguint.rs:2019:9)
  &lt;num_bigint_dig::biguint::BigUint&gt;::to_radix_be (num-bigint/src/biguint.rs:2353:21)
  &lt;num_bigint_dig::bigint::BigInt&gt;::to_radix_be (num-bigint/src/bigint.rs:3032:31)
  repro_num_bigint_src_biguint_rs_2596_5_78fcf0::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>num_bigint_dig::algorithms::mac3(&amp;mut vec![], &amp;[0u64, 0u64], &amp;[0u64, 0u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/algorithms/mac.rs:68:27:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[u64] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  num_bigint_dig::algorithms::mac::long (num-bigint/src/algorithms/mac.rs:68:27)
  num_bigint_dig::algorithms::mac::mac3 (num-bigint/src/algorithms/mac.rs:57:9)
  repro_num_bigint_src_algorithms_mac_rs_68_27_d903b8::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_bigint_dig::prime::probably_prime_miller_rabin(
    &amp;&lt;num_bigint_dig::BigUint as ::core::ops::Div&lt;u64&gt;&gt;::div(BigUint::new(
    vec![45u32]), 21u64), 1usize, false);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/bigrand.rs:88:9:
assertion failed: !bound.is_zero()
backtrace:
  &lt;rand::rngs::std::StdRng as num_bigint_dig::bigrand::RandBigInt&gt;::gen_biguint_below (num-bigint/src/bigrand.rs:88:9)
  num_bigint_dig::prime::probably_prime_miller_rabin (num-bigint/src/prime.rs:198:17)
  repro_num_bigint_src_bigrand_rs_88_9_f9e45b::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_bigint_dig::prime::probably_prime_miller_rabin(&amp;BigUint::from(
    1usize), 0usize, false);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/prime.rs:187:34:
called `Option::unwrap()` on a `None` value
backtrace:
  num_bigint_dig::prime::probably_prime_miller_rabin (num-bigint/src/prime.rs:187:34)
  repro_num_bigint_src_prime_rs_187_34_836f2e::main (src/main.rs:4:13)</pre></td></tr>
</table>
