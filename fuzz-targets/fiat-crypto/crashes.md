# fiat-crypto - crashes

Source: https://github.com/mit-plv/fiat-crypto @ 64e8f2791b77e080114512e8ff70f20660c611a6

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_square(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 1000000000u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:454:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_square (fiat-rust/src/curve25519_32.rs:454:18)
  repro_fiat_rust_src_curve25519_32_rs_454_18_9ccd59::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32, 0u32,
    0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32], &amp;[0u32, 0u32, 0u32,
    u32::MAX, 0u32, 1000000000u32, 0u32, 0u32, 0u32, 0u32], &amp;[100u32, 0u32,
    u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:110:285:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:110:285)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_110_285_f962c3::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([1u32, 0u32 /* x14 */, 1u32, 0u32, 0u32,
    0u32]);
fiat_crypto::p521_32::fiat_p521_add(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 0u32,
    0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32 /* x9 */,
    u32::MAX, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1165:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_add (fiat-rust/src/p521_32.rs:1165:18)
  repro_fiat_rust_src_p521_32_rs_1165_18_fb3a6a::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([1u32,
    100000000u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:585:17:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry (fiat-rust/src/curve25519_32.rs:585:17)
  repro_fiat_rust_src_curve25519_32_rs_585_17_389872::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_loose_field_element([0u64, 1u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_add(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, u64::MAX]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 1u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:571:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_add (fiat-rust/src/p448_solinas_64.rs:571:17)
  repro_fiat_rust_src_p448_solinas_64_rs_571_17_77ac68::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x9 */, 1u32, 0u32, 1u32, 1u32,
    0u32, 1u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32,
    1000000000u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 1u32,
    0u32 /* x11 */, u32::MAX, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:626:27:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:626:27)
  repro_fiat_rust_src_p448_solinas_32_rs_626_27_f25d29::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_add(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    1u32, 0u32 /* x12 */, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    u32::MAX, 0u32 /* x12 */, 1u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1160:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_add (fiat-rust/src/p448_solinas_32.rs:1160:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1160_17_8ea8b2::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x9 */, 1u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::p521_32::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 1u32,
    0u32 /* x16 */, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 1000000000u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1210:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_sub (fiat-rust/src/p521_32.rs:1210:17)
  repro_fiat_rust_src_p521_32_rs_1210_17_79a04b::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32 /* x8 */, 1u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_sub(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32,
    0u32 /* x14 */, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 1000000000u32, 1u32, 0u32 /* x8 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1206:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_sub (fiat-rust/src/p448_solinas_32.rs:1206:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1206_17_d78d42::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_square(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:160:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_square (fiat-rust/src/secp256k1_dettman_32.rs:160:17)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_160_17_4b9af6::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, u32::MAX, 1u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32,
    1000000000u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32,
    0u32 /* x8 */, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:627:51:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:627:51)
  repro_fiat_rust_src_p448_solinas_32_rs_627_51_d46f72::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_32::
    fiat_25519_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_opp(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32,
    1000000000u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:689:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_opp (fiat-rust/src/curve25519_32.rs:689:17)
  repro_fiat_rust_src_curve25519_32_rs_689_17_bca61f::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_loose_field_element = fiat_crypto::poly1305_64::
    fiat_poly1305_loose_field_element([0u64, 0u64, 0u64]);
fiat_crypto::poly1305_64::fiat_poly1305_add(
    &amp;mut fiat_poly1305_loose_field_element,
    &amp;fiat_crypto::poly1305_64::fiat_poly1305_tight_field_element([0u64, 1u64,
    0u64]), &amp;fiat_crypto::poly1305_64::fiat_poly1305_tight_field_element([0u64,
    u64::MAX, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_64.rs:382:17:
attempt to add with overflow
backtrace:
  fiat_crypto::poly1305_64::fiat_poly1305_add (fiat-rust/src/poly1305_64.rs:382:17)
  repro_fiat_rust_src_poly1305_64_rs_382_17_e8bba1::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_64::
    fiat_poly1305_tight_field_element([0u64, 0u64, 0u64]);
fiat_crypto::poly1305_64::fiat_poly1305_carry_square(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([0u64,
    1000000000000000000u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_64.rs:337:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::poly1305_64::fiat_poly1305_carry_square (fiat-rust/src/poly1305_64.rs:337:18)
  repro_fiat_rust_src_poly1305_64_rs_337_18_ee3a49::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_mul(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32,
    0u32 /* x8 */, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    1u32, 0u32, 1u32, 0u32, 1000000000u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:277:59:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_mul (fiat-rust/src/curve25519_32.rs:277:59)
  repro_fiat_rust_src_curve25519_32_rs_277_59_223627::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_tight_field_element([0u32, 1u32, 1u32, 0u32, 1u32]);
fiat_crypto::poly1305_32::fiat_poly1305_carry_square(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_loose_field_element([0u32, 0u32,
    0u32, 0u32, 576532217u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:287:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_carry_square (fiat-rust/src/poly1305_32.rs:287:17)
  repro_fiat_rust_src_poly1305_32_rs_287_17_088d3b::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 1u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    u64::MAX, 68u64, 14988368381062527468u64, 3383299671908264572u64, 0u64,
    16u64]), &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([2u64,
    1u64, 18446744073709551614u64, 15035455111019105998u64, u64::MAX, u64::MAX,
    13170709496967601630u64, 1u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:319:34:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_64.rs:319:34)
  repro_fiat_rust_src_p448_solinas_64_rs_319_34_029d5b::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_loose_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::poly1305_32::fiat_poly1305_sub(
    &amp;mut fiat_poly1305_loose_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_tight_field_element([u32::MAX,
    0u32, 0u32, 0u32, 1u32]),
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:401:18:
attempt to add with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_sub (fiat-rust/src/poly1305_32.rs:401:18)
  repro_fiat_rust_src_poly1305_32_rs_401_18_6da334::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32,
    0u32 /* x8 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_sub(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([u32::MAX,
    0u32 /* x14 */, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1201:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_sub (fiat-rust/src/p448_solinas_32.rs:1201:18)
  repro_fiat_rust_src_p448_solinas_32_rs_1201_18_5c652f::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1u32, 1u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_square(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, u32::MAX,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:459:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_square (fiat-rust/src/curve25519_32.rs:459:18)
  repro_fiat_rust_src_curve25519_32_rs_459_18_f64574::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    u64::MAX, 0u64, 0u64, 0u64, 1u64, 100u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    0u64, 0u64, u64::MAX, 1u64, 0u64, 1000000000000000000u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:320:34:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_64.rs:320:34)
  repro_fiat_rust_src_p448_solinas_64_rs_320_34_9495f7::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32, 0u32,
    0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32], &amp;[0u32, 0u32, 0u32, 1u32,
    0u32, 1000000000u32, u32::MAX, 1u32, 0u32, 0u32], &amp;[0u32, u32::MAX,
    1000000000u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:63:433:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:63:433)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_63_433_a81e55::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[u32::MAX, 0u32, 0u32, 0u32, 1u32, 100u32, 0u32,
    1u32, 0u32, 0u32], &amp;[1000000000u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32,
    0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:110:36:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:110:36)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_110_36_49d52c::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_64::
    fiat_25519_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_carry_square(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_loose_field_element([0u64, 0u64,
    10000000000000000000u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:292:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_carry_square (fiat-rust/src/curve25519_64.rs:292:17)
  repro_fiat_rust_src_curve25519_64_rs_292_17_a2bd83::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::poly1305_32::fiat_poly1305_carry_square(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_loose_field_element([0u32, 0u32,
    0u32, 798175352u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:290:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_carry_square (fiat-rust/src/poly1305_32.rs:290:17)
  repro_fiat_rust_src_poly1305_32_rs_290_17_9c8aeb::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_32::
    fiat_25519_loose_field_element([0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_sub(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([u32::MAX, 0u32,
    0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([1u32, 1u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:657:18:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_sub (fiat-rust/src/curve25519_32.rs:657:18)
  repro_fiat_rust_src_curve25519_32_rs_657_18_d5c153::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 1u32, 1u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry_square(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([1u32, 0u32 /* x16 */,
    u32::MAX, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:751:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_square (fiat-rust/src/p521_32.rs:751:17)
  repro_fiat_rust_src_p521_32_rs_751_17_74f746::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 0u64, 1u64, 0u64, 1u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, u64::MAX, 0u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([u64::MAX,
    50u64, 2u64, u64::MAX, 9818933512143876836u64, 41u64, 8u64, 7u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:320:62:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_64.rs:320:62)
  repro_fiat_rust_src_p448_solinas_64_rs_320_62_2ae3b1::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 1u32,
    0u32 /* x9 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_sub(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32,
    0u32 /* x14 */, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 1u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1209:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_sub (fiat-rust/src/p448_solinas_32.rs:1209:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1209_17_ac98eb::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[u32::MAX, 0u32, 0u32, 1000u32, 0u32, 0u32, 1u32,
    0u32, 0u32, 0u32], &amp;[0u32, 1u32, 0u32, 1000000000u32, 0u32, 0u32, u32::MAX,
    0u32, 0u32, 1u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:116:36:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:116:36)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_116_36_88794d::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::poly1305_32::fiat_poly1305_carry_square(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_loose_field_element([0u32, 0u32,
    1u32, 0u32, 1000000000u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:286:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_carry_square (fiat-rust/src/poly1305_32.rs:286:17)
  repro_fiat_rust_src_poly1305_32_rs_286_17_a5813f::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32, 0u32, 0u32, 1u32, 0u32 /* x10 */,
    0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 1u32, u32::MAX,
    1000000000u32, 0u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32,
    0u32 /* x8 */, 1000000000u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:626:179:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:626:179)
  repro_fiat_rust_src_p448_solinas_32_rs_626_179_056c4a::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_64::
    fiat_25519_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_carry_mul(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_loose_field_element([0u64, 0u64,
    0u64, 0u64, 0u64]),
    &amp;fiat_crypto::curve25519_64::fiat_25519_loose_field_element([0u64,
    1000000000000000000u64, 0u64, 1u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:221:61:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_carry_mul (fiat-rust/src/curve25519_64.rs:221:61)
  repro_fiat_rust_src_curve25519_64_rs_221_61_5f37b5::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_square(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1000000000u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:442:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_square (fiat-rust/src/curve25519_32.rs:442:17)
  repro_fiat_rust_src_curve25519_32_rs_442_17_be7446::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 1u32,
    0u32 /* x9 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([u32::MAX,
    0u32, 1u32, 1u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    100000000u32, 0u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([1u32, 0u32,
    0u32, 0u32, u32::MAX, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32,
    1u32, 100000000u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:635:19:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:635:19)
  repro_fiat_rust_src_p448_solinas_32_rs_635_19_335591::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1000u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:626:67:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:626:67)
  repro_fiat_rust_src_p448_solinas_32_rs_626_67_8076a0::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_square(&amp;mut [0u64,
    0u64, 0u64, 0u64, 0u64], &amp;[0u64, 0u64, u64::MAX, 85u64, 5u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_64.rs:118:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_square (fiat-rust/src/secp256k1_dettman_64.rs:118:17)
  repro_fiat_rust_src_secp256k1_dettman_64_rs_118_17_3cb56a::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_64::
    fiat_25519_loose_field_element([0u64, 1u64, 0u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_opp(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_tight_field_element(
    [100000000000000000u64, 0u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:422:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_opp (fiat-rust/src/curve25519_64.rs:422:17)
  repro_fiat_rust_src_curve25519_64_rs_422_17_5c41a0::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x11 */, 1u32, 0u32, 0u32,
    0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1000000000u32, u32::MAX, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1098:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry (fiat-rust/src/p448_solinas_32.rs:1098:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1098_17_cdad66::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_square(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, u32::MAX, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:768:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_square (fiat-rust/src/p448_solinas_32.rs:768:18)
  repro_fiat_rust_src_p448_solinas_32_rs_768_18_5c8940::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_add(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32 /* x8 */, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    0u32, 0u32, 1u32, 0u32, 1u32, 0u32 /* x8 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1164:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_add (fiat-rust/src/p448_solinas_32.rs:1164:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1164_17_c9f161::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_32::
    fiat_25519_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_opp(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 0u32,
    1u32, 0u32, 1000000000u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:692:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_opp (fiat-rust/src/curve25519_32.rs:692:17)
  repro_fiat_rust_src_curve25519_32_rs_692_17_2ee38c::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_32::
    fiat_25519_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_sub(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32,
    0u32 /* x8 */, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 0u32,
    1u32, 1000000000u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:660:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_sub (fiat-rust/src/curve25519_32.rs:660:17)
  repro_fiat_rust_src_curve25519_32_rs_660_17_11208d::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 100000000u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32,
    u32::MAX, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32,
    0u32 /* x10 */, u32::MAX, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:625:51:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:625:51)
  repro_fiat_rust_src_p448_solinas_32_rs_625_51_ef1b65::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 1u32,
    0u32, 0u32, 1000000000u32, u32::MAX, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:588:17:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry (fiat-rust/src/curve25519_32.rs:588:17)
  repro_fiat_rust_src_curve25519_32_rs_588_17_cbb536::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_loose_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::poly1305_32::fiat_poly1305_sub(
    &amp;mut fiat_poly1305_loose_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32]),
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_tight_field_element([0u32, 0u32,
    1000000000u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:403:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_sub (fiat-rust/src/poly1305_32.rs:403:17)
  repro_fiat_rust_src_poly1305_32_rs_403_17_5f7127::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_64::
    fiat_25519_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_sub(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_tight_field_element([u64::MAX, 0u64,
    0u64, 0u64, 0u64]),
    &amp;fiat_crypto::curve25519_64::fiat_25519_tight_field_element([0u64, 0u64,
    0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:401:18:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_sub (fiat-rust/src/curve25519_64.rs:401:18)
  repro_fiat_rust_src_curve25519_64_rs_401_18_5a8d07::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64,
    10000000000000000000u64, u64::MAX, 40u64, 18446744073709551614u64, 78u64,
    8u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:544:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry (fiat-rust/src/p521_64.rs:544:17)
  repro_fiat_rust_src_p521_64_rs_544_17_e57d63::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[0u32, 0u32, 0u32, 0u32, 1u32, u32::MAX,
    100000000u32, 1u32, 0u32, 8u32], &amp;[0u32, 0u32, 1u32, 0u32, 1000000000u32,
    0u32, 0u32, 10u32, u32::MAX, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:95:117:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:95:117)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_95_117_ab262f::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([1u32, 0u32, 0u32, 1u32, 0u32 /* x11 */,
    0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32,
    0u32 /* x8 */, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    1u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    8u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:627:203:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:627:203)
  repro_fiat_rust_src_p448_solinas_32_rs_627_203_da8092::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x9 */, 1u32, 0u32, 0u32, 0u32,
    0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32, 1u32, 0u32, 0u32,
    u32::MAX, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([1000000000u32,
    0u32 /* x10 */, u32::MAX, 1u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:622:75:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:622:75)
  repro_fiat_rust_src_p448_solinas_32_rs_622_75_87a6d1::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x12 */, 1u32, 0u32, 0u32, 0u32,
    0u32, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry_mul(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([1u32, 0u32 /* x13 */,
    1u32, 0u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([1u32, 0u32 /* x16 */,
    u32::MAX, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:275:60:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_mul (fiat-rust/src/p521_32.rs:275:60)
  repro_fiat_rust_src_p521_32_rs_275_60_cdd1d0::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([1u32, 0u32, 0u32, 1u32, 0u32 /* x14 */,
    1u32]);
fiat_crypto::p521_32::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32 /* x10 */,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 1000000000u32,
    0u32 /* x9 */, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1205:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_sub (fiat-rust/src/p521_32.rs:1205:17)
  repro_fiat_rust_src_p521_32_rs_1205_17_091e88::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_32::
    fiat_25519_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32,
    1u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_add(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 0u32,
    1u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 1u32,
    0u32, 0u32, 0u32, 1u32, 1u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:631:17:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_add (fiat-rust/src/curve25519_32.rs:631:17)
  repro_fiat_rust_src_curve25519_32_rs_631_17_19d228::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32, 1u32, 0u32, 0u32, 1u32,
    0u32 /* x9 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_add(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([1u32, 0u32,
    0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32,
    0u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32,
    0u32 /* x9 */, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1168:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_add (fiat-rust/src/p448_solinas_32.rs:1168:18)
  repro_fiat_rust_src_p448_solinas_32_rs_1168_18_7bd869::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x14 */, 1u32, 0u32, 0u32,
    1u32]);
fiat_crypto::p521_32::fiat_p521_opp(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32 /* x10 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1260:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_opp (fiat-rust/src/p521_32.rs:1260:17)
  repro_fiat_rust_src_p521_32_rs_1260_17_a75f01::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [1u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32], &amp;[0u32, 0u32, 0u32, 0u32,
    0u32, 1000000000u32, 0u32, 0u32, 0u32, u32::MAX], &amp;[0u32, 1u32, u32::MAX,
    0u32, 0u32, 0u32, 1000000000u32, 0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:83:283:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:83:283)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_83_283_a2d11b::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::poly1305_64::fiat_poly1305_addcarryx_u43(&amp;mut 0u64, &amp;mut 0u8, 0u8,
    1u64, u64::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_64.rs:205:17:
attempt to add with overflow
backtrace:
  fiat_crypto::poly1305_64::fiat_poly1305_addcarryx_u43 (fiat-rust/src/poly1305_64.rs:205:17)
  repro_fiat_rust_src_poly1305_64_rs_205_17_dd5d68::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element(
    [10000000000000000000u64, u64::MAX, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:529:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry (fiat-rust/src/p448_solinas_64.rs:529:17)
  repro_fiat_rust_src_p448_solinas_64_rs_529_17_83ceef::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_square(&amp;mut [0u64,
    0u64, 0u64, 0u64, 0u64], &amp;[0u64, u64::MAX, 256u64, 0u64,
    5146411136993599675u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_64.rs:119:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_square (fiat-rust/src/secp256k1_dettman_64.rs:119:17)
  repro_fiat_rust_src_secp256k1_dettman_64_rs_119_17_cc01d1::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_32::
    fiat_25519_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_sub(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32,
    0u32 /* x8 */, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 443984080u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:663:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_sub (fiat-rust/src/curve25519_32.rs:663:17)
  repro_fiat_rust_src_curve25519_32_rs_663_17_e27364::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 1u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_mul(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32,
    0u32 /* x8 */, 1000000000u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:274:59:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_mul (fiat-rust/src/curve25519_32.rs:274:59)
  repro_fiat_rust_src_curve25519_32_rs_274_59_81acc1::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 1u64]);
fiat_crypto::p521_64::fiat_p521_opp(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 1000000000000000000u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:646:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_opp (fiat-rust/src/p521_64.rs:646:17)
  repro_fiat_rust_src_p521_64_rs_646_17_bb1472::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 1u32,
    0u32 /* x12 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_add(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 1u32,
    0u32, u32::MAX, 0u32 /* x13 */, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 1u32, 0u32,
    0u32, 1u32, 0u32 /* x11 */, 1u32, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1159:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_add (fiat-rust/src/p521_32.rs:1159:17)
  repro_fiat_rust_src_p521_32_rs_1159_17_25817d::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_64::
    fiat_25519_tight_field_element([0u64, 0u64, 0u64, 0u64, 1u64]);
fiat_crypto::curve25519_64::fiat_25519_carry(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_loose_field_element(
    [1000000000000000000u64, u64::MAX, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:353:17:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_carry (fiat-rust/src/curve25519_64.rs:353:17)
  repro_fiat_rust_src_curve25519_64_rs_353_17_6b55de::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_add(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 0u32,
    0u32, 0u32, 1u32, 0u32 /* x12 */, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 1u32, 0u32,
    1u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1160:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_add (fiat-rust/src/p521_32.rs:1160:17)
  repro_fiat_rust_src_p521_32_rs_1160_17_304b29::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 1u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_add(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, u64::MAX,
    0u64, 0u64, 0u64, 0u64, 1u64, 0u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 1u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:582:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_add (fiat-rust/src/p521_64.rs:582:17)
  repro_fiat_rust_src_p521_64_rs_582_17_3ee09f::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 1u32,
    0u32 /* x12 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_add(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32 /* x15 */,
    1u32, 0u32, u32::MAX]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 1u32,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32 /* x8 */, 1u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1173:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_add (fiat-rust/src/p521_32.rs:1173:18)
  repro_fiat_rust_src_p521_32_rs_1173_18_af1102::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([u64::MAX,
    0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 1000000000000000000u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    0u64, 0u64, u64::MAX, 28u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:322:20:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_64.rs:322:20)
  repro_fiat_rust_src_p448_solinas_64_rs_322_20_a78dca::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::poly1305_32::fiat_poly1305_carry_mul(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32]),
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_loose_field_element([0u32,
    1000000000u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:221:59:
attempt to multiply with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_carry_mul (fiat-rust/src/poly1305_32.rs:221:59)
  repro_fiat_rust_src_poly1305_32_rs_221_59_f1335d::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_64::
    fiat_poly1305_tight_field_element([0u64, 0u64, 0u64]);
fiat_crypto::poly1305_64::fiat_poly1305_carry_square(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([0u64, 0u64,
    10000000000000000000u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_64.rs:316:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::poly1305_64::fiat_poly1305_carry_square (fiat-rust/src/poly1305_64.rs:316:17)
  repro_fiat_rust_src_poly1305_64_rs_316_17_d58787::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_64::
    fiat_poly1305_tight_field_element([0u64, 0u64, 1u64]);
fiat_crypto::poly1305_64::fiat_poly1305_carry_mul(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([0u64, 1u64,
    0u64]), &amp;fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([0u64,
    0u64, 2709226863620216900u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_64.rs:276:61:
attempt to multiply with overflow
backtrace:
  fiat_crypto::poly1305_64::fiat_poly1305_carry_mul (fiat-rust/src/poly1305_64.rs:276:61)
  repro_fiat_rust_src_poly1305_64_rs_276_61_487467::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::curve25519_32::fiat_25519_addcarryx_u25(&amp;mut 0u32, &amp;mut 0u8, 1u8,
    u32::MAX, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:205:18:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_addcarryx_u25 (fiat-rust/src/curve25519_32.rs:205:18)
  repro_fiat_rust_src_curve25519_32_rs_205_18_d121eb::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32, 0u32,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32], &amp;[u32::MAX, 0u32, 0u32,
    0u32, 1000000000u32, 0u32, 0u32, 0u32, 0u32, 0u32], &amp;[10000u32, 0u32, 0u32,
    0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:104:36:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:104:36)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_104_36_d57526::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32, 0u32, 0u32, 1u32, 1u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([1000000000u32,
    u32::MAX, 0u32, 0u32, 1u32, 0u32 /* x10 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1099:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry (fiat-rust/src/p448_solinas_32.rs:1099:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1099_17_8e9cde::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_add(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64,
    1u64, 0u64, 0u64, 0u64, 1u64, 1u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, u64::MAX]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:588:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_add (fiat-rust/src/p521_64.rs:588:17)
  repro_fiat_rust_src_p521_64_rs_588_17_b4e767::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64,
    1000000000000000000u64, u64::MAX, 0u64, 0u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:543:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry (fiat-rust/src/p521_64.rs:543:17)
  repro_fiat_rust_src_p521_64_rs_543_17_e85877::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    0u64, 0u64, 100000000000000000u64, u64::MAX, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:528:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry (fiat-rust/src/p448_solinas_64.rs:528:17)
  repro_fiat_rust_src_p448_solinas_64_rs_528_17_ce75d8::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, u64::MAX,
    0u64, 0u64, 0u64, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:611:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_sub (fiat-rust/src/p521_64.rs:611:18)
  repro_fiat_rust_src_p521_64_rs_611_18_e661aa::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32,
    1000000000u32, 1u32], &amp;[0u32, 1000000000u32, 0u32, 0u32, u32::MAX, 0u32,
    1u32, 0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:72:451:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:72:451)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_72_451_eb0e21::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_loose_field_element([1u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_sub(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    0u64, 0u64, u64::MAX, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([1u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:595:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_sub (fiat-rust/src/p448_solinas_64.rs:595:18)
  repro_fiat_rust_src_p448_solinas_64_rs_595_18_b8d998::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([0u64, 0u64, 0u64, 1u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry_square(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 1u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 4611686018427387904u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:434:62:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry_square (fiat-rust/src/p521_64.rs:434:62)
  repro_fiat_rust_src_p521_64_rs_434_62_47e3db::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_opp(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    1000000000u32, 0u32 /* x12 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1246:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_opp (fiat-rust/src/p448_solinas_32.rs:1246:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1246_17_8b13ed::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[u32::MAX, 0u32 /* x8 */, 10000000u32],
    &amp;[1000000000u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:72:36:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:72:36)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_72_36_2adae4::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::poly1305_32::fiat_poly1305_carry_square(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_loose_field_element([0u32, 0u32,
    0u32, 1000000000u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:289:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_carry_square (fiat-rust/src/poly1305_32.rs:289:17)
  repro_fiat_rust_src_poly1305_32_rs_289_17_2ef526::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([1u64, 0u64, 0u64, 0u64, 0u64, 1u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry_square(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 1u64, 0u64, 0u64, u64::MAX]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:418:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry_square (fiat-rust/src/p521_64.rs:418:17)
  repro_fiat_rust_src_p521_64_rs_418_17_2bb09c::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry_mul(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 10000000000000000000u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:274:61:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry_mul (fiat-rust/src/p521_64.rs:274:61)
  repro_fiat_rust_src_p521_64_rs_274_61_31a39b::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x11 */, 1u32, 0u32, 0u32,
    0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([1u32, 0u32,
    0u32, 0u32, 0u32, 1u32, 0u32, 1000000000u32, u32::MAX, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1096:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry (fiat-rust/src/p448_solinas_32.rs:1096:18)
  repro_fiat_rust_src_p448_solinas_32_rs_1096_18_db3c51::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_opp(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 1u32, 0u32, 1u32, 0u32, 1000000000u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1253:18:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_opp (fiat-rust/src/p448_solinas_32.rs:1253:18)
  repro_fiat_rust_src_p448_solinas_32_rs_1253_18_a9448f::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_square(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32,
    0u32, 1u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:765:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_square (fiat-rust/src/p448_solinas_32.rs:765:18)
  repro_fiat_rust_src_p448_solinas_32_rs_765_18_b90a3e::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_mul(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    1u32, 0u32, 1000000000u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:279:59:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_mul (fiat-rust/src/curve25519_32.rs:279:59)
  repro_fiat_rust_src_curve25519_32_rs_279_59_6e2289::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32,
    0u32, 0u32, 1u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry_square(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32, 0u32,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, u32::MAX, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:772:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_square (fiat-rust/src/p521_32.rs:772:18)
  repro_fiat_rust_src_p521_32_rs_772_18_af25e8::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_add(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element(
    [10000000000000000000u64, 0u64, 66u64, 0u64, 13628641190857162675u64, 25u64,
    11971366163859023337u64, 12907745166445320433u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element(
    [18446744073709551614u64, 4600271130141445088u64, u64::MAX, u64::MAX, 1u64,
    7070801945689275461u64, u64::MAX, 46u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:564:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_add (fiat-rust/src/p448_solinas_64.rs:564:17)
  repro_fiat_rust_src_p448_solinas_64_rs_564_17_115559::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 1u32,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32,
    0u32 /* x9 */, 1u32, 0u32, 0u32, u32::MAX, 0u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32,
    1000000000u32, 1u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, u32::MAX, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:626:123:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:626:123)
  repro_fiat_rust_src_p448_solinas_32_rs_626_123_ef0b0c::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([0u64, 1u64, 0u64, 0u64, 1u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry_mul(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64,
    0u64, 1u64, 8785861306923880144u64, u64::MAX, 0u64, 0u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([u64::MAX, 0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 8561540067636029732u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:361:54:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry_mul (fiat-rust/src/p521_64.rs:361:54)
  repro_fiat_rust_src_p521_64_rs_361_54_107e41::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    0u64, 1000000000000000000u64, u64::MAX, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:526:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry (fiat-rust/src/p448_solinas_64.rs:526:18)
  repro_fiat_rust_src_p448_solinas_64_rs_526_18_77d487::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_tight_field_element([0u32, 0u32, 0u32, 0u32, 1u32]);
fiat_crypto::poly1305_32::fiat_poly1305_carry_mul(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32]),
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_loose_field_element([0u32, 0u32,
    895058735u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:220:59:
attempt to multiply with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_carry_mul (fiat-rust/src/poly1305_32.rs:220:59)
  repro_fiat_rust_src_poly1305_32_rs_220_59_b0cd2a::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 1u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 1u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64,
    1000000000000000000u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:611:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_sub (fiat-rust/src/p521_64.rs:611:17)
  repro_fiat_rust_src_p521_64_rs_611_17_324cc9::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::poly1305_64::fiat_poly1305_addcarryx_u43(&amp;mut 0u64, &amp;mut 0u8, 1u8,
    u64::MAX, 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_64.rs:205:18:
attempt to add with overflow
backtrace:
  fiat_crypto::poly1305_64::fiat_poly1305_addcarryx_u43 (fiat-rust/src/poly1305_64.rs:205:18)
  repro_fiat_rust_src_poly1305_64_rs_205_18_fa378b::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 1u32, 1000000000u32, u32::MAX, 0u32 /* x8 */,
    0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1094:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry (fiat-rust/src/p521_32.rs:1094:18)
  repro_fiat_rust_src_p521_32_rs_1094_18_d592e3::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    1u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_opp(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64,
    10000000000000000000u64, 255u64, 255u64, 1u64, 16u64,
    17740083202064477018u64, 1084218626996030245u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:640:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_opp (fiat-rust/src/p521_64.rs:640:17)
  repro_fiat_rust_src_p521_64_rs_640_17_df7e2c::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x15 */, 1u32, 0u32, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry_square(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32, 0u32,
    0u32, 0u32, u32::MAX, 0u32 /* x8 */, 1u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:778:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_square (fiat-rust/src/p521_32.rs:778:18)
  repro_fiat_rust_src_p521_32_rs_778_18_d840eb::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_mul(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32,
    0u32 /* x8 */, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    1000000000u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:281:59:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_mul (fiat-rust/src/curve25519_32.rs:281:59)
  repro_fiat_rust_src_curve25519_32_rs_281_59_e77c6f::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    u64::MAX, 0u64, 0u64, 0u64, 1000000000000000000u64,
    4611686018427387904u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64,
    u64::MAX, 0u64, 0u64, 7u64, 7u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:316:33:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_64.rs:316:33)
  repro_fiat_rust_src_p448_solinas_64_rs_316_33_32c493::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_64::
    fiat_25519_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_carry_square(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_loose_field_element([0u64,
    10000000000000000000u64, 3978487153708794799u64, 1u64, 256u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:293:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_carry_square (fiat-rust/src/curve25519_64.rs:293:17)
  repro_fiat_rust_src_curve25519_64_rs_293_17_52a3ee::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32, 1u32, 0u32 /* x12 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 1000000000u32, u32::MAX, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1107:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry (fiat-rust/src/p448_solinas_32.rs:1107:18)
  repro_fiat_rust_src_p448_solinas_32_rs_1107_18_3c6996::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32, 0u32,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32], &amp;[0u32, 1u32, 0u32, 0u32,
    0u32, 0u32, 1000000000u32, 0u32, u32::MAX, 1u32], &amp;[0u32, u32::MAX, 0u32,
    1000000000u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:72:534:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:72:534)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_72_534_06ebea::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32,
    0u32, 10u32], &amp;[0u32, 0u32, 1000000000u32, 1u32, 0u32, 0u32, 0u32, u32::MAX,
    0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:83:200:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:83:200)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_83_200_a020fe::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_64::
    fiat_25519_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_carry(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_loose_field_element([0u64, 1u64,
    10000000000000000000u64, 18446744073709551614u64, 4294967295u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:355:17:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_carry (fiat-rust/src/curve25519_64.rs:355:17)
  repro_fiat_rust_src_curve25519_64_rs_355_17_7c333d::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_opp(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    1u32, 0u32, 0u32, 1000000000u32, 0u32 /* x9 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1249:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_opp (fiat-rust/src/p448_solinas_32.rs:1249:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1249_17_cd33d0::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_64::
    fiat_25519_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_carry_mul(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_loose_field_element([0u64, 0u64,
    10000000000000000000u64, 256u64, 18446744073709551614u64]),
    &amp;fiat_crypto::curve25519_64::fiat_25519_loose_field_element([u64::MAX,
    67u64, 14u64, 26u64, 1000u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:246:33:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_carry_mul (fiat-rust/src/curve25519_64.rs:246:33)
  repro_fiat_rust_src_curve25519_64_rs_246_33_40827e::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_64::
    fiat_25519_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_sub(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_tight_field_element([0u64, 0u64,
    0u64, 0u64, 0u64]),
    &amp;fiat_crypto::curve25519_64::fiat_25519_tight_field_element([0u64, 0u64,
    1u64, 0u64, 1000000000000000000u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:405:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_sub (fiat-rust/src/curve25519_64.rs:405:17)
  repro_fiat_rust_src_curve25519_64_rs_405_17_3682bc::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    8u32, u32::MAX], &amp;[u32::MAX, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 1u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:72:700:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:72:700)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_72_700_bc7fef::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_add(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 1u64, 0u64,
    0u64, 1u64, 0u64, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, u64::MAX, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:581:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_add (fiat-rust/src/p521_64.rs:581:17)
  repro_fiat_rust_src_p521_64_rs_581_17_6ccab7::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1000000000u32, 0u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 1u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:626:59:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:626:59)
  repro_fiat_rust_src_p448_solinas_32_rs_626_59_2d6331::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32, 0u32,
    0u32, 0u32, 1000000000u32, u32::MAX, 0u32 /* x10 */, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1091:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry (fiat-rust/src/p521_32.rs:1091:17)
  repro_fiat_rust_src_p521_32_rs_1091_17_568e19::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 1u32,
    0u32, 0u32, 1u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_square(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:448:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_square (fiat-rust/src/curve25519_32.rs:448:17)
  repro_fiat_rust_src_curve25519_32_rs_448_17_31670e::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64,
    u64::MAX, 1u64, 0u64, 0u64, 1u64, 0u64, 0u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 1u64, u64::MAX]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:322:55:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_64.rs:322:55)
  repro_fiat_rust_src_p448_solinas_64_rs_322_55_1bbfaa::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    0u64, 0u64, u64::MAX, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    0u64, 1000000000000000000u64, 0u64, 0u64, 0u64, u64::MAX]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:319:48:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_64.rs:319:48)
  repro_fiat_rust_src_p448_solinas_64_rs_319_48_c91e54::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 1u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_sub(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    1u64, 0u64, 0u64, 0u64, 1u64, 0u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 356024115197798989u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:598:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_sub (fiat-rust/src/p448_solinas_64.rs:598:17)
  repro_fiat_rust_src_p448_solinas_64_rs_598_17_4ccb2e::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::poly1305_32::fiat_poly1305_carry_square(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_loose_field_element([0u32, 0u32,
    u32::MAX, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:292:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_carry_square (fiat-rust/src/poly1305_32.rs:292:17)
  repro_fiat_rust_src_poly1305_32_rs_292_17_f15233::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_loose_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_loose_field_element([1u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::poly1305_32::fiat_poly1305_opp(
    &amp;mut fiat_poly1305_loose_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_tight_field_element([0u32, 0u32,
    0u32, 778121783u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:425:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_opp (fiat-rust/src/poly1305_32.rs:425:17)
  repro_fiat_rust_src_poly1305_32_rs_425_17_776d90::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_square(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, u32::MAX]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:729:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_square (fiat-rust/src/p448_solinas_32.rs:729:17)
  repro_fiat_rust_src_p448_solinas_32_rs_729_17_0e434b::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_opp(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 1u64,
    0u64, 0u64, 10000000000000000000u64, 1966608797484433845u64,
    2046354619379167194u64, 4611686018427387904u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:643:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_opp (fiat-rust/src/p521_64.rs:643:17)
  repro_fiat_rust_src_p521_64_rs_643_17_ee069a::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32, 1u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32], &amp;[0u32, u32::MAX, 0u32,
    0u32, 0u32, 0u32, 0u32, 10u32, 0u32, 0u32], &amp;[0u32, 0u32, 1000000000u32,
    1u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:72:119:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:72:119)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_72_119_ee71b0::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry_square(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    0u64, 1u64, 0u64, 10000000000000000000u64, 1000u64, 1u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:393:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry_square (fiat-rust/src/p448_solinas_64.rs:393:18)
  repro_fiat_rust_src_p448_solinas_64_rs_393_18_2b703f::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32,
    1000000000u32, u32::MAX, 0u32, 0u32, 0u32, 1u32, 0u32 /* x8 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1101:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry (fiat-rust/src/p448_solinas_32.rs:1101:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1101_17_17c4e1::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    1u64]);
fiat_crypto::p448_solinas_64::fiat_p448_opp(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    0u64, 0u64, 1000000000000000000u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:622:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_opp (fiat-rust/src/p448_solinas_64.rs:622:17)
  repro_fiat_rust_src_p448_solinas_64_rs_622_17_953f81::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 1u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_opp(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64,
    1000000000000000000u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:639:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_opp (fiat-rust/src/p521_64.rs:639:17)
  repro_fiat_rust_src_p521_64_rs_639_17_a387cc::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 1u32,
    0u32, 1u32, 0u32 /* x9 */, 1u32, 0u32, u32::MAX]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 1u32, 0u32, 0u32, 0u32, u32::MAX, 1u32, 1u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 1u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:626:187:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:626:187)
  repro_fiat_rust_src_p448_solinas_32_rs_626_187_ba77dc::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1u32, 0u32 /* x10 */, 1u32]);
fiat_crypto::p521_32::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([u32::MAX, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 1u32,
    0u32 /* x16 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1204:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_sub (fiat-rust/src/p521_32.rs:1204:18)
  repro_fiat_rust_src_p521_32_rs_1204_18_84476b::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_opp(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element(
    [10000000000000000000u64, 8u64, 8965607643552440008u64, 0u64, 0u64,
    731325735078940678u64, 10006887957558925050u64, 72381077612263818u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:618:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_opp (fiat-rust/src/p448_solinas_64.rs:618:17)
  repro_fiat_rust_src_p448_solinas_64_rs_618_17_f51401::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_sub(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    1u64, 0u64, u64::MAX, 6636102302863855952u64, 4294967295u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:595:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_sub (fiat-rust/src/p448_solinas_64.rs:595:17)
  repro_fiat_rust_src_p448_solinas_64_rs_595_17_115d14::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_square(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[1u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:161:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_square (fiat-rust/src/secp256k1_dettman_32.rs:161:17)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_161_17_9a4be2::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([1000000000u32,
    u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:584:17:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry (fiat-rust/src/curve25519_32.rs:584:17)
  repro_fiat_rust_src_curve25519_32_rs_584_17_3974b5::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x13 */, 1u32, 0u32, 0u32, 0u32,
    0u32]);
fiat_crypto::p521_32::fiat_p521_opp(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 1000000000u32,
    0u32 /* x16 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1254:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_opp (fiat-rust/src/p521_32.rs:1254:17)
  repro_fiat_rust_src_p521_32_rs_1254_17_dd47bb::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32,
    1u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32]);
fiat_crypto::p521_32::fiat_p521_carry_mul(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 1u32, 0u32 /* x10 */, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32, 0u32,
    0u32, u32::MAX, 0u32 /* x11 */, 1u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:306:61:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_mul (fiat-rust/src/p521_32.rs:306:61)
  repro_fiat_rust_src_p521_32_rs_306_61_ff740f::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_mul(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32,
    0u32 /* x8 */, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:276:59:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_mul (fiat-rust/src/curve25519_32.rs:276:59)
  repro_fiat_rust_src_curve25519_32_rs_276_59_c61d57::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32], &amp;[0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, u32::MAX, 0u32, 600733470u32], &amp;[0u32, 0u32, 51u32, 0u32,
    u32::MAX, 0u32, 0u32, 0u32, 1u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:83:449:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:83:449)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_83_449_659844::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX,
    0u32, 91u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 100000000u32, 0u32, u32::MAX, 0u32,
    0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:622:123:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:622:123)
  repro_fiat_rust_src_p448_solinas_32_rs_622_123_b27be9::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_add(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    0u32, 0u32, 1u32, u32::MAX, 0u32 /* x9 */, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 1u32, 0u32, 1u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1163:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_add (fiat-rust/src/p448_solinas_32.rs:1163:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1163_17_a48820::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 1u64, 1u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([u64::MAX,
    0u64, 0u64, 0u64, 0u64, 0u64, 10000000000000000000u64, 255u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([255u64, 79u64,
    1000u64, 8u64, 4294967296u64, 7u64, 96u64, u64::MAX]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:319:20:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_64.rs:319:20)
  repro_fiat_rust_src_p448_solinas_64_rs_319_20_4a56f9::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_32::
    fiat_25519_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_sub(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 1u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 1u32, 0u32, 1000000000u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:664:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_sub (fiat-rust/src/curve25519_32.rs:664:17)
  repro_fiat_rust_src_curve25519_32_rs_664_17_ec052f::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x14 */, 1u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32,
    0u32 /* x14 */, u32::MAX]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([1u32,
    0u32 /* x14 */, u32::MAX]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:626:211:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:626:211)
  repro_fiat_rust_src_p448_solinas_32_rs_626_211_425815::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1u32, 0u32, 1u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_mul(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32,
    0u32 /* x8 */, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    0u32, 1000000000u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:280:59:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_mul (fiat-rust/src/curve25519_32.rs:280:59)
  repro_fiat_rust_src_curve25519_32_rs_280_59_d9beff::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x13 */, 1u32, 0u32, 0u32, 1u32,
    0u32]);
fiat_crypto::p521_32::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, u32::MAX, 1u32,
    1u32, 0u32 /* x10 */, 1u32, 0u32, 0u32, 1u32, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32 /* x17 */,
    0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1205:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_sub (fiat-rust/src/p521_32.rs:1205:18)
  repro_fiat_rust_src_p521_32_rs_1205_18_a1d0e7::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1u32, 0u32 /* x10 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_add(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, u32::MAX,
    1u32, 0u32 /* x9 */, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 1u32,
    0u32 /* x15 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1157:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_add (fiat-rust/src/p521_32.rs:1157:17)
  repro_fiat_rust_src_p521_32_rs_1157_17_fc1d50::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64,
    u64::MAX, 0u64, 0u64, 0u64, 1000000000000000000u64, 0u64, 0u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, u64::MAX, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:319:27:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_64.rs:319:27)
  repro_fiat_rust_src_p448_solinas_64_rs_319_27_122078::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry_square(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:776:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_square (fiat-rust/src/p521_32.rs:776:18)
  repro_fiat_rust_src_p521_32_rs_776_18_e838f6::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_opp(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1000000000u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1252:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_opp (fiat-rust/src/p448_solinas_32.rs:1252:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1252_17_1c2590::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x11 */, 1u32, 1u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry_square(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32 /* x14 */,
    u32::MAX, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:757:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_square (fiat-rust/src/p521_32.rs:757:18)
  repro_fiat_rust_src_p521_32_rs_757_18_aec7c1::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x8 */, 1u32, 0u32 /* x8 */,
    0u32]);
fiat_crypto::p521_32::fiat_p521_add(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32 /* x11 */,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 1u32,
    0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1167:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_add (fiat-rust/src/p521_32.rs:1167:18)
  repro_fiat_rust_src_p521_32_rs_1167_18_5c35bd::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry_mul(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64,
    10000000000000000000u64, 3391617135753145415u64, 2u64, 0u64, 2u64,
    12271124293904696155u64, 16593284771299102874u64, 255u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([4294967295u64, 255u64,
    7157680848120156213u64, 0u64, 1u64, 6834338645875110267u64, 32u64,
    1202058731829442497u64, 7734471760046062691u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:355:26:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry_mul (fiat-rust/src/p521_64.rs:355:26)
  repro_fiat_rust_src_p521_64_rs_355_26_3da5c9::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 1u32,
    0u32, 0u32, 1u32, 0u32 /* x8 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32,
    1000000000u32, u32::MAX, 0u32, 1u32, 0u32 /* x9 */, 1u32, 0u32, 0u32,
    0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1088:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry (fiat-rust/src/p521_32.rs:1088:17)
  repro_fiat_rust_src_p521_32_rs_1088_17_05bde1::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_sub(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    0u32, 0u32, 1u32, 0u32, u32::MAX, 0u32 /* x8 */, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32,
    0u32 /* x13 */, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1207:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_sub (fiat-rust/src/p448_solinas_32.rs:1207:18)
  repro_fiat_rust_src_p448_solinas_32_rs_1207_18_76516f::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[0u32, u32::MAX, 0u32, 0u32, 0u32, 1000000000u32,
    0u32, 1u32, 0u32, 0u32], &amp;[0u32, 0u32, 0u32, 0u32, 0u32, 1000000000u32,
    0u32, 0u32, 0u32, u32::MAX]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:75:35:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:75:35)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_75_35_7a1f61::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_64::
    fiat_25519_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_opp(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_tight_field_element([0u64, 0u64,
    u64::MAX, 1u64, 13u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:424:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_opp (fiat-rust/src/curve25519_64.rs:424:17)
  repro_fiat_rust_src_curve25519_64_rs_424_17_56e55b::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x8 */, 1u32, 0u32 /* x8 */,
    0u32]);
fiat_crypto::p521_32::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32 /* x17 */,
    0u32]), &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32,
    1000000000u32, 0u32 /* x12 */, 1u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1206:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_sub (fiat-rust/src/p521_32.rs:1206:17)
  repro_fiat_rust_src_p521_32_rs_1206_17_7df2aa::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_loose_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::poly1305_32::fiat_poly1305_add(
    &amp;mut fiat_poly1305_loose_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_tight_field_element([0u32, 0u32,
    0u32, u32::MAX, 0u32]),
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_tight_field_element([0u32, 0u32,
    0u32, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:383:17:
attempt to add with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_add (fiat-rust/src/poly1305_32.rs:383:17)
  repro_fiat_rust_src_poly1305_32_rs_383_17_f3a250::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([u32::MAX,
    0u32 /* x9 */, 1u32, 0u32, 0u32, 1000000000u32, 0u32, 1u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32,
    1000000000u32, 0u32 /* x12 */, u32::MAX, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:626:19:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:626:19)
  repro_fiat_rust_src_p448_solinas_32_rs_626_19_838bc2::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_square(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32,
    0u32 /* x11 */, u32::MAX, 1u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:744:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_square (fiat-rust/src/p448_solinas_32.rs:744:18)
  repro_fiat_rust_src_p448_solinas_32_rs_744_18_20d764::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 0u32,
    0u32, 0u32, u32::MAX, 0u32 /* x12 */, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32 /* x17 */,
    0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1209:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_sub (fiat-rust/src/p521_32.rs:1209:18)
  repro_fiat_rust_src_p521_32_rs_1209_18_857b10::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_square(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:451:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_square (fiat-rust/src/curve25519_32.rs:451:18)
  repro_fiat_rust_src_curve25519_32_rs_451_18_f7fa65::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64,
    u64::MAX, 0u64, 0u64, 0u64, 0u64, 0u64, 1000000000u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    u64::MAX, 0u64, 1000000000000000000u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:316:26:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_64.rs:316:26)
  repro_fiat_rust_src_p448_solinas_64_rs_316_26_22f484::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32, 1u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32], &amp;[0u32, 0u32, 0u32,
    u32::MAX, 0u32, 1000000000u32, 1u32, 0u32, 0u32, 0u32], &amp;[0u32, 0u32, 1u32,
    0u32, 0u32, 1u32, 1000000000u32, 8u32, 0u32, u32::MAX]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:89:34:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:89:34)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_89_34_3fb481::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([0u64, 0u64, 0u64, 1u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry_square(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64,
    u64::MAX, 65535u64, 15u64, 8479369336086608358u64, 91u64, 7u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:430:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry_square (fiat-rust/src/p521_64.rs:430:18)
  repro_fiat_rust_src_p521_64_rs_430_18_206087::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32, 0u32,
    0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32], &amp;[0u32, 0u32, 0u32, 0u32,
    u32::MAX, 0u32, 0u32, 1000000000u32, 0u32, 1u32], &amp;[0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 10u32, 0u32, 0u32, u32::MAX]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:95:34:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:95:34)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_95_34_9b1509::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_square(&amp;mut [0u64,
    0u64, 0u64, 0u64, 0u64], &amp;[0u64, 0u64, 0u64, u64::MAX,
    10409254604494641331u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_64.rs:117:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_square (fiat-rust/src/secp256k1_dettman_64.rs:117:17)
  repro_fiat_rust_src_secp256k1_dettman_64_rs_117_17_0e7af0::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    1u64, 1u64]);
fiat_crypto::p521_64::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64,
    1u64, u64::MAX, 0u64, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:613:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_sub (fiat-rust/src/p521_64.rs:613:18)
  repro_fiat_rust_src_p521_64_rs_613_18_4c7028::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    1u32, 0u32, 0u32, u32::MAX, 0u32 /* x9 */, 1000000000u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([1000000000u32,
    0u32 /* x9 */, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:625:59:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:625:59)
  repro_fiat_rust_src_p448_solinas_32_rs_625_59_4ab5a0::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_square(&amp;mut [0u64,
    0u64, 0u64, 0u64, 0u64], &amp;[u64::MAX, 41u64, 0u64, 16u64,
    6058503858619484894u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_64.rs:120:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_square (fiat-rust/src/secp256k1_dettman_64.rs:120:17)
  repro_fiat_rust_src_secp256k1_dettman_64_rs_120_17_2c5af3::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32 /* x16 */,
    1000000000u32, u32::MAX]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1103:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry (fiat-rust/src/p521_32.rs:1103:18)
  repro_fiat_rust_src_p521_32_rs_1103_18_8837b9::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_loose_field_element = fiat_crypto::poly1305_64::
    fiat_poly1305_loose_field_element([0u64, 1u64, 0u64]);
fiat_crypto::poly1305_64::fiat_poly1305_add(
    &amp;mut fiat_poly1305_loose_field_element,
    &amp;fiat_crypto::poly1305_64::fiat_poly1305_tight_field_element([0u64, 0u64,
    u64::MAX]),
    &amp;fiat_crypto::poly1305_64::fiat_poly1305_tight_field_element([0u64, 0u64,
    1u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_64.rs:383:17:
attempt to add with overflow
backtrace:
  fiat_crypto::poly1305_64::fiat_poly1305_add (fiat-rust/src/poly1305_64.rs:383:17)
  repro_fiat_rust_src_poly1305_64_rs_383_17_855435::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([1u32, 0u32,
    u32::MAX, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32,
    100u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32, 0u32, 0u32, u32::MAX,
    0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:627:35:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:627:35)
  repro_fiat_rust_src_p448_solinas_32_rs_627_35_5ab345::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 1u64, 1u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry_mul(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 1u64, 0u64,
    0u64, 1u64, 0u64, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 1u64, 10000000000000000000u64, 29u64, 7u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:276:61:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry_mul (fiat-rust/src/p521_64.rs:276:61)
  repro_fiat_rust_src_p521_64_rs_276_61_7f6ebc::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry_square(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, u32::MAX, 0u32,
    0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:782:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_square (fiat-rust/src/p521_32.rs:782:18)
  repro_fiat_rust_src_p521_32_rs_782_18_513138::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_carry_square(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64,
    10000000000000000000u64, 7331314549758190858u64, 7u64, 65535u64, 7u64,
    70u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:400:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_carry_square (fiat-rust/src/p448_solinas_64.rs:400:18)
  repro_fiat_rust_src_p448_solinas_64_rs_400_18_7628db::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[0u32, 0u32, u32::MAX, 0u32, 0u32, 1u32, 0u32,
    10000u32, 0u32, 0u32], &amp;[1000000000u32, 0u32, 0u32, 0u32, 0u32, u32::MAX,
    0u32, 0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:63:184:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:63:184)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_63_184_4bebfa::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_opp(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 0u32,
    0u32, 0u32, 1000000000u32, 0u32 /* x12 */, 1u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1258:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_opp (fiat-rust/src/p521_32.rs:1258:17)
  repro_fiat_rust_src_p521_32_rs_1258_17_583372::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x12 */, 1u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 1u32,
    0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, u32::MAX, 0u32, 1000000000u32,
    0u32, 0u32, 0u32, 3553025663u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([3692082540u32,
    0u32, 0u32, 0u32, 1000000000u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 1073741824u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:625:107:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:625:107)
  repro_fiat_rust_src_p448_solinas_32_rs_625_107_252694::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32 /* x13 */, 1u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 1000000000u32, 1u32, 0u32, 0u32,
    0u32, u32::MAX, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 1000000000u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:626:91:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:626:91)
  repro_fiat_rust_src_p448_solinas_32_rs_626_91_fc5639::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x11 */, 1u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry_mul(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32 /* x17 */,
    0u32]), &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32,
    u32::MAX, 0u32, 0u32, 1u32, 1u32, 0u32, 1u32, 0u32 /* x8 */, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:308:61:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_mul (fiat-rust/src/p521_32.rs:308:61)
  repro_fiat_rust_src_p521_32_rs_308_61_2c44c8::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 0u32,
    u32::MAX, 0u32 /* x14 */, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32 /* x17 */,
    0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1207:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_sub (fiat-rust/src/p521_32.rs:1207:18)
  repro_fiat_rust_src_p521_32_rs_1207_18_d733b8::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [1u32,
    0u32 /* x8 */, 0u32], &amp;[0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32,
    1000000000u32, 0u32, 0u32, 0u32], &amp;[u32::MAX, 0u32, 0u32, 0u32, 0u32,
    1073741824u32, 0u32, 0u32, 1u32, 1u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:92:18:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:92:18)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_92_18_24258c::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 1u64, 0u64,
    1u64, 0u64, 0u64, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64,
    1000000000000000000u64, 1u64, 0u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:612:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_sub (fiat-rust/src/p521_64.rs:612:17)
  repro_fiat_rust_src_p521_64_rs_612_17_3ac5b5::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32 /* x8 */, 1u32, 0u32, 1u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_opp(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1251:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_opp (fiat-rust/src/p448_solinas_32.rs:1251:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1251_17_e6c41e::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x8 */, 1u32, 0u32 /* x8 */,
    0u32]);
fiat_crypto::p521_32::fiat_p521_carry_square(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 1u32, 1u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:777:18:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_square (fiat-rust/src/p521_32.rs:777:18)
  repro_fiat_rust_src_p521_32_rs_777_18_c80986::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([1u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_add(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 1u64, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, u64::MAX, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:585:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_add (fiat-rust/src/p521_64.rs:585:17)
  repro_fiat_rust_src_p521_64_rs_585_17_b5d8c0::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64,
    1000000000000000000u64, 0u64, 0u64, 0u64, 0u64, 0u64, 1u64, 1u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:610:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_sub (fiat-rust/src/p521_64.rs:610:17)
  repro_fiat_rust_src_p521_64_rs_610_17_57a38c::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_64::
    fiat_25519_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_sub(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_tight_field_element([0u64, 0u64,
    0u64, 0u64, 0u64]),
    &amp;fiat_crypto::curve25519_64::fiat_25519_tight_field_element(
    [1000000000000000000u64, 0u64, 0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:401:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_sub (fiat-rust/src/curve25519_64.rs:401:17)
  repro_fiat_rust_src_curve25519_64_rs_401_17_91adbd::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_32::
    fiat_25519_loose_field_element([0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 1u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_sub(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 1u32,
    u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 0u32,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:659:18:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_sub (fiat-rust/src/curve25519_32.rs:659:18)
  repro_fiat_rust_src_curve25519_32_rs_659_18_bc7b98::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_sub(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 1u64,
    0u64, 0u64, 1u64, 0u64, 0u64, 0u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    u64::MAX, 3342601094461837497u64, 4418826780289787915u64,
    12191843333304873439u64, 7u64, 17843022385921707698u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:593:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_sub (fiat-rust/src/p448_solinas_64.rs:593:17)
  repro_fiat_rust_src_p448_solinas_64_rs_593_17_386d99::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32], &amp;[0u32, 1000000000u32,
    0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32], &amp;[0u32, 0u32, 0u32,
    0u32, u32::MAX, 0u32, 1000000000u32, 0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:63:101:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:63:101)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_63_101_c5d59e::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x12 */, 1u32, 0u32, 1u32, 0u32,
    0u32, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry_square(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32, 0u32,
    0u32, 1u32, 0u32 /* x11 */, u32::MAX, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:754:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_square (fiat-rust/src/p521_32.rs:754:17)
  repro_fiat_rust_src_p521_32_rs_754_17_983ee6::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, u64::MAX,
    u64::MAX, u64::MAX, u64::MAX, 65535u64, 77u64, 3718242967858612509u64,
    9575840534231439296u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:542:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry (fiat-rust/src/p521_64.rs:542:17)
  repro_fiat_rust_src_p521_64_rs_542_17_3f6cfb::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_add(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([1u64, 0u64,
    u64::MAX, u64::MAX, 33u64, 8u64, 255u64, 6783989175344568052u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element(
    [1071995610357239386u64, 8u64, 16183568768227408170u64, 33u64,
    4674527400073804740u64, 7791641835520423956u64, 58u64, 7u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:566:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_add (fiat-rust/src/p448_solinas_64.rs:566:17)
  repro_fiat_rust_src_p448_solinas_64_rs_566_17_7315d9::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry_mul(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32 /* x16 */,
    1u32, 0u32]), &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32,
    0u32 /* x10 */, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:299:61:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_mul (fiat-rust/src/p521_32.rs:299:61)
  repro_fiat_rust_src_p521_32_rs_299_61_5ce1a2::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1u32, 0u32, 0u32]);
fiat_crypto::p521_32::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 0u32,
    1u32, 0u32, 0u32, 1u32, 0u32 /* x11 */, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([1000000000u32, 0u32,
    0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1204:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_sub (fiat-rust/src/p521_32.rs:1204:17)
  repro_fiat_rust_src_p521_32_rs_1204_17_f13c19::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 1u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_opp(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64,
    10000000000000000000u64, 26u64, 18061200934070259849u64, 10u64,
    16436676763281053554u64, 8u64, 7707047541251958929u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:619:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_opp (fiat-rust/src/p448_solinas_64.rs:619:17)
  repro_fiat_rust_src_p448_solinas_64_rs_619_17_649910::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_32::
    fiat_25519_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_opp(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 0u32,
    0u32, 0u32, 1u32, 1000000000u32, 0u32, 0u32, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:693:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_opp (fiat-rust/src/curve25519_32.rs:693:17)
  repro_fiat_rust_src_curve25519_32_rs_693_17_b85163::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_add(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    0u64, 0u64, 0u64, u64::MAX, 0u64, 0u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    0u64, 0u64, 0u64, 1u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:569:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_add (fiat-rust/src/p448_solinas_64.rs:569:17)
  repro_fiat_rust_src_p448_solinas_64_rs_569_17_9ab8f7::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_loose_field_element = fiat_crypto::poly1305_64::
    fiat_poly1305_loose_field_element([0u64, 0u64, 0u64]);
fiat_crypto::poly1305_64::fiat_poly1305_add(
    &amp;mut fiat_poly1305_loose_field_element,
    &amp;fiat_crypto::poly1305_64::fiat_poly1305_tight_field_element([u64::MAX,
    0u64, 0u64]),
    &amp;fiat_crypto::poly1305_64::fiat_poly1305_tight_field_element([1u64, 0u64,
    0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_64.rs:381:17:
attempt to add with overflow
backtrace:
  fiat_crypto::poly1305_64::fiat_poly1305_add (fiat-rust/src/poly1305_64.rs:381:17)
  repro_fiat_rust_src_poly1305_64_rs_381_17_ee5be6::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32], &amp;[0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, u32::MAX, 0u32, 5u32, 0u32], &amp;[u32::MAX, 0u32, u32::MAX, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:69:533:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:69:533)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_69_533_bf9a81::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry_square(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 10000000000000000000u64, 2315927608592840551u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:421:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry_square (fiat-rust/src/p521_64.rs:421:17)
  repro_fiat_rust_src_p521_64_rs_421_17_f95bba::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_64::
    fiat_poly1305_tight_field_element([0u64, 0u64, 0u64]);
fiat_crypto::poly1305_64::fiat_poly1305_carry_mul(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([0u64, 0u64,
    0u64]), &amp;fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([0u64,
    0u64, 10000000000000000000u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_64.rs:274:61:
attempt to multiply with overflow
backtrace:
  fiat_crypto::poly1305_64::fiat_poly1305_carry_mul (fiat-rust/src/poly1305_64.rs:274:61)
  repro_fiat_rust_src_poly1305_64_rs_274_61_5056a2::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 1000000000000000000u64, u64::MAX]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:548:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry (fiat-rust/src/p521_64.rs:548:17)
  repro_fiat_rust_src_p521_64_rs_548_17_6de34e::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_loose_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::poly1305_32::fiat_poly1305_sub(
    &amp;mut fiat_poly1305_loose_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32]),
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_tight_field_element([0u32, 0u32,
    0u32, 1000000000u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:404:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_sub (fiat-rust/src/poly1305_32.rs:404:17)
  repro_fiat_rust_src_poly1305_32_rs_404_17_908691::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32 /* x12 */, 1u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_sub(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    1u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([550847972u32,
    0u32, 1u32, 0u32 /* x12 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1201:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_sub (fiat-rust/src/p448_solinas_32.rs:1201:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1201_17_773528::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_carry_mul(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32 /* x17 */,
    0u32]), &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32,
    0u32, u32::MAX, 0u32 /* x14 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:289:61:
attempt to multiply with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry_mul (fiat-rust/src/p521_32.rs:289:61)
  repro_fiat_rust_src_p521_32_rs_289_61_ded9fd::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_opp(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64,
    10000000000000000000u64, 67u64, 12148215332203628892u64,
    14123133107686034899u64, 16268969613788349593u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:641:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_opp (fiat-rust/src/p521_64.rs:641:17)
  repro_fiat_rust_src_p521_64_rs_641_17_beab05::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x14 */, 1u32, 0u32, 0u32,
    0u32]);
fiat_crypto::p521_32::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, u32::MAX,
    0u32 /* x13 */, 1u32, 1u32, 1u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([1u32, 0u32 /* x17 */,
    0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1206:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_sub (fiat-rust/src/p521_32.rs:1206:18)
  repro_fiat_rust_src_p521_32_rs_1206_18_390d76::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32 /* x9 */,
    u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32 /* x17 */,
    0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1214:19:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_sub (fiat-rust/src/p521_32.rs:1214:19)
  repro_fiat_rust_src_p521_32_rs_1214_19_a38db7::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::
    fiat_p521_tight_field_element([0u32, 0u32, 0u32, 1u32, 0u32 /* x14 */,
    0u32]);
fiat_crypto::p521_32::fiat_p521_carry(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32 /* x15 */,
    1000000000u32, u32::MAX, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1102:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_carry (fiat-rust/src/p521_32.rs:1102:18)
  repro_fiat_rust_src_p521_32_rs_1102_18_806ebd::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_64::
    fiat_poly1305_tight_field_element([0u64, 0u64, 0u64]);
fiat_crypto::poly1305_64::fiat_poly1305_carry_mul(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([0u64, 0u64,
    0u64]), &amp;fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([1u64,
    10000000000000000000u64, 90u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_64.rs:275:61:
attempt to multiply with overflow
backtrace:
  fiat_crypto::poly1305_64::fiat_poly1305_carry_mul (fiat-rust/src/poly1305_64.rs:275:61)
  repro_fiat_rust_src_poly1305_64_rs_275_61_5c267a::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::poly1305_32::fiat_poly1305_carry(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_loose_field_element([1000000000u32,
    u32::MAX, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:353:17:
attempt to add with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_carry (fiat-rust/src/poly1305_32.rs:353:17)
  repro_fiat_rust_src_poly1305_32_rs_353_17_cb81f0::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_64::
    fiat_poly1305_tight_field_element([0u64, 0u64, 0u64]);
fiat_crypto::poly1305_64::fiat_poly1305_carry_square(
    &amp;mut fiat_poly1305_tight_field_element,
    &amp;fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([u64::MAX,
    4611686018427387904u64, 1u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_64.rs:326:19:
attempt to add with overflow
backtrace:
  fiat_crypto::poly1305_64::fiat_poly1305_carry_square (fiat-rust/src/poly1305_64.rs:326:19)
  repro_fiat_rust_src_poly1305_64_rs_326_19_44a260::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([1u32, 0u32 /* x14 */, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_sub(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([1u32, 0u32,
    0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 1u32, 0u32, 0u32, 0u32,
    0u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32,
    0u32 /* x13 */, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1204:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_sub (fiat-rust/src/p448_solinas_32.rs:1204:18)
  repro_fiat_rust_src_p448_solinas_32_rs_1204_18_4a4f5d::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_mul(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:275:59:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_mul (fiat-rust/src/curve25519_32.rs:275:59)
  repro_fiat_rust_src_curve25519_32_rs_275_59_5ba1a5::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1000000000u32,
    0u32, 0u32, u32::MAX], &amp;[0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32,
    1000000000u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:95:200:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:95:200)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_95_200_22a3d8::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_64::
    fiat_25519_tight_field_element([0u64, 1u64, 1u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_carry(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_loose_field_element([0u64,
    100000000000000000u64, u64::MAX, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:354:17:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_carry (fiat-rust/src/curve25519_64.rs:354:17)
  repro_fiat_rust_src_curve25519_64_rs_354_17_9c4f33::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_poly1305_loose_field_element = fiat_crypto::poly1305_32::
    fiat_poly1305_loose_field_element([0u32, 0u32, 0u32, 1u32, 0u32]);
fiat_crypto::poly1305_32::fiat_poly1305_sub(
    &amp;mut fiat_poly1305_loose_field_element,
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32]),
    &amp;fiat_crypto::poly1305_32::fiat_poly1305_tight_field_element([1000000000u32,
    0u32, 0u32, 1u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/poly1305_32.rs:401:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::poly1305_32::fiat_poly1305_sub (fiat-rust/src/poly1305_32.rs:401:17)
  repro_fiat_rust_src_poly1305_32_rs_401_17_2d2a7d::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32,
    0u32 /* x8 */, 1000000000u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1100:17:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry (fiat-rust/src/p448_solinas_32.rs:1100:17)
  repro_fiat_rust_src_p448_solinas_32_rs_1100_17_68294f::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32, 0u32,
    0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32], &amp;[0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, u32::MAX, 100000000u32], &amp;[0u32, 0u32, 0u32,
    10000u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:89:449:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:89:449)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_89_449_6af8e0::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_32::
    fiat_25519_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_sub(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 0u32,
    0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 1u32, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 1u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:661:18:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_sub (fiat-rust/src/curve25519_32.rs:661:18)
  repro_fiat_rust_src_curve25519_32_rs_661_18_6f8733::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_tight_field_element([0u32, 0u32, 0u32, 1u32, 0u32 /* x11 */,
    0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(
    &amp;mut fiat_p448_tight_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32,
    1u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32, 0u32, 0u32, u32::MAX, 0u32,
    0u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([1u32,
    1000000000u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32,
    0u32, u32::MAX, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:622:67:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_carry_mul (fiat-rust/src/p448_solinas_32.rs:622:67)
  repro_fiat_rust_src_p448_solinas_32_rs_622_67_0c9c9e::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x8 */, 1u32, 0u32, 0u32, 0u32,
    0u32, 1u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::p521_32::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32 /* x17 */,
    0u32]), &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32,
    0u32, 0u32, 1000000000u32, 0u32 /* x10 */, 1u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1208:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_sub (fiat-rust/src/p521_32.rs:1208:17)
  repro_fiat_rust_src_p521_32_rs_1208_17_530318::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::
    fiat_p521_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_carry_mul(&amp;mut fiat_p521_tight_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element([1u64, 0u64, 1u64,
    u64::MAX, u64::MAX, 14u64, 1u64, 1u64, 11u64]),
    &amp;fiat_crypto::p521_64::fiat_p521_loose_field_element(
    [18446744073709551614u64, 4294967296u64, 4294967295u64, 1u64, 4294967295u64,
    255u64, 8u64, 14u64, 65535u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:362:40:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_carry_mul (fiat-rust/src/p521_64.rs:362:40)
  repro_fiat_rust_src_p521_64_rs_362_40_db32f3::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_mul(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 1u32, 1u32, 0u32, 0u32, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 682631082u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:278:59:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_mul (fiat-rust/src/curve25519_32.rs:278:59)
  repro_fiat_rust_src_curve25519_32_rs_278_59_960f2c::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32 /* x17 */, 0u32]);
fiat_crypto::p521_32::fiat_p521_opp(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 1u32, 0u32,
    0u32, 1u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32 /* x9 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1261:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_opp (fiat-rust/src/p521_32.rs:1261:17)
  repro_fiat_rust_src_p521_32_rs_1261_17_bbea6d::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_32::
    fiat_p521_loose_field_element([0u32, 0u32, 0u32, 1u32, 0u32 /* x12 */, 1u32,
    0u32, 0u32]);
fiat_crypto::p521_32::fiat_p521_sub(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32 /* x9 */, 0u32]),
    &amp;fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32 /* x9 */,
    1u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_32.rs:1212:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p521_32::fiat_p521_sub (fiat-rust/src/p521_32.rs:1212:18)
  repro_fiat_rust_src_p521_32_rs_1212_18_f29e04::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_opp(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64,
    0u64, 0u64, 0u64, 10000000000000000000u64, 1u64, 1000u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:644:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_opp (fiat-rust/src/p521_64.rs:644:17)
  repro_fiat_rust_src_p521_64_rs_644_17_50b50c::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32, 0u32,
    1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32], &amp;[0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, u32::MAX, 0u32, 10000u32], &amp;[0u32, 100000000u32, 0u32,
    u32::MAX, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:75:533:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:75:533)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_75_533_9a42c4::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1000000000u32, u32::MAX, 0u32], &amp;[u32::MAX, 1000000000u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:69:616:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:69:616)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_69_616_98ee57::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::
    fiat_25519_tight_field_element([0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_carry_mul(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32,
    0u32 /* x8 */, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32,
    1000000000u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:282:59:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_carry_mul (fiat-rust/src/curve25519_32.rs:282:59)
  repro_fiat_rust_src_curve25519_32_rs_282_59_ff4b28::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_64::
    fiat_25519_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_carry_square(
    &amp;mut fiat_25519_tight_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_loose_field_element([0u64, 0u64,
    0u64, 10000000000000000000u64, 256u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:289:17:
attempt to multiply with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_carry_square (fiat-rust/src/curve25519_64.rs:289:17)
  repro_fiat_rust_src_curve25519_64_rs_289_17_850053::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_32::
    fiat_p448_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
fiat_crypto::p448_solinas_32::fiat_p448_sub(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32,
    0u32, 0u32, u32::MAX, 0u32 /* x10 */, 0u32]),
    &amp;fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32,
    0u32 /* x14 */, 0u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_32.rs:1205:18:
attempt to add with overflow
backtrace:
  fiat_crypto::p448_solinas_32::fiat_p448_sub (fiat-rust/src/p448_solinas_32.rs:1205:18)
  repro_fiat_rust_src_p448_solinas_32_rs_1205_18_d64839::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_mul(&amp;mut [0u64, 0u64,
    0u64, 0u64, 0u64], &amp;[u64::MAX, 0u64, 0u64, 1u64, 0u64], &amp;[0u64, 0u64,
    u64::MAX, 0u64, 1000000000000000000u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_64.rs:86:19:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_64.rs:86:19)
  repro_fiat_rust_src_secp256k1_dettman_64_rs_86_19_673c10::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_64::
    fiat_25519_loose_field_element([0u64, 0u64, 1u64, 0u64, 0u64]);
fiat_crypto::curve25519_64::fiat_25519_add(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_64::fiat_25519_tight_field_element([1u64, 0u64,
    0u64, 0u64, 0u64]),
    &amp;fiat_crypto::curve25519_64::fiat_25519_tight_field_element([u64::MAX, 0u64,
    0u64, 0u64, 0u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_64.rs:380:17:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_64::fiat_25519_add (fiat-rust/src/curve25519_64.rs:380:17)
  repro_fiat_rust_src_curve25519_64_rs_380_17_14de78::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_32::
    fiat_25519_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32,
    0u32, 0u32, 0u32]);
fiat_crypto::curve25519_32::fiat_25519_sub(&amp;mut fiat_25519_loose_field_element,
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32]),
    &amp;fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 1u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/curve25519_32.rs:664:18:
attempt to add with overflow
backtrace:
  fiat_crypto::curve25519_32::fiat_25519_sub (fiat-rust/src/curve25519_32.rs:664:18)
  repro_fiat_rust_src_curve25519_32_rs_664_18_7819e0::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::
    fiat_p448_loose_field_element([0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 0u64,
    0u64]);
fiat_crypto::p448_solinas_64::fiat_p448_sub(&amp;mut fiat_p448_loose_field_element,
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    0u64, 0u64, 0u64, 0u64, 0u64, 1u64]),
    &amp;fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64,
    0u64, u64::MAX, 2u64, 1u64, 65535u64, 58u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p448_solinas_64.rs:594:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p448_solinas_64::fiat_p448_sub (fiat-rust/src/p448_solinas_64.rs:594:17)
  repro_fiat_rust_src_p448_solinas_64_rs_594_17_3f9839::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&amp;mut [0u32,
    0u32 /* x8 */, 0u32], &amp;[0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    1000000000u32, u32::MAX], &amp;[0u32, 0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX,
    100000000u32, 0u32, 0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/secp256k1_dettman_32.rs:107:200:
attempt to add with overflow
backtrace:
  fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul (fiat-rust/src/secp256k1_dettman_32.rs:107:200)
  repro_fiat_rust_src_secp256k1_dettman_32_rs_107_200_1bd899::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::
    fiat_p521_loose_field_element([0u64, 1u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    0u64, 0u64]);
fiat_crypto::p521_64::fiat_p521_opp(&amp;mut fiat_p521_loose_field_element,
    &amp;fiat_crypto::p521_64::fiat_p521_tight_field_element([u64::MAX, 1u64, 1u64,
    13790311855668357254u64, 4294967295u64, 0u64, 1u64, 96u64, 41u64]));</pre></td><td valign="top"><pre>thread 'main' panicked at fiat-rust/src/p521_64.rs:638:17:
attempt to subtract with overflow
backtrace:
  fiat_crypto::p521_64::fiat_p521_opp (fiat-rust/src/p521_64.rs:638:17)
  repro_fiat_rust_src_p521_64_rs_638_17_9c050c::main (src/main.rs:3:5)</pre></td></tr>
</table>
