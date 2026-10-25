# realfft - crashes

Source: https://github.com/HEnquist/realfft @ d0d4eee0525fd27c96c8a046d6d107acd5ed84a6

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let fft_error = FftError::InputValues(false, false);
let _ = fft_error.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at realfft/src/lib.rs:64:35:
internal error: entered unreachable code
backtrace:
  &lt;realfft::FftError&gt;::fmt_internal (realfft/src/lib.rs:64:35)
  &lt;realfft::FftError as core::fmt::Display&gt;::fmt (realfft/src/lib.rs:79:14)
  &lt;realfft::FftError as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;realfft::FftError as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_realfft_src_lib_rs_64_35_eb7cf4::main (src/main.rs:5:23)</pre></td></tr>
</table>
