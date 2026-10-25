# oorandom - crashes

Source: oorandom 11.1.5

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut rand32 = Rand32::new_inc(0u64, 0u64);
let _ = rand32.rand_range(0u32..0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at oorandom/src/lib.rs:138:9:
assertion failed: range.start &lt; range.end
backtrace:
  &lt;oorandom::Rand32&gt;::rand_range (oorandom/src/lib.rs:138:9)
  repro_oorandom_src_lib_rs_138_9_ff2a3c::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let u128 = Rand64::DEFAULT_INC;
let mut rand64 = Rand64::new(u128);
let _ = rand64.rand_range(10000000000000000000u64..3145772903569345471u64);</pre></td><td valign="top"><pre>thread 'main' panicked at oorandom/src/lib.rs:247:9:
assertion failed: range.start &lt; range.end
backtrace:
  &lt;oorandom::Rand64&gt;::rand_range (oorandom/src/lib.rs:247:9)
  repro_oorandom_src_lib_rs_247_9_0e12d1::main (src/main.rs:6:20)</pre></td></tr>
</table>
