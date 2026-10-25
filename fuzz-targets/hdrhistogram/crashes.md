# hdrhistogram - crashes

Source: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let histogram = &lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new_from::&lt;u32&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u32&gt;&gt;::new(0u8).unwrap());
let sync_histogram = histogram.clone().into_sync();
let mut recorder = sync_histogram.recorder();
let _ = recorder.record_n_correct(u64::MAX, 0u64, 2u64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hdrhistogram::Histogram&lt;u64&gt;::record_n_correct&lt;u64&gt; (hdrhistogram_rust/src/lib.rs:967)
  hdrhistogram::sync::{impl#6}::record_n_correct::{closure#0}&lt;u64&gt; (hdrhistogram_rust/src/sync/mod.rs:286)
  hdrhistogram::sync::Recorder&lt;u64&gt;::with_hist&lt;u64, hdrhistogram::sync::{impl#6}::record_n_correct::{closure_env#0}&lt;u64&gt;, core::result::Result&lt;(), hdrhistogram::errors::RecordError&gt;&gt; (hdrhistogram_rust/src/sync/mod.rs:168)
  hdrhistogram::sync::Recorder&lt;u64&gt;::record_n_correct&lt;u64&gt; (hdrhistogram_rust/src/sync/mod.rs:286)
  repro_timeout_in_hdrhistogram_sync_recorder_u64_record_n_correct_53a382::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let histogram = &lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new_from::&lt;u64&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new_from::&lt;u8&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u8&gt;&gt;::new_from::&lt;u16&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u16&gt;&gt;::new_with_max(u64::MAX, 1u8).unwrap())));
let sync_histogram = histogram.clone().into_sync();
let mut recorder = sync_histogram.recorder();
let _ = recorder.record_correct(18422672277416534739u64, 4294967295u64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hdrhistogram::Histogram&lt;u64&gt;::index_for&lt;u64&gt; (hdrhistogram_rust/src/lib.rs:370)
  hdrhistogram::Histogram&lt;u64&gt;::mut_at&lt;u64&gt; (hdrhistogram_rust/src/lib.rs:396)
  hdrhistogram::Histogram&lt;u64&gt;::record_n_inner&lt;u64&gt; (hdrhistogram_rust/src/lib.rs:887)
  hdrhistogram::Histogram&lt;u64&gt;::record_n_correct&lt;u64&gt; (hdrhistogram_rust/src/lib.rs:967)
  hdrhistogram::Histogram&lt;u64&gt;::record_correct&lt;u64&gt; (hdrhistogram_rust/src/lib.rs:936)
  hdrhistogram::sync::{impl#6}::record_correct::{closure#0}&lt;u64&gt; (hdrhistogram_rust/src/sync/mod.rs:276)
  hdrhistogram::sync::Recorder&lt;u64&gt;::with_hist&lt;u64, hdrhistogram::sync::{impl#6}::record_correct::{closure_env#0}&lt;u64&gt;, core::result::Result&lt;(), hdrhistogram::errors::RecordError&gt;&gt; (hdrhistogram_rust/src/sync/mod.rs:168)
  hdrhistogram::sync::Recorder&lt;u64&gt;::record_correct&lt;u64&gt; (hdrhistogram_rust/src/sync/mod.rs:276)
  repro_timeout_in_hdrhistogram_sync_recorder_u64_record_correct_723db2::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let sync_histogram = &lt;hdrhistogram::SyncHistogram&lt;u16&gt; as ::core::convert::
    From&lt;hdrhistogram::Histogram&lt;u16&gt;&gt;&gt;::from(
    &lt;hdrhistogram::Histogram&lt;u16&gt;&gt;::new(0u8).unwrap());
let mut recorder = sync_histogram.recorder();
let _ = recorder.record_n_correct(10000000000000000000u64, 82u16, 1000u64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hdrhistogram::Histogram&lt;u16&gt;::sub_bucket_for&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:1702)
  hdrhistogram::Histogram&lt;u16&gt;::index_for&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:361)
  hdrhistogram::Histogram&lt;u16&gt;::mut_at&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:396)
  hdrhistogram::Histogram&lt;u16&gt;::record_n_inner&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:887)
  hdrhistogram::Histogram&lt;u16&gt;::record_n_correct&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:967)
  hdrhistogram::sync::{impl#6}::record_n_correct::{closure#0}&lt;u16&gt; (hdrhistogram_rust/src/sync/mod.rs:286)
  hdrhistogram::sync::Recorder&lt;u16&gt;::with_hist&lt;u16, hdrhistogram::sync::{impl#6}::record_n_correct::{closure_env#0}&lt;u16&gt;, core::result::Result&lt;(), hdrhistogram::errors::RecordError&gt;&gt; (hdrhistogram_rust/src/sync/mod.rs:168)
  hdrhistogram::sync::Recorder&lt;u16&gt;::record_n_correct&lt;u16&gt; (hdrhistogram_rust/src/sync/mod.rs:286)
  repro_timeout_in_hdrhistogram_sync_recorder_u16_record_n_correct_c46006::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let mut histogram = &lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new_with_max(
    1000000000000000000u64, 0u8).unwrap();
let _ = histogram.record_n_correct(4294967296u64, 1u64, 1u64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hdrhistogram::Histogram&lt;u64&gt;::update_min_max&lt;u64&gt; (hdrhistogram_rust/src/lib.rs:1806)
  hdrhistogram::Histogram&lt;u64&gt;::record_n_inner&lt;u64&gt; (hdrhistogram_rust/src/lib.rs:927)
  hdrhistogram::Histogram&lt;u64&gt;::record_n_correct&lt;u64&gt; (hdrhistogram_rust/src/lib.rs:967)
  repro_timeout_in_hdrhistogram_histogram_u64_record_n_correct_0353e7::main (src/main.rs:3)</pre></td></tr>
<tr><td valign="top"><pre>let _ = AdditionError::OtherAddendValueExceedsRange;
let histogram = &lt;hdrhistogram::Histogram&lt;u32&gt;&gt;::new(0u8).unwrap();
let sync_histogram = histogram.clone().into_sync();
let mut recorder = sync_histogram.recorder();
let _ = recorder.record_n_correct(u64::MAX, 1585395343u32, 1u64);
let _ = DeserializeError::EncodedArrayTooLong;</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hdrhistogram::Histogram&lt;u32&gt;::record_n_correct&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:967)
  hdrhistogram::sync::{impl#6}::record_n_correct::{closure#0}&lt;u32&gt; (hdrhistogram_rust/src/sync/mod.rs:286)
  hdrhistogram::sync::Recorder&lt;u32&gt;::with_hist&lt;u32, hdrhistogram::sync::{impl#6}::record_n_correct::{closure_env#0}&lt;u32&gt;, core::result::Result&lt;(), hdrhistogram::errors::RecordError&gt;&gt; (hdrhistogram_rust/src/sync/mod.rs:168)
  hdrhistogram::sync::Recorder&lt;u32&gt;::record_n_correct&lt;u32&gt; (hdrhistogram_rust/src/sync/mod.rs:286)
  repro_timeout_in_hdrhistogram_sync_recorder_u32_record_n_correct_57be24::main (src/main.rs:9)</pre></td></tr>
<tr><td valign="top"><pre>let histogram = &lt;hdrhistogram::Histogram&lt;u8&gt;&gt;::new_from::&lt;u16&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u16&gt;&gt;::new_from::&lt;u32&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u32&gt;&gt;::new_from::&lt;u64&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new_with_max(u64::MAX, 2u8).unwrap())));
let sync_histogram = histogram.clone().into_sync();
let mut recorder = sync_histogram.recorder();
let _ = recorder.record_n_correct(4380101545244474084u64, 26u8, 7u64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hdrhistogram::Histogram&lt;u8&gt;::index_for&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:382)
  hdrhistogram::Histogram&lt;u8&gt;::mut_at&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:396)
  hdrhistogram::Histogram&lt;u8&gt;::record_n_inner&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:887)
  hdrhistogram::Histogram&lt;u8&gt;::record_n_correct&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:967)
  hdrhistogram::sync::{impl#6}::record_n_correct::{closure#0}&lt;u8&gt; (hdrhistogram_rust/src/sync/mod.rs:286)
  hdrhistogram::sync::Recorder&lt;u8&gt;::with_hist&lt;u8, hdrhistogram::sync::{impl#6}::record_n_correct::{closure_env#0}&lt;u8&gt;, core::result::Result&lt;(), hdrhistogram::errors::RecordError&gt;&gt; (hdrhistogram_rust/src/sync/mod.rs:168)
  hdrhistogram::sync::Recorder&lt;u8&gt;::record_n_correct&lt;u8&gt; (hdrhistogram_rust/src/sync/mod.rs:286)
  repro_timeout_in_hdrhistogram_sync_recorder_u8_record_n_correct_e1bdaf::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let string = "(\\)".to_string();
let tag = Tag::new(&amp;string).unwrap();
let _ = tag.as_str();
let mut histogram = &lt;hdrhistogram::Histogram&lt;u16&gt;&gt;::new_from::&lt;u8&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u8&gt;&gt;::new_from::&lt;u32&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u32&gt; as ::core::convert::From&lt;hdrhistogram::
    Histogram&lt;u32&gt;&gt;&gt;::from(&lt;hdrhistogram::Histogram&lt;u32&gt;&gt;::new(
    0u8).unwrap())).clone_correct(256u64));
let _ = histogram.record_n_correct(u64::MAX, 56596u16, 82u64);
let _ = V2DeflateSerializeError::InternalSerializationError(
    V2SerializeError::UsizeTypeTooSmall);
let _ = AdditionError::ResizeFailedUsizeTypeTooSmall;</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hdrhistogram::Histogram&lt;u16&gt;::sub_bucket_for&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:1693)
  hdrhistogram::Histogram&lt;u16&gt;::index_for&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:361)
  hdrhistogram::Histogram&lt;u16&gt;::mut_at&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:396)
  hdrhistogram::Histogram&lt;u16&gt;::record_n_inner&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:887)
  hdrhistogram::Histogram&lt;u16&gt;::record_n_correct&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:967)
  repro_timeout_in_hdrhistogram_histogram_u16_record_n_correct_cce15e::main (src/main.rs:11)</pre></td></tr>
<tr><td valign="top"><pre>let mut histogram = &lt;hdrhistogram::Histogram&lt;u32&gt; as ::core::convert::
    From&lt;hdrhistogram::Histogram&lt;u32&gt;&gt;&gt;::from(
    &lt;hdrhistogram::Histogram&lt;u32&gt;&gt;::new_from::&lt;u64&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new_from::&lt;u32&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u32&gt;&gt;::new_from::&lt;u64&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new(1u8).unwrap()))));
let _ = histogram.record_correct(10000000000000000000u64, 2u64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hdrhistogram::Histogram&lt;u32&gt;::index_for&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:382)
  hdrhistogram::Histogram&lt;u32&gt;::mut_at&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:396)
  hdrhistogram::Histogram&lt;u32&gt;::record_n_inner&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:887)
  hdrhistogram::Histogram&lt;u32&gt;::record_n_correct&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:967)
  hdrhistogram::Histogram&lt;u32&gt;::record_correct&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:936)
  repro_timeout_in_hdrhistogram_histogram_u32_record_correct_4f8402::main (src/main.rs:3)</pre></td></tr>
<tr><td valign="top"><pre>let histogram = &lt;hdrhistogram::Histogram&lt;u8&gt;&gt;::new_from::&lt;u32&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u32&gt;&gt;::new_with_bounds(1u64,
    12164973431588361683u64, 2u8).unwrap());
let sync_histogram = histogram.clone().into_sync();
let mut recorder = sync_histogram.recorder();
let _ = recorder.record_correct(1000000000000000000u64, 1u64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hdrhistogram::Histogram&lt;u8&gt;::update_min_max&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:1809)
  hdrhistogram::Histogram&lt;u8&gt;::record_n_inner&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:927)
  hdrhistogram::Histogram&lt;u8&gt;::record_n_correct&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:967)
  hdrhistogram::Histogram&lt;u8&gt;::record_correct&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:936)
  hdrhistogram::sync::{impl#6}::record_correct::{closure#0}&lt;u8&gt; (hdrhistogram_rust/src/sync/mod.rs:276)
  hdrhistogram::sync::Recorder&lt;u8&gt;::with_hist&lt;u8, hdrhistogram::sync::{impl#6}::record_correct::{closure_env#0}&lt;u8&gt;, core::result::Result&lt;(), hdrhistogram::errors::RecordError&gt;&gt; (hdrhistogram_rust/src/sync/mod.rs:168)
  hdrhistogram::sync::Recorder&lt;u8&gt;::record_correct&lt;u8&gt; (hdrhistogram_rust/src/sync/mod.rs:276)
  repro_timeout_in_hdrhistogram_sync_recorder_u8_record_correct_57500b::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = UsizeTypeTooSmall;
let mut histogram = &lt;hdrhistogram::Histogram&lt;u8&gt;&gt;::new_from::&lt;u64&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new_from::&lt;u64&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new_with_bounds(7u64,
    13826826444047866644u64, 1u8).unwrap()));
let _ = histogram.record_correct(u64::MAX, 4294967296u64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hdrhistogram::Histogram&lt;u8&gt;::index_for&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:370)
  hdrhistogram::Histogram&lt;u8&gt;::mut_at&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:396)
  hdrhistogram::Histogram&lt;u8&gt;::record_n_inner&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:887)
  hdrhistogram::Histogram&lt;u8&gt;::record_n_correct&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:967)
  hdrhistogram::Histogram&lt;u8&gt;::record_correct&lt;u8&gt; (hdrhistogram_rust/src/lib.rs:936)
  repro_timeout_in_hdrhistogram_histogram_u8_record_correct_174cfe::main (src/main.rs:6)</pre></td></tr>
<tr><td valign="top"><pre>let histogram = &lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new_from::&lt;u32&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u32&gt;&gt;::new_from::&lt;u16&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u16&gt;&gt;::new(0u8).unwrap()));
let _ = IntervalLogWriterBuilder::default();
let _ = V2DeflateSerializer::default();
let mut histogram2 = &lt;hdrhistogram::Histogram&lt;u16&gt;&gt;::new_from::&lt;u64&gt;(
    &amp;histogram);
let _ = histogram2.record_correct(100000000000000000u64, 1u64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hdrhistogram::Histogram&lt;u16&gt;::record_n_inner&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:888)
  hdrhistogram::Histogram&lt;u16&gt;::record_n_correct&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:967)
  hdrhistogram::Histogram&lt;u16&gt;::record_correct&lt;u16&gt; (hdrhistogram_rust/src/lib.rs:936)
  repro_timeout_in_hdrhistogram_histogram_u16_record_correct_ed1af2::main (src/main.rs:9)</pre></td></tr>
<tr><td valign="top"><pre>let histogram = &lt;hdrhistogram::Histogram&lt;u32&gt;&gt;::new_from::&lt;u8&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u8&gt;&gt;::new_from::&lt;u64&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new_from::&lt;u8&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u8&gt;&gt;::new(1u8).unwrap())));
let histogram2 = histogram.clone_correct(0u64);
let mut histogram3 = histogram2.clone_correct(0u64);
let _ = histogram3.record_n_correct(u64::MAX, 255u32, 255u64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_traits::cast::{impl#6}::to_usize (num-traits/src/cast.rs:155)
  hdrhistogram::Histogram&lt;u32&gt;::index_for&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:383)
  hdrhistogram::Histogram&lt;u32&gt;::mut_at&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:396)
  hdrhistogram::Histogram&lt;u32&gt;::record_n_inner&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:887)
  hdrhistogram::Histogram&lt;u32&gt;::record_n_correct&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:967)
  repro_timeout_in_hdrhistogram_histogram_u32_record_n_correct_c1a047::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut histogram = &lt;hdrhistogram::Histogram&lt;u32&gt; as ::core::convert::
    From&lt;hdrhistogram::Histogram&lt;u32&gt;&gt;&gt;::from(
    &lt;hdrhistogram::Histogram&lt;u32&gt;&gt;::new(1u8).unwrap());
let _ = histogram.record(u64::MAX);
let _ = &lt;hdrhistogram::SyncHistogram&lt;u32&gt; as ::core::convert::
    From&lt;hdrhistogram::Histogram&lt;u32&gt;&gt;&gt;::from(histogram.clone_correct(2u64));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hdrhistogram::{impl#0}::mut_at::{closure#0}&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:397)
  hdrhistogram::Histogram&lt;u32&gt;::mut_at&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:397)
  hdrhistogram::Histogram&lt;u32&gt;::record_n_inner&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:887)
  hdrhistogram::Histogram&lt;u32&gt;::record_n_correct&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:967)
  hdrhistogram::Histogram&lt;u32&gt;::clone_correct&lt;u32&gt; (hdrhistogram_rust/src/lib.rs:431)
  repro_timeout_in_hdrhistogram_synchistogram_u32_from_hdrhistogram__b96ea0::main (src/main.rs:4)
  (3 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let histogram = &lt;hdrhistogram::Histogram&lt;u32&gt;&gt;::new_with_max(8u64,
    1u8).unwrap();
let _ = &lt;hdrhistogram::iterators::log::Iter&lt;u32&gt;&gt;::new(&amp;histogram, 1u64,
    0.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at hdrhistogram_rust/src/iterators/log.rs:29:9:
log_base must be &gt; 1.0
backtrace:
  &lt;hdrhistogram::iterators::log::Iter&lt;u32&gt;&gt;::new (hdrhistogram_rust/src/iterators/log.rs:29:9)
  repro_hdrhistogram_rust_src_iterators_log_rs_29_9_96fad7::main (src/main.rs:3:13)</pre></td></tr>
<tr><td valign="top"><pre>let histogram = &lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new(0u8).unwrap();
let _ = histogram.iter_quantiles(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at hdrhistogram_rust/src/iterators/quantile.rs:19:9:
Ticks per half distance must be &gt; 0
backtrace:
  &lt;hdrhistogram::iterators::quantile::Iter&lt;u64&gt;&gt;::new (hdrhistogram_rust/src/iterators/quantile.rs:19:9)
  &lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::iter_quantiles (hdrhistogram_rust/src/lib.rs:1050:9)
  repro_hdrhistogram_rust_src_iterators_quantile_rs_19_9_53319a::main (src/main.rs:3:23)</pre></td></tr>
<tr><td valign="top"><pre>let histogram = &lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::new_from::&lt;u16&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u16&gt;&gt;::new_from::&lt;u32&gt;(
    &amp;&lt;hdrhistogram::Histogram&lt;u32&gt;&gt;::new_with_max(10u64, 0u8).unwrap()));
let _ = histogram.iter_linear(0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at hdrhistogram_rust/src/iterators/linear.rs:21:9:
value_units_per_bucket must be &gt; 0
backtrace:
  &lt;hdrhistogram::iterators::linear::Iter&lt;u64&gt;&gt;::new (hdrhistogram_rust/src/iterators/linear.rs:21:9)
  &lt;hdrhistogram::Histogram&lt;u64&gt;&gt;::iter_linear (hdrhistogram_rust/src/lib.rs:1112:9)
  repro_hdrhistogram_rust_src_iterators_linear_rs_21_9_ed9892::main (src/main.rs:3:23)</pre></td></tr>
<tr><td valign="top"><pre>let histogram = &lt;hdrhistogram::Histogram&lt;u8&gt;&gt;::new(1u8).unwrap();
let _ = histogram.iter_log(0u64, 0.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at hdrhistogram_rust/src/iterators/log.rs:25:9:
value_units_per_bucket must be &gt; 0
backtrace:
  &lt;hdrhistogram::iterators::log::Iter&lt;u8&gt;&gt;::new (hdrhistogram_rust/src/iterators/log.rs:25:9)
  &lt;hdrhistogram::Histogram&lt;u8&gt;&gt;::iter_log (hdrhistogram_rust/src/lib.rs:1154:9)
  repro_hdrhistogram_rust_src_iterators_log_rs_25_9_ba9075::main (src/main.rs:3:23)</pre></td></tr>
</table>
