# colored - crashes

Source: https://github.com/mackwic/colored @ 5204a26619e10fda0adc77f07588811d6102a40d

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Color::from("#\u{200b}".to_string());</pre></td><td valign="top"><pre>thread 'main' panicked at colored/src/color.rs:252:38:
end byte index 1 is not a char boundary; it is inside '\u{200b}' (bytes 0..3 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  colored::color::parse_hex (colored/src/color.rs:252:38)
  &lt;colored::color::Color as core::str::traits::FromStr&gt;::from_str (colored/src/color.rs:239:40)
  &lt;str&gt;::parse::&lt;colored::color::Color&gt; (core/src/str/mod.rs:2783:9)
  &lt;colored::color::Color as core::convert::From&lt;alloc::string::String&gt;&gt;::from (colored/src/color.rs:212:13)
  repro_colored_src_color_rs_252_38_6867a6::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Color::from("#k\u{9fb49}z".to_string());</pre></td><td valign="top"><pre>thread 'main' panicked at colored/src/color.rs:247:38:
end byte index 2 is not a char boundary; it is inside '\u{9fb49}' (bytes 1..5 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  colored::color::parse_hex (colored/src/color.rs:247:38)
  &lt;colored::color::Color as core::str::traits::FromStr&gt;::from_str (colored/src/color.rs:239:40)
  &lt;str&gt;::parse::&lt;colored::color::Color&gt; (core/src/str/mod.rs:2783:9)
  &lt;colored::color::Color as core::convert::From&lt;alloc::string::String&gt;&gt;::from (colored/src/color.rs:212:13)
  repro_colored_src_color_rs_247_38_da9c8b::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;colored::Color as ::core::str::FromStr&gt;::from_str("#EB\u{854ad}");</pre></td><td valign="top"><pre>thread 'main' panicked at colored/src/color.rs:248:38:
end byte index 4 is not a char boundary; it is inside '\u{854ad}' (bytes 2..6 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  colored::color::parse_hex (colored/src/color.rs:248:38)
  &lt;colored::color::Color as core::str::traits::FromStr&gt;::from_str (colored/src/color.rs:239:40)
  repro_colored_src_color_rs_248_38_83905a::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;colored::Color as ::core::str::FromStr&gt;::from_str("#Eǫ");</pre></td><td valign="top"><pre>thread 'main' panicked at colored/src/color.rs:254:38:
end byte index 2 is not a char boundary; it is inside 'ǫ' (bytes 1..3 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  colored::color::parse_hex (colored/src/color.rs:254:38)
  &lt;colored::color::Color as core::str::traits::FromStr&gt;::from_str (colored/src/color.rs:239:40)
  repro_colored_src_color_rs_254_38_03716e::main (src/main.rs:2:13)</pre></td></tr>
</table>
