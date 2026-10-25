# thread_local - crashes

Source: https://github.com/Amanieu/thread_local-rs @ 69fb0508498fc4484cf5696ed16f2c9d6c50f9d4

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;thread_local::ThreadLocal&lt;String&gt;&gt;::with_capacity(
    10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at thread_local-rs/src/lib.rs:185:35:
range end index 64 out of range for slice of length 63
backtrace:
  &lt;[core::sync::atomic::Atomic&lt;*mut thread_local::Entry&lt;alloc::string::String&gt;&gt;] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;[core::sync::atomic::Atomic&lt;*mut thread_local::Entry&lt;alloc::string::String&gt;&gt;; 63] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/array/mod.rs:403:9)
  &lt;thread_local::ThreadLocal&lt;alloc::string::String&gt;&gt;::with_capacity (thread_local-rs/src/lib.rs:185:35)
  repro_thread_local_rs_src_lib_rs_185_35_d517c9::main (src/main.rs:2:13)</pre></td></tr>
</table>
