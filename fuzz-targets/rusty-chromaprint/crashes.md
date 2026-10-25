# rusty-chromaprint - crashes

Source: https://github.com/darksv/rusty-chromaprint @ b1524bf2cf25c892931c0df59aa3226ba4ad651d

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let configuration = Configuration::preset_test3();
let _ = Fingerprinter::new(&amp;configuration.clone().with_frame_size(
    1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  rusty_chromaprint::chroma::Chroma&lt;rusty_chromaprint::chroma_filter::ChromaFilter&lt;rusty_chromaprint::chroma_normalizer::ChromaNormalizer&lt;rusty_chromaprint::fingerprint_calculator::FingerprintCalculator&gt;&gt;&gt;::new&lt;rusty_chromaprint::chroma_filter::ChromaFilter&lt;rusty_chromaprint::chroma_normalizer::Chro... (+99 chars)
  rusty_chromaprint::fingerprinter::Fingerprinter::new (src/fingerprinter.rs:185)
  repro_out_of_memory_in_rusty_chromaprint_fingerprinter_new_c2ef4f::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut fingerprinter = Fingerprinter::new(&amp;Configuration::preset_test1());
fingerprinter.consume(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at chromaprint/src/audio_processor.rs:180:20:
attempt to calculate the remainder with a divisor of zero
backtrace:
  &lt;rusty_chromaprint::audio_processor::AudioProcessor&lt;alloc::boxed::Box&lt;dyn rusty_chromaprint::stages::AudioConsumer&lt;f64, Output = [u32]&gt;&gt;&gt; as rusty_chromaprint::stages::AudioConsumer&gt;::consume (chromaprint/src/audio_processor.rs:180:20)
  &lt;rusty_chromaprint::fingerprinter::Fingerprinter&gt;::consume (chromaprint/src/fingerprinter.rs:208:24)
  repro_chromaprint_src_audio_processor_rs_180_20_5f72c9::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let mut fingerprinter = Fingerprinter::new(&amp;Configuration::default());
let _ = fingerprinter.start(1000000000u32, 10000u32);
fingerprinter.consume(&amp;[0i16]);</pre></td><td valign="top"><pre>thread 'main' panicked at chromaprint/src/audio_processor.rs:180:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;rusty_chromaprint::audio_processor::AudioProcessor&lt;alloc::boxed::Box&lt;dyn rusty_chromaprint::stages::AudioConsumer&lt;f64, Output = [u32]&gt;&gt;&gt; as rusty_chromaprint::stages::AudioConsumer&gt;::consume (chromaprint/src/audio_processor.rs:180:9)
  &lt;rusty_chromaprint::fingerprinter::Fingerprinter&gt;::consume (chromaprint/src/fingerprinter.rs:208:24)
  repro_chromaprint_src_audio_processor_rs_180_9_42a924::main (src/main.rs:7:19)</pre></td></tr>
<tr><td valign="top"><pre>let configuration = Configuration::preset_test3();
let configuration2 = configuration.clone().with_frame_overlap(10000usize);
let configuration3 = configuration2.clone().with_interpolation(false);
let segment = Segment { offset1: 0usize, offset2: 0usize, items_count: 0usize,
    score: 0.0f64 };
let _ = segment.start1(&amp;configuration3);</pre></td><td valign="top"><pre>thread 'main' panicked at chromaprint/src/fingerprinter.rs:139:9:
attempt to subtract with overflow
backtrace:
  &lt;rusty_chromaprint::fingerprinter::Configuration&gt;::samples_in_item (chromaprint/src/fingerprinter.rs:139:9)
  &lt;rusty_chromaprint::fingerprinter::Configuration&gt;::item_duration_in_seconds (chromaprint/src/fingerprinter.rs:149:14)
  &lt;rusty_chromaprint::fingerprint_matcher::Segment&gt;::start1 (chromaprint/src/fingerprint_matcher.rs:212:16)
  repro_chromaprint_src_fingerprinter_rs_139_9_7139ec::main (src/main.rs:9:21)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Fingerprinter::new(&amp;Configuration::preset_test3().with_frame_size(
    10000000000000000000usize));</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:51:31)
capacity overflow
backtrace:
  &lt;u8 as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:51:31)
  &lt;rusty_chromaprint::chroma::Chroma&lt;rusty_chromaprint::chroma_filter::ChromaFilter&lt;rusty_chromaprint::chroma_normalizer::ChromaNormalizer&lt;rusty_chromaprint::fingerprint_calculator::FingerprintCalculator&gt;&gt;&gt;&gt;::new (chromaprint/src/chroma.rs:25:20)
  &lt;rusty_chromaprint::fingerprinter::Fingerprinter&gt;::new (chromaprint/src/fingerprinter.rs:185:22)
  repro_chromaprint_src_chroma_rs_25_20_1977cb::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let configuration = Configuration::preset_test2();
let configuration2 = configuration.clone().with_frame_size(
    1000000000000000000usize);
let _ = configuration2.delay();</pre></td><td valign="top"><pre>thread 'main' panicked at chromaprint/src/fingerprinter.rs:154:9:
attempt to multiply with overflow
backtrace:
  &lt;rusty_chromaprint::fingerprinter::Configuration&gt;::delay (chromaprint/src/fingerprinter.rs:154:9)
  repro_chromaprint_src_fingerprinter_rs_154_9_fc8f46::main (src/main.rs:6:28)</pre></td></tr>
</table>
