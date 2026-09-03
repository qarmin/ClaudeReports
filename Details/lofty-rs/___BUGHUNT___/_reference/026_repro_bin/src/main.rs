use std::io::Cursor;
use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::mp4::{Atom, AtomData, AtomIdent, DataType, Ilst, Mp4File};

fn main() {
    let raw = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();
    let control = raw[0];
    let file_bytes = &raw[1..];

    let mut cursor = Cursor::new(file_bytes.to_vec());
    let mut mp4 = Mp4File::read_from(&mut cursor, ParseOptions::new()).unwrap();

    if control & 1 == 0 {
        let mut tag = Ilst::default();
        let n = usize::from(control >> 1) % 8;
        let len = file_bytes.len().max(1);
        for i in 0..n {
            let start = (i.saturating_mul(7)) % len;
            let end = (start + 24).min(file_bytes.len());
            let chunk = file_bytes.get(start..end).unwrap_or(&[]);
            let s = String::from_utf8_lossy(chunk).into_owned();

            let fourccs: [[u8; 4]; 4] = [*b"\xa9nam", *b"\xa9ART", *b"\xa9alb", *b"covr"];
            let fourcc = fourccs[i % fourccs.len()];

            if fourcc == *b"covr" {
                tag.insert(Atom::new(
                    AtomIdent::Fourcc(fourcc),
                    AtomData::Unknown { code: DataType::Other(13), data: chunk.to_vec() },
                ));
            } else {
                tag.insert(Atom::new(AtomIdent::Fourcc(fourcc), AtomData::UTF8(s)));
            }
        }
        mp4.set_ilst(tag);
    } else {
        mp4.set_ilst(Ilst::default());
    }

    let mut out = Cursor::new(file_bytes.to_vec());
    mp4.save_to(&mut out, WriteOptions::default()).unwrap();
    println!("save_to succeeded, out len = {}", out.into_inner().len());
}
