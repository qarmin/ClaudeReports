# pxfm - crashes

Source: https://github.com/awxkee/pxfm @ acf7acd00cfc845307eda7b7b2de19e42303de82

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = pxfm::f_betainc_reg(1.0f64, 1e300f64, 4.9980524806478024e-203f64);</pre></td><td valign="top"><pre>thread 'main' panicked at pxfm/src/logs/log1p_dd.rs:241:40:
attempt to subtract with overflow
backtrace:
  pxfm::logs::log1p_dd::log1p_fast_dd (pxfm/src/logs/log1p_dd.rs:241:40)
  pxfm::gamma::betainc::f_betainc_reg (pxfm/src/gamma/betainc.rs:105:24)
  repro_pxfm_src_logs_log1p_dd_rs_241_40_8839b2::main (src/main.rs:2:13)</pre></td></tr>
</table>
