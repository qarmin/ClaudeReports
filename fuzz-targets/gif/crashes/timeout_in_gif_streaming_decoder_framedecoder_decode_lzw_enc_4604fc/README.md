# timeout in gif::streaming_decoder::FrameDecoder::decode_lzw_encoded_frame

- crate: https://github.com/image-rs/image-gif @ a5f89ddc2b29424334244c97456b4a26e208aa7d
- kind: timeout
- last API call: `gif::streaming_decoder::FrameDecoder::decode_lzw_encoded_frame`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling gif::streaming_decoder::FrameDecoder::decode_lzw_encoded_frame
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gif = { git = "https://github.com/image-rs/image-gif", rev = "a5f89ddc2b29424334244c97456b4a26e208aa7d" }
```

`src/main.rs`:

```rust
use gif::DecodeOptions;
use gif::Frame;
use gif::streaming_decoder::FrameDecoder;

fn main() {
    let decode_options = DecodeOptions::new();
    let mut frame = Frame::from_indexed_pixels(0u16, 0u16, vec![], None);
    frame.make_lzw_pre_encoded();
    let mut frame_decoder = FrameDecoder::new(decode_options.clone());
    let _ = frame_decoder.decode_lzw_encoded_frame(&mut frame);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_gif_streaming_decoder_framedecoder_decode_lzw_enc_4604fc`).

