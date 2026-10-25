# cpp_demangle - crashes

Source: https://github.com/gimli-rs/cpp_demangle @ 076632f924b1c87c7de8ef6a5c604edb97964978

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = ParseOptions::default().recursion_limit(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at cpp_demangle/src/lib.rs:75:60:
Recursion limit must be &gt; 0
backtrace:
  &lt;cpp_demangle::ParseOptions&gt;::recursion_limit (cpp_demangle/src/lib.rs:75:60)
  repro_cpp_demangle_src_lib_rs_75_60_624dde::main (src/main.rs:4:37)</pre></td></tr>
<tr><td valign="top"><pre>let demangle_options = DemangleOptions::default();
let _ = demangle_options.recursion_limit(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at cpp_demangle/src/lib.rs:122:60:
Recursion limit must be &gt; 0
backtrace:
  &lt;cpp_demangle::DemangleOptions&gt;::recursion_limit (cpp_demangle/src/lib.rs:122:60)
  repro_cpp_demangle_src_lib_rs_122_60_969e55::main (src/main.rs:5:30)</pre></td></tr>
</table>
