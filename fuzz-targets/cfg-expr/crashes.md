# cfg-expr - crashes

Source: https://github.com/EmbarkStudios/cfg-expr @ 2ccf04b84e32345bbb04d21124ecb1c2fd8e42cd

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let parse_error = ParseError { original: "".to_string(),
    span: 11usize..11660897532026927812usize,
    reason: Reason::InvalidNot(18446744073709551614usize) };
let _ = parse_error.to_string();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  cfg_expr::error::{impl#0}::fmt (src/error.rs:65)
  repro_timeout_in_cfg_expr_parseerror_to_string_e6c382::main (src/main.rs:6)
  (10 std/libc frames omitted)</pre></td></tr>
</table>
