# gif - crashes

Source: https://github.com/image-rs/image-gif @ a5f89ddc2b29424334244c97456b4a26e208aa7d

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let decode_options = DecodeOptions::new();
let mut frame = Frame::from_indexed_pixels(0u16, 0u16, vec![], None);
frame.make_lzw_pre_encoded();
let mut frame_decoder = FrameDecoder::new(decode_options.clone());
let _ = frame_decoder.decode_lzw_encoded_frame(&amp;mut frame);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  weezl::decode::Decoder::decode_bytes (src/decode.rs:340)
  gif::reader::decoder::LzwReader::decode_bytes (src/reader/decoder.rs:336)
  gif::reader::decoder::{impl#8}::decode_lzw_encoded_frame_into_buffer::{closure#0} (src/reader/decoder.rs:262)
  gif::reader::converter::PixelConverter::fill_buffer (src/reader/converter.rs:125)
  gif::reader::converter::PixelConverter::read_into_buffer (src/reader/converter.rs:232)
  gif::reader::decoder::FrameDecoder::decode_lzw_encoded_frame_into_buffer (src/reader/decoder.rs:261)
  gif::reader::decoder::FrameDecoder::decode_lzw_encoded_frame (image-gif/src/reader/decoder.rs:243)
  repro_timeout_in_gif_streaming_decoder_framedecoder_decode_lzw_enc_4604fc::main (src/main.rs:10)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let mut frame = Frame::from_grayscale_with_alpha(0u16, 0u16, &amp;vec);
frame.make_lzw_pre_encoded();
let mut frame_decoder = FrameDecoder::new(DecodeOptions::new());
let _ = frame_decoder.decode_lzw_encoded_frame_into_buffer(&amp;frame, &amp;mut vec![]);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  gif::reader::decoder::LzwReader::decode_bytes (src/reader/decoder.rs:349)
  gif::reader::decoder::{impl#8}::decode_lzw_encoded_frame_into_buffer::{closure#0} (src/reader/decoder.rs:262)
  gif::reader::converter::PixelConverter::fill_buffer (src/reader/converter.rs:125)
  gif::reader::converter::PixelConverter::read_into_buffer (src/reader/converter.rs:232)
  gif::reader::decoder::FrameDecoder::decode_lzw_encoded_frame_into_buffer (src/reader/decoder.rs:261)
  repro_timeout_in_gif_streaming_decoder_framedecoder_decode_lzw_enc_b10e93::main (src/main.rs:10)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut streaming_decoder = StreamingDecoder::new();
let _ = streaming_decoder.current_frame_mut();</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:5:31:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;gif::reader::decoder::StreamingDecoder&gt;::current_frame_mut (image-gif/src/reader/decoder.rs:507:31)
  repro_image_gif_src_reader_decoder_rs_507_31_27137a::main (src/main.rs:5:31)</pre></td></tr>
<tr><td valign="top"><pre>let streaming_decoder = StreamingDecoder::new();
let _ = streaming_decoder.current_frame();</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:5:31:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;gif::reader::decoder::StreamingDecoder&gt;::current_frame (image-gif/src/reader/decoder.rs:514:31)
  repro_image_gif_src_reader_decoder_rs_514_31_226ac2::main (src/main.rs:5:31)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let mut frame = Frame::from_grayscale_with_alpha(0u16, 0u16, &amp;vec);
frame.make_lzw_pre_encoded();
let mut encoder = &lt;gif::Encoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(vec![], 0u16, 0u16,
    &amp;vec![]).unwrap();
let _ = encoder.write_frame(&amp;frame);</pre></td><td valign="top"><pre>thread 'main' panicked at image-gif/src/encoder.rs:208:9:
the frame has 0 pixels, but non-empty buffer
backtrace:
  &lt;gif::encoder::Encoder&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::write_frame (image-gif/src/encoder.rs:208:9)
  repro_image_gif_src_encoder_rs_208_9_b72980::main (src/main.rs:8:21)</pre></td></tr>
</table>
