use simdvar::{FuzzConfig, fuzz_decode};
use std::io::Write;
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = &args[0];
    std::fs::create_dir_all(out).unwrap();
    for p in &args[1..] {
        let d = std::fs::read(p).unwrap();
        let r = std::panic::catch_unwind(|| fuzz_decode(&d, FuzzConfig::default()));
        let name = std::path::Path::new(p).file_name().unwrap().to_string_lossy().to_string();
        match r {
            Ok(Ok(frames)) => {
                let mut f = std::fs::File::create(format!("{out}/{name}.f32")).unwrap();
                for fr in &frames { for ch in fr { for y in 0..ch.size().1 { for v in ch.row(y) { f.write_all(&v.to_le_bytes()).unwrap(); } } } }
            }
            Ok(Err(_)) => { std::fs::write(format!("{out}/{name}.err"), b"").unwrap(); }
            Err(_) => { std::fs::write(format!("{out}/{name}.panic"), b"").unwrap(); }
        }
    }
}
