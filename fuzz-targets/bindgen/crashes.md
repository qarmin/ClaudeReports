# bindgen - crashes

Source: https://github.com/rust-lang/rust-bindgen @ f08d4fff0e060fb4081222b0ad624e0bd7ec247d

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let builder = Builder::default();
let builder2 = builder.clone().header::&lt;String&gt;("".to_string());
let builder3 = builder2.clone().clang_arg::&lt;String&gt;("\0".to_string());
let _ = builder3.clone().generate();</pre></td><td valign="top"><pre>thread 'main' panicked at clang-sys/src/support.rs:189:45:
called `Result::unwrap()` on an `Err` value: "could not run executable `/usr/lib/llvm-21/bin/clang`: nul byte found in provided data"
backtrace:
  clang_sys::support::run_clang (clang-sys/src/support.rs:189:45)
  clang_sys::support::parse_search_paths (clang-sys/src/support.rs:227:18)
  &lt;clang_sys::support::Clang&gt;::new::&lt;std::path::PathBuf&gt; (clang-sys/src/support.rs:39:29)
  &lt;clang_sys::support::Clang&gt;::find (clang-sys/src/support.rs:124:29)
  &lt;bindgen::Bindings&gt;::generate::detect_include_paths (/home/rafal/.cargo/git/checkouts/rust-bindgen-d3080e0c1112964e/f08d4ff/bindgen/lib.rs:858:31)
  &lt;bindgen::Bindings&gt;::generate (/home/rafal/.cargo/git/checkouts/rust-bindgen-d3080e0c1112964e/f08d4ff/bindgen/lib.rs:890:9)
  &lt;bindgen::Builder&gt;::generate (/home/rafal/.cargo/git/checkouts/rust-bindgen-d3080e0c1112964e/f08d4ff/bindgen/lib.rs:380:9)
  repro_clang_sys_src_support_rs_189_45_726c49::main (src/main.rs:7:30)</pre></td></tr>
</table>
