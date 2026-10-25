use image::ImageDecoder;
use std::io::Cursor;
fn main() {
    let (mut ok, mut err, mut pan) = (0, 0, 0);
    for p in std::env::args().skip(1) {
        let d = std::fs::read(&p).unwrap();
        let r = std::panic::catch_unwind(|| {
            let dec = jxl_image_rs_integration::JxlDecoder::new(Cursor::new(&d[..]))?;
            let (w, h) = dec.dimensions();
            if (w as u64) * (h as u64) > (1 << 24) { return Err(image::ImageError::Limits(image::error::LimitError::from_kind(image::error::LimitErrorKind::DimensionError))); }
            let mut buf = vec![0u8; dec.total_bytes() as usize];
            dec.read_image(&mut buf)
        });
        match r {
            Ok(Ok(())) => { ok += 1; }
            Ok(Err(e)) => { err += 1; if std::env::var("V").is_ok() { println!("ERR {p}: {e}"); } }
            Err(_) => { pan += 1; println!("PANIC {p}"); }
        }
    }
    println!("ok={ok} err={err} panics={pan}");
}
