# timeout in gif::streaming_decoder::FrameDecoder::decode_lzw_encoded_frame_into_buffer

- crate: https://github.com/image-rs/image-gif @ a5f89ddc2b29424334244c97456b4a26e208aa7d
- kind: timeout
- last API call: `gif::streaming_decoder::FrameDecoder::decode_lzw_encoded_frame_into_buffer`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling gif::streaming_decoder::FrameDecoder::decode_lzw_encoded_frame_into_buffer
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
    let vec = vec![];
    let mut frame = Frame::from_grayscale_with_alpha(0u16, 0u16, &vec);
    frame.make_lzw_pre_encoded();
    let mut frame_decoder = FrameDecoder::new(DecodeOptions::new());
    let _ = frame_decoder.decode_lzw_encoded_frame_into_buffer(&frame, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_gif_streaming_decoder_framedecoder_decode_lzw_enc_b10e93`).

