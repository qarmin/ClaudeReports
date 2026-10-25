# symphonia - crashes

Source: https://github.com/pdeljanov/Symphonia @ ee35874b571a35a9a6e15d3bc9a3aaf8f11fbeee

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = WavReader::try_new({ let bytes = include_bytes!(
    "data_0.bin").to_vec(); symphonia_core::io::MediaSourceStream::new(Box::new(
    std::io::Cursor::new(bytes)), Default::default()) },
    { symphonia_core::formats::FormatOptions::default() });</pre></td><td valign="top"><pre>thread 'main' panicked at symphonia-format-riff/src/wave/chunks.rs:100:36:
attempt to multiply with overflow
backtrace:
  &lt;symphonia_format_riff::wave::chunks::WaveFormatChunk&gt;::read_pcm_fmt::&lt;symphonia_core::io::media_source_stream::MediaSourceStream&gt; (symphonia-format-riff/src/wave/chunks.rs:100:36)
  &lt;symphonia_format_riff::wave::chunks::WaveFormatChunk as symphonia_format_riff::common::ParseChunk&gt;::parse::&lt;symphonia_core::io::media_source_stream::MediaSourceStream&gt; (symphonia-format-riff/src/wave/chunks.rs:439:17)
  &lt;symphonia_format_riff::common::ChunkParser&lt;symphonia_format_riff::wave::chunks::WaveFormatChunk&gt;&gt;::parse::&lt;symphonia_core::io::media_source_stream::MediaSourceStream&gt; (symphonia-format-riff/src/common.rs:184:9)
  &lt;symphonia_format_riff::wave::WavReader&gt;::try_new (symphonia-format-riff/src/wave/mod.rs:113:38)
  repro_symphonia_format_riff_src_wave_chunks_rs_100_36_14ca92::main (src/main.rs:4:13)</pre></td></tr>
</table>
