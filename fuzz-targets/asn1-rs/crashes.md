# asn1-rs - crashes

Source: https://github.com/rusticata/asn1-rs @ 6fb39fb14ba71cf1741e5ea5c69e43f9a85fb33f

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;asn1_rs::Real as asn1_rs::ToDer&gt;::der_content_len(&amp;Real::binary(
    -0.0f64, 2u32, 0i32));</pre></td><td valign="top"><pre>thread 'main' panicked at asn1-rs/src/asn1_types/real.rs:390:21:
assertion failed: m != 0
backtrace:
  &lt;asn1_rs::asn1_types::real::Real as asn1_rs::to_ber::ToBer&gt;::ber_write_content::&lt;core::io::util::Sink&gt; (asn1-rs/src/asn1_types/real.rs:390:21)
  &lt;asn1_rs::asn1_types::real::Real as asn1_rs::to_ber::ToBer&gt;::ber_content_len (asn1-rs/src/asn1_types/real.rs:348:34)
  &lt;asn1_rs::asn1_types::real::Real as asn1_rs::to_der::ToDer&gt;::der_content_len (asn1-rs/src/to_der.rs:195:17)
  repro_asn1_rs_src_asn1_types_real_rs_390_21_990ffd::main (src/main.rs:4:13)</pre></td></tr>
</table>
