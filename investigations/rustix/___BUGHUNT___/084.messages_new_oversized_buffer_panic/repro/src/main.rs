//! Repro for candidate 084: `rustix::net::AncillaryDrain::parse` panics when
//! handed a real, caller-supplied buffer at or beyond `u32::MAX + 1` bytes, on
//! any target where the real C `msghdr.msg_controllen` field is narrower than
//! `usize` (a 32-bit `socklen_t`, e.g. BSD/macOS/Solaris - NOT Linux, where
//! `msg_controllen` is `size_t`/`usize`, so the conversion is always
//! infallible there).
//!
//! This is a single-participant repro (CONFIRMATION.md section 11 does not
//! require a second participant here): the "external input" is simply the
//! size of the buffer a real caller legitimately allocates and hands to the
//! real, unmodified public API `AncillaryDrain::parse`. No kernel syscall,
//! peer, or mocked boundary is involved at all - the panic fires purely from
//! `buf.len()`, before any ancillary-message content is ever read. Because
//! this target OS/libc combination (`x86_64-apple-darwin`) cannot be linked
//! or executed in this sandboxed environment (no macOS SDK/cross-linker; see
//! RESULT.md), this is run via Miri's cross-interpretation
//! (`cargo miri run --target x86_64-apple-darwin`), which interprets MIR
//! directly against the real, unmodified source and needs no native
//! execution or any FFI shim for this code path (no actual syscall is ever
//! made before the panic point).

use rustix::net::AncillaryDrain;

fn try_len(label: &str, size: usize) {
    println!("--- {label}: allocating {size} bytes ---");
    let mut buf: Vec<u8> = vec![0u8; size];
    println!("allocated. calling AncillaryDrain::parse...");
    // Safety: an all-zero buffer is valid "empty" ancillary data content-wise;
    // the length-conversion panic under test happens before any ancillary
    // message content is ever interpreted, so buffer *content* is irrelevant
    // here - only buffer *length* drives the bug.
    let drain = unsafe { AncillaryDrain::parse(&mut buf[..]) };
    // Deliberately do NOT iterate `drain` - this repro only probes the
    // length-conversion panic inside `Messages::new`/`AncillaryDrain::parse`
    // itself, not anything about iterating cmsg content (a separate,
    // unrelated concern - iterating a zeroed buffer hits a different,
    // already-known bug, candidate 027).
    std::mem::forget(drain);
    println!("{label}: NO PANIC during AncillaryDrain::parse construction");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("small") => try_len("small (16 bytes)", 16),
        Some("boundary_ok") => try_len("boundary_ok (u32::MAX bytes)", u32::MAX as usize),
        Some("boundary_panic") => {
            try_len("boundary_panic (u32::MAX + 1 bytes)", u32::MAX as usize + 1)
        }
        Some("far_over") => try_len("far_over (3 * u32::MAX bytes)", 3 * (u32::MAX as usize)),
        _ => try_len("default (u32::MAX + 1 bytes)", u32::MAX as usize + 1),
    }
}
