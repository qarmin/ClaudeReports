use rustix::fs::{getxattr, listxattr, setxattr, XattrFlags};
use std::fs::File;
use std::io::Write;
use std::mem::MaybeUninit;

fn setup(attr_val: &[u8]) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("rustix_xattr_repro3_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("f{}", attr_val.len()));
    let mut f = File::create(&path).unwrap();
    f.write_all(b"hello").unwrap();
    setxattr(&path, "user.test", attr_val, XattrFlags::empty()).expect("setxattr failed");
    path
}

fn main() {
    let mut failures = 0;

    // Case 1: zero-length buffer against a 64-byte attribute (original trigger)
    let path = setup(&vec![b'A'; 64]);
    let mut buf: [u8; 0] = [];
    let n = getxattr(&path, "user.test", &mut buf[..]).unwrap();
    println!("case1 (64-byte attr, 0-len buf)  -> {}", n);
    if n != 0 { failures += 1; }

    // Case 2: zero-length buffer against a 1-byte attribute (boundary near 0)
    let path = setup(&[b'Z']);
    let mut buf: [u8; 0] = [];
    let n = getxattr(&path, "user.test", &mut buf[..]).unwrap();
    println!("case2 (1-byte attr, 0-len buf)   -> {}", n);
    if n != 0 { failures += 1; }

    // Case 3: zero-length buffer against a larger (1024-byte) attribute
    let path = setup(&vec![b'Q'; 1024]);
    let mut buf: [u8; 0] = [];
    let n = getxattr(&path, "user.test", &mut buf[..]).unwrap();
    println!("case3 (1024-byte attr, 0-len buf)-> {}", n);
    if n != 0 { failures += 1; }

    // Case 4: normal-sized, adequate buffer still gets the real data (regression check)
    let path = setup(&vec![b'X'; 32]);
    let mut buf = [0u8; 64];
    let n = getxattr(&path, "user.test", &mut buf[..]).unwrap();
    println!("case4 (32-byte attr, 64-len buf) -> {} bytes, data correct: {}", n, &buf[..n] == &vec![b'X'; 32][..]);
    if n != 32 || &buf[..n] != &vec![b'X'; 32][..] { failures += 1; }

    // Case 5: MaybeUninit zero-length buffer no longer panics
    let path = setup(&vec![b'M'; 16]);
    let result = std::panic::catch_unwind(|| {
        let mut buf2: [MaybeUninit<u8>; 0] = [];
        let r = getxattr(&path, "user.test", &mut buf2[..]).unwrap();
        r.0.len()
    });
    let ok = matches!(result, Ok(0));
    println!("case5 (MaybeUninit 0-len buf)    -> ok={}", ok);
    if !ok { failures += 1; }

    // Case 6: listxattr zero-length buffer
    let path = setup(&vec![b'L'; 8]);
    let mut lbuf: [u8; 0] = [];
    let ln = listxattr(&path, &mut lbuf[..]).unwrap();
    println!("case6 (listxattr 0-len buf)      -> {}", ln);
    if ln != 0 { failures += 1; }

    if failures > 0 {
        println!("BUG PRESENT: {} case(s) show a bogus/OOB-claiming Output for a 0-length buffer (expected 0 in every case)", failures);
    } else {
        println!("NO BUG: all cases returned 0 for a 0-length buffer, as the API contract requires");
    }
}
