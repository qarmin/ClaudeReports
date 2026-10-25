# pbkdf2 - crashes

Source: https://github.com/RustCrypto/password-hashes @ 958527c7d855e1687e28f9e592e537003833ed1e

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let params = Params::recommended_for(Algorithm::default());
let algorithm = Algorithm::Pbkdf2Sha256;
pbkdf2::pbkdf2_hmac_with_params(&amp;vec![], &amp;vec![], algorithm, params,
    &amp;mut include_bytes!("image_0.webp").to_vec());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  sha2::sha256::x86_sha::load_state (src/sha256/x86_sha.rs:61)
  sha2::sha256::x86_sha::compress (src/sha256/x86_sha.rs:76)
  sha2::sha256::compress (src/sha256.rs:62)
  sha2::sha256::compress256 (src/sha256.rs:82)
  sha2::block_api::{impl#5}::finalize_variable_core::{closure#0} (sha2/src/block_api.rs:67)
  block_buffer::BlockBuffer&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UTerm, typenum::bit::B1&gt;, typenum::bit::B0&gt;, typenum::bit::B0&gt;, typenum::bit::B0&gt;, typenum::bit::B0&gt;, typenum::bit::B... (+448 chars)
  block_buffer::BlockBuffer&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UTerm, typenum::bit::B1&gt;, typenum::bit::B0&gt;, typenum::bit::B0&gt;, typenum::bit::B0&gt;, typenum::bit::B0&gt;, typenum::bit::B... (+454 chars)
  sha2::block_api::{impl#5}::finalize_variable_core (sha2/src/block_api.rs:67)
  digest::block_api::ct_variable::{impl#8}::finalize_fixed_core&lt;sha2::block_api::Sha256VarCore, typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UInt&lt;typenum::uint::UTerm, typenum::bit::B1&gt;, typenum::bit::B0&gt;, typenum::bit::B0&gt;, typen... (+93 chars)
  hmac::block_api::{impl#8}::finalize_fixed_core&lt;sha2::Sha256&gt; (hmac/src/block_api.rs:89)
  hmac::{impl#8}::finalize_into&lt;sha2::Sha256&gt; (digest/src/buffer_macros/fixed.rs:224)
  digest::FixedOutput::finalize_fixed&lt;hmac::Hmac&lt;sha2::Sha256&gt;&gt; (digest/src/lib.rs:104)
  pbkdf2::pbkdf2_body&lt;hmac::Hmac&lt;sha2::Sha256&gt;&gt; (src/lib.rs:140)
  pbkdf2::pbkdf2&lt;hmac::Hmac&lt;sha2::Sha256&gt;&gt; (src/lib.rs:176)
  pbkdf2::pbkdf2_hmac&lt;sha2::Sha256&gt; (src/lib.rs:225)
  pbkdf2::pbkdf2_hmac_with_params (src/lib.rs:284)
  repro_timeout_in_pbkdf2_pbkdf2_hmac_with_params_fd74fb::main (src/main.rs:7)
  (4 std/libc frames omitted)</pre></td></tr>
</table>
