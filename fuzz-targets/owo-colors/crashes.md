# owo-colors - crashes

Source: https://github.com/owo-colors/owo-colors @ 7392e40d0093a02ec12b3695e388d123817283b8

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;owo_colors::DynColors as ::core::str::FromStr&gt;::from_str(
    "#\u{10ffff}_8");</pre></td><td valign="top"><pre>thread 'main' panicked at owo-colors/src/dyn_colors.rs:85:42:
end byte index 3 is not a char boundary; it is inside '\u{10ffff}' (bytes 1..5 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;owo_colors::dyn_colors::DynColors as core::str::traits::FromStr&gt;::from_str (owo-colors/src/dyn_colors.rs:85:42)
  repro_owo_colors_src_dyn_colors_rs_85_42_9e42df::main (src/main.rs:2:13)</pre></td></tr>
</table>
