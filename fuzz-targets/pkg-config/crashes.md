# pkg-config - crashes

Source: https://github.com/rust-lang/pkg-config-rs @ a8852cc148f8f7d504bf3a62444c298dcd3637f0

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = pkg_config::get_variable("Zażółć gęślą jaźń".repeat(947),
    "\u{feff}".repeat(909));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  pkg_config::WrappedCommand::output (src/lib.rs:224)
  pkg_config::Config::run (src/lib.rs:704)
  pkg_config::get_variable (src/lib.rs:475)
  repro_timeout_in_pkg_config_get_variable_a732ff::main (src/main.rs:2)
  (8 std/libc frames omitted)</pre></td></tr>
</table>
