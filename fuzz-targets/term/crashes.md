# term - crashes

Source: https://github.com/Stebalien/term @ 5325c789f24aa4e9378733b8432befed964462d0

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut variables = Variables::default();
let _ = term::terminfo::parm::expand(&amp;include_bytes!("data_0.bin").to_vec(),
    &amp;[Param::Number(i32::MAX)], &amp;mut variables);</pre></td><td valign="top"><pre>thread 'main' panicked at term/src/terminfo/parm.rs:246:49:
attempt to add with overflow
backtrace:
  term::terminfo::parm::expand (term/src/terminfo/parm.rs:246:49)
  repro_term_src_terminfo_parm_rs_246_49_c04b3b::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut variables = Variables::new();
let _ = term::terminfo::parm::expand(&amp;vec![37u8, 112u8, 48u8], &amp;[],
    &amp;mut variables);</pre></td><td valign="top"><pre>thread 'main' panicked at term/src/terminfo/parm.rs:295:36:
attempt to subtract with overflow
backtrace:
  term::terminfo::parm::expand (term/src/terminfo/parm.rs:295:36)
  repro_term_src_terminfo_parm_rs_295_36_ddd080::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut variables = Variables::new();
let _ = term::terminfo::parm::expand(&amp;vec![37u8, 105u8], &amp;[Param::default(),
    Param::Number(i32::MAX)], &amp;mut variables);</pre></td><td valign="top"><pre>thread 'main' panicked at term/src/terminfo/parm.rs:247:49:
attempt to add with overflow
backtrace:
  term::terminfo::parm::expand (term/src/terminfo/parm.rs:247:49)
  repro_term_src_terminfo_parm_rs_247_49_453f22::main (src/main.rs:6:13)</pre></td></tr>
</table>
