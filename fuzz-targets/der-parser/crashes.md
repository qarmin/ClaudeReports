# der-parser - crashes

Source: https://github.com/rusticata/der-parser @ d14e092b7df3bea7102ee9b286e553a1bc5aef63

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let ber_object = BerObject::from_seq(vec![]);
let pretty_ber = ber_object.as_pretty(usize::MAX, 1usize);
let vec = vec![];
let ber_object2 = BerObject::from_int_slice(&amp;vec);
let _ = pretty_ber.next_indent(&amp;ber_object2);</pre></td><td valign="top"><pre>thread 'main' panicked at der-parser/src/ber/print.rs:69:21:
attempt to add with overflow
backtrace:
  &lt;der_parser::ber::print::PrettyBer&gt;::next_indent (der-parser/src/ber/print.rs:69:21)
  repro_der_parser_src_ber_print_rs_69_21_9a71af::main (src/main.rs:8:24)</pre></td></tr>
</table>
