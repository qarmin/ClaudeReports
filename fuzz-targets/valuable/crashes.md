# valuable - crashes

Source: https://github.com/tokio-rs/valuable @ e5135dadbf1e8d7f72859889986e3bee03726f44

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let named_field = NamedField::new(&amp;string);
let vec = vec![];
let vec2 = vec![];
let named_values = NamedValues::new(&amp;vec, &amp;vec2);
let _ = named_values.get(&amp;named_field);</pre></td><td valign="top"><pre>thread 'main' panicked at valuable/src/named_values.rs:75:50:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;valuable::named_values::NamedValues&gt;::get (valuable/src/named_values.rs:75:50)
  repro_valuable_src_named_values_rs_75_50_afc11a::main (src/main.rs:10:26)</pre></td></tr>
</table>
