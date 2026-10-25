use rustix::net::{bind, getsockname, socket, AddressFamily, SocketAddrUnix, SocketType};
use std::convert::TryInto;

fn main() {
    let mut panics = 0usize;
    let mut oks = 0usize;

    // Sweep abstract-name lengths 0..=108 (107 is the real max: sun_path has
    // 108 bytes, byte 0 is the abstract-name marker, leaving 107 usable; 108
    // should be rejected at construction time with NAMETOOLONG).
    for len in 0..=108usize {
        let name: Vec<u8> = (0..len).map(|i| b'a' + (i % 26) as u8).collect();
        let addr = match SocketAddrUnix::new_abstract_name(&name) {
            Ok(a) => a,
            Err(e) => {
                println!("abstract len={len} construct error: {e:?} (expected only at len=108, NAMETOOLONG)");
                continue;
            }
        };
        let sock = socket(AddressFamily::UNIX, SocketType::DGRAM, None).unwrap();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            bind(&sock, &addr).unwrap();
            let any = getsockname(&sock).unwrap();
            let unix: SocketAddrUnix = any.try_into().unwrap();
            unix
        }));
        match r {
            Ok(got) => {
                oks += 1;
                assert_eq!(
                    got.abstract_name(),
                    Some(&name[..]),
                    "roundtrip mismatch at abstract len={len}"
                );
            }
            Err(_) => {
                panics += 1;
                println!("abstract len={len} PANICKED round-tripping through real bind+getsockname");
            }
        }
    }

    // Sweep filesystem-path lengths at the real boundary (each length gets
    // its own directory, so no bind()/leftover-file collisions between
    // iterations). Skip the exact-107 case (path.to_bytes_with_nul() fits
    // sun_path exactly, a pre-existing path_bytes() ambiguity unrelated to
    // this candidate's OOB-indexing/assert-panic question) - we only care
    // here about whether bind+getsockname ever PANICS, not exact-roundtrip
    // byte equality.
    for len in [0usize, 1, 50, 100, 106, 107] {
        let dir = std::env::temp_dir().join(format!("sockaddr_sweep_{}_{}", std::process::id(), len));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let base = dir.to_str().unwrap();
        let fname_len = len.saturating_sub(base.len() + 1);
        let path = format!("{}/{}", base, "f".repeat(fname_len));
        if path.len() != len {
            println!("fs path len={len}: can't hit this exact length in this environment (base too long), skipping");
            let _ = std::fs::remove_dir_all(&dir);
            continue;
        }
        let addr = match SocketAddrUnix::new(&path) {
            Ok(a) => a,
            Err(e) => {
                println!("fs path len={len} (actual {}) construct error: {e:?}", path.len());
                let _ = std::fs::remove_dir_all(&dir);
                continue;
            }
        };
        let sock = socket(AddressFamily::UNIX, SocketType::STREAM, None).unwrap();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            bind(&sock, &addr).unwrap();
            let any = getsockname(&sock).unwrap();
            let _unix: SocketAddrUnix = any.try_into().unwrap();
        }));
        match r {
            Ok(()) => {
                oks += 1;
            }
            Err(_) => {
                panics += 1;
                println!("fs path len={len} PANICKED round-tripping through real bind+getsockname");
            }
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    println!("TOTAL: oks={oks} panics={panics}");
    if panics > 0 {
        std::process::exit(1);
    }
}
