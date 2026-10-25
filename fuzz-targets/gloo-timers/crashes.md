# gloo-timers - crashes

Source: https://github.com/rustwasm/gloo @ ebd0b8d7062c4157497eb30649afafc59c94c4b0

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = IntervalStream::new(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at wasm-bindgen/src/lib.rs:1323:1:
function not implemented on non-wasm32 targets
backtrace:
  wasm_bindgen::__wbindgen_describe (wasm-bindgen/src/lib.rs:117:17)
  wasm_bindgen::describe::inform (wasm-bindgen/src/describe.rs:20:14)
  wasm_bindgen::__rt::wbg_cast::breaks_if_inlined::&lt;wasm_bindgen::closure::OwnedClosure&lt;dyn core::ops::function::FnMut&lt;(), Output = ()&gt;, true&gt;, wasm_bindgen::closure::JsClosure&gt; (wasm-bindgen/src/rt/mod.rs:102:9)
  wasm_bindgen::__rt::wbg_cast::&lt;wasm_bindgen::closure::OwnedClosure&lt;dyn core::ops::function::FnMut&lt;(), Output = ()&gt;, true&gt;, wasm_bindgen::closure::JsClosure&gt; (wasm-bindgen/src/rt/mod.rs:129:27)
  &lt;wasm_bindgen::closure::ScopedClosure&lt;dyn core::ops::function::FnMut&lt;(), Output = ()&gt;&gt;&gt;::wrap_maybe_aborting::&lt;true, dyn core::ops::function::FnMut&lt;(), Output = ()&gt;&gt; (wasm-bindgen/src/closure.rs:359:17)
  &lt;wasm_bindgen::closure::ScopedClosure&lt;dyn core::ops::function::FnMut&lt;(), Output = ()&gt;&gt;&gt;::wrap::&lt;dyn core::ops::function::FnMut&lt;(), Output = ()&gt;&gt; (wasm-bindgen/src/closure.rs:327:9)
  &lt;gloo_timers::callback::Interval&gt;::new::&lt;&lt;gloo_timers::future::IntervalStream&gt;::new::{closure#0}&gt; (timers/src/callback.rs:201:23)
  &lt;gloo_timers::future::IntervalStream&gt;::new (timers/src/future.rs:181:21)
  repro_wasm_bindgen_src_lib_rs_1323_1_7d278e::main (src/main.rs:4:13)</pre></td></tr>
</table>
