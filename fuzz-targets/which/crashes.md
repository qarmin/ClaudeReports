# which - crashes

Source: https://github.com/harryfei/which-rs @ 8e84cb67761b6e19d2cfa512c5d55cbe5fd9e8bc

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let which_config = &lt;which::WhichConfig&lt;which::sys::RealSys,
    which::Noop&gt;&gt;::new_with_sys(RealSys);
let which_config2 = which_config.system_cwd(false);
let which_config3 = which_config2.system_path_list();
let _ = which_config3.all_results();</pre></td><td valign="top"><pre>thread 'main' panicked at which-rs/src/lib.rs:479:34:
binary_name not set! You must set binary_name or regex before searching!
backtrace:
  &lt;which::WhichConfig&lt;which::sys::RealSys&gt;&gt;::all_results (which-rs/src/lib.rs:479:34)
  repro_which_rs_src_lib_rs_479_34_cb8e8b::main (src/main.rs:7:27)</pre></td></tr>
</table>
