// Reproduction for: zune-inflate's `StreamingDecoder::decode_chunk` corrupts/fails decoding
// whenever a caller feeds it more than one chunk, following the exact usage pattern documented
// in `decode_chunk`'s own doc comment (streaming_decoder.rs, "# Usage" section).
//
// `StreamingBitStreamReader::position` is never reset to 0 between chunks (only `is_final`/the
// bit buffer are updated), yet `refill()` indexes into whatever fresh chunk slice the caller
// passes for THIS call, which always starts back at index 0. After the first chunk, `position`
// stays stuck at a stale offset from the previous (now-gone) chunk, so `refill()` silently stops
// pulling in any further bytes - the decoder starves and returns a spurious `Error` (or, in a
// worse case not hit by this minimal repro, could silently truncate output).
//
// Exit code convention: 0 = did not reproduce (decoded correctly), 1 = reproduced (decode
// error or wrong output).
use zune_inflate::{DecodeStatus, StreamingDecoder};

fn run_streaming(
    uncompressed: &[u8], compressed: &[u8], chunk_size: usize, initial_out: usize,
) -> Result<(), String> {
    let mut decoder = StreamingDecoder::new();
    let mut out_vec = vec![0u8; initial_out];
    let mut input_pos = 0usize;

    loop {
        let end = (input_pos + chunk_size.max(1)).min(compressed.len());
        let chunk = &compressed[input_pos..end];
        let is_final = end == compressed.len();

        match decoder.decode_chunk(chunk, is_final, &mut out_vec) {
            DecodeStatus::NeedsMoreInput => {
                if is_final {
                    return Err("NeedsMoreInput on the final chunk".to_string());
                }
                input_pos = end;
            }
            DecodeStatus::NeedsMoreOutput { at_least } => {
                out_vec.resize(out_vec.len() + at_least + 1024, 0);
                // do NOT advance input_pos - same chunk, per the documented contract
            }
            DecodeStatus::Finished => {
                if &out_vec[..decoder.current_dest_offset()] != uncompressed {
                    return Err("decoded output does not match original input".to_string());
                }
                return Ok(());
            }
            DecodeStatus::Error(e) => {
                return Err(format!("decoder returned Error: {:?}", e));
            }
            other => return Err(format!("unexpected status: {:?}", other)),
        }
    }
}

fn main() {
    // A single byte of input, compressed with a real deflate encoder (miniz_oxide, independent
    // of zune-inflate), then fed to the streaming decoder ONE COMPRESSED BYTE AT A TIME - this
    // is exactly the chunking pattern `decode_chunk`'s own doc comment says is valid ("Any chunk
    // size from 1 byte upward is valid ... do not affect correctness").
    let data: Vec<u8> = vec![0u8];
    let compressed = miniz_oxide::deflate::compress_to_vec(&data, 6);

    println!("input: {} byte(s), compressed to {} byte(s)", data.len(), compressed.len());

    match run_streaming(&data, &compressed, 1, data.len()) {
        Ok(()) => {
            println!("did not reproduce - decoded correctly via 1-byte-at-a-time chunks");
            std::process::exit(0);
        }
        Err(e) => {
            println!("REPRODUCED - streaming decode with 1-byte chunks failed: {e}");
            std::process::exit(1);
        }
    }
}
