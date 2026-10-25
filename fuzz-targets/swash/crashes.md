# swash - crashes

Source: https://github.com/dfrg/swash @ 7773843df0d63cd468db61a29c152b5e7a99d4ab

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let vec = vec![255u8, 216u8, 255u8, 224u8, 0u8, 16u8, 74u8, 70u8, 73u8, 70u8,
    0u8, 1u8, 2u8, 0u8, 0u8, 1u8, 0u8, 1u8, 0u8, 0u8];
let font_ref = FontRef::from_offset(&amp;vec, 16u32).unwrap();
let vec2 = vec![];
let glyph_metrics = font_ref.glyph_metrics(&amp;vec2);
let _ = glyph_metrics.advance_width(0u16);</pre></td><td valign="top"><pre>thread 'main' panicked at swash/src/internal/xmtx.rs:14:9:
attempt to subtract with overflow
backtrace:
  swash::internal::xmtx::advance (swash/src/internal/xmtx.rs:14:9)
  &lt;swash::metrics::GlyphMetrics&gt;::advance_width (swash/src/metrics.rs:381:21)
  repro_swash_src_internal_xmtx_rs_14_9_316f20::main (src/main.rs:8:27)</pre></td></tr>
</table>
