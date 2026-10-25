// Reproduction for candidate 083: align_for_cmsghdr out-of-bounds slice panic.
//
// `align_for_cmsghdr` (src/net/send_recv/msg.rs) rounds a caller-supplied buffer's start address
// up to `align_of::<cmsghdr>()` and then slices the buffer by that many bytes of padding, without
// checking the padding needed against the buffer's actual length. A short, non-empty buffer whose
// runtime address happens to need more alignment padding than it is long causes the slice index
// `&mut buffer[adjusted - addr..]` to panic with "range start index out of range".
//
// This drives the real public API (`SendAncillaryBuffer::new`) with a real address: we don't
// reimplement `align_for_cmsghdr` ourselves, we just supply buffers at every possible alignment
// offset (any `align_of::<cmsghdr>()` in practice is <= 16) until one of them lands at the
// misaligned offset that trips the bug - exactly the situation a caller with a small
// stack/heap-allocated buffer can hit by chance, just made deterministic here instead of
// address-dependent.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

use rustix::net::SendAncillaryBuffer;
use std::mem::MaybeUninit;

// Force a base address more aligned than any plausible `align_of::<cmsghdr>()`, so every
// sub-offset 0..32 is reachable and the "needed padding" for each offset is exactly the offset
// itself (mod align).
#[repr(align(32))]
struct AlignedStorage([MaybeUninit<u8>; 64]);

fn main() {
    let mut storage = AlignedStorage([MaybeUninit::uninit(); 64]);
    let base_addr = storage.0.as_ptr() as usize;
    assert_eq!(base_addr % 32, 0, "backing storage must be 32-byte aligned");

    // Use a short buffer length (2 bytes) - shorter than any alignment padding from 3 up to 16.
    const LEN: usize = 2;

    let mut reproduced = false;

    for offset in 0..32usize {
        // Buffer length 2, starting at `base + offset`, i.e. address mod 32 == offset.
        if offset + LEN > storage.0.len() {
            break;
        }
        let buf: &mut [MaybeUninit<u8>] = &mut storage.0[offset..offset + LEN];
        let addr = buf.as_ptr() as usize;

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _cmsg_buffer = SendAncillaryBuffer::new(buf);
        }));

        match result {
            Ok(()) => {
                println!(
                    "offset {offset:2} (addr % 32 = {}): OK, no panic",
                    addr % 32
                );
            }
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| payload.downcast_ref::<&str>().copied())
                    .unwrap_or("<non-string panic payload>");
                println!(
                    "offset {offset:2} (addr % 32 = {}): REPRODUCED panic: {msg}",
                    addr % 32
                );
                reproduced = true;
            }
        }
    }

    if reproduced {
        eprintln!(
            "REPRODUCED - SendAncillaryBuffer::new panicked with an out-of-bounds slice index \
             for at least one alignment offset of a {LEN}-byte buffer"
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - no offset in 0..32 triggered a panic");
    }
}
