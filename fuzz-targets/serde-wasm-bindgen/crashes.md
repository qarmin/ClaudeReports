# serde-wasm-bindgen - crashes

Source: https://github.com/RReverser/serde-wasm-bindgen @ a16f4ce2468c0491b92b38ad3ad3bfb1c6f9323f

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Error::new::&lt;String&gt;("".to_string());</pre></td><td valign="top"><pre>thread 'main' panicked at wasm-bindgen/src/lib.rs:1227:1:
cannot call wasm-bindgen imported functions on non-wasm targets
backtrace:
  wasm_bindgen::__wbindgen_error_new::__wbg_Error_30c8987f7c2ed4e2 (wasm-bindgen/src/lib.rs:1242:8)
  wasm_bindgen::__wbindgen_error_new (wasm-bindgen/src/lib.rs:1242:8)
  &lt;wasm_bindgen::JsError&gt;::new (wasm-bindgen/src/lib.rs:1857:20)
  &lt;serde_wasm_bindgen::error::Error&gt;::new::&lt;alloc::string::String&gt; (serde-wasm-bindgen/src/error.rs:24:15)
  repro_wasm_bindgen_src_lib_rs_1227_1_9bcf70::main (src/main.rs:4:13)</pre></td></tr>
</table>
