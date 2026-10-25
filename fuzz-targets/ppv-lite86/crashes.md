# ppv-lite86 - crashes

Source: https://github.com/cryptocorrosion/cryptocorrosion @ aadd0158fc0000105404925fb2730eac11db1125

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;ppv_lite86::generic::GenericMachine as ppv_lite86::Machine&gt;::
    read_le::&lt;ppv_lite86::generic::u64x2_generic&gt;(GenericMachine, &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at ppv-lite86/src/generic.rs:601:55:
called `Result::unwrap()` on an `Err` value: SizeError
backtrace:
  &lt;ppv_lite86::generic::u64x2_generic as ppv_lite86::types::StoreBytes&gt;::unsafe_read_le (ppv-lite86/src/generic.rs:601:55)
  &lt;ppv_lite86::generic::GenericMachine as ppv_lite86::types::Machine&gt;::read_le::&lt;ppv_lite86::generic::u64x2_generic&gt; (ppv-lite86/src/types.rs:263:18)
  repro_ppv_lite86_src_generic_rs_601_55_e669aa::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>&lt;ppv_lite86::generic::u32x4_generic as ppv_lite86::StoreBytes&gt;::write_le(
    &lt;ppv_lite86::generic::GenericMachine as ppv_lite86::Machine&gt;::
    read_le::&lt;ppv_lite86::generic::u32x4_generic&gt;(GenericMachine, &amp;vec![0u8,
    34u8, 0u8, 67u8, 0u8, 86u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 113u8, 94u8, 0u8,
    0u8]), &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at ppv-lite86/src/generic.rs:590:25:
called `Result::unwrap()` on an `Err` value: SizeError
backtrace:
  &lt;ppv_lite86::generic::u32x4_generic as ppv_lite86::types::StoreBytes&gt;::write_le (ppv-lite86/src/generic.rs:590:25)
  repro_ppv_lite86_src_generic_rs_590_25_6109c1::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let generic_machine = GenericMachine;
let _ = &lt;ppv_lite86::generic::GenericMachine as ppv_lite86::Machine&gt;::
    read_be::&lt;ppv_lite86::generic::u32x4_generic&gt;(generic_machine, &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at ppv-lite86/src/generic.rs:584:55:
called `Result::unwrap()` on an `Err` value: SizeError
backtrace:
  &lt;ppv_lite86::generic::u32x4_generic as ppv_lite86::types::StoreBytes&gt;::unsafe_read_be (ppv-lite86/src/generic.rs:584:55)
  &lt;ppv_lite86::generic::GenericMachine as ppv_lite86::types::Machine&gt;::read_be::&lt;ppv_lite86::generic::u32x4_generic&gt; (ppv-lite86/src/types.rs:271:18)
  repro_ppv_lite86_src_generic_rs_584_55_10502a::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let generic_machine = GenericMachine;
let _ = &lt;ppv_lite86::generic::GenericMachine as ppv_lite86::Machine&gt;::
    read_le::&lt;ppv_lite86::generic::u32x4_generic&gt;(generic_machine, &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at ppv-lite86/src/generic.rs:579:55:
called `Result::unwrap()` on an `Err` value: SizeError
backtrace:
  &lt;ppv_lite86::generic::u32x4_generic as ppv_lite86::types::StoreBytes&gt;::unsafe_read_le (ppv-lite86/src/generic.rs:579:55)
  &lt;ppv_lite86::generic::GenericMachine as ppv_lite86::types::Machine&gt;::read_le::&lt;ppv_lite86::generic::u32x4_generic&gt; (ppv-lite86/src/types.rs:263:18)
  repro_ppv_lite86_src_generic_rs_579_55_31b1ef::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let generic_machine = GenericMachine;
let _ = &lt;ppv_lite86::generic::GenericMachine as ppv_lite86::Machine&gt;::
    read_be::&lt;ppv_lite86::generic::u64x2_generic&gt;(generic_machine, &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at ppv-lite86/src/generic.rs:606:55:
called `Result::unwrap()` on an `Err` value: SizeError
backtrace:
  &lt;ppv_lite86::generic::u64x2_generic as ppv_lite86::types::StoreBytes&gt;::unsafe_read_be (ppv-lite86/src/generic.rs:606:55)
  &lt;ppv_lite86::generic::GenericMachine as ppv_lite86::types::Machine&gt;::read_be::&lt;ppv_lite86::generic::u64x2_generic&gt; (ppv-lite86/src/types.rs:271:18)
  repro_ppv_lite86_src_generic_rs_606_55_133c20::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>&lt;ppv_lite86::generic::u32x4_generic as ppv_lite86::StoreBytes&gt;::write_be(
    &lt;ppv_lite86::generic::GenericMachine as ppv_lite86::Machine&gt;::
    read_le::&lt;ppv_lite86::generic::u32x4_generic&gt;(GenericMachine, &amp;vec![57u8,
    0u8, 0u8, 120u8, 0u8, 50u8, 0u8, 54u8, 116u8, 0u8, 71u8, 0u8, 0u8, 0u8, 0u8,
    101u8]), &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at ppv-lite86/src/generic.rs:595:25:
called `Result::unwrap()` on an `Err` value: SizeError
backtrace:
  &lt;ppv_lite86::generic::u32x4_generic as ppv_lite86::types::StoreBytes&gt;::write_be (ppv-lite86/src/generic.rs:595:25)
  repro_ppv_lite86_src_generic_rs_595_25_934293::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let u64x2_generic = &lt;ppv_lite86::generic::u64x2_generic as ppv_lite86::Swap64&gt;::
    swap4(
    &lt;ppv_lite86::generic::GenericMachine as ppv_lite86::Machine&gt;::
    read_be::&lt;ppv_lite86::generic::u64x2_generic&gt;(GenericMachine, &amp;vec![66u8,
    77u8, 70u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 54u8, 0u8, 0u8, 0u8, 40u8,
    0u8]));
&lt;ppv_lite86::generic::u64x2_generic as ppv_lite86::StoreBytes&gt;::write_be(
    u64x2_generic, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at ppv-lite86/src/generic.rs:617:25:
called `Result::unwrap()` on an `Err` value: SizeError
backtrace:
  &lt;ppv_lite86::generic::u64x2_generic as ppv_lite86::types::StoreBytes&gt;::write_be (ppv-lite86/src/generic.rs:617:25)
  repro_ppv_lite86_src_generic_rs_617_25_962d66::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;ppv_lite86::generic::u64x2_generic as ppv_lite86::StoreBytes&gt;::write_le(
    &lt;ppv_lite86::generic::GenericMachine as ppv_lite86::Machine&gt;::
    read_be::&lt;ppv_lite86::generic::u64x2_generic&gt;(GenericMachine, &amp;vec![0u8,
    0u8 /* x14 */, 0u8]), &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at ppv-lite86/src/generic.rs:612:25:
called `Result::unwrap()` on an `Err` value: SizeError
backtrace:
  &lt;ppv_lite86::generic::u64x2_generic as ppv_lite86::types::StoreBytes&gt;::write_le (ppv-lite86/src/generic.rs:612:25)
  repro_ppv_lite86_src_generic_rs_612_25_d16fac::main (src/main.rs:4:5)</pre></td></tr>
</table>
