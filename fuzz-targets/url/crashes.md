# url - crashes

Source: https://github.com/servo/rust-url @ 00a6ce58d02f4e0d43c5ca0702c0bedb8b1ebf3a

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Config::default().use_idna_2008_rules(true);</pre></td><td valign="top"><pre>thread 'main' panicked at idna/src/deprecated.rs:212:9:
IDNA 2008 rules are no longer supported
backtrace:
  &lt;idna::deprecated::Config&gt;::use_idna_2008_rules (idna/src/deprecated.rs:212:9)
  repro_idna_src_deprecated_rs_212_9_993350::main (src/main.rs:4:31)</pre></td></tr>
<tr><td valign="top"><pre>let ascii_set = &lt;percent_encoding::AsciiSet as ::core::ops::Not&gt;::not(
    AsciiSet::EMPTY);
let _ = ascii_set.add(255u8);</pre></td><td valign="top"><pre>thread 'main' panicked at percent_encoding/src/ascii_set.rs:58:9:
index out of bounds: the len is 4 but the index is 7
backtrace:
  &lt;percent_encoding::ascii_set::AsciiSet&gt;::add (percent_encoding/src/ascii_set.rs:58:9)
  repro_percent_encoding_src_ascii_set_rs_58_9_3aac34::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let ascii_set = AsciiSet::EMPTY;
let _ = ascii_set.remove(255u8);</pre></td><td valign="top"><pre>thread 'main' panicked at percent_encoding/src/ascii_set.rs:64:9:
index out of bounds: the len is 4 but the index is 7
backtrace:
  &lt;percent_encoding::ascii_set::AsciiSet&gt;::remove (percent_encoding/src/ascii_set.rs:64:9)
  repro_percent_encoding_src_ascii_set_rs_64_9_21430f::main (src/main.rs:5:23)</pre></td></tr>
</table>
