# bs58 - crashes

Source: https://github.com/Nullus157/bs58-rs @ e4a65a9ec641f26b61fb20518b51e83efd6352a0

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let alphabet = Alphabet::FLICKR;
let encode_builder = &lt;bs58::encode::EncodeBuilder&lt;Vec&lt;u8&gt;&gt;&gt;::new(include_bytes!(
    "data_0.bin").to_vec(), alphabet);
let _ = encode_builder.onto(include_bytes!("Cargo.toml").to_vec());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  bs58::encode::encode_into&lt;&amp;[u8]&gt; (bs58-rs/src/encode.rs:443)
  bs58::encode::{impl#5}::onto::{closure#0}&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (bs58-rs/src/encode.rs:405)
  bs58::encode::{impl#1}::encode_with&lt;bs58::encode::{impl#5}::onto::{closure_env#0}&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (bs58-rs/src/encode.rs:65)
  bs58::encode::EncodeBuilder&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::onto&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (bs58-rs/src/encode.rs:404)
  repro_timeout_in_bs58_encode_encodebuilder_vec_u8_onto_vec_u8_ec3030::main (src/main.rs:6)</pre></td></tr>
<tr><td valign="top"><pre>let encode_builder = bs58::encode::&lt;Vec&lt;u8&gt;&gt;(include_bytes!(
    "data_0.bin").to_vec());
let encode_builder2 = encode_builder.as_cb58(Some(0u8));
let _ = encode_builder2.into_vec();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  bs58::encode::encode_into&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::option::Iter&lt;u8&gt;, core::slice::iter::Iter&lt;u8&gt;&gt;, core::slice::iter::Iter&lt;u8&gt;&gt;&gt; (src/encode.rs:444)
  bs58::encode::encode_cb58_into (src/encode.rs:514)
  bs58::encode::{impl#9}::onto::{closure#4}&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, &amp;mut alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (bs58-rs/src/encode.rs:418)
  bs58::encode::{impl#1}::encode_with&lt;bs58::encode::{impl#9}::onto::{closure_env#4}&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, &amp;mut alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (bs58-rs/src/encode.rs:65)
  bs58::encode::{impl#0}::encode_with&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, bs58::encode::{impl#9}::onto::{closure_env#4}&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, &amp;mut alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (bs58-rs/src/encode.rs:52)
  bs58::encode::EncodeBuilder&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::onto&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, &amp;mut alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (bs58-rs/src/encode.rs:417)
  bs58::encode::EncodeBuilder&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::into_vec&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (bs58-rs/src/encode.rs:332)
  repro_timeout_in_bs58_encode_encodebuilder_vec_u8_into_vec_6bf676::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let encode_builder = bs58::encode::&lt;Vec&lt;u8&gt;&gt;(include_bytes!(
    "data_0.bin").to_vec());
let _ = encode_builder.onto("(\\)".to_string());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  bs58::encode::encode_into&lt;&amp;[u8]&gt; (bs58-rs/src/encode.rs:444)
  bs58::encode::{impl#5}::onto::{closure#0}&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, alloc::string::String&gt; (bs58-rs/src/encode.rs:405)
  bs58::encode::{impl#1}::encode_with&lt;bs58::encode::{impl#5}::onto::{closure_env#0}&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, alloc::string::String&gt;&gt; (bs58-rs/src/encode.rs:65)
  bs58::encode::{impl#2}::encode_with&lt;bs58::encode::{impl#5}::onto::{closure_env#0}&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, alloc::string::String&gt;&gt; (bs58-rs/src/encode.rs:146)
  bs58::encode::EncodeBuilder&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::onto&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, alloc::string::String&gt; (bs58-rs/src/encode.rs:404)
  repro_timeout_in_bs58_encode_encodebuilder_vec_u8_onto_string_0d206e::main (src/main.rs:3)</pre></td></tr>
<tr><td valign="top"><pre>let error = Error::NonAsciiCharacter { index: 0usize };
let _ = error.unwrap_const();</pre></td><td valign="top"><pre>thread 'main' panicked at bs58-rs/src/decode.rs:657:17:
provided string contained non-ascii character
backtrace:
  &lt;bs58::decode::Error&gt;::unwrap_const (bs58-rs/src/decode.rs:657:17)
  repro_bs58_rs_src_decode_rs_657_17_235d57::main (src/main.rs:5:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Alphabet::new_unwrap(&amp;[0u8, 0u8, 0u8, 1u8, 0u8 /* x13 */, 1u8, 0u8, 0u8,
    1u8, 0u8 /* x10 */, 1u8, 0u8 /* x25 */, 0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at bs58-rs/src/alphabet.rs:156:9:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;bs58::alphabet::Alphabet&gt;::new_unwrap (bs58-rs/src/alphabet.rs:156:9)
  repro_bs58_rs_src_alphabet_rs_156_9_1e2005::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let error = Error::InvalidCharacter { character: 'a', index: 0usize };
let _ = error.unwrap_const();</pre></td><td valign="top"><pre>thread 'main' panicked at bs58-rs/src/decode.rs:655:47:
provided string contained invalid character
backtrace:
  &lt;bs58::decode::Error&gt;::unwrap_const (bs58-rs/src/decode.rs:655:47)
  repro_bs58_rs_src_decode_rs_655_47_ceb415::main (src/main.rs:5:19)</pre></td></tr>
<tr><td valign="top"><pre>let error = Error::InvalidVersion { ver: 0u8, expected_ver: 0u8 };
let _ = error.unwrap_const();</pre></td><td valign="top"><pre>thread 'main' panicked at bs58-rs/src/decode.rs:662:45:
invalid version
backtrace:
  &lt;bs58::decode::Error&gt;::unwrap_const (bs58-rs/src/decode.rs:662:45)
  repro_bs58_rs_src_decode_rs_662_45_187b9a::main (src/main.rs:5:19)</pre></td></tr>
<tr><td valign="top"><pre>let error = Error::NoChecksum;
let _ = error.unwrap_const();</pre></td><td valign="top"><pre>thread 'main' panicked at bs58-rs/src/decode.rs:664:34:
provided string is too small to contain a checksum
backtrace:
  &lt;bs58::decode::Error&gt;::unwrap_const (bs58-rs/src/decode.rs:664:34)
  repro_bs58_rs_src_decode_rs_664_34_d0d8f0::main (src/main.rs:5:19)</pre></td></tr>
<tr><td valign="top"><pre>let error = Error::BufferTooSmall;
let _ = error.unwrap_const();</pre></td><td valign="top"><pre>thread 'main' panicked at bs58-rs/src/decode.rs:653:17:
buffer provided to decode base58 encoded string into was too small
backtrace:
  &lt;bs58::decode::Error&gt;::unwrap_const (bs58-rs/src/decode.rs:653:17)
  repro_bs58_rs_src_decode_rs_653_17_022a77::main (src/main.rs:5:19)</pre></td></tr>
<tr><td valign="top"><pre>let error = Error::InvalidChecksum { checksum: [0u8, 0u8, 0u8, 0u8],
    expected_checksum: [0u8, 0u8, 0u8, 0u8] };
let _ = error.unwrap_const();</pre></td><td valign="top"><pre>thread 'main' panicked at bs58-rs/src/decode.rs:660:46:
invalid checksum
backtrace:
  &lt;bs58::decode::Error&gt;::unwrap_const (bs58-rs/src/decode.rs:660:46)
  repro_bs58_rs_src_decode_rs_660_46_01b8b3::main (src/main.rs:5:19)</pre></td></tr>
</table>
