use jxl_fuzz_formats::{FuzzConfig, fuzz_decode};
fn main() {
    let mut ok = 0;
    let mut err = 0;
    let mut panics = 0;
    for p in std::env::args().skip(1) {
        let d = std::fs::read(&p).unwrap();
        let r = std::panic::catch_unwind(|| fuzz_decode(&d, FuzzConfig::default()).map(|f| f.len()));
        match r {
            Ok(Ok(n)) => { ok += 1; println!("OK {p} frames={n}"); }
            Ok(Err(_)) => { err += 1; println!("ERR {p}"); }
            Err(_) => { panics += 1; println!("PANIC {p}"); }
        }
    }
    println!("ok={ok} err={err} panics={panics}");
}
