# sha1_smol - crashes

Source: https://github.com/mitsuhiko/sha1-smol @ f9d88bd70e9bd577c7c3afdc6acd0313af605324

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;sha1_smol::Digest as ::core::str::FromStr&gt;::from_str(
    "\u{97a59}x\u{d4f98} 㚺\u{102a1e}D/FkC*A\u{9ccaa}ÿ\u{52441}cN;[#S");</pre></td><td valign="top"><pre>thread 'main' panicked at sha1-smol/src/lib.rs:568:46:
end byte index 8 is not a char boundary; it is inside '\u{d4f98}' (bytes 5..9 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;sha1_smol::Digest as core::str::traits::FromStr&gt;::from_str (sha1-smol/src/lib.rs:568:46)
  repro_sha1_smol_src_lib_rs_568_46_c1d992::main (src/main.rs:2:13)</pre></td></tr>
</table>
