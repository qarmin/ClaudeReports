# piper - crashes

Source: https://github.com/smol-rs/piper @ ef802bcc97f6b93af640ff7d60286ab12875552e

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = piper::pipe(2147483648usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  piper::pipe (src/lib.rs:224)
  repro_out_of_memory_in_piper_pipe_617360::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
</table>
