# secp256k1 - crashes

Source: https://git.rust-bitcoin.org/rust-bitcoin/rust-secp256k1 @ ef04e0bd5928ff3fcb1834ed56e465882cdda385

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = secp256k1::ecdsa::sign_grind_r(Message::from_digest([0u8, 1u8,
    0u8 /* x8 */, 1u8, 0u8, 0u8, 2u8, 0u8, 255u8, 38u8, 2u8, 60u8, 36u8, 255u8,
    152u8, 15u8, 232u8, 1u8, 1u8, 222u8, 64u8, 74u8, 255u8, 16u8, 66u8]),
    &amp;SecretKey::from_secret_bytes([1u8, 15u8, 255u8, 255u8, 232u8, 0u8, 2u8,
    205u8, 255u8, 192u8, 114u8, 16u8, 56u8, 255u8, 40u8, 1u8, 0u8, 16u8, 255u8,
    16u8, 1u8, 11u8, 1u8, 28u8, 1u8, 99u8, 0u8, 191u8, 255u8, 255u8, 0u8,
    45u8]).unwrap(), 9usize);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  rustsecp256k1_v0_14_u128_accum_mul (secp256k1/src/int128_native_impl.h:16)
  rustsecp256k1_v0_14_fe_mul_inner (secp256k1/src/field_5x52_int128_impl.h:62)
  rustsecp256k1_v0_14_fe_impl_mul (secp256k1/src/field_5x52_impl.h:314)
  rustsecp256k1_v0_14_gej_add_ge (secp256k1/src/group_impl.h:816)
  rustsecp256k1_v0_14_ecmult_gen_gej (secp256k1/src/ecmult_gen_impl.h:264)
  rustsecp256k1_v0_14_ecmult_gen_ge (secp256k1/src/ecmult_gen_impl.h:286)
  rustsecp256k1_v0_14_ecmult_gen_blind (secp256k1/src/ecmult_gen_impl.h:336)
  rustsecp256k1_v0_14_context_randomize (secp256k1/src/secp256k1.c:738)
  secp256k1::Secp256k1&lt;secp256k1::context::alloc_only::All&gt;::seeded_randomize&lt;secp256k1::context::alloc_only::All&gt; (src/lib.rs:390)
  secp256k1::context::internal::rerandomize_global_context::{closure#0} (src/context/internal_std.rs:82)
  secp256k1::context::internal::rerandomize_global_context (src/context/internal_std.rs:68)
  secp256k1::context::internal::with_raw_global_context::{closure#0}&lt;i32, secp256k1::context::internal::with_global_context::{closure_env#0}&lt;i32, secp256k1::context::AllPreallocated, secp256k1::ecdsa::sign_grind_with_check::{closure_env#0}&lt;secp256k1::Message, secp256k1::ecdsa::sign_grind_r::{closure... (+73 chars)
  secp256k1::context::internal::with_raw_global_context&lt;i32, secp256k1::context::internal::with_global_context::{closure_env#0}&lt;i32, secp256k1::context::AllPreallocated, secp256k1::ecdsa::sign_grind_with_check::{closure_env#0}&lt;secp256k1::Message, secp256k1::ecdsa::sign_grind_r::{closure_env#0}&lt;secp2... (+60 chars)
  secp256k1::context::internal::with_global_context&lt;i32, secp256k1::context::AllPreallocated, secp256k1::ecdsa::sign_grind_with_check::{closure_env#0}&lt;secp256k1::Message, secp256k1::ecdsa::sign_grind_r::{closure_env#0}&lt;secp256k1::Message&gt;&gt;&gt; (secp256k1/src/context/internal_std.rs:29)
  secp256k1::ecdsa::sign_grind_with_check&lt;secp256k1::Message, secp256k1::ecdsa::sign_grind_r::{closure_env#0}&lt;secp256k1::Message&gt;&gt; (secp256k1/src/ecdsa/mod.rs:312)
  secp256k1::ecdsa::sign_grind_r&lt;secp256k1::Message&gt; (secp256k1/src/ecdsa/mod.rs:358)
  repro_timeout_in_secp256k1_ecdsa_sign_grind_r_d32b2e::main (src/main.rs:5)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = secp256k1::ecdsa::sign_grind_r(Message::from_digest([0u8, 0u8 /* x30 */,
    0u8]), &amp;SecretKey::from_byte_array([0u8, 0u8 /* x15 */, 1u8, 0u8 /* x14 */,
    0u8]).unwrap(), usize::MAX).serialize_der();</pre></td><td valign="top"><pre>thread 'main' panicked at rust-secp256k1/src/ecdsa/mod.rs:356:5:
assertion failed: bytes_to_grind &lt;= 71
backtrace:
  secp256k1::ecdsa::sign_grind_r::&lt;secp256k1::Message&gt; (rust-secp256k1/src/ecdsa/mod.rs:356:5)
  repro_rust_secp256k1_src_ecdsa_mod_rs_356_5_c8e0e1::main (src/main.rs:5:13)</pre></td></tr>
</table>
