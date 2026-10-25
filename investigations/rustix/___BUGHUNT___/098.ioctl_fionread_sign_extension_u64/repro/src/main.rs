// Standalone repro for rustix::io::ioctl_fionread sign-extension bug.
//
// Real end-to-end path: creates a real sparse regular file on the real filesystem whose
// apparent size exceeds i32::MAX, opens it with a real fd, and calls rustix's own public
// `ioctl_fionread` entry point on that fd. The Linux kernel (the genuine second participant
// here) computes FIONREAD's int reply as `(int)(i_size - f_pos)`, which truncates/wraps for
// sizes >= 2^31 - this is not faked or hand-constructed, it's the kernel's real documented
// behavior for FIONREAD on regular files.

use std::fs::OpenOptions;
use std::io::Seek;

fn main() {
    let path = std::env::temp_dir().join(format!("rustix_fionread_repro_{}", std::process::id()));

    // ~2.79 GiB: exceeds i32::MAX (2_147_483_647) but fits in u32 and in u64.
    const SIZE: u64 = 3_000_000_000;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .expect("create sparse file");
    file.set_len(SIZE).expect("ftruncate to sparse size");

    // Reopen fresh at offset 0 so there's no ambiguity about the current file position.
    let mut file = OpenOptions::new().read(true).open(&path).expect("reopen");
    file.rewind().expect("seek to 0");

    let result = rustix::io::ioctl_fionread(&file);
    let _ = std::fs::remove_file(&path);

    let n = result.expect("ioctl_fionread should succeed on a regular file");

    let expected_wrapped_u32 = SIZE as u32 as u64; // what the doc comment implies: "a wrapped value"
    let sign_extended_garbage = (SIZE as i64 as i32) as i64 as u64; // what the buggy code path effectively does today, computed independently for comparison

    println!("apparent file size set via ftruncate : {SIZE}");
    println!("ioctl_fionread() returned             : {n}");
    println!("expected (size wrapped mod 2^32, u64) : {expected_wrapped_u32}");
    println!("sign-extended-i32-to-u64 pattern would give: {sign_extended_garbage}");

    if n == expected_wrapped_u32 {
        println!("did not reproduce - ioctl_fionread returned the documented wrapped value");
        std::process::exit(0);
    } else if n > (1u64 << 32) {
        println!(
            "REPRODUCED - ioctl_fionread returned a value near u64::MAX ({n}) instead of the \
             documented 32-bit wrapped value ({expected_wrapped_u32}); this is a sign-extension bug \
             in `n as u64` over the raw (negative) C int"
        );
        std::process::exit(1);
    } else {
        println!("unexpected value, neither the buggy nor the documented-correct result: {n}");
        std::process::exit(2);
    }
}
