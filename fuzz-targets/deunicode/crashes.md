# deunicode - crashes

Source: https://github.com/kornelski/deunicode @ cfb8552fbbdf6d1f3f996ee4f2e78ec5e482bcef

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let string = deunicode::deunicode_with_tofu("�\u{10ffff}".repeat(926),
    "�\u{10ffff}".repeat(660));
let _ = deunicode::deunicode_with_tofu(&amp;string,
    "a\u{301}\u{301}\u{301}".repeat(477));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2190876160 bytes failed
backtrace:
  deunicode::deunicode_with_tofu_cow (src/lib.rs:134)
  deunicode::deunicode_with_tofu (deunicode/src/lib.rs:95)
  repro_out_of_memory_in_deunicode_deunicode_with_tofu_eaf1c6::main (src/main.rs:3)
  (30 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let string = deunicode::deunicode_with_tofu("�\u{10ffff}".repeat(75),
    "\u{10354d}\u{7f}𩈊\u{a7169}g_a?G\u{47b81}\u{4c9dc}*yxBav\u{921c0}\u{6dcfa\
    }");
let string2 = deunicode::deunicode_with_tofu(&amp;string, "\u{feff}".repeat(1352));
let _ = deunicode::deunicode_with_tofu_cow(&amp;string, &amp;string2);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2181428224 bytes failed
backtrace:
  deunicode::deunicode_with_tofu_cow (src/lib.rs:134)
  repro_out_of_memory_in_deunicode_deunicode_with_tofu_cow_1e7b65::main (src/main.rs:4)
  (30 std/libc frames omitted)</pre></td></tr>
</table>
