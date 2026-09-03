struct NullDecoder;

impl hayro_jbig2::Decoder for NullDecoder {
    fn push_pixel(&mut self, _black: bool) {}
    fn push_pixel_chunk(&mut self, _black: bool, _chunk_count: u32) {}
    fn next_line(&mut self) {}
}

fn main() {
    let dir = std::env::args().nth(1).expect("usage: sweep <dir>");
    let mut panicked = 0;
    let mut ok = 0;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if !path.is_file() {
            continue;
        }
        let data = std::fs::read(&path).unwrap();
        let result = std::panic::catch_unwind(|| {
            if let Ok(image) = hayro_jbig2::Image::new(&data) {
                let mut decoder = NullDecoder;
                let _ = image.decode(&mut decoder);
            }
        });
        match result {
            Ok(_) => ok += 1,
            Err(e) => {
                panicked += 1;
                eprintln!("PANIC on {:?}: {:?}", path.file_name().unwrap(), e);
            }
        }
    }
    println!("ok={ok} panicked={panicked}");
}
