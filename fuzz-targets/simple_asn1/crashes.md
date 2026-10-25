# simple_asn1 - crashes

Source: https://github.com/acw/simple_asn1 @ 7712c7cb4d72e52b6896ba56990ed1095a317de5

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let a_s_n1_block = ASN1Block::BitString(0usize, 1usize, vec![]);
let _ = simple_asn1::to_der(&amp;a_s_n1_block);</pre></td><td valign="top"><pre>thread 'main' panicked at simple_asn1/src/lib.rs:711:29:
attempt to subtract with overflow
backtrace:
  simple_asn1::to_der (simple_asn1/src/lib.rs:711:29)
  repro_simple_asn1_src_lib_rs_711_29_ba9a50::main (src/main.rs:5:13)</pre></td></tr>
</table>
