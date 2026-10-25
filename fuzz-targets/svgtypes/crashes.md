# svgtypes - crashes

Source: https://github.com/linebender/svgtypes @ 3ee4db5bcbfe8db817d87b55a3ae4f889bf2d3fa

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let error = Error::InvalidChar(vec![], 0usize);
let _ = error.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at svgtypes/src/error.rs:80:26:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[u8]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[u8] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;svgtypes::error::Error as core::fmt::Display&gt;::fmt (svgtypes/src/error.rs:80:26)
  &lt;svgtypes::error::Error as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;svgtypes::error::Error as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_svgtypes_src_error_rs_80_26_067ba3::main (src/main.rs:5:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;svgtypes::TransformOrigin as ::core::str::FromStr&gt;::from_str(" ");</pre></td><td valign="top"><pre>thread 'main' panicked at svgtypes/src/transform_origin.rs:184:18:
internal error: entered unreachable code
backtrace:
  &lt;svgtypes::transform_origin::TransformOrigin as core::str::traits::FromStr&gt;::from_str (svgtypes/src/transform_origin.rs:184:18)
  repro_svgtypes_src_transform_origin_rs_184_18_8e2162::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let error = Error::from(Error::InvalidChar(vec![32u8, 197u8], 0usize));
let _ = error.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at svgtypes/src/error.rs:73:58:
called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [197], error: Utf8Error { valid_up_to: 0, error_len: None } }
backtrace:
  &lt;svgtypes::error::Error as core::fmt::Display&gt;::fmt::{closure#0} (svgtypes/src/error.rs:73:58)
  &lt;svgtypes::error::Error as core::fmt::Display&gt;::fmt (svgtypes/src/error.rs:74:22)
  &lt;svgtypes::error::Error as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;svgtypes::error::Error as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_svgtypes_src_error_rs_73_58_7860fa::main (src/main.rs:5:19)</pre></td></tr>
<tr><td valign="top"><pre>let error = Error::InvalidString(vec![], 0usize);
let _ = error.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at svgtypes/src/error.rs:88:28:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[alloc::string::String] as core::ops::index::Index&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;svgtypes::error::Error as core::fmt::Display&gt;::fmt (svgtypes/src/error.rs:88:28)
  &lt;svgtypes::error::Error as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;svgtypes::error::Error as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_svgtypes_src_error_rs_88_28_48ebb3::main (src/main.rs:5:19)</pre></td></tr>
</table>
