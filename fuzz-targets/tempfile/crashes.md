# tempfile - crashes

Source: https://github.com/Stebalien/tempfile @ 1c294cc29d2a48746637252a0e30b1bebc05b3e6

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut builder = Builder::new();
let _ = builder.rand_bytes(10000000000000000000usize);
let _ = builder.tempfile();</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from tempfile/src/util.rs:12:19)
capacity overflow
backtrace:
  tempfile::util::tmpname (tempfile/src/util.rs:12:19)
  tempfile::util::create_helper::&lt;tempfile::file::NamedTempFile, &lt;tempfile::Builder&gt;::tempfile_in&lt;std::path::PathBuf&gt;::{closure#0}&gt; (tempfile/src/util.rs:67:30)
  &lt;tempfile::Builder&gt;::tempfile_in::&lt;std::path::PathBuf&gt; (tempfile/src/lib.rs:556:9)
  &lt;tempfile::Builder&gt;::tempfile (tempfile/src/lib.rs:527:14)
  repro_tempfile_src_util_rs_12_19_ffb5aa::main (src/main.rs:6:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut builder = Builder::new();
let _ = builder.rand_bytes(1000000000000000000usize);
let _ = builder.tempfile();</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000000000004 bytes failed
backtrace:
  tempfile::util::tmpname (src/util.rs:12)
  tempfile::util::create_helper&lt;tempfile::file::NamedTempFile&lt;std::fs::File&gt;, tempfile::{impl#1}::tempfile_in::{closure_env#0}&lt;std::path::PathBuf&gt;&gt; (src/util.rs:67)
  tempfile::Builder::tempfile_in&lt;std::path::PathBuf&gt; (src/lib.rs:556)
  tempfile::Builder::tempfile (src/lib.rs:527)
  repro_out_of_memory_in_tempfile_builder_tempfile_cb3d4d::main (src/main.rs:6)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut builder = Builder::new();
let _ = builder.rand_bytes(4294967295usize);
let _ = builder.tempdir();</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967299 bytes failed
backtrace:
  tempfile::util::tmpname (src/util.rs:12)
  tempfile::util::create_helper&lt;tempfile::dir::TempDir, tempfile::{impl#1}::tempdir_in::{closure_env#0}&lt;std::path::PathBuf&gt;&gt; (src/util.rs:67)
  tempfile::Builder::tempdir_in&lt;std::path::PathBuf&gt; (src/lib.rs:622)
  tempfile::Builder::tempdir (src/lib.rs:596)
  repro_out_of_memory_in_tempfile_builder_tempdir_bf3d68::main (src/main.rs:6)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut spooled_temp_file = tempfile::spooled_tempfile(
    1000000000000000000usize);
let _ = spooled_temp_file.set_len(1000000000000000000u64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000000000000 bytes failed
backtrace:
  tempfile::spooled::SpooledTempFile::set_len (src/spooled.rs:140)
  repro_out_of_memory_in_tempfile_spooledtempfile_set_len_c48bb0::main (src/main.rs:3)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut spooled_temp_file = SpooledTempFile::new(usize::MAX);
let _ = spooled_temp_file.set_len(u64::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from tempfile/src/spooled.rs:140:34)
capacity overflow
backtrace:
  &lt;tempfile::spooled::SpooledTempFile&gt;::set_len (tempfile/src/spooled.rs:140:34)
  repro_tempfile_src_spooled_rs_140_34_f417b8::main (src/main.rs:5:31)</pre></td></tr>
</table>
