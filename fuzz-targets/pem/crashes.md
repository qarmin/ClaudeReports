# pem - crashes

Source: https://github.com/jcreekmore/pem-rs @ 99c15c08f1389153ed037c33750f8605bbf8bd55

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = pem::encode_config(&amp;Pem::new("".to_string(), vec![]),
    EncodeConfig::new().set_line_wrap(0usize));</pre></td><td valign="top"><pre>thread 'main' panicked at pem-rs/src/lib.rs:530:34:
chunk size must be non-zero
backtrace:
  &lt;[u8]&gt;::chunks (core/src/slice/mod.rs:1155:9)
  pem::encode_config (pem-rs/src/lib.rs:530:34)
  repro_pem_rs_src_lib_rs_530_34_2da272::main (src/main.rs:5:13)</pre></td></tr>
</table>
