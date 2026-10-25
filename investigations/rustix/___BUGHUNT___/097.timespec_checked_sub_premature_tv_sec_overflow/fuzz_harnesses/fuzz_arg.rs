#![no_main]

use libfuzzer_sys::fuzz_target;
use rustix::net::SocketAddrUnix;
use rustix::path::{Arg, DecInt};

// Shallow, one-off harness (per bug-hunt's own scope) exercising rustix's
// pure-computation public API surface that handles untrusted byte input
// without making a real syscall: path/CStr construction (`Arg`), Unix socket
// address construction (`SocketAddrUnix`), and decimal-integer path
// formatting (`DecInt`). None of these reach a real syscall, so they're
// fuzzable directly (unlike most of linux_raw's inline-asm backend).
fuzz_target!(|data: &[u8]| {
    // `path::Arg` / `with_c_str`: exercises the stack-buffer fast path
    // (SMALL_PATH_BUFFER_SIZE = 256) and the slow/allocating path, including
    // right at the boundary and with embedded NUL bytes.
    let _ = data.into_with_c_str(|_cstr| Ok(()));

    // `SocketAddrUnix::new`: bounded-write into a fixed 108-byte sun_path.
    let _ = SocketAddrUnix::new(data);

    // `SocketAddrUnix::new_abstract_name`: separate bounds-check path.
    let _ = SocketAddrUnix::new_abstract_name(data);

    // `DecInt::new` over all integer widths/signs, using the first few
    // bytes of input (if present) to build boundary-adjacent values.
    if data.len() >= 8 {
        let v = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let _ = DecInt::new(v);
        let _ = DecInt::new(v as i64);
        let _ = DecInt::new(v as u32);
        let _ = DecInt::new(v as i32);
        let _ = DecInt::new(u64::MAX.wrapping_sub(v % 4));
        let _ = DecInt::new(i64::MIN.wrapping_add((v % 4) as i64));
    }
});
