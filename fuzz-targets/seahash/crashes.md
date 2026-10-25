# seahash - crashes

Source: https://gitlab.redox-os.org/redox-os/seahash @ 74c02182146d1edd7c91a9e6eddefc7390682a70

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let u64_v = seahash::reference::hash(&amp;vec![]);
let mut state = State::new(0u64, 0u64, 0u64, u64_v);
state.pop(0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at seahash/src/buffer.rs:205:9:
attempt to subtract with overflow
backtrace:
  &lt;seahash::buffer::State&gt;::pop (seahash/src/buffer.rs:205:9)
  repro_seahash_src_buffer_rs_205_9_4308db::main (src/main.rs:6:11)</pre></td></tr>
</table>
